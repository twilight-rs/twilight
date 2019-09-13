pub mod config;

use crate::{
    error::{
        BuildingClient,
        CreatingHeader,
        InvalidUrl,
        Result,
    },
    pending::{Pending, PendingBody, PendingText},
    ratelimiting::Ratelimiter,
    request::*,
    routing::{Path, Route},
};
use futures_util::FutureExt;
use http::header::HeaderValue;
use dawn_model::{
    channel::{Channel, GuildChannel, Message, PrivateChannel, Webhook},
    guild::{
        Ban,
        Emoji,
        Guild,
        GuildEmbed,
        GuildIntegration,
        Member,
        Permissions,
        Role,
    },
    id::{
        ChannelId,
        EmojiId,
        GuildId,
        IntegrationId,
        MessageId,
        RoleId,
        UserId,
        WebhookId,
    },
    invite::Invite,
    user::{Connection, CurrentUser, User},
    voice::VoiceRegion,
};
use reqwest::{
    Body,
    Client as ReqwestClient,
    ClientBuilder as ReqwestClientBuilder,
    Response,
    Url,
};
use self::config::ConfigBuilder;
use serde::{
    de::DeserializeOwned,
    Deserialize,
    Serialize,
};
use serde_json::json;
use snafu::ResultExt;
use std::{
    future::Future,
    ops::{Deref, DerefMut},
    str::FromStr,
    sync::Arc,
};

#[derive(Clone, Debug, Default)]
pub struct ClientBuilder(pub ConfigBuilder);

impl ClientBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn build(self) -> Result<Client> {
        let config = self.0.build();

        let mut builder = ReqwestClientBuilder::new()
            .timeout(config.timeout);

        if let Some(proxy) = config.proxy {
            builder = builder.proxy(proxy)
        }

        Ok(Client {
            state: Arc::new(State {
                http: Arc::new(builder.build().context(BuildingClient)?),
                ratelimiter: Ratelimiter::new(),
                token: config.token,
            }),
        })
    }
}

impl Deref for ClientBuilder {
    type Target = ConfigBuilder;

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
    token: Option<String>,
}

pub struct Client {
    state: Arc<State>,
}

impl Client {
    pub fn new(token: impl Into<String>) -> Self {
        Self {
            state: Arc::new(State {
                http: Arc::new(ReqwestClient::new()),
                ratelimiter: Ratelimiter::new(),
                token: Some(token.into()),
            }),
        }
    }

    pub fn builder() -> ClientBuilder {
        ClientBuilder::new()
    }

    pub fn token(&self) -> Option<&str> {
        self.state.token.as_ref().map(AsRef::as_ref)
    }

    pub async fn add_role(
        &self,
        guild_id: impl Into<GuildId>,
        user_id: impl Into<UserId>,
        role_id: impl Into<RoleId>,
    ) -> Result<()> {
        self.verify(Request::from(Route::AddMemberRole {
            guild_id: guild_id.into().0,
            role_id: role_id.into().0,
            user_id: user_id.into().0,
        }))?.await
    }

    pub fn audit_log(&self, guild_id: impl Into<GuildId>) -> GetAuditLog<'_> {
        GetAuditLog::new(self, guild_id)
    }

    pub async fn bans(
        &self,
        guild_id: impl Into<GuildId>,
    ) -> Result<Vec<Ban>> {
        self.request(Request::from(Route::GetBans {
            guild_id: guild_id.into().0,
        }))?.await
    }

    pub async fn ban(
        &self,
        guild_id: impl Into<GuildId>,
        user_id: impl Into<UserId>,
    ) -> Result<Ban> {
        self.request(Request::from(Route::GetBan {
            guild_id: guild_id.into().0,
            user_id: user_id.into().0,
        }))?.await
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
    /// use dawn_http::Client;
    /// use dawn_model::id::{GuildId, UserId};
    ///
    /// # async fn foo() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = Client::new("my token");
    ///
    /// let guild_id = GuildId(377840580245585931);
    /// let user_id = UserId(114941315417899012);
    /// client.create_ban(guild_id, user_id)
    ///     .delete_message_days(1)
    ///     .reason("memes")
    ///     .await?;
    ///
    /// println!("Banned!");
    /// # Ok(()) }
    /// #
    /// # fn main() { }
    /// ```
    pub fn create_ban(
        &self,
        guild_id: impl Into<GuildId>,
        user_id: impl Into<UserId>,
    ) -> CreateBan<'_> {
        CreateBan::new(self, guild_id, user_id)
    }

    pub async fn delete_ban(
        &self,
        guild_id: impl Into<GuildId>,
        user_id: impl Into<UserId>,
    ) -> Result<()> {
        self.verify(Request::from(Route::DeleteBan {
            guild_id: guild_id.into().0,
            user_id: user_id.into().0,
        }))?.await
    }

    pub async fn channel(
        &self,
        channel_id: impl Into<ChannelId>,
    ) -> Result<Channel> {
        self.request(Request::from(Route::GetChannel {
            channel_id: channel_id.into().0,
        }))?.await
    }

    pub async fn delete_channel(
        &self,
        channel_id: impl Into<ChannelId>,
    ) -> Result<Channel> {
        self.request(Request::from(Route::DeleteChannel {
            channel_id: channel_id.into().0,
        }))?.await
    }

    pub fn update_channel(
        &self,
        channel_id: impl Into<ChannelId>,
    ) -> UpdateChannel<'_> {
        UpdateChannel::new(self, channel_id)
    }

    pub async fn channel_invites(
        &self,
        channel_id: impl Into<ChannelId>,
    ) -> Result<Vec<Invite>> {
        self.request(Request::from(Route::GetChannelInvites {
            channel_id: channel_id.into().0,
        }))?.await
    }

    pub fn channel_messages(
        &self,
        channel_id: impl Into<ChannelId>,
    ) -> GetChannelMessages<'_> {
        GetChannelMessages::new(self, channel_id)
    }

    pub async fn delete_channel_permission(
        &self,
        channel_id: impl Into<ChannelId>,
        target_id: u64,
    ) -> Result<()> {
        self.verify(Request::from(Route::DeletePermissionOverwrite {
            channel_id: channel_id.into().0,
            target_id,
        }))?.await
    }

    pub fn update_channel_permission(
        &self,
        channel_id: impl Into<ChannelId>,
        allow: Permissions,
        deny: Permissions,
    ) -> UpdateChannelPermission<'_> {
        UpdateChannelPermission::new(self, channel_id, allow, deny)
    }

    pub async fn channel_webhooks(
        &self,
        channel_id: impl Into<ChannelId>,
    ) -> Result<Vec<Webhook>> {
        self.request(Request::from(Route::GetChannelWebhooks {
            channel_id: channel_id.into().0,
        }))?.await
    }

    pub async fn current_user(&self) -> Result<CurrentUser> {
        self.request(Request::from(Route::GetUser {
            target_user: "@me",
        }))?.await
    }

    pub fn update_current_user(&self) -> UpdateCurrentUser<'_> {
        UpdateCurrentUser::new(self)
    }

    pub async fn current_user_connections(&self) -> Result<Vec<Connection>> {
        self.request(Request::from(Route::GetUserConnections {
            target_user: "@me",
        }))?.await
    }

    /// Returns a list of guilds for the current user.
    ///
    /// # Examples
    ///
    /// Get the first 25 guilds with an ID after `300000000000000000` and before
    /// `400000000000000000`:
    ///
    /// ```rust,no_run
    /// use dawn_http::Client;
    /// use dawn_model::id::GuildId;
    ///
    /// # async fn foo() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = Client::new("my token");
    ///
    /// let after = GuildId(300000000000000000);
    /// let before = GuildId(400000000000000000);
    /// let guilds = client.current_user_guilds()
    ///     .after(after)
    ///     .before(before)
    ///     .limit(25)
    ///     .await?;
    ///
    /// println!("{:?}", guilds);
    /// # Ok(()) }
    /// #
    /// # fn main() { }
    /// ```
    pub fn current_user_guilds(&self) -> GetCurrentUserGuilds<'_> {
        GetCurrentUserGuilds::new(self)
    }

    pub async fn update_current_user_nick(
        &self,
        guild_id: impl Into<GuildId>,
        nick: impl Into<String>,
    ) -> Result<()> {
        self.verify(Request::from((
            serde_json::to_vec(&json!({
                "nick": nick.into(),
            }))?,
            Route::UpdateNickname {
                guild_id: guild_id.into().0,
            },
        )))?.await
    }

    pub async fn current_user_private_channels(&self) -> Result<Vec<PrivateChannel>> {
        self.request(Request::from(Route::GetUserPrivateChannels {
            target_user: "@me",
        }))?.await
    }

    pub async fn emojis(
        &self,
        guild_id: impl Into<GuildId>,
    ) -> Result<Vec<Emoji>> {
        self.request(Request::from(Route::GetEmojis {
            guild_id: guild_id.into().0,
        }))?.await
    }

    pub async fn emoji(
        &self,
        guild_id: impl Into<GuildId>,
        emoji_id: impl Into<EmojiId>,
    ) -> Result<Emoji> {
        self.request(Request::from(Route::GetEmoji {
            emoji_id: emoji_id.into().0,
            guild_id: guild_id.into().0,
        }))?.await
    }

    pub fn create_emoji(
        &self,
        guild_id: impl Into<GuildId>,
        name: impl Into<String>,
        image: impl Into<String>,
    ) -> CreateEmoji<'_> {
        CreateEmoji::new(self, guild_id, name, image)
    }

    pub async fn delete_emoji(
        &self,
        guild_id: impl Into<GuildId>,
        emoji_id: impl Into<EmojiId>,
    ) -> Result<()> {
        self.verify(Request::from(Route::DeleteEmoji {
            emoji_id: emoji_id.into().0,
            guild_id: guild_id.into().0,
        }))?.await
    }

    pub fn update_emoji(
        &self,
        guild_id: impl Into<GuildId>,
        emoji_id: impl Into<EmojiId>,
    ) -> UpdateEmoji<'_> {
        UpdateEmoji::new(self, guild_id, emoji_id)
    }

    /// Get information about the gateway, optionally with additional
    /// information detailing the number of shards to use and sessions
    /// remaining.
    ///
    /// # Examples
    ///
    /// Get the gateway connection URL, which doesn't require a token:
    ///
    /// ```rust,no_run
    /// use dawn_http::Client;
    ///
    /// # async fn foo() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = Client::new(None);
    ///
    /// let info = client.gateway().await?;
    ///
    /// println!("URL: {}", info.url);
    /// # Ok(()) }
    /// #
    /// # fn main() { }
    /// ```
    ///
    /// Get the gateway connection URL with additional shard and session
    /// information, which requires specifying a bot token:
    ///
    /// ```rust,no_run
    /// use dawn_http::Client;
    ///
    /// # async fn foo() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = Client::new("my token");
    ///
    /// let info = client.gateway().authed().await?;
    ///
    /// println!("URL: {}", info.url);
    /// println!("Recommended shards to use: {}", info.shards);
    /// # Ok(()) }
    /// #
    /// # fn main() { }
    /// ```
    pub fn gateway(&self) -> GetGateway<'_> {
        GetGateway::new(self)
    }

    pub async fn guild(&self, guild_id: impl Into<GuildId>) -> Result<Guild> {
        self.request(Request::from(Route::GetGuild {
            guild_id: guild_id.into().0,
        }))?.await
    }

    pub fn create_guild(&self, name: impl Into<String>) -> CreateGuild<'_> {
        CreateGuild::new(self, name)
    }

    pub async fn delete_guild(
        &self,
        guild_id: impl Into<GuildId>,
    ) -> Result<()> {
        self.verify(Request::from(Route::DeleteGuild {
            guild_id: guild_id.into().0,
        }))?.await
    }

    pub fn update_guild(
        &self,
        guild_id: impl Into<GuildId>,
    ) -> UpdateGuild<'_> {
        UpdateGuild::new(self, guild_id)
    }

    pub async fn leave_guild(
        &self,
        guild_id: impl Into<GuildId>,
    ) -> Result<()> {
        self.verify(Request::from(Route::LeaveGuild {
            guild_id: guild_id.into().0,
        }))?.await
    }

    pub async fn guild_channels(
        &self,
        guild_id: impl Into<GuildId>,
    ) -> Result<Vec<GuildChannel>> {
        self.request(Request::from(Route::GetChannels {
            guild_id: guild_id.into().0,
        }))?.await
    }

    pub fn create_guild_channel(
        &self,
        guild_id: impl Into<GuildId>,
        name: impl Into<String>,
    ) -> CreateGuildChannel<'_> {
        CreateGuildChannel::new(self, guild_id, name)
    }

    pub async fn update_guild_channel_positions(
        &self,
        guild_id: impl Into<GuildId>,
        channel_positions: impl Iterator<Item = (impl Into<ChannelId>, u64)>,
    ) -> Result<()> {
        #[derive(Serialize)]
        struct Position {
            id: ChannelId,
            position: u64,
        }

        let positions = channel_positions.map(|(id, pos)| Position {
            id: id.into(),
            position: pos,
        }).collect::<Vec<_>>();

        self.verify(Request::from((
            serde_json::to_vec(&positions)?,
            Route::UpdateGuildChannels {
                guild_id: guild_id.into().0,
            },
        )))?.await
    }

    pub async fn guild_embed(
        &self,
        guild_id: impl Into<GuildId>,
    ) -> Result<GuildEmbed> {
        self.request(Request::from(Route::GetGuildEmbed {
            guild_id: guild_id.into().0,
        }))?.await
    }

    pub fn update_guild_embed(
        &self,
        guild_id: impl Into<GuildId>,
    ) -> UpdateGuildEmbed<'_> {
        UpdateGuildEmbed::new(self, guild_id)
    }

    pub async fn guild_integrations(
        &self,
        guild_id: impl Into<GuildId>,
    ) -> Result<Vec<GuildIntegration>> {
        self.request(Request::from(Route::GetGuildIntegrations {
            guild_id: guild_id.into().0,
        }))?.await
    }

    pub async fn create_guild_integration(
        &self,
        guild_id: impl Into<GuildId>,
        integration_id: impl Into<IntegrationId>,
        kind: impl AsRef<str>,
    ) -> Result<()> {
        self.verify(Request::from((
            serde_json::to_vec(&json!({
                "id": integration_id.into(),
                "type": kind.as_ref(),
            }))?,
            Route::CreateGuildIntegration {
                guild_id: guild_id.into().0,
            },
        )))?.await
    }

    pub async fn delete_guild_integration(
        &self,
        guild_id: impl Into<GuildId>,
        integration_id: impl Into<IntegrationId>,
    ) -> Result<()> {
        self.verify(Request::from(Route::DeleteGuildIntegration {
            guild_id: guild_id.into().0,
            integration_id: integration_id.into().0,
        }))?.await
    }

    pub async fn update_guild_integration(
        &self,
        guild_id: impl Into<GuildId>,
        integration_id: impl Into<IntegrationId>,
        enable_emoticons: bool,
        expire_behavior: u64,
        expire_grace_period: u64,
    ) -> Result<()> {
        self.verify(Request::from((
            serde_json::to_vec(&json!({
                "enable_emoticons": enable_emoticons,
                "expire_behavior": expire_behavior,
                "expire_grace_period": expire_grace_period,
            }))?,
            Route::UpdateGuildIntegration {
                guild_id: guild_id.into().0,
                integration_id: integration_id.into().0,
            },
        )))?.await
    }

    pub async fn sync_guild_integration(
        &self,
        guild_id: impl Into<GuildId>,
        integration_id: impl Into<IntegrationId>,
    ) -> Result<()> {
        self.verify(Request::from(Route::SyncGuildIntegration {
            guild_id: guild_id.into().0,
            integration_id: integration_id.into().0,
        }))?.await
    }

    pub async fn guild_invites(
        &self,
        guild_id: impl Into<GuildId>,
    ) -> Result<Vec<Invite>> {
        self.request(Request::from(Route::GetGuildInvites {
            guild_id: guild_id.into().0,
        }))?.await
    }

    pub fn guild_members(
        &self,
        guild_id: impl Into<GuildId>,
    ) -> GetGuildMembers<'_> {
        GetGuildMembers::new(self, guild_id)
    }

    pub async fn guild_member(
        &self,
        guild_id: impl Into<GuildId>,
        user_id: impl Into<UserId>,
    ) -> Result<Member> {
        self.request(Request::from(Route::GetMember {
            guild_id: guild_id.into().0,
            user_id: user_id.into().0,
        }))?.await
    }

    pub async fn remove_guild_member(
        &self,
        guild_id: impl Into<GuildId>,
        user_id: impl Into<UserId>,
    ) -> Result<()> {
        self.verify(Request::from(Route::RemoveMember {
            guild_id: guild_id.into().0,
            user_id: user_id.into().0,
        }))?.await
    }

    pub fn update_guild_member(
        &self,
        guild_id: impl Into<GuildId>,
        user_id: impl Into<UserId>,
    ) -> UpdateGuildMember<'_> {
        UpdateGuildMember::new(self, guild_id, user_id)
    }

    pub async fn add_guild_member_role(
        &self,
        guild_id: impl Into<GuildId>,
        user_id: impl Into<UserId>,
        role_id: impl Into<RoleId>,
    ) -> Result<()> {
        self.request(Request::from(Route::AddMemberRole {
            guild_id: guild_id.into().0,
            role_id: role_id.into().0,
            user_id: user_id.into().0,
        }))?.await
    }

    pub async fn remove_guild_member_role(
        &self,
        guild_id: impl Into<GuildId>,
        user_id: impl Into<UserId>,
        role_id: impl Into<RoleId>,
    ) -> Result<()> {
        self.request(Request::from(Route::RemoveMemberRole {
            guild_id: guild_id.into().0,
            role_id: role_id.into().0,
            user_id: user_id.into().0,
        }))?.await
    }

    pub fn guild_prune_count(
        &self,
        guild_id: impl Into<GuildId>,
    ) -> GetGuildPruneCount<'_> {
        GetGuildPruneCount::new(self, guild_id)
    }

    pub fn create_guild_prune(
        &self,
        guild_id: impl Into<GuildId>,
    ) -> CreateGuildPrune<'_> {
        CreateGuildPrune::new(self, guild_id)
    }

    pub async fn guild_vanity_url(
        &self,
        guild_id: impl Into<GuildId>,
    ) -> Result<String> {
        #[derive(Deserialize)]
        struct VanityUrl {
            code: String,
        }

        let vanity = self.request::<VanityUrl>(Request::from(Route::GetGuildVanityUrl {
            guild_id: guild_id.into().0,
        }))?.await?;

        Ok(vanity.code)
    }

    pub async fn guild_voice_regions(
        &self,
        guild_id: impl Into<GuildId>,
    ) -> Result<Vec<VoiceRegion>> {
        self.request(Request::from(Route::GetGuildVoiceRegions {
            guild_id: guild_id.into().0,
        }))?.await
    }

    pub async fn guild_webhooks(
        &self,
        guild_id: impl Into<GuildId>,
    ) -> Result<Vec<Webhook>> {
        self.request(Request::from(Route::GetGuildWebhooks {
            guild_id: guild_id.into().0,
        }))?.await
    }

    pub fn invite(&self, code: impl Into<String>) -> GetInvite<'_> {
        GetInvite::new(self, code)
    }

    pub fn create_invite(
        &self,
        channel_id: impl Into<ChannelId>,
    ) -> CreateInvite<'_> {
        CreateInvite::new(self, channel_id)
    }

    pub async fn delete_invite(&self, code: impl AsRef<str>) -> Result<()> {
        self.verify(Request::from(Route::DeleteInvite {
            code: code.as_ref(),
        }))?.await
    }

    pub async fn message(
        &self,
        channel_id: impl Into<ChannelId>,
        message_id: impl Into<MessageId>,
    ) -> Result<Message> {
        self.request(Request::from(Route::GetMessage {
            channel_id: channel_id.into().0,
            message_id: message_id.into().0,
        }))?.await
    }

    pub fn create_message(
        &self,
        channel_id: impl Into<ChannelId>,
    ) -> CreateMessage<'_> {
        CreateMessage::new(self, channel_id)
    }

    pub async fn delete_message(
        &self,
        channel_id: impl Into<ChannelId>,
        message_id: impl Into<MessageId>,
    ) -> Result<()> {
        self.verify(Request::from(Route::DeleteMessage {
            channel_id: channel_id.into().0,
            message_id: message_id.into().0,
        }))?.await
    }

    pub async fn delete_messages(
        &self,
        channel_id: impl Into<ChannelId>,
        message_ids: impl Into<Vec<MessageId>>,
    ) -> Result<()> {
        self.verify(Request::from((
            serde_json::to_vec(&json!({
                "messages": message_ids.into(),
            }))?,
            Route::DeleteMessages {
                channel_id: channel_id.into().0,
            },
        )))?.await
    }

    pub fn update_message(
        &self,
        channel_id: impl Into<ChannelId>,
        message_id: impl Into<MessageId>,
    ) -> UpdateMessage<'_> {
        UpdateMessage::new(self, channel_id, message_id)
    }

    pub async fn pins(&self, channel_id: ChannelId) -> Result<Vec<Message>> {
        self.request(Request::from(Route::GetPins {
            channel_id: channel_id.0,
        }))?.await
    }

    pub async fn create_pin(
        &self,
        channel_id: impl Into<ChannelId>,
        message_id: impl Into<MessageId>,
    ) -> Result<()> {
        self.request(Request::from(Route::PinMessage {
            channel_id: channel_id.into().0,
            message_id: message_id.into().0,
        }))?.await
    }

    pub async fn delete_pin(
        &self,
        channel_id: impl Into<ChannelId>,
        message_id: impl Into<MessageId>,
    ) -> Result<()> {
        self.request(Request::from(Route::UnpinMessage {
            channel_id: channel_id.into().0,
            message_id: message_id.into().0,
        }))?.await
    }

    pub fn reactions(
        &self,
        channel_id: impl Into<ChannelId>,
        message_id: impl Into<MessageId>,
        emoji: impl Into<String>,
    ) -> GetReactions<'_> {
        GetReactions::new(self, channel_id, message_id, emoji)
    }

    pub async fn create_reaction(
        &self,
        channel_id: impl Into<ChannelId>,
        message_id: impl Into<MessageId>,
        emoji: impl AsRef<str>,
    ) -> Result<()> {
        self.verify(Request::from(Route::CreateReaction {
            channel_id: channel_id.into().0,
            emoji: emoji.as_ref(),
            message_id: message_id.into().0,
        }))?.await
    }

    pub async fn delete_current_user_reaction(
        &self,
        channel_id: impl Into<ChannelId>,
        message_id: impl Into<MessageId>,
        emoji: impl AsRef<str>,
    ) -> Result<()> {
        self.verify(Request::from(Route::DeleteReaction {
            channel_id: channel_id.into().0,
            emoji: emoji.as_ref(),
            message_id: message_id.into().0,
            user: "@me",
        }))?.await
    }

    pub async fn delete_reaction(
        &self,
        channel_id: impl Into<ChannelId>,
        message_id: impl Into<MessageId>,
        emoji: impl AsRef<str>,
        user_id: impl Into<UserId>,
    ) -> Result<()> {
        self.verify(Request::from(Route::DeleteReaction {
            channel_id: channel_id.into().0,
            emoji: emoji.as_ref(),
            message_id: message_id.into().0,
            user: &user_id.into().0.to_string(),
        }))?.await
    }

    pub async fn delete_all_reactions(
        &self,
        channel_id: impl Into<ChannelId>,
        message_id: impl Into<MessageId>,
    ) -> Result<()> {
        self.verify(Request::from(Route::DeleteMessageReactions {
            channel_id: channel_id.into().0,
            message_id: message_id.into().0,
        }))?.await
    }

    pub async fn create_typing_trigger(
        &self,
        channel_id: impl Into<ChannelId>,
    ) -> Result<()> {
        self.verify(Request::from(Route::CreateTypingTrigger {
            channel_id: channel_id.into().0,
        }))?.await
    }

    pub async fn create_private_channel(
        &self,
        recipient_id: impl Into<UserId>,
    ) -> Result<PrivateChannel> {
        self.request(Request::from((
            serde_json::to_vec(&json!({
                "recipient_id": recipient_id.into(),
            }))?,
            Route::CreatePrivateChannel,
        )))?.await
    }

    pub async fn roles(
        &self,
        guild_id: impl Into<GuildId>,
    ) -> Result<Vec<Role>> {
        self.request(Request::from(Route::GetGuildRoles {
            guild_id: guild_id.into().0,
        }))?.await
    }

    pub fn create_role(&self, guild_id: impl Into<GuildId>) -> CreateRole<'_> {
        CreateRole::new(self, guild_id)
    }

    pub async fn delete_role(
        &self,
        guild_id: impl Into<GuildId>,
        role_id: impl Into<RoleId>,
    ) -> Result<()> {
        self.request(Request::from(Route::DeleteRole {
            guild_id: guild_id.into().0,
            role_id: role_id.into().0,
        }))?.await
    }

    pub fn update_role(
        &self,
        guild_id: impl Into<GuildId>,
        role_id: impl Into<RoleId>,
    ) -> UpdateRole<'_> {
        UpdateRole::new(self, guild_id, role_id)
    }

    pub async fn update_role_positions(
        &self,
        guild_id: impl Into<GuildId>,
        roles: impl Iterator<Item = (RoleId, u64)>,
    ) -> Result<Vec<Role>> {
        self.request(Request::from((
            serde_json::to_vec(&roles.collect::<Vec<_>>())?,
            Route::UpdateRolePositions {
                guild_id: guild_id.into().0,
            },
        )))?.await
    }

    pub async fn user(&self, user_id: u64) -> Result<Option<User>> {
        self.request(Request::from(Route::GetUser {
            target_user: &user_id.to_string(),
        }))?.await
    }

    pub async fn voice_regions(&self) -> Result<Vec<VoiceRegion>> {
        self.request(Request::from(Route::GetVoiceRegions))?.await
    }

    pub fn webhook(&self, id: impl Into<WebhookId>) -> GetWebhook<'_> {
        GetWebhook::new(self, id)
    }

    pub fn create_webhook(
        &self,
        channel_id: ChannelId,
        name: impl Into<String>,
    ) -> CreateWebhook<'_> {
        CreateWebhook::new(self, channel_id,  name)
    }

    pub fn delete_webhook(
        &self,
        id: impl Into<WebhookId>,
    ) -> DeleteWebhook<'_> {
        DeleteWebhook::new(self, id)
    }

    pub fn update_webhook(
        &self,
        webhook_id: impl Into<WebhookId>,
    ) -> UpdateWebhook<'_> {
        UpdateWebhook::new(self, webhook_id)
    }

    pub fn update_webhook_with_token(
        &self,
        webhook_id: impl Into<WebhookId>,
        token: impl Into<String>,
    ) -> UpdateWebhookWithToken<'_> {
        UpdateWebhookWithToken::new(self, webhook_id, token)
    }

    pub fn execute_webhook(
        &self,
        webhook_id: impl Into<WebhookId>,
        token: impl Into<String>,
    ) -> ExecuteWebhook<'_> {
        ExecuteWebhook::new(self, webhook_id, token)
    }

    pub fn request<T: DeserializeOwned>(
        &self,
        request: Request<'_>,
    ) -> Result<PendingBody<'_, T>> {
        let (resp, bucket) = self.make_request(request)?;

        Ok(PendingBody::new(resp.boxed(), &self.state.ratelimiter, bucket))
    }

    pub fn text(&self, request: Request<'_>) -> Result<PendingText<'_>> {
        let (resp, bucket) = self.make_request(request)?;

        Ok(PendingText::new(resp.boxed(), &self.state.ratelimiter, bucket))
    }

    pub fn verify(&self, request: Request<'_>) -> Result<Pending<'_>> {
        let (resp, bucket) = self.make_request(request)?;

        Ok(Pending::new(resp.boxed(), &self.state.ratelimiter, bucket))
    }

    fn make_request(
        &self,
        request: Request<'_>,
    ) -> Result<(impl Future<Output = reqwest::Result<Response>> + Send + Unpin, Path)> {
        let Request {
            body,
            headers: req_headers,
            method,
            path: bucket,
            path_str: path,
        } = request;

        let url = format!("https://discordapp.com/api/v6/{}", path);

        let url = Url::from_str(&url).with_context(|| InvalidUrl {
            method: method.clone(),
            path: path.to_owned(),
        })?;

        let mut builder = self.state.http.request(method.clone(), url);

        if let Some(bytes) = body {
            builder = builder.body(Body::from(bytes));
        }

        if let Some(ref token) = self.state.token {
            let value = HeaderValue::from_str(&format!(
                "Bot {}",
                token,
            )).with_context(|| CreatingHeader {
                name: "Authroization".to_owned(),
            })?;

            builder = builder.header("Authorization", value);
        }

        let content_type = HeaderValue::from_static("application/json");
        let precision = HeaderValue::from_static("millisecond");
        let user_agent = HeaderValue::from_static(concat!(
            "dawn.rs (",
            env!("CARGO_PKG_HOMEPAGE"),
            ") ",
            env!("CARGO_PKG_VERSION"),
        ));
        builder = builder.header("Content-Type", content_type);
        builder = builder.header("X-RateLimit-Precision", precision);
        builder = builder.header("User-Agent", user_agent);

        if let Some(req_headers) = req_headers {
            builder = builder.headers(req_headers);
        }

        Ok((builder.send(), bucket))
    }
}

impl From<ReqwestClient> for Client {
    fn from(reqwest_client: ReqwestClient) -> Self {
        Self {
            state: Arc::new(State {
                http: Arc::new(reqwest_client),
                ratelimiter: Ratelimiter::new(),
                token: None,
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
                token: None,
            }),
        }
    }
}
