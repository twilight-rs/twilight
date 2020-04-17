pub mod config;

use self::config::ClientConfigBuilder;
use crate::{
    error::{Error, ResponseError, Result, UrlError},
    ratelimiting::{RatelimitHeaders, Ratelimiter},
    request::{
        guild::{create_guild::CreateGuildError, create_guild_channel::CreateGuildChannelError},
        prelude::*,
        Request,
    },
};
use log::{debug, warn};
use reqwest::{
    header::HeaderValue,
    Body,
    Client as ReqwestClient,
    ClientBuilder as ReqwestClientBuilder,
    Response,
    StatusCode,
};
use serde::de::DeserializeOwned;
use std::{
    convert::TryFrom,
    fmt::{Debug, Formatter, Result as FmtResult},
    ops::{Deref, DerefMut},
    result::Result as StdResult,
    sync::Arc,
};
use twilight_model::{
    guild::Permissions,
    id::{ChannelId, EmojiId, GuildId, IntegrationId, MessageId, RoleId, UserId, WebhookId},
};
use url::Url;

#[derive(Clone, Debug, Default)]
pub struct ClientBuilder(pub ClientConfigBuilder);

impl ClientBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    /// Build the Client
    ///
    /// # Errors
    /// Errors if `reqwest` fails to build the client.
    pub fn build(self) -> Result<Client> {
        let config = self.0.build();

        let mut builder = ReqwestClientBuilder::new().timeout(config.timeout);

        if let Some(proxy) = config.proxy {
            builder = builder.proxy(proxy)
        }

        Ok(Client {
            state: Arc::new(State {
                http: Arc::new(builder.build().map_err(|source| Error::BuildingClient {
                    source,
                })?),
                ratelimiter: Ratelimiter::new(),
                skip_ratelimiter: config.skip_ratelimiter,
                token: config.token,
                use_http: config.proxy_http,
            }),
        })
    }
}

impl Deref for ClientBuilder {
    type Target = ClientConfigBuilder;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for ClientBuilder {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

struct State {
    http: Arc<ReqwestClient>,
    ratelimiter: Ratelimiter,
    skip_ratelimiter: bool,
    token: Option<String>,
    use_http: bool,
}

impl Debug for State {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        f.debug_struct("State")
            .field("http", &"Reqwest HTTP client")
            .field("ratelimiter", &self.ratelimiter)
            .field("skip_ratelimiter", &self.skip_ratelimiter)
            .field("token", &self.token)
            .field("use_http", &self.use_http)
            .finish()
    }
}

#[derive(Clone, Debug)]
pub struct Client {
    state: Arc<State>,
}

impl Client {
    pub fn new(token: impl Into<String>) -> Self {
        let mut token = token.into();

        // Make sure it is a bot token.
        if !token.starts_with("Bot ") {
            token.insert_str(0, "Bot ");
        }

        Self {
            state: Arc::new(State {
                http: Arc::new(ReqwestClient::new()),
                ratelimiter: Ratelimiter::new(),
                skip_ratelimiter: false,
                token: Some(token),
                use_http: false,
            }),
        }
    }

    pub fn builder() -> ClientBuilder {
        ClientBuilder::new()
    }

    pub fn token(&self) -> Option<&str> {
        self.state.token.as_ref().map(AsRef::as_ref)
    }

    pub fn add_role(
        &self,
        guild_id: GuildId,
        user_id: UserId,
        role_id: RoleId,
    ) -> AddRoleToMember<'_> {
        AddRoleToMember::new(self, guild_id, user_id, role_id)
    }

    pub fn audit_log(&self, guild_id: GuildId) -> GetAuditLog<'_> {
        GetAuditLog::new(self, guild_id)
    }

    pub fn bans(&self, guild_id: GuildId) -> GetBans<'_> {
        GetBans::new(self, guild_id)
    }

    pub fn ban(&self, guild_id: GuildId, user_id: UserId) -> GetBan<'_> {
        GetBan::new(self, guild_id, user_id)
    }

    /// Bans a user from a guild, optionally with the number of days' worth of
    /// messages to delete and the reason.
    ///
    /// # Examples
    ///
    /// Ban user `114941315417899012` from guild `377840580245585931`, deleting
    /// 1 day's worth of messages, for the reason `"memes"`:
    ///
    /// ```rust,no_run
    /// use twilight_http::Client;
    /// use twilight_model::id::{GuildId, UserId};
    ///
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    /// let client = Client::new("my token");
    ///
    /// let guild_id = GuildId(377840580245585931);
    /// let user_id = UserId(114941315417899012);
    /// client.create_ban(guild_id, user_id)
    ///     .delete_message_days(1)?
    ///     .reason("memes")
    ///     .await?;
    ///
    /// println!("Banned!");
    /// # Ok(()) }
    /// ```
    pub fn create_ban(&self, guild_id: GuildId, user_id: UserId) -> CreateBan<'_> {
        CreateBan::new(self, guild_id, user_id)
    }

    pub fn delete_ban(&self, guild_id: GuildId, user_id: UserId) -> DeleteBan<'_> {
        DeleteBan::new(self, guild_id, user_id)
    }

    pub fn channel(&self, channel_id: ChannelId) -> GetChannel<'_> {
        GetChannel::new(self, channel_id)
    }

    pub fn delete_channel(&self, channel_id: ChannelId) -> DeleteChannel<'_> {
        DeleteChannel::new(self, channel_id)
    }

    pub fn update_channel(&self, channel_id: ChannelId) -> UpdateChannel<'_> {
        UpdateChannel::new(self, channel_id)
    }

    pub fn channel_invites(&self, channel_id: ChannelId) -> GetChannelInvites<'_> {
        GetChannelInvites::new(self, channel_id)
    }

    pub fn channel_messages(&self, channel_id: ChannelId) -> GetChannelMessages<'_> {
        GetChannelMessages::new(self, channel_id)
    }

    pub fn delete_channel_permission(
        &self,
        channel_id: ChannelId,
        target_id: u64,
    ) -> DeleteChannelPermission<'_> {
        DeleteChannelPermission::new(self, channel_id, target_id)
    }

    pub fn update_channel_permission(
        &self,
        channel_id: ChannelId,
        allow: Permissions,
        deny: Permissions,
    ) -> UpdateChannelPermission<'_> {
        UpdateChannelPermission::new(self, channel_id, allow, deny)
    }

    pub fn channel_webhooks(&self, channel_id: ChannelId) -> GetChannelWebhooks<'_> {
        GetChannelWebhooks::new(self, channel_id)
    }

    pub fn current_user(&self) -> GetCurrentUser<'_> {
        GetCurrentUser::new(self)
    }

    pub fn update_current_user(&self) -> UpdateCurrentUser<'_> {
        UpdateCurrentUser::new(self)
    }

    pub fn current_user_connections(&self) -> GetCurrentUserConnections<'_> {
        GetCurrentUserConnections::new(self)
    }

    /// Returns a list of guilds for the current user.
    ///
    /// # Examples
    ///
    /// Get the first 25 guilds with an ID after `300000000000000000` and before
    /// `400000000000000000`:
    ///
    /// ```rust,no_run
    /// use twilight_http::Client;
    /// use twilight_model::id::GuildId;
    ///
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    /// let client = Client::new("my token");
    ///
    /// let after = GuildId(300000000000000000);
    /// let before = GuildId(400000000000000000);
    /// let guilds = client.current_user_guilds()
    ///     .after(after)
    ///     .before(before)
    ///     .limit(25)?
    ///     .await?;
    ///
    /// println!("{:?}", guilds);
    /// # Ok(()) }
    /// ```
    pub fn current_user_guilds(&self) -> GetCurrentUserGuilds<'_> {
        GetCurrentUserGuilds::new(self)
    }

    pub fn update_current_user_nick(
        &self,
        guild_id: GuildId,
        nick: impl Into<String>,
    ) -> UpdateCurrentUserNick<'_> {
        UpdateCurrentUserNick::new(self, guild_id, nick)
    }

    pub fn current_user_private_channels(&self) -> GetCurrentUserPrivateChannels<'_> {
        GetCurrentUserPrivateChannels::new(self)
    }

    pub fn emojis(&self, guild_id: GuildId) -> GetEmojis<'_> {
        GetEmojis::new(self, guild_id)
    }

    pub fn emoji(&self, guild_id: GuildId, emoji_id: EmojiId) -> GetEmoji<'_> {
        GetEmoji::new(self, guild_id, emoji_id)
    }

    pub fn create_emoji(
        &self,
        guild_id: GuildId,
        name: impl Into<String>,
        image: impl Into<String>,
    ) -> CreateEmoji<'_> {
        CreateEmoji::new(self, guild_id, name, image)
    }

    pub fn delete_emoji(&self, guild_id: GuildId, emoji_id: EmojiId) -> DeleteEmoji<'_> {
        DeleteEmoji::new(self, guild_id, emoji_id)
    }

    pub fn update_emoji(&self, guild_id: GuildId, emoji_id: EmojiId) -> UpdateEmoji<'_> {
        UpdateEmoji::new(self, guild_id, emoji_id)
    }

    /// Get information about the gateway, optionally with additional
    /// information detailing the number of shards to use and sessions
    /// remaining.
    ///
    /// # Examples
    ///
    /// Get the gateway connection URL without bot information:
    ///
    /// ```rust,no_run
    /// use twilight_http::Client;
    /// use std::env;
    ///
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    /// let client = Client::new(env::var("TOKEN")?);
    ///
    /// let info = client.gateway().await?;
    ///
    /// println!("URL: {}", info.url);
    /// # Ok(()) }
    /// ```
    ///
    /// Get the gateway connection URL with additional shard and session
    /// information, which requires specifying a bot token:
    ///
    /// ```rust,no_run
    /// use twilight_http::Client;
    /// use std::env;
    ///
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    /// let client = Client::new(env::var("TOKEN")?);
    ///
    /// let info = client.gateway().authed().await?;
    ///
    /// println!("URL: {}", info.url);
    /// println!("Recommended shards to use: {}", info.shards);
    /// # Ok(()) }
    /// ```
    pub fn gateway(&self) -> GetGateway<'_> {
        GetGateway::new(self)
    }

    pub fn guild(&self, guild_id: GuildId) -> GetGuild<'_> {
        GetGuild::new(self, guild_id)
    }

    /// Create a new request to create a guild.
    ///
    /// The minimum length of the name is 2 UTF-8 characters and the maximum is
    /// 100 UTF-8 characters.
    ///
    /// # Errors
    ///
    /// Returns [`CreateGuildError::NameInvalid`] if the name length is too
    /// short or too long.
    ///
    /// [`CreateGuildError::NameInvalid`]: ../request/guild/enum.CreateGuildError.html#variant.NameInvalid
    pub fn create_guild(
        &self,
        name: impl Into<String>,
    ) -> StdResult<CreateGuild<'_>, CreateGuildError> {
        CreateGuild::new(self, name)
    }

    pub fn delete_guild(&self, guild_id: GuildId) -> DeleteGuild<'_> {
        DeleteGuild::new(self, guild_id)
    }

    pub fn update_guild(&self, guild_id: GuildId) -> UpdateGuild<'_> {
        UpdateGuild::new(self, guild_id)
    }

    pub fn leave_guild(&self, guild_id: GuildId) -> LeaveGuild<'_> {
        LeaveGuild::new(self, guild_id)
    }

    pub fn guild_channels(&self, guild_id: GuildId) -> GetGuildChannels<'_> {
        GetGuildChannels::new(self, guild_id)
    }

    /// Create a new request to create a guild channel.
    ///
    /// The minimum length of the name is 2 UTF-8 characters and the maximum is
    /// 100 UTF-8 characters.
    ///
    /// # Errors
    ///
    /// Returns [`CreateGuildChannelError::NameInvalid`] if the name length is too
    /// short or too long.
    ///
    /// [`CreateGuildChannelError::NameInvalid`]: ../request/guild/enum.CreateGuildChannelError.html#variant.NameInvalid
    pub fn create_guild_channel(
        &self,
        guild_id: GuildId,
        name: impl Into<String>,
    ) -> StdResult<CreateGuildChannel<'_>, CreateGuildChannelError> {
        CreateGuildChannel::new(self, guild_id, name)
    }

    pub fn update_guild_channel_positions(
        &self,
        guild_id: GuildId,
        channel_positions: impl Iterator<Item = (ChannelId, u64)>,
    ) -> UpdateGuildChannelPositions<'_> {
        UpdateGuildChannelPositions::new(self, guild_id, channel_positions)
    }

    pub fn guild_embed(&self, guild_id: GuildId) -> GetGuildEmbed<'_> {
        GetGuildEmbed::new(self, guild_id)
    }

    pub fn update_guild_embed(&self, guild_id: GuildId) -> UpdateGuildEmbed<'_> {
        UpdateGuildEmbed::new(self, guild_id)
    }

    pub fn guild_integrations(&self, guild_id: GuildId) -> GetGuildIntegrations<'_> {
        GetGuildIntegrations::new(self, guild_id)
    }

    pub fn create_guild_integration(
        &self,
        guild_id: GuildId,
        integration_id: IntegrationId,
        kind: impl Into<String>,
    ) -> CreateGuildIntegration<'_> {
        CreateGuildIntegration::new(self, guild_id, integration_id, kind)
    }

    pub fn delete_guild_integration(
        &self,
        guild_id: GuildId,
        integration_id: IntegrationId,
    ) -> DeleteGuildIntegration<'_> {
        DeleteGuildIntegration::new(self, guild_id, integration_id)
    }

    pub fn update_guild_integration(
        &self,
        guild_id: GuildId,
        integration_id: IntegrationId,
    ) -> UpdateGuildIntegration<'_> {
        UpdateGuildIntegration::new(self, guild_id, integration_id)
    }

    pub fn sync_guild_integration(
        &self,
        guild_id: GuildId,
        integration_id: IntegrationId,
    ) -> SyncGuildIntegration<'_> {
        SyncGuildIntegration::new(self, guild_id, integration_id)
    }

    pub fn guild_invites(&self, guild_id: GuildId) -> GetGuildInvites<'_> {
        GetGuildInvites::new(self, guild_id)
    }

    pub fn guild_members(&self, guild_id: GuildId) -> GetGuildMembers<'_> {
        GetGuildMembers::new(self, guild_id)
    }

    pub fn guild_member(&self, guild_id: GuildId, user_id: UserId) -> GetMember<'_> {
        GetMember::new(self, guild_id, user_id)
    }

    pub fn remove_guild_member(&self, guild_id: GuildId, user_id: UserId) -> RemoveMember<'_> {
        RemoveMember::new(self, guild_id, user_id)
    }

    pub fn update_guild_member(&self, guild_id: GuildId, user_id: UserId) -> UpdateGuildMember<'_> {
        UpdateGuildMember::new(self, guild_id, user_id)
    }

    pub fn add_guild_member_role(
        &self,
        guild_id: GuildId,
        user_id: UserId,
        role_id: RoleId,
    ) -> AddRoleToMember<'_> {
        AddRoleToMember::new(self, guild_id, user_id, role_id)
    }

    pub fn remove_guild_member_role(
        &self,
        guild_id: GuildId,
        user_id: UserId,
        role_id: RoleId,
    ) -> RemoveRoleFromMember<'_> {
        RemoveRoleFromMember::new(self, guild_id, user_id, role_id)
    }

    /// Note: This only works for public guilds.
    pub fn guild_preview(&self, guild_id: GuildId) -> GetGuildPreview<'_> {
        GetGuildPreview::new(self, guild_id)
    }

    pub fn guild_prune_count(&self, guild_id: GuildId) -> GetGuildPruneCount<'_> {
        GetGuildPruneCount::new(self, guild_id)
    }

    pub fn create_guild_prune(&self, guild_id: GuildId) -> CreateGuildPrune<'_> {
        CreateGuildPrune::new(self, guild_id)
    }

    pub fn guild_vanity_url(&self, guild_id: GuildId) -> GetGuildVanityUrl<'_> {
        GetGuildVanityUrl::new(self, guild_id)
    }

    pub fn guild_voice_regions(&self, guild_id: GuildId) -> GetGuildVoiceRegions<'_> {
        GetGuildVoiceRegions::new(self, guild_id)
    }

    pub fn guild_webhooks(&self, guild_id: GuildId) -> GetGuildWebhooks<'_> {
        GetGuildWebhooks::new(self, guild_id)
    }

    pub fn invite(&self, code: impl Into<String>) -> GetInvite<'_> {
        GetInvite::new(self, code)
    }

    pub fn create_invite(&self, channel_id: ChannelId) -> CreateInvite<'_> {
        CreateInvite::new(self, channel_id)
    }

    pub fn delete_invite(&self, code: impl Into<String>) -> DeleteInvite<'_> {
        DeleteInvite::new(self, code)
    }

    pub fn message(&self, channel_id: ChannelId, message_id: MessageId) -> GetMessage<'_> {
        GetMessage::new(self, channel_id, message_id)
    }

    pub fn create_message(&self, channel_id: ChannelId) -> CreateMessage<'_> {
        CreateMessage::new(self, channel_id)
    }

    pub fn delete_message(
        &self,
        channel_id: ChannelId,
        message_id: MessageId,
    ) -> DeleteMessage<'_> {
        DeleteMessage::new(self, channel_id, message_id)
    }

    pub fn delete_messages(
        &self,
        channel_id: ChannelId,
        message_ids: impl Into<Vec<MessageId>>,
    ) -> DeleteMessages<'_> {
        DeleteMessages::new(self, channel_id, message_ids)
    }

    pub fn update_message(
        &self,
        channel_id: ChannelId,
        message_id: MessageId,
    ) -> UpdateMessage<'_> {
        UpdateMessage::new(self, channel_id, message_id)
    }

    pub fn pins(&self, channel_id: ChannelId) -> GetPins<'_> {
        GetPins::new(self, channel_id)
    }

    pub fn create_pin(&self, channel_id: ChannelId, message_id: MessageId) -> CreatePin<'_> {
        CreatePin::new(self, channel_id, message_id)
    }

    pub fn delete_pin(&self, channel_id: ChannelId, message_id: MessageId) -> DeletePin<'_> {
        DeletePin::new(self, channel_id, message_id)
    }

    pub fn reactions(
        &self,
        channel_id: ChannelId,
        message_id: MessageId,
        emoji: impl Into<String>,
    ) -> GetReactions<'_> {
        GetReactions::new(self, channel_id, message_id, emoji)
    }

    pub fn create_reaction(
        &self,
        channel_id: ChannelId,
        message_id: MessageId,
        emoji: impl Into<String>,
    ) -> CreateReaction<'_> {
        CreateReaction::new(self, channel_id, message_id, emoji)
    }

    pub fn delete_current_user_reaction(
        &self,
        channel_id: ChannelId,
        message_id: MessageId,
        emoji: impl Into<String>,
    ) -> DeleteReaction<'_> {
        DeleteReaction::new(self, channel_id, message_id, emoji, "@me")
    }

    pub fn delete_reaction(
        &self,
        channel_id: ChannelId,
        message_id: MessageId,
        emoji: impl Into<String>,
        user_id: UserId,
    ) -> DeleteReaction<'_> {
        DeleteReaction::new(self, channel_id, message_id, emoji, user_id.to_string())
    }

    pub fn delete_all_reaction(
        &self,
        channel_id: ChannelId,
        message_id: MessageId,
        emoji: impl Into<String>,
    ) -> DeleteAllReaction<'_> {
        DeleteAllReaction::new(self, channel_id, message_id, emoji)
    }

    pub fn delete_all_reactions(
        &self,
        channel_id: ChannelId,
        message_id: MessageId,
    ) -> DeleteAllReactions<'_> {
        DeleteAllReactions::new(self, channel_id, message_id)
    }

    pub fn create_typing_trigger(&self, channel_id: ChannelId) -> CreateTypingTrigger<'_> {
        CreateTypingTrigger::new(self, channel_id)
    }

    pub fn create_private_channel(&self, recipient_id: UserId) -> CreatePrivateChannel<'_> {
        CreatePrivateChannel::new(self, recipient_id)
    }

    pub fn roles(&self, guild_id: GuildId) -> GetGuildRoles<'_> {
        GetGuildRoles::new(self, guild_id)
    }

    pub fn create_role(&self, guild_id: GuildId) -> CreateRole<'_> {
        CreateRole::new(self, guild_id)
    }

    pub fn delete_role(&self, guild_id: GuildId, role_id: RoleId) -> DeleteRole<'_> {
        DeleteRole::new(self, guild_id, role_id)
    }

    pub fn update_role(&self, guild_id: GuildId, role_id: RoleId) -> UpdateRole<'_> {
        UpdateRole::new(self, guild_id, role_id)
    }

    pub fn update_role_positions(
        &self,
        guild_id: GuildId,
        roles: impl Iterator<Item = (RoleId, u64)>,
    ) -> UpdateRolePositions<'_> {
        UpdateRolePositions::new(self, guild_id, roles)
    }

    pub fn user(&self, user_id: u64) -> GetUser<'_> {
        GetUser::new(self, user_id.to_string())
    }

    pub fn voice_regions(&self) -> GetVoiceRegions<'_> {
        GetVoiceRegions::new(self)
    }

    pub fn webhook(&self, id: WebhookId) -> GetWebhook<'_> {
        GetWebhook::new(self, id)
    }

    pub fn create_webhook(
        &self,
        channel_id: ChannelId,
        name: impl Into<String>,
    ) -> CreateWebhook<'_> {
        CreateWebhook::new(self, channel_id, name)
    }

    pub fn delete_webhook(&self, id: WebhookId) -> DeleteWebhook<'_> {
        DeleteWebhook::new(self, id)
    }

    pub fn delete_webhook_from_url(&self, url: impl AsRef<str>) -> Result<DeleteWebhook<'_>> {
        let (id, _) = parse_webhook_url(url)?;
        Ok(self.delete_webhook(id))
    }

    pub fn update_webhook(&self, webhook_id: WebhookId) -> UpdateWebhook<'_> {
        UpdateWebhook::new(self, webhook_id)
    }

    pub fn update_webhook_from_url(&self, url: impl AsRef<str>) -> Result<UpdateWebhook<'_>> {
        let (id, _) = parse_webhook_url(url)?;
        Ok(self.update_webhook(id))
    }

    pub fn update_webhook_with_token(
        &self,
        webhook_id: WebhookId,
        token: impl Into<String>,
    ) -> UpdateWebhookWithToken<'_> {
        UpdateWebhookWithToken::new(self, webhook_id, token)
    }

    pub fn update_webhook_with_token_from_url(
        &self,
        url: impl AsRef<str>,
    ) -> Result<UpdateWebhookWithToken<'_>> {
        let (id, token) = parse_webhook_url(url)?;
        Ok(self.update_webhook_with_token(id, token.ok_or(UrlError::SegmentMissing)?))
    }

    pub fn execute_webhook(
        &self,
        webhook_id: WebhookId,
        token: impl Into<String>,
    ) -> ExecuteWebhook<'_> {
        ExecuteWebhook::new(self, webhook_id, token)
    }

    pub fn execute_webhook_from_url(&self, url: impl AsRef<str>) -> Result<ExecuteWebhook<'_>> {
        let (id, token) = parse_webhook_url(url)?;
        Ok(self.execute_webhook(id, token.ok_or(UrlError::SegmentMissing)?))
    }

    pub async fn raw(&self, request: Request) -> Result<Response> {
        let Request {
            body,
            form,
            headers: req_headers,
            method,
            path: bucket,
            path_str: path,
        } = request;

        let protocol = if self.state.use_http { "http" } else { "https" };
        let url = format!("{}://discordapp.com/api/v6/{}", protocol, path);

        debug!("URL: {:?}", url);

        let mut builder = self.state.http.request(method.clone(), &url);

        if let Some(ref token) = self.state.token {
            let value = HeaderValue::from_str(&token).map_err(|source| Error::CreatingHeader {
                name: "Authroization".to_owned(),
                source,
            })?;

            builder = builder.header("Authorization", value);
        }

        if let Some(form) = form {
            builder = builder.multipart(form);
        } else {
            if let Some(bytes) = body {
                let len = bytes.len();

                builder = builder.body(Body::from(bytes));
                builder = builder.header("content-length", len);
            } else {
                builder = builder.header("content-length", 0);
            }

            let content_type = HeaderValue::from_static("application/json");
            builder = builder.header("Content-Type", content_type);
        }

        let precision = HeaderValue::from_static("millisecond");
        let user_agent = HeaderValue::from_static(concat!(
            "twilight.rs (",
            env!("CARGO_PKG_HOMEPAGE"),
            ") ",
            env!("CARGO_PKG_VERSION"),
        ));
        builder = builder.header("X-RateLimit-Precision", precision);
        builder = builder.header("User-Agent", user_agent);

        if let Some(req_headers) = req_headers {
            builder = builder.headers(req_headers);
        }

        if self.state.skip_ratelimiter {
            return builder.send().await.map_err(|source| Error::RequestError {
                source,
            });
        }

        let rx = self.state.ratelimiter.get(bucket).await;
        let tx = rx.await.map_err(|source| Error::RequestCanceled {
            source,
        })?;

        let resp = builder.send().await.map_err(|source| Error::RequestError {
            source,
        })?;

        match RatelimitHeaders::try_from(resp.headers()) {
            Ok(v) => {
                let _ = tx.send(Some(v));
            },
            Err(why) => {
                warn!("Err parsing headers: {:?}; {:?}", why, resp,);

                let _ = tx.send(None);
            },
        }

        Ok(resp)
    }

    pub async fn request<T: DeserializeOwned>(&self, request: Request) -> Result<T> {
        let resp = self.make_request(request).await?;

        let bytes = resp
            .bytes()
            .await
            .map_err(|source| Error::ChunkingResponse {
                source,
            })?;

        serde_json::from_slice(&bytes).map_err(|source| Error::Parsing {
            body: (*bytes).to_vec(),
            source,
        })
    }

    pub async fn verify(&self, request: Request) -> Result<()> {
        self.make_request(request).await?;

        Ok(())
    }

    async fn make_request(&self, request: Request) -> Result<Response> {
        let resp = self.raw(request).await?;

        if resp.status().is_client_error() {
            if resp.status() == StatusCode::IM_A_TEAPOT {
                warn!(
                    "Discord's API now runs off of teapots -- proceed to panic: {:?}",
                    resp,
                );
            }

            if resp.status() == StatusCode::TOO_MANY_REQUESTS {
                warn!("Response got 429: {:?}", resp);
            }

            return Err(Error::Response {
                source: ResponseError::Client {
                    response: resp,
                },
            });
        }

        if resp.status().is_server_error() {
            return Err(Error::Response {
                source: ResponseError::Server {
                    response: resp,
                },
            });
        }

        Ok(resp)
    }
}

impl From<ReqwestClient> for Client {
    fn from(reqwest_client: ReqwestClient) -> Self {
        Self {
            state: Arc::new(State {
                http: Arc::new(reqwest_client),
                ratelimiter: Ratelimiter::new(),
                skip_ratelimiter: false,
                token: None,
                use_http: false,
            }),
        }
    }
}

impl From<Arc<ReqwestClient>> for Client {
    fn from(reqwest_client: Arc<ReqwestClient>) -> Self {
        Self {
            state: Arc::new(State {
                http: reqwest_client,
                ratelimiter: Ratelimiter::new(),
                skip_ratelimiter: false,
                token: None,
                use_http: false,
            }),
        }
    }
}

// parse the webhook id and token, if it exists in the string
fn parse_webhook_url(
    url: impl AsRef<str>,
) -> std::result::Result<(WebhookId, Option<String>), UrlError> {
    let url = Url::parse(url.as_ref())?;
    let mut segments = url.path_segments().ok_or(UrlError::SegmentMissing)?;

    segments
        .next()
        .filter(|s| s == &"api")
        .ok_or(UrlError::SegmentMissing)?;
    segments
        .next()
        .filter(|s| s == &"webhooks")
        .ok_or(UrlError::SegmentMissing)?;
    let id = segments.next().ok_or(UrlError::SegmentMissing)?;
    let token = segments.next();

    Ok((WebhookId(id.parse()?), token.map(String::from)))
}

#[cfg(test)]
mod tests {
    use super::{parse_webhook_url, WebhookId};
    use std::error::Error;
    #[test]
    fn parse_webhook_id() -> Result<(), Box<dyn Error>> {
        assert_eq!(
            parse_webhook_url("https://discordapp.com/api/webhooks/123")?,
            (WebhookId(123), None)
        );
        assert!(parse_webhook_url("https://discordapp.com/foo/bar/456").is_err());
        assert!(parse_webhook_url("https://discordapp.com/api/webhooks/").is_err());

        Ok(())
    }

    #[test]
    fn parse_webhook_token() -> Result<(), Box<dyn Error>> {
        assert_eq!(
            parse_webhook_url("https://discordapp.com/api/webhooks/456/token")?,
            (WebhookId(456), Some("token".into()))
        );

        assert_eq!(
            parse_webhook_url("https://discordapp.com/api/webhooks/456/token/slack")?,
            (WebhookId(456), Some("token".into()))
        );

        Ok(())
    }
}
