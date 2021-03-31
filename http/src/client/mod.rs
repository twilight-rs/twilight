mod builder;

pub use self::builder::ClientBuilder;

use crate::{
    api_error::{ApiError, ErrorCode},
    error::{Error, Result},
    ratelimiting::{RatelimitHeaders, Ratelimiter},
    request::applications::InteractionCallback,
    request::{
        applications::{
            CreateGlobalCommand, CreateGuildCommand, DeleteGlobalCommand, DeleteGuildCommand,
            GetGlobalCommands, GetGuildCommands, InteractionError, SetGlobalCommands,
            SetGuildCommands, UpdateGlobalCommand, UpdateGuildCommand,
        },
        channel::allowed_mentions::AllowedMentions,
        guild::{create_guild::CreateGuildError, create_guild_channel::CreateGuildChannelError},
        prelude::*,
        GetUserApplicationInfo, Request,
    },
    API_VERSION,
};

use bytes::Bytes;
use hyper::{
    body::{self, Buf},
    client::{Client as HyperClient, HttpConnector},
    header::{HeaderMap, HeaderValue, AUTHORIZATION, CONTENT_LENGTH, CONTENT_TYPE, USER_AGENT},
    Body, Method, Response, StatusCode,
};
use serde::de::DeserializeOwned;
use std::{
    convert::TryFrom,
    fmt::{Debug, Formatter, Result as FmtResult},
    result::Result as StdResult,
    sync::{
        atomic::{AtomicBool, AtomicU64, Ordering},
        Arc,
    },
    time::Duration,
};
use tokio::time;

use twilight_model::{
    applications::{command::Command, response::InteractionResponse},
    guild::Permissions,
    id::{
        ApplicationId, ChannelId, CommandId, EmojiId, GuildId, IntegrationId, InteractionId,
        MessageId, RoleId, UserId, WebhookId,
    },
};

#[cfg(feature = "hyper-rustls")]
type HttpsConnector<T> = hyper_rustls::HttpsConnector<T>;
#[cfg(all(feature = "hyper-tls", not(feature = "hyper-rustls")))]
type HttpsConnector<T> = hyper_tls::HttpsConnector<T>;

struct State {
    http: HyperClient<HttpsConnector<HttpConnector>, Body>,
    default_headers: Option<HeaderMap>,
    proxy: Option<Box<str>>,
    ratelimiter: Option<Ratelimiter>,
    timeout: Duration,
    token_invalid: AtomicBool,
    token: Option<Box<str>>,
    use_http: bool,
    pub(crate) application_id: AtomicU64,
    pub(crate) default_allowed_mentions: Option<AllowedMentions>,
}

impl Debug for State {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        f.debug_struct("State")
            .field("http", &self.http)
            .field("default_headers", &self.default_headers)
            .field("proxy", &self.proxy)
            .field("ratelimiter", &self.ratelimiter)
            .field("token", &self.token)
            .field("use_http", &self.use_http)
            .finish()
    }
}

/// Twilight's http client.
///
/// Almost all of the client methods require authentication, and as such, the client must be
/// supplied with a Discord Token. Get yours [here].
///
/// # OAuth
///
/// To use Bearer tokens prefix the token with `"Bearer "`, including the space
/// at the end like so:
///
/// ```no_run
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
/// use std::env;
/// use twilight_http::Client;
///
/// let bearer = env::var("BEARER_TOKEN")?;
/// let token = format!("Bearer {}", bearer);
///
/// let client = Client::new(token);
/// # Ok(()) }
/// ```
///
/// # Cloning
///
/// The client internally wraps its data within an Arc. This means that the
/// client can be cloned and passed around tasks and threads cheaply.
///
/// # Unauthorized behavior
///
/// When the client encounters an Unauthorized response it will take note that
/// the configured token is invalid. This may occur when the token has been
/// revoked or expired. When this happens, you must create a new client with the
/// new token. The client will no longer execute requests in order to
/// prevent API bans and will always return [`Error::Unauthorized`].
///
/// # Examples
///
/// Create a client called `client`:
/// ```rust,no_run
/// use twilight_http::Client;
///
/// # #[tokio::main]
/// # async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
/// let client = Client::new("my token");
/// # Ok(()) }
/// ```
///
/// Use [`ClientBuilder`] to create a client called `client`, with a shorter timeout:
/// ```rust,no_run
/// use twilight_http::Client;
/// use std::time::Duration;
///
/// # #[tokio::main]
/// # async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
/// let client = Client::builder()
///     .token("my token")
///     .timeout(Duration::from_secs(5))
///     .build();
/// # Ok(()) }
/// ```
///
/// All the examples on this page assume you have already created a client, and have named it
/// `client`.
///
/// [here]: https://discord.com/developers/applications
#[derive(Clone, Debug)]
pub struct Client {
    state: Arc<State>,
}

impl Client {
    /// Create a new `hyper-rustls` or `hyper-tls` backed client with a token.
    #[cfg_attr(docsrs, doc(cfg(any(feature = "hyper-rustls", feature = "hyper-tls"))))]
    pub fn new(token: impl Into<String>) -> Self {
        ClientBuilder::default().token(token).build()
    }

    /// Create a new builder to create a client.
    ///
    /// Refer to its documentation for more information.
    pub fn builder() -> ClientBuilder {
        ClientBuilder::new()
    }

    /// Retrieve an immutable reference to the token used by the client.
    ///
    /// If the initial token provided is not prefixed with `Bot `, it will be, and this method
    /// reflects that.
    pub fn token(&self) -> Option<&str> {
        self.state.token.as_deref()
    }

    /// Retrieve the [`ApplicationId`] used by interaction methods.
    pub fn application_id(&self) -> Option<ApplicationId> {
        let id = self.state.application_id.load(Ordering::Relaxed);

        if id != 0 {
            return Some(ApplicationId(id));
        }

        None
    }

    /// Set a new [`ApplicationId`] after building the client.
    ///
    /// Returns the previous ID, if there was one.
    pub fn set_application_id(&self, application_id: ApplicationId) -> Option<ApplicationId> {
        let prev = self
            .state
            .application_id
            .swap(application_id.0, Ordering::Relaxed);

        if prev != 0 {
            return Some(ApplicationId(prev));
        }

        None
    }

    /// Get the default allowed mentions for sent messages.
    ///
    /// Refer to [`allowed_mentions`] for more information.
    ///
    /// [`allowed_mentions`]: crate::request::channel::allowed_mentions
    pub fn default_allowed_mentions(&self) -> Option<AllowedMentions> {
        self.state.default_allowed_mentions.clone()
    }

    /// Get the Ratelimiter used by the client internally.
    ///
    /// This will return `None` only if ratelimit handling
    /// has been explicitly disabled in the [`ClientBuilder`].
    pub fn ratelimiter(&self) -> Option<Ratelimiter> {
        self.state.ratelimiter.clone()
    }

    /// Get the audit log for a guild.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// # use twilight_http::Client;
    /// use twilight_model::id::GuildId;
    ///
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// # let client = Client::new("token");
    /// let guild_id = GuildId(101);
    /// let audit_log = client
    /// // not done
    ///     .audit_log(guild_id)
    ///     .await?;
    /// # Ok(()) }
    /// ```
    pub fn audit_log(&self, guild_id: GuildId) -> GetAuditLog<'_> {
        GetAuditLog::new(self, guild_id)
    }

    /// Retrieve the bans for a guild.
    ///
    /// # Examples
    ///
    /// Retrieve the bans for guild `1`:
    ///
    /// ```rust,no_run
    /// # use twilight_http::Client;
    /// use twilight_model::id::GuildId;
    /// #
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    /// # let client = Client::new("my token");
    /// #
    /// let guild_id = GuildId(1);
    ///
    /// let bans = client.bans(guild_id).await?;
    /// # Ok(()) }
    /// ```
    pub fn bans(&self, guild_id: GuildId) -> GetBans<'_> {
        GetBans::new(self, guild_id)
    }

    /// Get information about a ban of a guild.
    ///
    /// Includes the user banned and the reason.
    pub fn ban(&self, guild_id: GuildId, user_id: UserId) -> GetBan<'_> {
        GetBan::new(self, guild_id, user_id)
    }

    /// Bans a user from a guild, optionally with the number of days' worth of
    /// messages to delete and the reason.
    ///
    /// # Examples
    ///
    /// Ban user `200` from guild `100`, deleting
    /// 1 day's worth of messages, for the reason `"memes"`:
    ///
    /// ```rust,no_run
    /// # use twilight_http::{request::AuditLogReason, Client};
    /// use twilight_model::id::{GuildId, UserId};
    /// #
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    /// # let client = Client::new("my token");
    /// #
    /// let guild_id = GuildId(100);
    /// let user_id = UserId(200);
    /// client.create_ban(guild_id, user_id)
    ///     .delete_message_days(1)?
    ///     .reason("memes")?
    ///     .await?;
    /// # Ok(()) }
    /// ```
    pub fn create_ban(&self, guild_id: GuildId, user_id: UserId) -> CreateBan<'_> {
        CreateBan::new(self, guild_id, user_id)
    }

    /// Remove a ban from a user in a guild.
    ///
    /// # Examples
    ///
    /// Unban user `200` from guild `100`:
    ///
    /// ```rust,no_run
    /// # use twilight_http::Client;
    /// use twilight_model::id::{GuildId, UserId};
    /// #
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    /// # let client = Client::new("my token");
    /// #
    /// let guild_id = GuildId(100);
    /// let user_id = UserId(200);
    ///
    /// client.delete_ban(guild_id, user_id).await?;
    /// # Ok(()) }
    /// ```
    pub fn delete_ban(&self, guild_id: GuildId, user_id: UserId) -> DeleteBan<'_> {
        DeleteBan::new(self, guild_id, user_id)
    }

    /// Get a channel by its ID.
    ///
    /// # Examples
    ///
    /// Get channel `100`:
    ///
    /// ```rust,no_run
    /// # use twilight_http::Client;
    /// # use twilight_model::id::ChannelId;
    /// #
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    /// # let client = Client::new("my token");
    /// #
    /// let channel_id = ChannelId(100);
    /// #
    /// let channel = client.channel(channel_id).await?;
    /// # Ok(()) }
    /// ```
    pub fn channel(&self, channel_id: ChannelId) -> GetChannel<'_> {
        GetChannel::new(self, channel_id)
    }

    /// Delete a channel by ID.
    pub fn delete_channel(&self, channel_id: ChannelId) -> DeleteChannel<'_> {
        DeleteChannel::new(self, channel_id)
    }

    /// Update a channel.
    ///
    /// All fields are optional. The minimum length of the name is 2 UTF-16 characters and the
    /// maximum is 100 UTF-16 characters.
    ///
    /// # Errors
    ///
    /// Returns a [`UpdateChannelError::NameInvalid`] when the length of the name is either fewer
    /// than 2 UTF-16 characters or more than 100 UTF-16 characters.
    ///
    /// Returns a [`UpdateChannelError::RateLimitPerUserInvalid`] when the seconds of the rate limit
    /// per user is more than 21600.
    ///
    /// Returns a [`UpdateChannelError::TopicInvalid`] when the length of the topic is more than
    /// 1024 UTF-16 characters.
    ///
    /// [`UpdateChannelError::NameInvalid`]: crate::request::channel::update_channel::UpdateChannelError::NameInvalid
    /// [`UpdateChannelError::RateLimitPerUserInvalid`]: crate::request::channel::update_channel::UpdateChannelError::RateLimitPerUserInvalid
    /// [`UpdateChannelError::TopicInvalid`]: crate::request::channel::update_channel::UpdateChannelError::TopicInvalid
    pub fn update_channel(&self, channel_id: ChannelId) -> UpdateChannel<'_> {
        UpdateChannel::new(self, channel_id)
    }

    /// Follows a news channel by [`ChannelId`].
    ///
    /// The type returned is [`FollowedChannel`].
    ///
    /// [`FollowedChannel`]: ::twilight_model::channel::FollowedChannel
    pub fn follow_news_channel(
        &self,
        channel_id: ChannelId,
        webhook_channel_id: ChannelId,
    ) -> FollowNewsChannel<'_> {
        FollowNewsChannel::new(self, channel_id, webhook_channel_id)
    }

    /// Get the invites for a guild channel.
    ///
    /// This method only works if the channel is of type `GuildChannel`.
    pub fn channel_invites(&self, channel_id: ChannelId) -> GetChannelInvites<'_> {
        GetChannelInvites::new(self, channel_id)
    }

    /// Get channel messages, by [`ChannelId`].
    ///
    /// Only one of [`after`], [`around`], and [`before`] can be specified at a time.
    /// Once these are specified, the type returned is [`GetChannelMessagesConfigured`].
    ///
    /// If [`limit`] is unspecified, the default set by Discord is 50.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use twilight_http::Client;
    /// use twilight_model::id::{ChannelId, MessageId};
    ///
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    /// let client = Client::new("my token");
    /// let channel_id = ChannelId(123);
    /// let message_id = MessageId(234);
    /// let limit: u64 = 6;
    ///
    /// let messages = client
    ///     .channel_messages(channel_id)
    ///     .before(message_id)
    ///     .limit(limit)?
    ///     .await?;
    ///
    /// # Ok(()) }
    /// ```
    ///
    /// # Errors
    ///
    /// Returns [`GetChannelMessagesError::LimitInvalid`] if the amount is less than 1 or greater than 100.
    ///
    /// [`after`]: GetChannelMessages::after
    /// [`around`]: GetChannelMessages::around
    /// [`before`]: GetChannelMessages::before
    /// [`GetChannelMessagesConfigured`]: crate::request::channel::message::GetChannelMessagesConfigured
    /// [`limit`]: GetChannelMessages::limit
    /// [`GetChannelMessagesError::LimitInvalid`]: crate::request::channel::message::get_channel_messages::GetChannelMessagesError::LimitInvalid
    pub fn channel_messages(&self, channel_id: ChannelId) -> GetChannelMessages<'_> {
        GetChannelMessages::new(self, channel_id)
    }

    pub fn delete_channel_permission(&self, channel_id: ChannelId) -> DeleteChannelPermission<'_> {
        DeleteChannelPermission::new(self, channel_id)
    }

    /// Update the permissions for a role or a user in a channel.
    ///
    /// # Examples:
    ///
    /// Create permission overrides for a role to view the channel, but not send messages:
    ///
    /// ```rust,no_run
    /// # use twilight_http::Client;
    /// use twilight_model::guild::Permissions;
    /// use twilight_model::id::{ChannelId, RoleId};
    /// #
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    /// # let client = Client::new("my token");
    ///
    /// let channel_id = ChannelId(123);
    /// let allow = Permissions::VIEW_CHANNEL;
    /// let deny = Permissions::SEND_MESSAGES;
    /// let role_id = RoleId(432);
    ///
    /// client.update_channel_permission(channel_id, allow, deny)
    ///     .role(role_id)
    ///     .await?;
    /// # Ok(()) }
    /// ```
    pub fn update_channel_permission(
        &self,
        channel_id: ChannelId,
        allow: Permissions,
        deny: Permissions,
    ) -> UpdateChannelPermission<'_> {
        UpdateChannelPermission::new(self, channel_id, allow, deny)
    }

    /// Get all the webhooks of a channel.
    pub fn channel_webhooks(&self, channel_id: ChannelId) -> GetChannelWebhooks<'_> {
        GetChannelWebhooks::new(self, channel_id)
    }

    /// Get information about the current user.
    pub fn current_user(&self) -> GetCurrentUser<'_> {
        GetCurrentUser::new(self)
    }

    /// Get information about the current bot application.
    pub fn current_user_application(&self) -> GetUserApplicationInfo<'_> {
        GetUserApplicationInfo::new(self)
    }

    /// Update the current user.
    ///
    /// All paramaters are optional. If the username is changed, it may cause the discriminator to
    /// be randomized.
    pub fn update_current_user(&self) -> UpdateCurrentUser<'_> {
        UpdateCurrentUser::new(self)
    }

    /// Get the current user's connections.
    ///
    /// Requires the `connections` `OAuth2` scope.
    pub fn current_user_connections(&self) -> GetCurrentUserConnections<'_> {
        GetCurrentUserConnections::new(self)
    }

    /// Returns a list of guilds for the current user.
    ///
    /// # Examples
    ///
    /// Get the first 25 guilds with an ID after `300` and before
    /// `400`:
    ///
    /// ```rust,no_run
    /// # use twilight_http::Client;
    /// use twilight_model::id::GuildId;
    ///
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    /// # let client = Client::new("my token");
    /// #
    /// let after = GuildId(300);
    /// let before = GuildId(400);
    /// let guilds = client.current_user_guilds()
    ///     .after(after)
    ///     .before(before)
    ///     .limit(25)?
    ///     .await?;
    /// # Ok(()) }
    /// ```
    ///
    /// # Errors
    ///
    /// Returns [`GetCurrentUserGuildsError::LimitInvalid`] if the amount is greater
    /// than 100.
    ///
    /// [`GetCurrentUserGuildsError::LimitInvalid`]: crate::request::user::get_current_user_guilds::GetCurrentUserGuildsError::LimitInvalid
    pub fn current_user_guilds(&self) -> GetCurrentUserGuilds<'_> {
        GetCurrentUserGuilds::new(self)
    }

    /// Changes the user's nickname in a guild.
    pub fn update_current_user_nick(
        &self,
        guild_id: GuildId,
        nick: impl Into<String>,
    ) -> UpdateCurrentUserNick<'_> {
        UpdateCurrentUserNick::new(self, guild_id, nick)
    }

    /// Get a list of the current user's private channels.
    pub fn current_user_private_channels(&self) -> GetCurrentUserPrivateChannels<'_> {
        GetCurrentUserPrivateChannels::new(self)
    }

    /// Get the emojis for a guild, by the guild's id.
    ///
    /// # Examples
    ///
    /// Get the emojis for guild `100`:
    ///
    /// ```rust,no_run
    /// # use twilight_http::Client;
    /// # use twilight_model::id::GuildId;
    /// #
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    /// # let client = Client::new("my token");
    /// #
    /// let guild_id = GuildId(100);
    ///
    /// client.emojis(guild_id).await?;
    /// # Ok(()) }
    /// ```
    pub fn emojis(&self, guild_id: GuildId) -> GetEmojis<'_> {
        GetEmojis::new(self, guild_id)
    }

    /// Get an emoji for a guild by the the guild's ID and emoji's ID.
    ///
    /// # Examples
    ///
    /// Get emoji `100` from guild `50`:
    ///
    /// ```rust,no_run
    /// # use twilight_http::Client;
    /// # use twilight_model::id::{EmojiId, GuildId};
    /// #
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    /// # let client = Client::new("my token");
    /// #
    /// let guild_id = GuildId(50);
    /// let emoji_id = EmojiId(100);
    ///
    /// client.emoji(guild_id, emoji_id).await?;
    /// # Ok(()) }
    /// ```
    pub fn emoji(&self, guild_id: GuildId, emoji_id: EmojiId) -> GetEmoji<'_> {
        GetEmoji::new(self, guild_id, emoji_id)
    }

    /// Create an emoji in a guild.
    ///
    /// The emoji must be a Data URI, in the form of `data:image/{type};base64,{data}` where
    /// `{type}` is the image MIME type and `{data}` is the base64-encoded image.  Refer to [the
    /// discord docs] for more information about image data.
    ///
    /// [the discord docs]: https://discord.com/developers/docs/reference#image-data
    pub fn create_emoji(
        &self,
        guild_id: GuildId,
        name: impl Into<String>,
        image: impl Into<String>,
    ) -> CreateEmoji<'_> {
        CreateEmoji::new(self, guild_id, name, image)
    }

    /// Delete an emoji in a guild, by id.
    pub fn delete_emoji(&self, guild_id: GuildId, emoji_id: EmojiId) -> DeleteEmoji<'_> {
        DeleteEmoji::new(self, guild_id, emoji_id)
    }

    /// Update an emoji in a guild, by id.
    pub fn update_emoji(&self, guild_id: GuildId, emoji_id: EmojiId) -> UpdateEmoji<'_> {
        UpdateEmoji::new(self, guild_id, emoji_id)
    }

    /// Get information about the gateway, optionally with additional information detailing the
    /// number of shards to use and sessions remaining.
    ///
    /// # Examples
    ///
    /// Get the gateway connection URL without bot information:
    ///
    /// ```rust,no_run
    /// # use twilight_http::Client;
    /// #
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    /// # let client = Client::new("my token");
    /// #
    /// let info = client.gateway().await?;
    /// # Ok(()) }
    /// ```
    ///
    /// Get the gateway connection URL with additional shard and session information, which
    /// requires specifying a bot token:
    ///
    /// ```rust,no_run
    /// # use twilight_http::Client;
    /// #
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    /// # let client = Client::new("my token");
    /// #
    /// let info = client.gateway().authed().await?;
    ///
    /// println!("URL: {}", info.url);
    /// println!("Recommended shards to use: {}", info.shards);
    /// # Ok(()) }
    /// ```
    pub fn gateway(&self) -> GetGateway<'_> {
        GetGateway::new(self)
    }

    /// Get information about a guild.
    pub fn guild(&self, guild_id: GuildId) -> GetGuild<'_> {
        GetGuild::new(self, guild_id)
    }

    /// Create a new request to create a guild.
    ///
    /// The minimum length of the name is 2 UTF-16 characters and the maximum is 100 UTF-16
    /// characters. This endpoint can only be used by bots in less than 10 guilds.
    ///
    /// # Errors
    ///
    /// Returns [`CreateGuildError::NameInvalid`] if the name length is too short or too long.
    ///
    /// [`CreateGuildError::NameInvalid`]: crate::request::guild::create_guild::CreateGuildError::NameInvalid
    pub fn create_guild(
        &self,
        name: impl Into<String>,
    ) -> StdResult<CreateGuild<'_>, CreateGuildError> {
        CreateGuild::new(self, name)
    }

    /// Delete a guild permanently. The user must be the owner.
    pub fn delete_guild(&self, guild_id: GuildId) -> DeleteGuild<'_> {
        DeleteGuild::new(self, guild_id)
    }

    /// Update a guild.
    ///
    /// All endpoints are optional. Refer to [the discord docs] for more information.
    ///
    /// [the discord docs]: https://discord.com/developers/docs/resources/guild#modify-guild
    pub fn update_guild(&self, guild_id: GuildId) -> UpdateGuild<'_> {
        UpdateGuild::new(self, guild_id)
    }

    /// Leave a guild by id.
    pub fn leave_guild(&self, guild_id: GuildId) -> LeaveGuild<'_> {
        LeaveGuild::new(self, guild_id)
    }

    /// Get the channels in a guild.
    pub fn guild_channels(&self, guild_id: GuildId) -> GetGuildChannels<'_> {
        GetGuildChannels::new(self, guild_id)
    }

    /// Create a new request to create a guild channel.
    ///
    /// All fields are optional except for name. The minimum length of the name is 2 UTF-16
    /// characters and the maximum is 100 UTF-16 characters.
    ///
    /// # Errors
    ///
    /// Returns a [`CreateGuildChannelError::NameInvalid`] when the length of the name is either
    /// fewer than 2 UTF-16 characters or more than 100 UTF-16 characters.
    ///
    /// Returns a [`CreateGuildChannelError::RateLimitPerUserInvalid`] when the seconds of the rate
    /// limit per user is more than 21600.
    ///
    /// Returns a [`CreateGuildChannelError::TopicInvalid`] when the length of the topic is more
    /// than
    /// 1024 UTF-16 characters.
    ///
    /// [`CreateGuildChannelError::NameInvalid`]: crate::request::guild::create_guild_channel::CreateGuildChannelError::NameInvalid
    /// [`CreateGuildChannelError::RateLimitPerUserInvalid`]: crate::request::guild::create_guild_channel::CreateGuildChannelError::RateLimitPerUserInvalid
    /// [`CreateGuildChannelError::TopicInvalid`]: crate::request::guild::create_guild_channel::CreateGuildChannelError::TopicInvalid
    pub fn create_guild_channel(
        &self,
        guild_id: GuildId,
        name: impl Into<String>,
    ) -> StdResult<CreateGuildChannel<'_>, CreateGuildChannelError> {
        CreateGuildChannel::new(self, guild_id, name)
    }

    /// Modify the positions of the channels.
    ///
    /// The minimum amount of channels to modify, is a swap between two channels.
    pub fn update_guild_channel_positions(
        &self,
        guild_id: GuildId,
        channel_positions: impl Iterator<Item = (ChannelId, u64)>,
    ) -> UpdateGuildChannelPositions<'_> {
        UpdateGuildChannelPositions::new(self, guild_id, channel_positions)
    }

    /// Get the guild widget.
    ///
    /// Refer to [the discord docs] for more information.
    ///
    /// [the discord docs]: https://discord.com/developers/docs/resources/guild#get-guild-widget
    pub fn guild_widget(&self, guild_id: GuildId) -> GetGuildWidget<'_> {
        GetGuildWidget::new(self, guild_id)
    }

    /// Modify the guild widget.
    pub fn update_guild_widget(&self, guild_id: GuildId) -> UpdateGuildWidget<'_> {
        UpdateGuildWidget::new(self, guild_id)
    }

    /// Get the guild's integrations.
    pub fn guild_integrations(&self, guild_id: GuildId) -> GetGuildIntegrations<'_> {
        GetGuildIntegrations::new(self, guild_id)
    }

    /// Create a guild integration from the current user to the guild.
    ///
    /// Refer to [the discord docs] for more information.
    ///
    /// [the discord docs]: https://discord.com/developers/docs/resources/guild#create-guild-integration
    pub fn create_guild_integration(
        &self,
        guild_id: GuildId,
        integration_id: IntegrationId,
        kind: impl Into<String>,
    ) -> CreateGuildIntegration<'_> {
        CreateGuildIntegration::new(self, guild_id, integration_id, kind)
    }

    /// Delete an integration for a guild, by the integration's id.
    pub fn delete_guild_integration(
        &self,
        guild_id: GuildId,
        integration_id: IntegrationId,
    ) -> DeleteGuildIntegration<'_> {
        DeleteGuildIntegration::new(self, guild_id, integration_id)
    }

    /// Update a guild's integration, by its id.
    ///
    /// Refer to [the discord docs] for more information.
    ///
    /// [the discord docs]: https://discord.com/developers/docs/resources/guild#modify-guild-integrationb
    pub fn update_guild_integration(
        &self,
        guild_id: GuildId,
        integration_id: IntegrationId,
    ) -> UpdateGuildIntegration<'_> {
        UpdateGuildIntegration::new(self, guild_id, integration_id)
    }

    /// Synchronize a guild's integration by its id.
    pub fn sync_guild_integration(
        &self,
        guild_id: GuildId,
        integration_id: IntegrationId,
    ) -> SyncGuildIntegration<'_> {
        SyncGuildIntegration::new(self, guild_id, integration_id)
    }

    /// Get information about the invites of a guild.
    pub fn guild_invites(&self, guild_id: GuildId) -> GetGuildInvites<'_> {
        GetGuildInvites::new(self, guild_id)
    }

    /// Get the members of a guild, by id.
    ///
    /// The upper limit to this request is 1000. If more than 1000 members are needed, the requests
    /// must be chained. Discord defaults the limit to 1.
    ///
    /// # Examples
    ///
    /// Get the first 500 members of guild `100` after user ID `3000`:
    ///
    /// ```rust,no_run
    /// # use twilight_http::Client;
    /// use twilight_model::id::{GuildId, UserId};
    /// #
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    /// # let client = Client::new("my token");
    /// #
    /// let guild_id = GuildId(100);
    /// let user_id = UserId(3000);
    /// let members = client.guild_members(guild_id).after(user_id).await?;
    /// # Ok(()) }
    /// ```
    ///
    /// # Errors
    ///
    /// Returns [`GetGuildMembersError::LimitInvalid`] if the limit is invalid.
    ///
    /// [`GetGuildMembersError::LimitInvalid`]: crate::request::guild::member::get_guild_members::GetGuildMembersError::LimitInvalid
    pub fn guild_members(&self, guild_id: GuildId) -> GetGuildMembers<'_> {
        GetGuildMembers::new(self, guild_id)
    }

    /// Get a member of a guild, by their id.
    pub fn guild_member(&self, guild_id: GuildId, user_id: UserId) -> GetMember<'_> {
        GetMember::new(self, guild_id, user_id)
    }

    /// Add a user to a guild.
    ///
    /// An access token for the user with `guilds.join` scope is required. All
    /// other fields are optional. Refer to [the discord docs] for more
    /// information.
    ///
    /// # Errors
    ///
    /// Returns [`AddGuildMemberError::NicknameInvalid`] if the nickname is too
    /// short or too long.
    ///
    /// [`AddGuildMemberError::NickNameInvalid`]: crate::request::guild::member::add_guild_member::AddGuildMemberError::NicknameInvalid
    ///
    /// [the discord docs]: https://discord.com/developers/docs/resources/guild#add-guild-member
    pub fn add_guild_member(
        &self,
        guild_id: GuildId,
        user_id: UserId,
        access_token: impl Into<String>,
    ) -> AddGuildMember<'_> {
        AddGuildMember::new(self, guild_id, user_id, access_token)
    }

    /// Kick a member from a guild.
    pub fn remove_guild_member(&self, guild_id: GuildId, user_id: UserId) -> RemoveMember<'_> {
        RemoveMember::new(self, guild_id, user_id)
    }

    /// Update a guild member.
    ///
    /// All fields are optional. Refer to [the discord docs] for more information.
    ///
    /// # Errors
    ///
    /// Returns [`UpdateGuildMemberError::NicknameInvalid`] if the nickname length is too short or too
    /// long.
    ///
    /// [`UpdateGuildMemberError::NicknameInvalid`]: crate::request::guild::member::update_guild_member::UpdateGuildMemberError::NicknameInvalid
    ///
    /// [the discord docs]: https://discord.com/developers/docs/resources/guild#modify-guild-member
    pub fn update_guild_member(&self, guild_id: GuildId, user_id: UserId) -> UpdateGuildMember<'_> {
        UpdateGuildMember::new(self, guild_id, user_id)
    }

    /// Add a role to a member in a guild.
    ///
    /// # Examples
    ///
    /// In guild `1`, add role `2` to user `3`, for the reason `"test"`:
    ///
    /// ```rust,no_run
    /// # use twilight_http::{request::AuditLogReason, Client};
    /// use twilight_model::id::{GuildId, RoleId, UserId};
    /// #
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    /// # let client = Client::new("my token");
    /// #
    /// let guild_id = GuildId(1);
    /// let role_id = RoleId(2);
    /// let user_id = UserId(3);
    ///
    /// client.add_guild_member_role(guild_id, user_id, role_id).reason("test")?.await?;
    /// # Ok(()) }
    /// ```
    pub fn add_guild_member_role(
        &self,
        guild_id: GuildId,
        user_id: UserId,
        role_id: RoleId,
    ) -> AddRoleToMember<'_> {
        AddRoleToMember::new(self, guild_id, user_id, role_id)
    }

    /// Remove a role from a member in a guild, by id.
    pub fn remove_guild_member_role(
        &self,
        guild_id: GuildId,
        user_id: UserId,
        role_id: RoleId,
    ) -> RemoveRoleFromMember<'_> {
        RemoveRoleFromMember::new(self, guild_id, user_id, role_id)
    }

    /// For public guilds, get the guild preview.
    ///
    /// This works even if the user is not in the guild.
    pub fn guild_preview(&self, guild_id: GuildId) -> GetGuildPreview<'_> {
        GetGuildPreview::new(self, guild_id)
    }

    /// Get the counts of guild members to be pruned.
    pub fn guild_prune_count(&self, guild_id: GuildId) -> GetGuildPruneCount<'_> {
        GetGuildPruneCount::new(self, guild_id)
    }

    /// Begin a guild prune.
    ///
    /// Refer to [the discord docs] for more information.
    ///
    /// [the discord docs]: https://discord.com/developers/docs/resources/guild#begin-guild-prune
    pub fn create_guild_prune(&self, guild_id: GuildId) -> CreateGuildPrune<'_> {
        CreateGuildPrune::new(self, guild_id)
    }

    /// Get a guild's vanity url, if there is one.
    pub fn guild_vanity_url(&self, guild_id: GuildId) -> GetGuildVanityUrl<'_> {
        GetGuildVanityUrl::new(self, guild_id)
    }

    /// Get voice region data for the guild.
    ///
    /// Can return VIP servers if the guild is VIP-enabled.
    pub fn guild_voice_regions(&self, guild_id: GuildId) -> GetGuildVoiceRegions<'_> {
        GetGuildVoiceRegions::new(self, guild_id)
    }

    /// Get the webhooks of a guild.
    pub fn guild_webhooks(&self, guild_id: GuildId) -> GetGuildWebhooks<'_> {
        GetGuildWebhooks::new(self, guild_id)
    }

    /// Get information about an invite by its code.
    ///
    /// If [`with_counts`] is called, the returned invite will contain approximate member counts.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// # use twilight_http::Client;
    /// #
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    /// # let client = Client::new("my token");
    /// #
    /// let invite = client
    ///     .invite("code")
    ///     .with_counts()
    ///     .await?;
    /// # Ok(()) }
    /// ```
    ///
    /// [`with_counts`]: crate::request::channel::invite::GetInvite::with_counts
    pub fn invite(&self, code: impl Into<String>) -> GetInvite<'_> {
        GetInvite::new(self, code)
    }

    /// Create an invite, with options.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// # use twilight_http::Client;
    /// # use twilight_model::id::ChannelId;
    /// #
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    /// # let client = Client::new("my token");
    /// #
    /// let channel_id = ChannelId(123);
    /// let invite = client
    ///     .create_invite(channel_id)
    ///     .max_uses(3)
    ///     .await?;
    /// # Ok(()) }
    /// ```
    pub fn create_invite(&self, channel_id: ChannelId) -> CreateInvite<'_> {
        CreateInvite::new(self, channel_id)
    }

    /// Delete an invite by its code.
    pub fn delete_invite(&self, code: impl Into<String>) -> DeleteInvite<'_> {
        DeleteInvite::new(self, code)
    }

    /// Get a message by [`ChannelId`] and [`MessageId`].
    pub fn message(&self, channel_id: ChannelId, message_id: MessageId) -> GetMessage<'_> {
        GetMessage::new(self, channel_id, message_id)
    }

    /// Send a message to a channel.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// # use twilight_http::Client;
    /// # use twilight_model::id::ChannelId;
    /// #
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    /// # let client = Client::new("my token");
    /// #
    /// let channel_id = ChannelId(123);
    /// let message = client
    ///     .create_message(channel_id)
    ///     .content("Twilight is best pony")?
    ///     .tts(true)
    ///     .await?;
    /// # Ok(()) }
    /// ```
    ///
    /// # Errors
    ///
    /// The method [`content`] returns [`CreateMessageError::ContentInvalid`] if the content is
    /// over 2000 UTF-16 characters.
    ///
    /// The method [`embed`] returns [`CreateMessageError::EmbedTooLarge`] if the length of the
    /// embed is over 6000 characters.
    ///
    /// [`content`]: crate::request::channel::message::create_message::CreateMessage::content
    /// [`embed`]: crate::request::channel::message::create_message::CreateMessage::embed
    /// [`CreateMessageError::ContentInvalid`]:
    /// crate::request::channel::message::create_message::CreateMessageError::ContentInvalid
    /// [`CreateMessageError::EmbedTooLarge`]:
    /// crate::request::channel::message::create_message::CreateMessageError::EmbedTooLarge
    pub fn create_message(&self, channel_id: ChannelId) -> CreateMessage<'_> {
        CreateMessage::new(self, channel_id)
    }

    /// Delete a message by [`ChannelId`] and [`MessageId`].
    pub fn delete_message(
        &self,
        channel_id: ChannelId,
        message_id: MessageId,
    ) -> DeleteMessage<'_> {
        DeleteMessage::new(self, channel_id, message_id)
    }

    /// Delete messages by [`ChannelId`] and Vec<[`MessageId`]>.
    ///
    /// The vec count can be between 2 and 100. If the supplied [`MessageId`]s are invalid, they
    /// still count towards the lower and upper limits. This method will not delete messages older
    /// than two weeks. Refer to [the discord docs] for more information.
    ///
    /// [the discord docs]: https://discord.com/developers/docs/resources/channel#bulk-delete-messages
    pub fn delete_messages(
        &self,
        channel_id: ChannelId,
        message_ids: impl Into<Vec<MessageId>>,
    ) -> DeleteMessages<'_> {
        DeleteMessages::new(self, channel_id, message_ids)
    }

    /// Update a message by [`ChannelId`] and [`MessageId`].
    ///
    /// You can pass `None` to any of the methods to remove the associated field.
    /// For example, if you have a message with an embed you want to remove, you can
    /// use `.[embed](None)` to remove the embed.
    ///
    /// # Examples
    ///
    /// Replace the content with `"test update"`:
    ///
    /// ```rust,no_run
    /// use twilight_http::Client;
    /// use twilight_model::id::{ChannelId, MessageId};
    ///
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    /// let client = Client::new("my token");
    /// client.update_message(ChannelId(1), MessageId(2))
    ///     .content("test update".to_owned())?
    ///     .await?;
    /// # Ok(()) }
    /// ```
    ///
    /// Remove the message's content:
    ///
    /// ```rust,no_run
    /// # use twilight_http::Client;
    /// # use twilight_model::id::{ChannelId, MessageId};
    /// #
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    /// # let client = Client::new("my token");
    /// client.update_message(ChannelId(1), MessageId(2))
    ///     .content(None)?
    ///     .await?;
    /// # Ok(()) }
    /// ```
    ///
    /// [embed]: Self::embed
    pub fn update_message(
        &self,
        channel_id: ChannelId,
        message_id: MessageId,
    ) -> UpdateMessage<'_> {
        UpdateMessage::new(self, channel_id, message_id)
    }

    /// Crosspost a message by [`ChannelId`] and [`MessageId`].
    pub fn crosspost_message(
        &self,
        channel_id: ChannelId,
        message_id: MessageId,
    ) -> CrosspostMessage<'_> {
        CrosspostMessage::new(self, channel_id, message_id)
    }

    /// Get the pins of a channel.
    pub fn pins(&self, channel_id: ChannelId) -> GetPins<'_> {
        GetPins::new(self, channel_id)
    }

    /// Create a new pin in a channel, by ID.
    pub fn create_pin(&self, channel_id: ChannelId, message_id: MessageId) -> CreatePin<'_> {
        CreatePin::new(self, channel_id, message_id)
    }

    /// Delete a pin in a channel, by ID.
    pub fn delete_pin(&self, channel_id: ChannelId, message_id: MessageId) -> DeletePin<'_> {
        DeletePin::new(self, channel_id, message_id)
    }

    /// Get a list of users that reacted to a message with an `emoji`.
    ///
    /// This endpoint is limited to 100 users maximum, so if a message has more than 100 reactions,
    /// requests must be chained until all reactions are retireved.
    pub fn reactions(
        &self,
        channel_id: ChannelId,
        message_id: MessageId,
        emoji: RequestReactionType,
    ) -> GetReactions<'_> {
        GetReactions::new(self, channel_id, message_id, emoji)
    }

    /// Create a reaction in a [`ChannelId`] on a [`MessageId`].
    ///
    /// The reaction must be a variant of [`RequestReactionType`].
    ///
    /// # Examples
    /// ```rust,no_run
    /// # use twilight_http::{Client, request::channel::reaction::RequestReactionType};
    /// # use twilight_model::{
    /// #     id::{ChannelId, MessageId},
    /// # };
    /// #
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    /// # let client = Client::new("my token");
    /// #
    /// let channel_id = ChannelId(123);
    /// let message_id = MessageId(456);
    /// let emoji = RequestReactionType::Unicode { name: String::from("🌃") };
    ///
    /// let reaction = client
    ///     .create_reaction(channel_id, message_id, emoji)
    ///     .await?;
    /// # Ok(()) }
    /// ```
    pub fn create_reaction(
        &self,
        channel_id: ChannelId,
        message_id: MessageId,
        emoji: RequestReactionType,
    ) -> CreateReaction<'_> {
        CreateReaction::new(self, channel_id, message_id, emoji)
    }

    /// Delete the current user's (`@me`) reaction on a message.
    pub fn delete_current_user_reaction(
        &self,
        channel_id: ChannelId,
        message_id: MessageId,
        emoji: RequestReactionType,
    ) -> DeleteReaction<'_> {
        DeleteReaction::new(self, channel_id, message_id, emoji, "@me")
    }

    /// Delete a reaction by a user on a message.
    pub fn delete_reaction(
        &self,
        channel_id: ChannelId,
        message_id: MessageId,
        emoji: RequestReactionType,
        user_id: UserId,
    ) -> DeleteReaction<'_> {
        DeleteReaction::new(self, channel_id, message_id, emoji, user_id.to_string())
    }

    /// Remove all reactions on a message of an emoji.
    pub fn delete_all_reaction(
        &self,
        channel_id: ChannelId,
        message_id: MessageId,
        emoji: RequestReactionType,
    ) -> DeleteAllReaction<'_> {
        DeleteAllReaction::new(self, channel_id, message_id, emoji)
    }

    /// Delete all reactions by all users on a message.
    pub fn delete_all_reactions(
        &self,
        channel_id: ChannelId,
        message_id: MessageId,
    ) -> DeleteAllReactions<'_> {
        DeleteAllReactions::new(self, channel_id, message_id)
    }

    /// Fire a Typing Start event in the channel.
    pub fn create_typing_trigger(&self, channel_id: ChannelId) -> CreateTypingTrigger<'_> {
        CreateTypingTrigger::new(self, channel_id)
    }

    /// Create a group DM.
    ///
    /// This endpoint is limited to 10 active group DMs.
    pub fn create_private_channel(&self, recipient_id: UserId) -> CreatePrivateChannel<'_> {
        CreatePrivateChannel::new(self, recipient_id)
    }

    /// Get the roles of a guild.
    pub fn roles(&self, guild_id: GuildId) -> GetGuildRoles<'_> {
        GetGuildRoles::new(self, guild_id)
    }

    /// Create a role in a guild.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// # use twilight_http::Client;
    /// use twilight_model::id::GuildId;
    ///
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// # let client = Client::new("my token");
    /// let guild_id = GuildId(234);
    ///
    /// client.create_role(guild_id)
    ///     .color(0xd90083)
    ///     .name("Bright Pink")
    ///     .await?;
    /// # Ok(()) }
    /// ```
    pub fn create_role(&self, guild_id: GuildId) -> CreateRole<'_> {
        CreateRole::new(self, guild_id)
    }

    /// Delete a role in a guild, by id.
    pub fn delete_role(&self, guild_id: GuildId, role_id: RoleId) -> DeleteRole<'_> {
        DeleteRole::new(self, guild_id, role_id)
    }

    /// Update a role by guild id and its id.
    pub fn update_role(&self, guild_id: GuildId, role_id: RoleId) -> UpdateRole<'_> {
        UpdateRole::new(self, guild_id, role_id)
    }

    /// Modify the position of the roles.
    ///
    /// The minimum amount of roles to modify, is a swap between two roles.
    pub fn update_role_positions(
        &self,
        guild_id: GuildId,
        roles: impl Iterator<Item = (RoleId, u64)>,
    ) -> UpdateRolePositions<'_> {
        UpdateRolePositions::new(self, guild_id, roles)
    }

    /// Get a user's information by id.
    pub fn user(&self, user_id: UserId) -> GetUser<'_> {
        GetUser::new(self, user_id.to_string())
    }

    /// Get a list of voice regions that can be used when creating a guild.
    pub fn voice_regions(&self) -> GetVoiceRegions<'_> {
        GetVoiceRegions::new(self)
    }

    /// Get a webhook by ID.
    pub fn webhook(&self, id: WebhookId) -> GetWebhook<'_> {
        GetWebhook::new(self, id)
    }

    /// Create a webhook in a channel.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// # use twilight_http::Client;
    /// # use twilight_model::id::ChannelId;
    /// #
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// # let client = Client::new("my token");
    /// let channel_id = ChannelId(123);
    ///
    /// let webhook = client
    ///     .create_webhook(channel_id, "Twily Bot")
    ///     .await?;
    /// # Ok(()) }
    /// ```
    pub fn create_webhook(
        &self,
        channel_id: ChannelId,
        name: impl Into<String>,
    ) -> CreateWebhook<'_> {
        CreateWebhook::new(self, channel_id, name)
    }

    /// Delete a webhook by its ID.
    pub fn delete_webhook(&self, id: WebhookId) -> DeleteWebhook<'_> {
        DeleteWebhook::new(self, id)
    }

    /// Update a webhook by ID.
    pub fn update_webhook(&self, webhook_id: WebhookId) -> UpdateWebhook<'_> {
        UpdateWebhook::new(self, webhook_id)
    }

    /// Update a webhook, with a token, by ID.
    pub fn update_webhook_with_token(
        &self,
        webhook_id: WebhookId,
        token: impl Into<String>,
    ) -> UpdateWebhookWithToken<'_> {
        UpdateWebhookWithToken::new(self, webhook_id, token)
    }

    /// Executes a webhook, sending a message to its channel.
    ///
    /// You can only specify one of [`content`], [`embeds`], or [`file`].
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// # use twilight_http::Client;
    /// # use twilight_model::id::WebhookId;
    /// #
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// # let client = Client::new("my token");
    /// let id = WebhookId(432);
    /// #
    /// let webhook = client
    ///     .execute_webhook(id, "webhook token")
    ///     .content("Pinkie...")
    ///     .await?;
    /// # Ok(()) }
    /// ```
    ///
    /// [`content`]: crate::request::channel::webhook::ExecuteWebhook::content
    /// [`embeds`]: crate::request::channel::webhook::ExecuteWebhook::embeds
    /// [`file`]: crate::request::channel::webhook::ExecuteWebhook::file
    pub fn execute_webhook(
        &self,
        webhook_id: WebhookId,
        token: impl Into<String>,
    ) -> ExecuteWebhook<'_> {
        ExecuteWebhook::new(self, webhook_id, token)
    }

    /// Update a message executed by a webhook.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use twilight_http::Client;
    /// use twilight_model::id::{MessageId, WebhookId};
    ///
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// # let client = Client::new("token");
    /// client.update_webhook_message(WebhookId(1), "token here", MessageId(2))
    ///     .content(Some("new message content".to_owned()))?
    ///     .await?;
    /// # Ok(()) }
    /// ```
    pub fn update_webhook_message(
        &self,
        webhook_id: WebhookId,
        token: impl Into<String>,
        message_id: MessageId,
    ) -> UpdateWebhookMessage<'_> {
        UpdateWebhookMessage::new(self, webhook_id, token, message_id)
    }

    /// Delete a message executed by a webhook.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use twilight_http::Client;
    /// use twilight_model::id::{MessageId, WebhookId};
    ///
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// # let client = Client::new("token");
    /// client
    ///     .delete_webhook_message(WebhookId(1), "token here", MessageId(2))
    ///     .await?;
    /// # Ok(()) }
    /// ```
    pub fn delete_webhook_message(
        &self,
        webhook_id: WebhookId,
        token: impl Into<String>,
        message_id: MessageId,
    ) -> DeleteWebhookMessage<'_> {
        DeleteWebhookMessage::new(self, webhook_id, token, message_id)
    }

    /// Respond to an interaction, by ID and token.
    pub fn interaction_callback(
        &self,
        interaction_id: InteractionId,
        interaction_token: impl Into<String>,
        response: InteractionResponse,
    ) -> InteractionCallback<'_> {
        InteractionCallback::new(&self, interaction_id, interaction_token.into(), response)
    }

    /// Edit the original message, by its token.
    ///
    /// Functions the same as [`update_webhook_message`].
    ///
    /// [`update_webhook_message`]: Client::update_webhook_message
    pub fn update_interaction_original(
        &self,
        interaction_token: impl Into<String>,
    ) -> Result<UpdateWebhookMessage<'_>, InteractionError> {
        UpdateWebhookMessage::new_interaction(self, self.application_id(), interaction_token)
    }

    /// Delete the original message, by its token.
    pub fn delete_interaction_original(
        &self,
        interaction_token: impl Into<String>,
    ) -> Result<DeleteWebhookMessage<'_>, InteractionError> {
        DeleteWebhookMessage::new_interaction(self, self.application_id(), interaction_token)
    }

    /// Create a followup message, by an interaction token.
    ///
    /// Functions the same as [`execute_webhook`].
    ///
    /// [`execute_webhook`]: Client::execute_webhook
    pub fn create_interaction_followup(
        &self,
        interaction_token: impl Into<String>,
    ) -> Result<ExecuteWebhook<'_>, InteractionError> {
        // Use the application_id as the WebhookId as that is the only difference
        // between this method and execute_webhook.
        let application_id = self
            .application_id()
            .ok_or(InteractionError::ApplicationIdNotPresent)?;

        Ok(ExecuteWebhook::new(
            self,
            WebhookId(application_id.0),
            interaction_token,
        ))
    }

    /// Edit a followup message, by an interaction token.
    ///
    /// Functions the same as [`update_webhook_message`].
    ///
    /// [`update_webhook_message`]: Client::update_webhook_message
    pub fn update_interaction_followup(
        &self,
        interaction_token: impl Into<String>,
        message_id: MessageId,
    ) -> Result<UpdateWebhookMessage<'_>, InteractionError> {
        // Use application_id as webhook_id for same reason as
        // given in create_interaction_followup.
        let application_id = self
            .application_id()
            .ok_or(InteractionError::ApplicationIdNotPresent)?;

        Ok(UpdateWebhookMessage::new(
            self,
            WebhookId(application_id.0),
            interaction_token,
            message_id,
        ))
    }

    /// Delete a followup message by interaction token and the message's ID.
    pub fn delete_interaction_followup(
        &self,
        interaction_token: impl Into<String>,
        message_id: MessageId,
    ) -> Result<DeleteWebhookMessage<'_>, InteractionError> {
        // Use application_id as webhook_id for same reason as
        // given in create_interaction_followup.
        let application_id = self
            .application_id()
            .ok_or(InteractionError::ApplicationIdNotPresent)?;

        Ok(DeleteWebhookMessage::new(
            self,
            WebhookId(application_id.0),
            interaction_token,
            message_id,
        ))
    }

    /// Create a new command in a guild.
    ///
    /// The name must be between 3 and 32 characters in length, and the
    /// description must be between 1 and 100 characters in length. Creating a
    /// guild command with the same name as an already-existing guild command in
    /// the same guild will overwrite the old command. See [the discord docs]
    /// for more information.
    ///
    /// [the discord docs]: https://discord.com/developers/docs/interactions/slash-commands#create-guild-application-command
    pub fn create_guild_command(
        &self,
        guild_id: GuildId,
        name: impl Into<String>,
        description: impl Into<String>,
    ) -> Result<CreateGuildCommand<'_>, InteractionError> {
        CreateGuildCommand::new(
            &self,
            self.application_id(),
            guild_id,
            name.into(),
            description.into(),
        )
    }

    /// Fetch all commands for a guild, by ID.
    pub fn get_guild_commands(
        &self,
        guild_id: GuildId,
    ) -> Result<GetGuildCommands<'_>, InteractionError> {
        GetGuildCommands::new(&self, self.application_id(), guild_id)
    }

    /// Edit a command in a guild, by ID.
    ///
    /// You must specify a name and description. See [the discord docs] for more
    /// information.
    ///
    /// [the discord docs]: https://discord.com/developers/docs/interactions/slash-commands#edit-guild-application-command
    pub fn update_guild_command(
        &self,
        guild_id: GuildId,
        command_id: CommandId,
    ) -> Result<UpdateGuildCommand<'_>, InteractionError> {
        UpdateGuildCommand::new(&self, self.application_id(), guild_id, command_id)
    }

    /// Delete a command in a guild, by ID.
    pub fn delete_guild_command(
        &self,
        guild_id: GuildId,
        command_id: CommandId,
    ) -> Result<DeleteGuildCommand<'_>, InteractionError> {
        DeleteGuildCommand::new(&self, self.application_id(), guild_id, command_id)
    }

    /// Set a guild's commands.
    ///
    /// This method is idempotent: it can be used on every start, without being
    /// ratelimited if there aren't changes to the commands.
    pub fn set_guild_commands(
        &self,
        guild_id: GuildId,
        commands: Vec<Command>,
    ) -> Result<SetGuildCommands<'_>, InteractionError> {
        SetGuildCommands::new(&self, self.application_id(), guild_id, commands)
    }

    /// Create a new global command.
    ///
    /// The name must be between 3 and 32 characters in length, and the
    /// description must be between 1 and 100 characters in length. Creating a
    /// command with the same name as an already-existing global command will
    /// overwwrite the old command. See [the discord docs] for more information.
    ///
    /// [the discord docs]: https://discord.com/developers/docs/interactions/slash-commands#create-global-application-command
    pub fn create_global_command(
        &self,
        name: impl Into<String>,
        description: impl Into<String>,
    ) -> Result<CreateGlobalCommand<'_>, InteractionError> {
        CreateGlobalCommand::new(
            &self,
            self.application_id(),
            name.into(),
            description.into(),
        )
    }

    /// Fetch all global commands for your app.
    pub fn get_global_commands(&self) -> Result<GetGlobalCommands<'_>, InteractionError> {
        GetGlobalCommands::new(&self, self.application_id())
    }

    /// Edit a global command, by ID.
    ///
    /// You must specify a name and description. See [the discord docs] for more
    /// information.
    ///
    /// [the discord docs]: https://discord.com/developers/docs/interactions/slash-commands#edit-global-application-command
    pub fn update_global_command(
        &self,
        command_id: CommandId,
    ) -> Result<UpdateGlobalCommand<'_>, InteractionError> {
        UpdateGlobalCommand::new(&self, self.application_id(), command_id)
    }

    /// Delete a global command, by ID.
    pub fn delete_global_command(
        &self,
        command_id: CommandId,
    ) -> Result<DeleteGlobalCommand<'_>, InteractionError> {
        DeleteGlobalCommand::new(&self, self.application_id(), command_id)
    }

    /// Set global commands.
    ///
    /// This method is idempotent: it can be used on every start, without being
    /// ratelimited if there aren't changes to the commands.
    pub fn set_global_commands(
        &self,
        commands: Vec<Command>,
    ) -> Result<SetGlobalCommands<'_>, InteractionError> {
        SetGlobalCommands::new(&self, self.application_id(), commands)
    }

    /// Execute a request, returning the response.
    ///
    /// # Errors
    ///
    /// Returns [`Error::Unauthorized`] if the configured token has become
    /// invalid due to expiration, revokation, etc.
    #[allow(clippy::too_many_lines)]
    pub async fn raw(&self, request: Request) -> Result<Response<Body>> {
        if self.state.token_invalid.load(Ordering::Relaxed) {
            return Err(Error::Unauthorized);
        }

        let Request {
            body,
            form,
            headers: req_headers,
            method,
            path: bucket,
            path_str: path,
        } = request;

        let protocol = if self.state.use_http { "http" } else { "https" };
        let host = self.state.proxy.as_deref().unwrap_or("discord.com");

        let url = format!("{}://{}/api/v{}/{}", protocol, host, API_VERSION, path);
        tracing::debug!("URL: {:?}", url);

        let mut builder = hyper::Request::builder().method(method.clone()).uri(&url);

        if let Some(ref token) = self.state.token {
            let value = HeaderValue::from_str(&token).map_err(|source| {
                #[allow(clippy::borrow_interior_mutable_const)]
                let name = AUTHORIZATION.to_string();

                Error::CreatingHeader { name, source }
            })?;

            if let Some(headers) = builder.headers_mut() {
                headers.insert(AUTHORIZATION, value);
            }
        }

        let user_agent = HeaderValue::from_static(concat!(
            "DiscordBot (",
            env!("CARGO_PKG_HOMEPAGE"),
            ", ",
            env!("CARGO_PKG_VERSION"),
            ") Twilight-rs",
        ));

        if let Some(headers) = builder.headers_mut() {
            if let Some(form) = &form {
                if let Ok(content_type) = HeaderValue::try_from(form.content_type()) {
                    headers.insert(CONTENT_TYPE, content_type);
                }
            } else if let Some(bytes) = &body {
                let len = bytes.len();
                headers.insert(CONTENT_LENGTH, len.into());

                let content_type = HeaderValue::from_static("application/json");
                headers.insert(CONTENT_TYPE, content_type);
            }

            headers.insert(USER_AGENT, user_agent);

            if let Some(req_headers) = req_headers {
                for (maybe_name, value) in req_headers {
                    if let Some(name) = maybe_name {
                        headers.insert(name, value);
                    }
                }
            }

            if let Some(default_headers) = &self.state.default_headers {
                for (name, value) in default_headers {
                    headers.insert(name, HeaderValue::from(value));
                }
            }
        }

        let req = if let Some(form) = form {
            let form_bytes = form.build();
            if let Some(headers) = builder.headers_mut() {
                headers.insert(CONTENT_LENGTH, form_bytes.len().into());
            };
            builder
                .body(Body::from(form_bytes))
                .map_err(|source| Error::BuildingRequest { source })?
        } else if let Some(bytes) = body {
            builder
                .body(Body::from(bytes))
                .map_err(|source| Error::BuildingRequest { source })?
        } else if method == Method::PUT || method == Method::POST || method == Method::PATCH {
            if let Some(headers) = builder.headers_mut() {
                headers.insert(CONTENT_LENGTH, 0.into());
            }

            builder
                .body(Body::empty())
                .map_err(|source| Error::BuildingRequest { source })?
        } else {
            builder
                .body(Body::empty())
                .map_err(|source| Error::BuildingRequest { source })?
        };

        let inner = self.state.http.request(req);
        let fut = time::timeout(self.state.timeout, inner);

        let ratelimiter = match self.state.ratelimiter.as_ref() {
            Some(ratelimiter) => ratelimiter,
            None => {
                return fut
                    .await
                    .map_err(|source| Error::RequestTimedOut { source })?
                    .map_err(|source| Error::RequestError { source })
            }
        };

        let rx = ratelimiter.get(bucket).await;
        let tx = rx
            .await
            .map_err(|source| Error::RequestCanceled { source })?;

        let resp = fut
            .await
            .map_err(|source| Error::RequestTimedOut { source })?
            .map_err(|source| Error::RequestError { source })?;

        // If the API sent back an Unauthorized response, then the client's
        // configured token is permanently invalid and future requests must be
        // ignored to avoid API bans.
        if resp.status() == StatusCode::UNAUTHORIZED {
            self.state.token_invalid.store(true, Ordering::Relaxed);
        }

        match RatelimitHeaders::try_from(resp.headers()) {
            Ok(v) => {
                let _res = tx.send(Some(v));
            }
            Err(why) => {
                tracing::warn!("header parsing failed: {:?}; {:?}", why, resp);

                let _res = tx.send(None);
            }
        }

        Ok(resp)
    }

    /// Execute a request, chunking and deserializing the response.
    ///
    /// # Errors
    ///
    /// Returns [`Error::Unauthorized`] if the configured token has become
    /// invalid due to expiration, revokation, etc.
    pub async fn request<T: DeserializeOwned>(&self, request: Request) -> Result<T> {
        let resp = self.make_request(request).await?;

        let mut buf = body::aggregate(resp.into_body())
            .await
            .map_err(|source| Error::ChunkingResponse { source })?;

        let mut bytes = vec![0; buf.remaining()];
        buf.copy_to_slice(&mut bytes);

        let result = crate::json_from_slice(&mut bytes);

        result.map_err(|source| Error::Parsing {
            body: bytes,
            source,
        })
    }

    pub(crate) async fn request_bytes(&self, request: Request) -> Result<Bytes> {
        let resp = self.make_request(request).await?;

        hyper::body::to_bytes(resp.into_body())
            .await
            .map_err(|source| Error::ChunkingResponse { source })
    }

    /// Execute a request, checking only that the response was a success.
    ///
    /// This will not chunk and deserialize the body of the response.
    ///
    /// # Errors
    ///
    /// Returns [`Error::Unauthorized`] if the configured token has become
    /// invalid due to expiration, revokation, etc.
    pub async fn verify(&self, request: Request) -> Result<()> {
        self.make_request(request).await?;

        Ok(())
    }

    async fn make_request(&self, request: Request) -> Result<Response<Body>> {
        let resp = self.raw(request).await?;
        let status = resp.status();

        if status.is_success() {
            return Ok(resp);
        }

        match status {
            StatusCode::IM_A_TEAPOT => tracing::warn!(
                "discord's api now runs off of teapots -- proceed to panic: {:?}",
                resp,
            ),
            StatusCode::TOO_MANY_REQUESTS => tracing::warn!("429 response: {:?}", resp),
            StatusCode::SERVICE_UNAVAILABLE => {
                return Err(Error::ServiceUnavailable { response: resp })
            }
            _ => {}
        }

        let mut buf = hyper::body::aggregate(resp.into_body())
            .await
            .map_err(|source| Error::ChunkingResponse { source })?;

        let mut bytes = vec![0; buf.remaining()];
        buf.copy_to_slice(&mut bytes);

        let error =
            crate::json_from_slice::<ApiError>(&mut bytes).map_err(|source| Error::Parsing {
                body: bytes.clone(),
                source,
            })?;

        if let ApiError::General(ref general) = error {
            if let ErrorCode::Other(num) = general.code {
                tracing::debug!("got unknown API error code variant: {}; {:?}", num, error);
            }
        }

        Err(Error::Response {
            body: bytes,
            error,
            status,
        })
    }
}

impl From<HyperClient<HttpsConnector<HttpConnector>>> for Client {
    fn from(hyper_client: HyperClient<HttpsConnector<HttpConnector>>) -> Self {
        Self {
            state: Arc::new(State {
                http: hyper_client,
                default_headers: None,
                proxy: None,
                ratelimiter: Some(Ratelimiter::new()),
                timeout: Duration::from_secs(10),
                token_invalid: AtomicBool::new(false),
                token: None,
                use_http: false,
                application_id: AtomicU64::default(),
                default_allowed_mentions: None,
            }),
        }
    }
}
