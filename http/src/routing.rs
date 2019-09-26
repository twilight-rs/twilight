use http::Method;
use snafu::Snafu;
use std::{borrow::Cow, convert::TryFrom, fmt::Write, num::ParseIntError, str::FromStr};

#[derive(Clone, Debug, Eq, PartialEq, Snafu)]
pub enum PathParseError {
    IntegerParsing { source: ParseIntError },
    MessageIdWithoutMethod { channel_id: u64 },
    NoMatch,
}

impl From<ParseIntError> for PathParseError {
    fn from(source: ParseIntError) -> Self {
        PathParseError::IntegerParsing {
            source,
        }
    }
}

/// An enum representing a path, most useful for ratelimiting implementations.
// If adding to this enum, be sure to add to the `TryFrom` impl.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum Path {
    ChannelsId(u64),
    ChannelsIdInvites(u64),
    ChannelsIdMessages(u64),
    ChannelsIdMessagesBulkDelete(u64),
    ChannelsIdMessagesId(Method, u64),
    ChannelsIdMessagesIdReactions(u64),
    ChannelsIdMessagesIdReactionsUserIdType(u64),
    ChannelsIdPermissionsOverwriteId(u64),
    ChannelsIdPins(u64),
    ChannelsIdPinsMessageId(u64),
    ChannelsIdTyping(u64),
    ChannelsIdWebhooks(u64),
    Gateway,
    GatewayBot,
    Guilds,
    GuildsId(u64),
    GuildsIdBans(u64),
    GuildsIdBansId(u64),
    GuildsIdAuditLogs(u64),
    GuildsIdBansUserId(u64),
    GuildsIdChannels(u64),
    GuildsIdEmbed(u64),
    GuildsIdEmojis(u64),
    GuildsIdEmojisId(u64),
    GuildsIdIntegrations(u64),
    GuildsIdIntegrationsId(u64),
    GuildsIdIntegrationsIdSync(u64),
    GuildsIdInvites(u64),
    GuildsIdMembers(u64),
    GuildsIdMembersId(u64),
    GuildsIdMembersIdRolesId(u64),
    GuildsIdMembersMeNick(u64),
    GuildsIdPrune(u64),
    GuildsIdRegions(u64),
    GuildsIdRoles(u64),
    GuildsIdRolesId(u64),
    GuildsIdVanityUrl(u64),
    GuildsIdWebhooks(u64),
    InvitesCode,
    UsersId,
    UsersIdConnections,
    UsersIdChannels,
    UsersIdGuilds,
    UsersIdGuildsId,
    VoiceRegions,
    WebhooksId(u64),
}

impl FromStr for Path {
    type Err = PathParseError;

    /// Parses a string into a path.
    ///
    /// The string *may* start with a slash (`/`), which will be ignored.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use dawn_http::routing::Path;
    /// use std::str::FromStr;
    ///
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// assert_eq!(Path::VoiceRegions, Path::from_str("/voice/regions")?);
    /// assert_eq!(
    ///     Path::ChannelsIdMessages(123),
    ///     Path::from_str("channels/123/messages")?,
    /// );
    /// # Ok(()) }
    /// ```
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use Path::*;

        let skip = if s.chars().next() == Some('/') { 1 } else { 0 };

        let parts = s.split('/').skip(skip).collect::<Vec<&str>>();

        Ok(match parts.as_slice() {
            ["channels", id] => ChannelsId(id.parse()?),
            ["channels", id, "invites"] => ChannelsIdInvites(id.parse()?),
            ["channels", id, "messages"] => ChannelsIdMessages(id.parse()?),
            ["channels", id, "messages", _] => {
                return Err(PathParseError::MessageIdWithoutMethod {
                    channel_id: id.parse()?,
                });
            },
            ["channels", id, "messages", _, "reactions"] => {
                ChannelsIdMessagesIdReactions(id.parse()?)
            },
            ["channels", id, "messages", _, "reactions", _, _] => {
                ChannelsIdMessagesIdReactionsUserIdType(id.parse()?)
            },
            ["channels", id, "permissions", _] => ChannelsIdPermissionsOverwriteId(id.parse()?),
            ["channels", id, "pins"] => ChannelsIdPins(id.parse()?),
            ["channels", id, "pins", _] => ChannelsIdPinsMessageId(id.parse()?),
            ["channels", id, "typing"] => ChannelsIdTyping(id.parse()?),
            ["channels", id, "webhooks"] => ChannelsIdWebhooks(id.parse()?),
            ["gateway"] => Gateway,
            ["gateway", "bot"] => GatewayBot,
            ["guilds"] => Guilds,
            ["guilds", id] => GuildsId(id.parse()?),
            ["guilds", id, "bans"] => GuildsIdBans(id.parse()?),
            ["guilds", id, "bans", _] => GuildsIdBansUserId(id.parse()?),
            ["guilds", id, "channels"] => GuildsIdChannels(id.parse()?),
            ["guilds", id, "embed"] => GuildsIdEmbed(id.parse()?),
            ["guilds", id, "emojis"] => GuildsIdEmojis(id.parse()?),
            ["guilds", id, "emojis", _] => GuildsIdEmojisId(id.parse()?),
            ["guilds", id, "integrations"] => GuildsIdIntegrations(id.parse()?),
            ["guilds", id, "integrations", _] => GuildsIdIntegrationsId(id.parse()?),
            ["guilds", id, "integrations", _, "sync"] => GuildsIdIntegrationsIdSync(id.parse()?),
            ["guilds", id, "invites"] => GuildsIdInvites(id.parse()?),
            ["guilds", id, "members"] => GuildsIdMembers(id.parse()?),
            ["guilds", id, "members", _] => GuildsIdMembersId(id.parse()?),
            ["guilds", id, "members", _, "roles", _] => GuildsIdMembersIdRolesId(id.parse()?),
            ["guilds", id, "members", "@me", "nick"] => GuildsIdMembersMeNick(id.parse()?),
            ["guilds", id, "prune"] => GuildsIdPrune(id.parse()?),
            ["guilds", id, "regions"] => GuildsIdRegions(id.parse()?),
            ["guilds", id, "roles"] => GuildsIdRoles(id.parse()?),
            ["guilds", id, "roles", _] => GuildsIdRolesId(id.parse()?),
            ["guilds", id, "vanity-url"] => GuildsIdVanityUrl(id.parse()?),
            ["guilds", id, "webhooks"] => GuildsIdWebhooks(id.parse()?),
            ["invites", _] => InvitesCode,
            ["users", _] => UsersId,
            ["users", _, "connections"] => UsersIdConnections,
            ["users", _, "channels"] => UsersIdChannels,
            ["users", _, "guilds"] => UsersIdGuilds,
            ["users", _, "guilds", _] => UsersIdGuildsId,
            ["voice", "regions"] => VoiceRegions,
            ["webhooks", id] => WebhooksId(id.parse()?),
            _ => return Err(PathParseError::NoMatch),
        })
    }
}

impl TryFrom<(Method, &str)> for Path {
    type Error = PathParseError;

    fn try_from((method, s): (Method, &str)) -> Result<Self, Self::Error> {
        match Self::from_str(s) {
            Ok(v) => Ok(v),
            Err(PathParseError::MessageIdWithoutMethod {
                channel_id,
            }) => Ok(Path::ChannelsIdMessagesId(method, channel_id)),
            Err(why) => Err(why),
        }
    }
}

#[derive(Clone, Debug)]
pub enum Route {
    AddMemberRole {
        guild_id: u64,
        role_id: u64,
        user_id: u64,
    },
    CreateBan {
        guild_id: u64,
        user_id: u64,
        delete_message_days: Option<u64>,
        reason: Option<String>,
    },
    CreateChannel {
        guild_id: u64,
    },
    CreateEmoji {
        guild_id: u64,
    },
    CreateGuild,
    CreateGuildIntegration {
        guild_id: u64,
    },
    CreateGuildPrune {
        compute_prune_count: Option<bool>,
        days: Option<u64>,
        guild_id: u64,
    },
    CreateInvite {
        channel_id: u64,
    },
    CreateMessage {
        channel_id: u64,
    },
    CreatePrivateChannel,
    CreateReaction {
        channel_id: u64,
        emoji: String,
        message_id: u64,
    },
    CreateRole {
        guild_id: u64,
    },
    CreateTypingTrigger {
        channel_id: u64,
    },
    CreateWebhook {
        channel_id: u64,
    },
    DeleteBan {
        guild_id: u64,
        user_id: u64,
    },
    DeleteChannel {
        channel_id: u64,
    },
    DeleteEmoji {
        guild_id: u64,
        emoji_id: u64,
    },
    DeleteGuild {
        guild_id: u64,
    },
    DeleteGuildIntegration {
        guild_id: u64,
        integration_id: u64,
    },
    DeleteInvite {
        code: String,
    },
    DeleteMessage {
        channel_id: u64,
        message_id: u64,
    },
    DeleteMessages {
        channel_id: u64,
    },
    DeleteMessageReactions {
        channel_id: u64,
        message_id: u64,
    },
    DeletePermissionOverwrite {
        channel_id: u64,
        target_id: u64,
    },
    DeleteReaction {
        channel_id: u64,
        emoji: String,
        message_id: u64,
        user: String,
    },
    DeleteRole {
        guild_id: u64,
        role_id: u64,
    },
    DeleteWebhook {
        token: Option<String>,
        webhook_id: u64,
    },
    ExecuteWebhook {
        token: String,
        wait: Option<bool>,
        webhook_id: u64,
    },
    GetAuditLogs {
        action_type: Option<u64>,
        before: Option<u64>,
        guild_id: u64,
        limit: Option<u64>,
        user_id: Option<u64>,
    },
    GetBan {
        guild_id: u64,
        user_id: u64,
    },
    GetBans {
        guild_id: u64,
    },
    GetChannel {
        channel_id: u64,
    },
    GetChannelInvites {
        channel_id: u64,
    },
    GetChannelWebhooks {
        channel_id: u64,
    },
    GetChannels {
        guild_id: u64,
    },
    GetEmoji {
        emoji_id: u64,
        guild_id: u64,
    },
    GetEmojis {
        guild_id: u64,
    },
    GetGateway,
    GetGatewayBot,
    GetGuild {
        guild_id: u64,
    },
    GetGuildEmbed {
        guild_id: u64,
    },
    GetGuildIntegrations {
        guild_id: u64,
    },
    GetGuildInvites {
        guild_id: u64,
    },
    GetGuildMembers {
        after: Option<u64>,
        limit: Option<u64>,
        guild_id: u64,
    },
    GetGuildPruneCount {
        days: Option<u64>,
        guild_id: u64,
    },
    GetGuildRoles {
        guild_id: u64,
    },
    GetGuildVanityUrl {
        guild_id: u64,
    },
    GetGuildVoiceRegions {
        guild_id: u64,
    },
    GetGuildWebhooks {
        guild_id: u64,
    },
    GetGuilds {
        after: Option<u64>,
        before: Option<u64>,
        limit: Option<u64>,
    },
    GetInvite {
        code: String,
        with_counts: bool,
    },
    GetMember {
        guild_id: u64,
        user_id: u64,
    },
    GetMessage {
        channel_id: u64,
        message_id: u64,
    },
    GetMessages {
        after: Option<u64>,
        around: Option<u64>,
        before: Option<u64>,
        channel_id: u64,
        limit: Option<u64>,
    },
    GetPins {
        channel_id: u64,
    },
    GetReactionUsers {
        after: Option<u64>,
        before: Option<u64>,
        channel_id: u64,
        emoji: String,
        limit: Option<u64>,
        message_id: u64,
    },
    GetUser {
        target_user: String,
    },
    GetUserConnections,
    GetUserPrivateChannels,
    GetVoiceRegions,
    GetWebhook {
        token: Option<String>,
        webhook_id: u64,
    },
    LeaveGuild {
        guild_id: u64,
    },
    PinMessage {
        channel_id: u64,
        message_id: u64,
    },
    RemoveMember {
        guild_id: u64,
        user_id: u64,
    },
    RemoveMemberRole {
        guild_id: u64,
        role_id: u64,
        user_id: u64,
    },
    SyncGuildIntegration {
        guild_id: u64,
        integration_id: u64,
    },
    UnpinMessage {
        channel_id: u64,
        message_id: u64,
    },
    UpdateChannel {
        channel_id: u64,
    },
    UpdateCurrentUser,
    UpdateEmoji {
        guild_id: u64,
        emoji_id: u64,
    },
    UpdateGuild {
        guild_id: u64,
    },
    UpdateGuildChannels {
        guild_id: u64,
    },
    UpdateGuildEmbed {
        guild_id: u64,
    },
    UpdateGuildIntegration {
        guild_id: u64,
        integration_id: u64,
    },
    UpdateMember {
        guild_id: u64,
        user_id: u64,
    },
    UpdateMessage {
        channel_id: u64,
        message_id: u64,
    },
    UpdateNickname {
        guild_id: u64,
    },
    UpdatePermissionOverwrite {
        channel_id: u64,
        target_id: u64,
    },
    UpdateRole {
        guild_id: u64,
        role_id: u64,
    },
    UpdateRolePositions {
        guild_id: u64,
    },
    UpdateWebhook {
        token: Option<String>,
        webhook_id: u64,
    },
}

impl Route {
    // This function contains some `write!`s, but they can't fail, so we ignore
    // them to remove an unnecessary Result here.
    pub fn into_parts(self) -> (Method, Path, Cow<'static, str>) {
        match self {
            Route::AddMemberRole {
                guild_id,
                role_id,
                user_id,
            } => (
                Method::PUT,
                Path::GuildsIdMembersIdRolesId(guild_id),
                format!("guilds/{}/members/{}/roles/{}", guild_id, user_id, role_id).into(),
            ),
            Route::CreateBan {
                guild_id,
                delete_message_days,
                reason,
                user_id,
            } => {
                let mut path = format!("guilds/{}/bans/{}", guild_id, user_id);

                if let Some(delete_message_days) = delete_message_days {
                    let _ = write!(path, "delete-message-days={}", delete_message_days);
                }

                if let Some(reason) = reason {
                    let _ = write!(path, "&reason={}", reason);
                }

                (Method::PUT, Path::GuildsIdBansUserId(guild_id), path.into())
            },
            Route::CreateChannel {
                guild_id,
            } => (
                Method::POST,
                Path::GuildsIdChannels(guild_id),
                format!("guilds/{}/channels", guild_id).into(),
            ),
            Route::CreateEmoji {
                guild_id,
            } => (
                Method::POST,
                Path::GuildsIdEmojis(guild_id),
                format!("guilds/{}/emojis", guild_id).into(),
            ),
            Route::CreateGuild => (Method::POST, Path::Guilds, "guilds".into()),
            Route::CreateGuildIntegration {
                guild_id,
            } => (
                Method::POST,
                Path::GuildsIdIntegrationsId(guild_id),
                format!("guilds/{}/integrations", guild_id).into(),
            ),
            Route::CreateGuildPrune {
                compute_prune_count,
                days,
                guild_id,
            } => {
                let mut path = format!("guilds/{}/prune?", guild_id);

                if let Some(compute_prune_count) = compute_prune_count {
                    let _ = write!(path, "compute_prune_count={}", compute_prune_count,);
                }

                if let Some(days) = days {
                    let _ = write!(path, "&days={}", days);
                }

                (Method::POST, Path::GuildsIdPrune(guild_id), path.into())
            },
            Route::CreateInvite {
                channel_id,
            } => (
                Method::POST,
                Path::ChannelsIdInvites(channel_id),
                format!("channels/{}/invites", channel_id).into(),
            ),
            Route::CreateMessage {
                channel_id,
            } => (
                Method::POST,
                Path::ChannelsIdMessages(channel_id),
                format!("channels/{}/messages", channel_id).into(),
            ),
            Route::CreatePrivateChannel => (
                Method::POST,
                Path::UsersIdChannels,
                "users/@me/channels".into(),
            ),
            Route::CreateReaction {
                channel_id,
                emoji,
                message_id,
            } => (
                Method::PUT,
                Path::ChannelsIdMessagesIdReactionsUserIdType(channel_id),
                format!(
                    "channels/{}/messages/{}/reactions/{}",
                    channel_id, message_id, emoji,
                )
                .into(),
            ),
            Route::CreateRole {
                guild_id,
            } => (
                Method::POST,
                Path::GuildsIdRoles(guild_id),
                format!("guilds/{}/roles", guild_id).into(),
            ),
            Route::CreateTypingTrigger {
                channel_id,
            } => (
                Method::POST,
                Path::ChannelsIdTyping(channel_id),
                format!("channels/{}/typing", channel_id).into(),
            ),
            Route::CreateWebhook {
                channel_id,
            } => (
                Method::POST,
                Path::ChannelsIdWebhooks(channel_id),
                format!("channels/{}/webhooks", channel_id).into(),
            ),
            Route::DeleteBan {
                guild_id,
                user_id,
            } => (
                Method::DELETE,
                Path::GuildsIdBansUserId(guild_id),
                format!("guilds/{}/bans/{}", guild_id, user_id).into(),
            ),
            Route::DeleteChannel {
                channel_id,
            } => (
                Method::DELETE,
                Path::ChannelsId(channel_id),
                format!("channels/{}", channel_id).into(),
            ),
            Route::DeleteEmoji {
                emoji_id,
                guild_id,
            } => (
                Method::DELETE,
                Path::GuildsIdEmojisId(guild_id),
                format!("guilds/{}/emojis/{}", guild_id, emoji_id).into(),
            ),
            Route::DeleteGuild {
                guild_id,
            } => (
                Method::DELETE,
                Path::GuildsId(guild_id),
                format!("guilds/{}", guild_id).into(),
            ),
            Route::DeleteGuildIntegration {
                guild_id,
                integration_id,
            } => (
                Method::DELETE,
                Path::GuildsIdIntegrationsId(guild_id),
                format!("guilds/{}/integrations/{}", guild_id, integration_id).into(),
            ),
            Route::DeleteInvite {
                code,
            } => (
                Method::DELETE,
                Path::InvitesCode,
                format!("invites/{}", code).into(),
            ),
            Route::DeleteMessageReactions {
                channel_id,
                message_id,
            } => (
                Method::DELETE,
                Path::ChannelsIdMessagesIdReactions(channel_id),
                format!("channels/{}/messages/{}/reactions", channel_id, message_id).into(),
            ),
            Route::DeleteMessage {
                channel_id,
                message_id,
            } => (
                Method::DELETE,
                Path::ChannelsIdMessagesId(Method::DELETE, message_id),
                format!("channels/{}/messages/{}", channel_id, message_id).into(),
            ),
            Route::DeleteMessages {
                channel_id,
            } => (
                Method::POST,
                Path::ChannelsIdMessagesBulkDelete(channel_id),
                format!("channels/{}/messages/bulk-delete", channel_id).into(),
            ),
            Route::DeletePermissionOverwrite {
                channel_id,
                target_id,
            } => (
                Method::DELETE,
                Path::ChannelsIdPermissionsOverwriteId(channel_id),
                format!("channels/{}/permissions/{}", channel_id, target_id).into(),
            ),
            Route::DeleteReaction {
                channel_id,
                emoji,
                message_id,
                user,
            } => (
                Method::DELETE,
                Path::ChannelsIdMessagesIdReactionsUserIdType(channel_id),
                format!(
                    "channels/{}/messages/{}/reactions/{}/{}",
                    channel_id, message_id, emoji, user,
                )
                .into(),
            ),
            Route::DeleteRole {
                guild_id,
                role_id,
            } => (
                Method::DELETE,
                Path::GuildsIdRolesId(guild_id),
                format!("guilds/{}/roles/{}", guild_id, role_id).into(),
            ),
            Route::DeleteWebhook {
                token,
                webhook_id,
            } => {
                let mut path = format!("webhooks/{}", webhook_id);

                if let Some(token) = token {
                    path.push('/');
                    path.push_str(&token);
                }

                (Method::DELETE, Path::WebhooksId(webhook_id), path.into())
            },
            Route::ExecuteWebhook {
                token,
                wait,
                webhook_id,
            } => {
                let mut path = format!("webhooks/{}/{}", webhook_id, token);

                if let Some(wait) = wait {
                    let _ = write!(path, "?wait={}", wait);
                }

                (Method::POST, Path::WebhooksId(webhook_id), path.into())
            },
            Route::GetAuditLogs {
                action_type,
                before,
                guild_id,
                limit,
                user_id,
            } => {
                let mut path = format!("guilds/{}/audit-logs", guild_id);

                if let Some(action_type) = action_type {
                    let _ = write!(path, "action_type={}", action_type);
                }

                if let Some(before) = before {
                    let _ = write!(path, "&before={}", before);
                }

                if let Some(limit) = limit {
                    let _ = write!(path, "&limit={}", limit);
                }

                if let Some(user_id) = user_id {
                    let _ = write!(path, "&user_id={}", user_id);
                }

                (Method::GET, Path::GuildsIdAuditLogs(guild_id), path.into())
            },
            Route::GetBan {
                guild_id,
                user_id,
            } => (
                Method::GET,
                Path::GuildsIdBansId(guild_id),
                format!("guilds/{}/bans/{}", guild_id, user_id).into(),
            ),
            Route::GetBans {
                guild_id,
            } => (
                Method::GET,
                Path::GuildsIdBans(guild_id),
                format!("guilds/{}/bans", guild_id).into(),
            ),
            Route::GetGatewayBot => (Method::GET, Path::GatewayBot, "gateway/bot".into()),
            Route::GetChannel {
                channel_id,
            } => (
                Method::GET,
                Path::ChannelsId(channel_id),
                format!("channels/{}", channel_id).into(),
            ),
            Route::GetChannelInvites {
                channel_id,
            } => (
                Method::GET,
                Path::ChannelsIdInvites(channel_id),
                format!("channels/{}/invites", channel_id).into(),
            ),
            Route::GetChannelWebhooks {
                channel_id,
            } => (
                Method::GET,
                Path::ChannelsIdWebhooks(channel_id),
                format!("channels/{}/webhooks", channel_id).into(),
            ),
            Route::GetChannels {
                guild_id,
            } => (
                Method::GET,
                Path::GuildsIdChannels(guild_id),
                format!("guilds/{}/channels", guild_id).into(),
            ),
            Route::GetEmoji {
                emoji_id,
                guild_id,
            } => (
                Method::GET,
                Path::GuildsIdEmojisId(guild_id),
                format!("guilds/{}/emojis/{}", guild_id, emoji_id).into(),
            ),
            Route::GetEmojis {
                guild_id,
            } => (
                Method::GET,
                Path::GuildsIdEmojis(guild_id),
                format!("guilds/{}/emojis", guild_id).into(),
            ),
            Route::GetGateway => (Method::GET, Path::Gateway, "gateway".into()),
            Route::GetGuild {
                guild_id,
            } => (
                Method::GET,
                Path::GuildsId(guild_id),
                format!("guilds/{}", guild_id).into(),
            ),
            Route::GetGuildEmbed {
                guild_id,
            } => (
                Method::GET,
                Path::GuildsIdEmbed(guild_id),
                format!("guilds/{}/embed", guild_id).into(),
            ),
            Route::GetGuildIntegrations {
                guild_id,
            } => (
                Method::GET,
                Path::GuildsIdIntegrations(guild_id),
                format!("guilds/{}/integrations", guild_id).into(),
            ),
            Route::GetGuildInvites {
                guild_id,
            } => (
                Method::GET,
                Path::GuildsIdInvites(guild_id),
                format!("guilds/{}/invites", guild_id).into(),
            ),
            Route::GetGuildMembers {
                after,
                guild_id,
                limit,
            } => {
                let mut path = format!("guilds/{}/members?", guild_id);

                if let Some(after) = after {
                    let _ = write!(path, "after={}", after);
                }

                if let Some(limit) = limit {
                    let _ = write!(path, "&limit={}", limit);
                }

                (Method::GET, Path::GuildsIdMembers(guild_id), path.into())
            },
            Route::GetGuildPruneCount {
                days,
                guild_id,
            } => {
                let mut path = format!("guilds/{}/prune?", guild_id);

                if let Some(days) = days {
                    let _ = write!(path, "days={}", days);
                }

                (Method::GET, Path::GuildsIdPrune(guild_id), path.into())
            },
            Route::GetGuildRoles {
                guild_id,
            } => (
                Method::GET,
                Path::GuildsIdRoles(guild_id),
                format!("guilds/{}/roles", guild_id).into(),
            ),
            Route::GetGuildVanityUrl {
                guild_id,
            } => (
                Method::GET,
                Path::GuildsIdVanityUrl(guild_id),
                format!("guilds/{}/vanity-url", guild_id).into(),
            ),
            Route::GetGuildVoiceRegions {
                guild_id,
            } => (
                Method::GET,
                Path::GuildsIdRegions(guild_id),
                format!("guilds/{}/regions", guild_id).into(),
            ),
            Route::GetGuildWebhooks {
                guild_id,
            } => (
                Method::GET,
                Path::GuildsIdWebhooks(guild_id),
                format!("guilds/{}/webhooks", guild_id).into(),
            ),
            Route::GetGuilds {
                after,
                before,
                limit,
            } => {
                let mut path = "users/@me/guilds?".to_owned();

                if let Some(after) = after {
                    let _ = write!(path, "after={}", after);
                }

                if let Some(before) = before {
                    let _ = write!(path, "&before={}", before);
                }

                if let Some(limit) = limit {
                    let _ = write!(path, "&limit={}", limit);
                }

                (Method::GET, Path::UsersIdGuilds, path.into())
            },
            Route::GetInvite {
                code,
                with_counts,
            } => (
                Method::GET,
                Path::InvitesCode,
                format!("invites/{}?with-counts={}", code, with_counts).into(),
            ),
            Route::GetMember {
                guild_id,
                user_id,
            } => (
                Method::GET,
                Path::GuildsIdMembersId(guild_id),
                format!("guilds/{}/members/{}", guild_id, user_id).into(),
            ),
            Route::GetMessage {
                channel_id,
                message_id,
            } => (
                Method::GET,
                Path::ChannelsIdMessagesId(Method::GET, channel_id),
                format!("channels/{}/messages/{}", channel_id, message_id).into(),
            ),
            Route::GetMessages {
                channel_id,
                after,
                around,
                before,
                limit,
            } => {
                let mut path = format!("channels/{}/messages?", channel_id);

                if let Some(after) = after {
                    let _ = write!(path, "after={}", after);
                }

                if let Some(around) = around {
                    let _ = write!(path, "&around={}", around);
                }

                if let Some(before) = before {
                    let _ = write!(path, "&before={}", before);
                }

                if let Some(limit) = limit {
                    let _ = write!(path, "&limit={}", limit);
                }

                (
                    Method::GET,
                    Path::ChannelsIdMessages(channel_id),
                    path.into(),
                )
            },
            Route::GetPins {
                channel_id,
            } => (
                Method::GET,
                Path::ChannelsIdPins(channel_id),
                format!("channels/{}/pins", channel_id).into(),
            ),
            Route::GetReactionUsers {
                after,
                before,
                channel_id,
                ref emoji,
                limit,
                message_id,
            } => {
                let mut path = format!(
                    "channels/{}/messages/{}/reactions/{}?",
                    channel_id, message_id, emoji,
                );

                if let Some(after) = after {
                    let _ = write!(path, "after={}", after);
                }

                if let Some(before) = before {
                    let _ = write!(path, "before={}", before);
                }

                if let Some(limit) = limit {
                    let _ = write!(path, "&limit={}", limit);
                }

                (
                    Method::GET,
                    Path::ChannelsIdMessagesIdReactions(channel_id),
                    path.into(),
                )
            },
            Route::GetUserConnections => (
                Method::GET,
                Path::UsersIdConnections,
                "users/@me/connections".into(),
            ),
            Route::GetUserPrivateChannels => (
                Method::GET,
                Path::UsersIdChannels,
                "users/@me/channels".into(),
            ),
            Route::GetUser {
                target_user,
            } => (
                Method::GET,
                Path::UsersId,
                format!("users/{}", target_user).into(),
            ),
            Route::GetVoiceRegions => (Method::GET, Path::VoiceRegions, "voice/regions".into()),
            Route::GetWebhook {
                token,
                webhook_id,
            } => {
                let mut path = format!("webhooks/{}", webhook_id);

                if let Some(token) = token {
                    path.push('/');
                    path.push_str(&token);
                }

                (Method::GET, Path::WebhooksId(webhook_id), path.into())
            },
            Route::LeaveGuild {
                guild_id,
            } => (
                Method::DELETE,
                Path::UsersIdGuildsId,
                format!("users/@me/guilds/{}", guild_id).into(),
            ),
            Route::PinMessage {
                channel_id,
                message_id,
            } => (
                Method::PUT,
                Path::ChannelsIdPins(channel_id),
                format!("channels/{}/pins/{}", channel_id, message_id).into(),
            ),
            Route::RemoveMember {
                guild_id,
                user_id,
            } => (
                Method::DELETE,
                Path::GuildsIdMembersId(guild_id),
                format!("guilds/{}/members/{}", guild_id, user_id).into(),
            ),
            Route::RemoveMemberRole {
                guild_id,
                role_id,
                user_id,
            } => (
                Method::DELETE,
                Path::GuildsIdMembersIdRolesId(guild_id),
                format!("guilds/{}/members/{}/roles/{}", guild_id, user_id, role_id).into(),
            ),
            Route::SyncGuildIntegration {
                guild_id,
                integration_id,
            } => (
                Method::POST,
                Path::GuildsIdIntegrationsIdSync(guild_id),
                format!("guilds/{}/integrations/{}/sync", guild_id, integration_id).into(),
            ),
            Route::UnpinMessage {
                channel_id,
                message_id,
            } => (
                Method::DELETE,
                Path::ChannelsIdPinsMessageId(channel_id),
                format!("channels/{}/pins/{}", channel_id, message_id).into(),
            ),
            Route::UpdateChannel {
                channel_id,
            } => (
                Method::PATCH,
                Path::ChannelsId(channel_id),
                format!("channels/{}", channel_id).into(),
            ),
            Route::UpdateCurrentUser => (Method::PATCH, Path::UsersId, "users/@me".into()),
            Route::UpdateEmoji {
                emoji_id,
                guild_id,
            } => (
                Method::PATCH,
                Path::GuildsIdEmojisId(guild_id),
                format!("guilds/{}/emojis/{}", guild_id, emoji_id).into(),
            ),
            Route::UpdateGuild {
                guild_id,
            } => (
                Method::PATCH,
                Path::GuildsId(guild_id),
                format!("guilds/{}", guild_id).into(),
            ),
            Route::UpdateGuildChannels {
                guild_id,
            } => (
                Method::PATCH,
                Path::GuildsIdChannels(guild_id),
                format!("guilds/{}/channels", guild_id).into(),
            ),
            Route::UpdateGuildEmbed {
                guild_id,
            } => (
                Method::PATCH,
                Path::GuildsIdEmbed(guild_id),
                format!("guilds/{}/embed", guild_id).into(),
            ),
            Route::UpdateGuildIntegration {
                guild_id,
                integration_id,
            } => (
                Method::PATCH,
                Path::GuildsIdIntegrationsId(guild_id),
                format!("guilds/{}/integrations/{}", guild_id, integration_id,).into(),
            ),
            Route::UpdateMember {
                guild_id,
                user_id,
            } => (
                Method::PATCH,
                Path::GuildsIdMembersId(guild_id),
                format!("guilds/{}/members/{}", guild_id, user_id).into(),
            ),
            Route::UpdateMessage {
                channel_id,
                message_id,
            } => (
                Method::PATCH,
                Path::ChannelsIdMessagesId(Method::PATCH, channel_id),
                format!("channels/{}/messages/{}", channel_id, message_id).into(),
            ),
            Route::UpdateNickname {
                guild_id,
            } => (
                Method::PATCH,
                Path::GuildsIdMembersMeNick(guild_id),
                format!("guilds/{}/members/@me/nick", guild_id).into(),
            ),
            Route::UpdatePermissionOverwrite {
                channel_id,
                target_id,
            } => (
                Method::PUT,
                Path::ChannelsIdPermissionsOverwriteId(channel_id),
                format!("channels/{}/permissions/{}", channel_id, target_id).into(),
            ),
            Route::UpdateRole {
                guild_id,
                role_id,
            } => (
                Method::PATCH,
                Path::GuildsIdRolesId(guild_id),
                format!("guilds/{}/roles/{}", guild_id, role_id).into(),
            ),
            Route::UpdateRolePositions {
                guild_id,
            } => (
                Method::PATCH,
                Path::GuildsIdRolesId(guild_id),
                format!("guilds/{}/roles", guild_id).into(),
            ),
            Route::UpdateWebhook {
                token,
                webhook_id,
            } => {
                let mut path = format!("webhooks/{}", webhook_id);

                if let Some(token) = token {
                    path.push('/');
                    path.push_str(&token);
                }

                (Method::PATCH, Path::WebhooksId(webhook_id), path.into())
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{Path, PathParseError};
    use http::Method;
    use std::{convert::TryFrom, error::Error, str::FromStr};

    #[test]
    fn test_path_prefix_unimportant() -> Result<(), Box<dyn Error>> {
        assert_eq!(Path::Guilds, Path::from_str("guilds")?);
        assert_eq!(Path::Guilds, Path::from_str("/guilds")?);

        Ok(())
    }

    #[test]
    fn test_path_from_str() -> Result<(), Box<dyn Error>> {
        assert_eq!(Path::ChannelsId(123), Path::from_str("/channels/123")?);
        assert_eq!(Path::WebhooksId(123), Path::from_str("/webhooks/123")?);
        assert_eq!(Path::InvitesCode, Path::from_str("/invites/abc")?);

        Ok(())
    }

    #[test]
    fn test_path_message_id() -> Result<(), Box<dyn Error>> {
        assert_eq!(
            PathParseError::MessageIdWithoutMethod {
                channel_id: 123
            },
            Path::from_str("channels/123/messages/456").unwrap_err()
        );
        assert_eq!(
            Path::ChannelsIdMessagesId(Method::GET, 123),
            Path::try_from((Method::GET, "/channels/123/messages/456"))?,
        );

        Ok(())
    }
}
