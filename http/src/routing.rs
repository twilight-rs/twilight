use crate::request::Method;
use percent_encoding::{utf8_percent_encode, NON_ALPHANUMERIC};
use std::{
    borrow::Cow,
    convert::TryFrom,
    error::Error,
    fmt::{Display, Formatter, Result as FmtResult, Write},
    str::FromStr,
};

#[derive(Debug)]
pub struct PathParseError {
    kind: PathParseErrorType,
    source: Option<Box<dyn Error + Send + Sync>>,
}

impl PathParseError {
    /// Immutable reference to the type of error that occurred.
    #[must_use = "retrieving the type has no effect if left unused"]
    pub fn kind(&self) -> &PathParseErrorType {
        &self.kind
    }

    /// Consume the error, returning the source error if there is any.
    #[must_use = "consuming the error and retrieving the source has no effect if left unused"]
    pub fn into_source(self) -> Option<Box<dyn Error + Send + Sync>> {
        self.source
    }

    /// Consume the error, returning the owned error type and the source error.
    #[must_use = "consuming the error into its parts has no effect if left unused"]
    pub fn into_parts(self) -> (PathParseErrorType, Option<Box<dyn Error + Send + Sync>>) {
        (self.kind, self.source)
    }
}

impl Display for PathParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match &self.kind {
            PathParseErrorType::IntegerParsing { .. } => {
                f.write_str("An ID in a segment was invalid")
            }
            PathParseErrorType::MessageIdWithoutMethod { .. } => {
                f.write_str("A message path was detected but the method wasn't given")
            }
            PathParseErrorType::NoMatch => f.write_str("There was no matched path"),
        }
    }
}

impl Error for PathParseError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        self.source
            .as_ref()
            .map(|source| &**source as &(dyn Error + 'static))
    }
}

#[derive(Debug)]
#[non_exhaustive]
pub enum PathParseErrorType {
    /// The ID couldn't be parsed as an integer.
    IntegerParsing,
    /// When parsing into a [`Path::ChannelsIdMessagesId`] variant, the method
    /// must also be specified via its `TryFrom` impl.
    MessageIdWithoutMethod {
        /// The ID of the channel.
        channel_id: u64,
    },
    /// A static path for the provided path string wasn't found.
    NoMatch,
}

/// An enum representing a path, most useful for ratelimiting implementations.
// If adding to this enum, be sure to add to the `TryFrom` impl.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
#[non_exhaustive]
pub enum Path {
    /// Operating on a channel.
    ChannelsId(u64),
    /// Operating on a channel's invites.
    ChannelsIdInvites(u64),
    /// Operating on a channel's messages.
    ChannelsIdMessages(u64),
    /// Operating on a channel's messages by bulk deleting.
    ChannelsIdMessagesBulkDelete(u64),
    /// Operating on an individual channel's message.
    ChannelsIdMessagesId(Method, u64),
    /// Crossposting an individual channel's message.
    ChannelsIdMessagesIdCrosspost(u64),
    /// Operating on an individual channel's message's reactions.
    ChannelsIdMessagesIdReactions(u64),
    /// Operating on an individual channel's message's reactions while
    /// specifying the user ID and emoji type.
    ChannelsIdMessagesIdReactionsUserIdType(u64),
    /// Operating on a channel's permission overwrites by ID.
    ChannelsIdPermissionsOverwriteId(u64),
    /// Operating on a channel's pins.
    ChannelsIdPins(u64),
    /// Operating on a channel's individual pinned message.
    ChannelsIdPinsMessageId(u64),
    /// Operating on a group DM's recipients.
    ChannelsIdRecipients(u64),
    /// Operating on a channel's typing indicator.
    ChannelsIdTyping(u64),
    /// Operating on a channel's webhooks.
    ChannelsIdWebhooks(u64),
    /// Operating on a channel's followers.
    ChannelsIdFollowers(u64),
    /// Operating with the gateway information.
    Gateway,
    /// Operating with the gateway information tailored to the current user.
    GatewayBot,
    /// Operating on the guild resource.
    Guilds,
    /// Operating on one of user's guilds.
    GuildsId(u64),
    GuildsIdBans(u64),
    GuildsIdBansId(u64),
    GuildsIdAuditLogs(u64),
    GuildsIdBansUserId(u64),
    GuildsIdChannels(u64),
    GuildsIdWidget(u64),
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
    GuildsIdMembersSearch(u64),
    GuildsIdPreview(u64),
    GuildsIdPrune(u64),
    GuildsIdRegions(u64),
    GuildsIdRoles(u64),
    GuildsIdRolesId(u64),
    GuildsIdTemplates(u64),
    GuildsIdTemplatesCode(u64),
    GuildsIdVanityUrl(u64),
    GuildsIdVoiceStates(u64),
    GuildsIdWelcomeScreen(u64),
    GuildsIdWebhooks(u64),
    InvitesCode,
    UsersId,
    OauthApplicationsMe,
    UsersIdConnections,
    UsersIdChannels,
    /// Operating on the state of a guild that the user is in.
    UsersIdGuilds,
    /// Operating on the state of a guild that the user is in.
    UsersIdGuildsId,
    /// Operating on the voice regions available to the current user.
    VoiceRegions,
    /// Operating on a message created by a webhook.
    WebhooksIdTokenMessagesId(u64),
    /// Operating on a webhook.
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
    /// use twilight_http::routing::Path;
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
    #[allow(clippy::enum_glob_use)]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use Path::*;

        fn parse_id(id: &str) -> Result<u64, PathParseError> {
            id.parse().map_err(|source| PathParseError {
                kind: PathParseErrorType::IntegerParsing,
                source: Some(Box::new(source)),
            })
        }

        let skip = usize::from(s.starts_with('/'));

        let parts = s.split('/').skip(skip).collect::<Vec<&str>>();

        Ok(match parts.as_slice() {
            ["channels", id] => ChannelsId(parse_id(id)?),
            ["channels", id, "followers"] => ChannelsIdFollowers(parse_id(id)?),
            ["channels", id, "invites"] => ChannelsIdInvites(parse_id(id)?),
            ["channels", id, "messages"] => ChannelsIdMessages(parse_id(id)?),
            ["channels", id, "messages", "bulk-delete"] => {
                ChannelsIdMessagesBulkDelete(parse_id(id)?)
            }
            ["channels", id, "messages", _] => {
                // can not map to path without method since they have different ratelimits
                return Err(PathParseError {
                    kind: PathParseErrorType::MessageIdWithoutMethod {
                        channel_id: parse_id(id)?,
                    },
                    source: None,
                });
            }
            ["channels", id, "messages", _, "crosspost"] => {
                ChannelsIdMessagesIdCrosspost(parse_id(id)?)
            }
            ["channels", id, "messages", _, "reactions"]
            | ["channels", id, "messages", _, "reactions", _] => {
                ChannelsIdMessagesIdReactions(parse_id(id)?)
            }
            ["channels", id, "messages", _, "reactions", _, _] => {
                ChannelsIdMessagesIdReactionsUserIdType(parse_id(id)?)
            }
            ["channels", id, "permissions", _] => ChannelsIdPermissionsOverwriteId(parse_id(id)?),
            ["channels", id, "pins"] => ChannelsIdPins(parse_id(id)?),
            ["channels", id, "pins", _] => ChannelsIdPinsMessageId(parse_id(id)?),
            ["channels", id, "recipients"] | ["channels", id, "recipients", _] => {
                ChannelsIdRecipients(parse_id(id)?)
            }
            ["channels", id, "typing"] => ChannelsIdTyping(parse_id(id)?),
            ["channels", id, "webhooks"] | ["channels", id, "webhooks", _] => {
                ChannelsIdWebhooks(parse_id(id)?)
            }
            ["gateway"] => Gateway,
            ["gateway", "bot"] => GatewayBot,
            ["guilds"] => Guilds,
            ["guilds", id] => GuildsId(parse_id(id)?),
            ["guilds", id, "audit-logs"] => GuildsIdAuditLogs(parse_id(id)?),
            ["guilds", id, "bans"] => GuildsIdBans(parse_id(id)?),
            ["guilds", id, "bans", _] => GuildsIdBansUserId(parse_id(id)?),
            ["guilds", id, "channels"] => GuildsIdChannels(parse_id(id)?),
            ["guilds", id, "widget"] | ["guilds", id, "widget.json"] => {
                GuildsIdWidget(parse_id(id)?)
            }
            ["guilds", id, "emojis"] => GuildsIdEmojis(parse_id(id)?),
            ["guilds", id, "emojis", _] => GuildsIdEmojisId(parse_id(id)?),
            ["guilds", id, "integrations"] => GuildsIdIntegrations(parse_id(id)?),
            ["guilds", id, "integrations", _] => GuildsIdIntegrationsId(parse_id(id)?),
            ["guilds", id, "integrations", _, "sync"] => GuildsIdIntegrationsIdSync(parse_id(id)?),
            ["guilds", id, "invites"] => GuildsIdInvites(parse_id(id)?),
            ["guilds", id, "members"] => GuildsIdMembers(parse_id(id)?),
            ["guilds", id, "members", "search"] => GuildsIdMembersSearch(parse_id(id)?),
            ["guilds", id, "members", _] => GuildsIdMembersId(parse_id(id)?),
            ["guilds", id, "members", _, "roles", _] => GuildsIdMembersIdRolesId(parse_id(id)?),
            ["guilds", id, "members", "@me", "nick"] => GuildsIdMembersMeNick(parse_id(id)?),
            ["guilds", id, "preview"] => GuildsIdPreview(parse_id(id)?),
            ["guilds", id, "prune"] => GuildsIdPrune(parse_id(id)?),
            ["guilds", id, "regions"] => GuildsIdRegions(parse_id(id)?),
            ["guilds", id, "roles"] => GuildsIdRoles(parse_id(id)?),
            ["guilds", id, "roles", _] => GuildsIdRolesId(parse_id(id)?),
            ["guilds", id, "templates"] => GuildsIdTemplates(parse_id(id)?),
            ["guilds", id, "templates", _] => GuildsIdTemplatesCode(parse_id(id)?),
            ["guilds", id, "vanity-url"] => GuildsIdVanityUrl(parse_id(id)?),
            ["guilds", id, "voice-states", _] => GuildsIdVoiceStates(parse_id(id)?),
            ["guilds", id, "welcome-screen"] => GuildsIdWelcomeScreen(parse_id(id)?),
            ["guilds", id, "webhooks"] => GuildsIdWebhooks(parse_id(id)?),
            ["invites", _] => InvitesCode,
            ["oauth2", "applications", "@me"] => OauthApplicationsMe,
            ["users", _] => UsersId,
            ["users", _, "connections"] => UsersIdConnections,
            ["users", _, "channels"] => UsersIdChannels,
            ["users", _, "guilds"] => UsersIdGuilds,
            ["users", _, "guilds", _] => UsersIdGuildsId,
            ["voice", "regions"] => VoiceRegions,
            ["webhooks", id] | ["webhooks", id, _] => WebhooksId(parse_id(id)?),
            ["webhooks", id, _, "messages", _] => WebhooksIdTokenMessagesId(parse_id(id)?),
            _ => {
                return Err(PathParseError {
                    kind: PathParseErrorType::NoMatch,
                    source: None,
                })
            }
        })
    }
}

impl TryFrom<(Method, &str)> for Path {
    type Error = PathParseError;

    fn try_from((method, s): (Method, &str)) -> Result<Self, Self::Error> {
        match Self::from_str(s) {
            Ok(v) => Ok(v),
            Err(why) => {
                if let PathParseErrorType::MessageIdWithoutMethod { channel_id } = why.kind() {
                    Ok(Self::ChannelsIdMessagesId(method, *channel_id))
                } else {
                    Err(why)
                }
            }
        }
    }
}

#[derive(Clone, Debug)]
#[non_exhaustive]
pub enum Route {
    /// Route information to add a user to a guild.
    AddGuildMember { guild_id: u64, user_id: u64 },
    /// Route information to add a role to guild member.
    AddMemberRole {
        /// The ID of the guild.
        guild_id: u64,
        /// The ID of the role.
        role_id: u64,
        /// The ID of the user.
        user_id: u64,
    },
    /// Route information to create a ban on a user in a guild.
    CreateBan {
        /// The number of days' worth of the user's messages to delete in the
        /// guild's channels.
        delete_message_days: Option<u64>,
        /// The ID of the guild.
        guild_id: u64,
        /// The reason for the ban.
        reason: Option<String>,
        /// The ID of the user.
        user_id: u64,
    },
    /// Route information to create a channel in a guild.
    CreateChannel {
        /// The ID of the guild.
        guild_id: u64,
    },
    /// Route information to create an emoji in a guild.
    CreateEmoji {
        /// The ID of the guild.
        guild_id: u64,
    },
    /// Route information to create a guild.
    CreateGuild,
    /// Route information to create a guild from a template.
    CreateGuildFromTemplate {
        /// Code of the template.
        template_code: String,
    },
    /// Route information to create a guild's integration.
    CreateGuildIntegration {
        /// The ID of the guild.
        guild_id: u64,
    },
    /// Route information to create a prune in a guild.
    CreateGuildPrune {
        /// Whether to compute the number of pruned users.
        compute_prune_count: Option<bool>,
        /// The number of days that a user must be offline before being able to
        /// be pruned.
        days: Option<u64>,
        /// The ID of the guild.
        guild_id: u64,
        /// The roles to filter the prune by.
        ///
        /// A user must have at least one of these roles to be able to be
        /// pruned.
        include_roles: Vec<u64>,
    },
    /// Route information to create an invite to a channel.
    CreateInvite {
        /// The ID of the channel.
        channel_id: u64,
    },
    /// Route information to create a message in a channel.
    CreateMessage {
        /// The ID of the channel.
        channel_id: u64,
    },
    /// Route information to create a private channel.
    CreatePrivateChannel,
    /// Route information to create a reaction on a message.
    CreateReaction {
        /// The ID of the channel.
        channel_id: u64,
        /// The URI encoded custom or unicode emoji.
        emoji: String,
        /// The ID of the message.
        message_id: u64,
    },
    /// Route information to create a role in a guild.
    CreateRole {
        /// The ID of the guild.
        guild_id: u64,
    },
    /// Route information to create a guild template.
    CreateTemplate {
        /// The ID of the guild.
        guild_id: u64,
    },
    /// Route information to create a typing trigger in a channel.
    CreateTypingTrigger {
        /// The ID of the channel.
        channel_id: u64,
    },
    /// Route information to create a webhook in a channel.
    CreateWebhook {
        /// The ID of the channel.
        channel_id: u64,
    },
    /// Route information to crosspost a message to following guilds.
    CrosspostMessage {
        /// The ID of the channel.
        channel_id: u64,
        /// The ID of the message.
        message_id: u64,
    },
    /// Route information to delete a ban on a user in a guild.
    DeleteBan {
        /// The ID of the guild.
        guild_id: u64,
        /// The ID of the user.
        user_id: u64,
    },
    /// Route information to delete a channel.
    DeleteChannel {
        /// The ID of the channel.
        channel_id: u64,
    },
    /// Route information to delete a guild's custom emoji.
    DeleteEmoji {
        /// The ID of the emoji.
        emoji_id: u64,
        /// The ID of the guild.
        guild_id: u64,
    },
    /// Route information to delete a guild.
    DeleteGuild {
        /// The ID of the guild.
        guild_id: u64,
    },
    /// Route information to delete a guild integration.
    DeleteGuildIntegration {
        /// The ID of the guild.
        guild_id: u64,
        /// The ID of the integration.
        integration_id: u64,
    },
    /// Route information to delete an invite.
    DeleteInvite {
        /// The unique invite code.
        code: String,
    },
    /// Route information to delete a channel's message.
    DeleteMessage {
        /// The ID of the channel.
        channel_id: u64,
        /// The ID of the message.
        message_id: u64,
    },
    /// Route information to bulk delete messages in a channel.
    DeleteMessages {
        /// The ID of the channel.
        channel_id: u64,
    },
    /// Route information to delete all of the reactions on a message.
    DeleteMessageReactions {
        /// The ID of the channel.
        channel_id: u64,
        /// The ID of the message.
        message_id: u64,
    },
    /// Route information to delete all of the reactions on a message with a
    /// specific emoji.
    DeleteMessageSpecficReaction {
        /// The ID of the channel.
        channel_id: u64,
        /// The URI encoded custom or unicode emoji.
        emoji: String,
        /// The ID of the message.
        message_id: u64,
    },
    /// Route information to delete a permission overwrite for a role or user in
    /// a channel.
    DeletePermissionOverwrite {
        /// The ID of the channel.
        channel_id: u64,
        /// The ID of the target role or user.
        target_id: u64,
    },
    /// Route information to delete a user's reaction on a message.
    DeleteReaction {
        /// The ID of the channel.
        channel_id: u64,
        /// The URI encoded custom or unicode emoji.
        emoji: String,
        /// The ID of the message.
        message_id: u64,
        /// The ID of the user. This can be `@me` to specify the current user.
        user: String,
    },
    /// Route information to delete a guild's role.
    DeleteRole {
        /// The ID of the guild.
        guild_id: u64,
        /// The ID of the role.
        role_id: u64,
    },
    /// Route information to delete a guild template.
    DeleteTemplate {
        /// The ID of the guild.
        guild_id: u64,
        /// The target template code.
        template_code: String,
    },
    /// Route information to delete a message created by a webhook.
    DeleteWebhookMessage {
        message_id: u64,
        token: String,
        webhook_id: u64,
    },
    /// Route information to delete a webhook.
    DeleteWebhook {
        /// The token of the webhook.
        token: Option<String>,
        /// The ID of the webhook.
        webhook_id: u64,
    },
    /// Route information to execute a webhook by ID and token.
    ExecuteWebhook {
        /// The token of the webhook.
        token: String,
        /// Whether to wait for a message response.
        wait: Option<bool>,
        /// The ID of the webhook.
        webhook_id: u64,
    },
    /// Route information to follow a news channel.
    FollowNewsChannel {
        /// The ID of the channel to follow.
        channel_id: u64,
    },
    /// Route information to get a paginated list of audit logs in a guild.
    GetAuditLogs {
        /// The type of action to get audit logs for.
        action_type: Option<u64>,
        /// The maximum ID of audit logs to get.
        before: Option<u64>,
        /// The ID of the guild.
        guild_id: u64,
        /// The maximum number of audit logs to get.
        limit: Option<u64>,
        /// The ID of the user, if specified.
        user_id: Option<u64>,
    },
    /// Route information to get information about a single ban in a guild.
    GetBan {
        /// The ID of the guild.
        guild_id: u64,
        /// The ID of the user.
        user_id: u64,
    },
    /// Route information to get a guild's bans.
    GetBans {
        /// The ID of the guild.
        guild_id: u64,
    },
    /// Route information to get a channel.
    GetChannel {
        /// The ID of the channel.
        channel_id: u64,
    },
    /// Route information to get a channel's invites.
    GetChannelInvites {
        /// The ID of the channel.
        channel_id: u64,
    },
    /// Route information to get a channel's webhooks.
    GetChannelWebhooks {
        /// The ID of the channel.
        channel_id: u64,
    },
    /// Route information to get a guild's channels.
    GetChannels {
        /// The ID of the guild.
        guild_id: u64,
    },
    /// Route information to get info about application the current bot user belongs to
    GetCurrentUserApplicationInfo,
    /// Route information to get an emoji by ID within a guild.
    GetEmoji {
        /// The ID of the emoji.
        emoji_id: u64,
        /// The ID of the guild.
        guild_id: u64,
    },
    /// Route information to get a guild's emojis.
    GetEmojis {
        /// The ID of the guild.
        guild_id: u64,
    },
    /// Route information to get basic gateway information.
    GetGateway,
    /// Route information to get gateway information tailored to the current
    /// user.
    GetGatewayBot,
    /// Route information to get a guild.
    GetGuild {
        /// The ID of the guild.
        guild_id: u64,
        /// Whether to include approximate member and presence counts for the
        /// guild.
        with_counts: bool,
    },
    /// Route information to get a guild's widget.
    GetGuildWidget {
        /// The ID of the guild.
        guild_id: u64,
    },
    /// Route information to get a guild's integrations.
    GetGuildIntegrations {
        /// The ID of the guild.
        guild_id: u64,
    },
    /// Route information to get a guild's invites.
    GetGuildInvites {
        /// The ID of the guild.
        guild_id: u64,
    },
    /// Route information to get a guild's members.
    GetGuildMembers {
        /// The minimum ID of members to get.
        after: Option<u64>,
        /// The ID of the guild.
        guild_id: u64,
        /// The maximum number of members to get.
        limit: Option<u64>,
        /// Whether to get the members' presences.
        presences: Option<bool>,
    },
    /// Route information to get a guild's preview.
    GetGuildPreview {
        /// The ID of the guild.
        guild_id: u64,
    },
    /// Route information to get the number of members that would be pruned,
    /// filtering by inactivity and users with one of the provided roles.
    GetGuildPruneCount {
        /// The number of days that a user must be offline before being able to
        /// be pruned.
        days: Option<u64>,
        /// The ID of the guild.
        guild_id: u64,
        /// The roles to filter the prune by.
        ///
        /// A user must have at least one of these roles to be able to be
        /// pruned.
        include_roles: Vec<u64>,
    },
    /// Route information to get a guild's roles.
    GetGuildRoles {
        /// The ID of the guild.
        guild_id: u64,
    },
    /// Route information to get a guild's vanity URL.
    GetGuildVanityUrl {
        /// The ID of the guild.
        guild_id: u64,
    },
    /// Route information to get a guild's available voice regions.
    GetGuildVoiceRegions {
        /// The ID of the guild.
        guild_id: u64,
    },
    /// Route information to get a guild's welcome screen.
    GetGuildWelcomeScreen {
        /// ID of the guild.
        guild_id: u64,
    },
    /// Route information to get a guild's webhooks.
    GetGuildWebhooks {
        /// The ID of the guild.
        guild_id: u64,
    },
    /// Route information to get a paginated list of guilds.
    GetGuilds {
        /// The minimum ID of guilds to get.
        after: Option<u64>,
        /// The maximum ID of guilds to get.
        before: Option<u64>,
        /// The maximum number of guilds to get.
        limit: Option<u64>,
    },
    /// Route information to get an invite.
    GetInvite {
        /// The unique invite code.
        code: String,
        /// Whether to retrieve statistics about the invite.
        with_counts: bool,
    },
    /// Route information to get a member.
    GetMember {
        /// The ID of the guild.
        guild_id: u64,
        /// The ID of the user.
        user_id: u64,
    },
    /// Route information to get a single message in a channel.
    GetMessage {
        /// The ID of the channel.
        channel_id: u64,
        /// The ID of the message.
        message_id: u64,
    },
    /// Route information to get a paginated list of messages in a channel.
    GetMessages {
        /// The minimum ID of messages to get.
        after: Option<u64>,
        /// The message ID to get the messages around.
        around: Option<u64>,
        /// The maximum ID of messages to get.
        before: Option<u64>,
        /// The ID of the channel.
        channel_id: u64,
        /// The maximum number of messages to get.
        limit: Option<u64>,
    },
    /// Route information to get a channel's pins.
    GetPins {
        /// The ID of the channel.
        channel_id: u64,
    },
    /// Route information to get the users who reacted to a message with a
    /// specified emoji.
    GetReactionUsers {
        /// The minimum ID of users to get.
        after: Option<u64>,
        /// The maximum ID of users to get.
        before: Option<u64>,
        /// The ID of the channel.
        channel_id: u64,
        /// The URI encoded custom or unicode emoji.
        emoji: String,
        /// The maximum number of users to retrieve.
        limit: Option<u64>,
        /// The ID of the message.
        message_id: u64,
    },
    /// Route information to get a template.
    GetTemplate {
        /// The template code.
        template_code: String,
    },
    /// Route information to get a list of templates from a guild.
    GetTemplates {
        /// The ID of the guild.
        guild_id: u64,
    },
    /// Route information to get the current user.
    GetUser {
        /// The ID of the target user. This can be `@me` to specify the current
        /// user.
        target_user: String,
    },
    /// Route information to get the current user's connections.
    GetUserConnections,
    /// Route information to get the current user's private channels and groups.
    GetUserPrivateChannels,
    /// Route information to get a list of the voice regions.
    GetVoiceRegions,
    /// Route information to get a webhook by ID, optionally with a token if the
    /// current user doesn't have access to it.
    GetWebhook {
        /// The token of the webhook.
        token: Option<String>,
        /// The ID of the webhook.
        webhook_id: u64,
    },
    /// Route information to leave the guild.
    LeaveGuild {
        /// The ID of the guild.
        guild_id: u64,
    },
    /// Route information to pin a message to a channel.
    PinMessage {
        /// The ID of the channel.
        channel_id: u64,
        /// The ID of the message.
        message_id: u64,
    },
    /// Route information to remove a member from a guild.
    RemoveMember {
        /// The ID of the guild.
        guild_id: u64,
        /// The ID of the user.
        user_id: u64,
    },
    /// Route information to remove a role from a member.
    RemoveMemberRole {
        /// The ID of the guild.
        guild_id: u64,
        /// The ID of the role.
        role_id: u64,
        /// The ID of the user.
        user_id: u64,
    },
    /// Route information to search for members in a guild.
    SearchGuildMembers {
        /// ID of the guild to search in.
        guild_id: u64,
        /// Upper limit of members to query for.
        limit: Option<u64>,
        /// Query to search by.
        query: String,
    },
    /// Route information to sync a guild's integration.
    SyncGuildIntegration {
        /// The ID of the guild.
        guild_id: u64,
        /// The ID of the integration.
        integration_id: u64,
    },
    /// Route information to sync a template.
    SyncTemplate {
        /// The ID of the guild.
        guild_id: u64,
        /// The template code.
        template_code: String,
    },
    /// Route information to unpin a message from a channel.
    UnpinMessage {
        /// The ID of the channel.
        channel_id: u64,
        /// The ID of the message.
        message_id: u64,
    },
    /// Route information to update a channel, such as a guild channel or group.
    UpdateChannel {
        /// The ID of the channel.
        channel_id: u64,
    },
    /// Route information to update the current user.
    UpdateCurrentUser,
    /// Route information to update the current user's voice state.
    UpdateCurrentUserVoiceState {
        /// ID of the guild.
        guild_id: u64,
    },
    /// Route information to update an emoji.
    UpdateEmoji {
        /// The ID of the emoji.
        emoji_id: u64,
        /// The ID of the guild.
        guild_id: u64,
    },
    /// Route information to update a guild.
    UpdateGuild {
        /// The ID of the guild.
        guild_id: u64,
    },
    /// Route information to update a guild channel.
    UpdateGuildChannels {
        /// The ID of the guild.
        guild_id: u64,
    },
    /// Route information to update a guild's widget.
    UpdateGuildWidget {
        /// The ID of the guild.
        guild_id: u64,
    },
    /// Route information to update a guild's integration.
    UpdateGuildIntegration {
        /// The ID of the guild.
        guild_id: u64,
        /// The ID of the integration.
        integration_id: u64,
    },
    /// Route information to update a guild's welcome screen.
    UpdateGuildWelcomeScreen {
        /// ID of the guild.
        guild_id: u64,
    },
    /// Route information to update a member.
    UpdateMember {
        /// The ID of the guild.
        guild_id: u64,
        /// The ID of the user.
        user_id: u64,
    },
    /// Route information to update a message.
    UpdateMessage {
        /// The ID of the channel.
        channel_id: u64,
        /// The ID of the message.
        message_id: u64,
    },
    /// Route information to update the current member's nickname.
    UpdateNickname {
        /// The ID of the guild.
        guild_id: u64,
    },
    /// Route information to update the permission overwrite of a role or user
    /// in a channel.
    UpdatePermissionOverwrite {
        /// The ID of the channel.
        channel_id: u64,
        /// The ID of the role or user.
        target_id: u64,
    },
    /// Route information to update a role.
    UpdateRole {
        /// The ID of the guild.
        guild_id: u64,
        /// The ID of the role.
        role_id: u64,
    },
    /// Route information to update the positions of roles.
    UpdateRolePositions {
        /// The ID of the guild.
        guild_id: u64,
    },
    /// Route information to update a template.
    UpdateTemplate {
        /// The ID of the guild.
        guild_id: u64,
        /// The template code.
        template_code: String,
    },
    /// Route information to update a user's voice state.
    UpdateUserVoiceState {
        /// ID of the guild.
        guild_id: u64,
        /// ID of the user.
        user_id: u64,
    },
    /// Route information to update a message created by a webhook.
    UpdateWebhookMessage {
        message_id: u64,
        token: String,
        webhook_id: u64,
    },
    /// Route information to update a webhook.
    UpdateWebhook {
        /// The token of the webhook.
        token: Option<String>,
        /// The ID of the webhook.
        webhook_id: u64,
    },
}

impl Route {
    /// Separate a route into its parts: the HTTP method, the path enum to use
    /// for ratelimit buckets, and the URI path.
    ///
    /// The method and URI path are useful for actually performing requests,
    /// while the returned path enum is useful for ratelimiting.
    // This function contains some `write!`s, but they can't fail, so we ignore
    // them to remove an unnecessary Result here.
    #[allow(clippy::cognitive_complexity, clippy::too_many_lines)]
    pub fn into_parts(self) -> (Method, Path, Cow<'static, str>) {
        match self {
            Self::AddGuildMember { guild_id, user_id } => (
                Method::Put,
                Path::GuildsIdMembersId(guild_id),
                format!("guilds/{}/members/{}", guild_id, user_id).into(),
            ),
            Self::AddMemberRole {
                guild_id,
                role_id,
                user_id,
            } => (
                Method::Put,
                Path::GuildsIdMembersIdRolesId(guild_id),
                format!("guilds/{}/members/{}/roles/{}", guild_id, user_id, role_id).into(),
            ),
            Self::CreateBan {
                guild_id,
                delete_message_days,
                reason,
                user_id,
            } => {
                let mut path = format!("guilds/{}/bans/{}?", guild_id, user_id);

                if let Some(delete_message_days) = delete_message_days {
                    let _ = write!(path, "delete_message_days={}", delete_message_days);
                    if reason.is_some() {
                        let _ = write!(path, "&");
                    }
                }

                if let Some(reason) = reason {
                    let encoded_reason = utf8_percent_encode(&reason, NON_ALPHANUMERIC).to_string();
                    let _ = write!(path, "reason={}", encoded_reason);
                }

                (Method::Put, Path::GuildsIdBansUserId(guild_id), path.into())
            }
            Self::CreateChannel { guild_id } => (
                Method::Post,
                Path::GuildsIdChannels(guild_id),
                format!("guilds/{}/channels", guild_id).into(),
            ),
            Self::CreateEmoji { guild_id } => (
                Method::Post,
                Path::GuildsIdEmojis(guild_id),
                format!("guilds/{}/emojis", guild_id).into(),
            ),
            Self::CreateGuild => (Method::Post, Path::Guilds, "guilds".into()),
            Self::CreateGuildFromTemplate { template_code } => (
                Method::Post,
                Path::Guilds,
                format!("guilds/templates/{}", template_code).into(),
            ),
            Self::CreateGuildIntegration { guild_id } => (
                Method::Post,
                Path::GuildsIdIntegrationsId(guild_id),
                format!("guilds/{}/integrations", guild_id).into(),
            ),
            Self::CreateGuildPrune {
                compute_prune_count,
                days,
                guild_id,
                include_roles,
            } => {
                let mut path = format!("guilds/{}/prune?", guild_id);

                if let Some(compute_prune_count) = compute_prune_count {
                    let _ = write!(path, "compute_prune_count={}&", compute_prune_count,);
                }

                if let Some(days) = days {
                    let _ = write!(path, "days={}&", days);
                }

                if !include_roles.is_empty() {
                    let _ = write!(
                        path,
                        "include_roles={}",
                        include_roles
                            .iter()
                            .map(ToString::to_string)
                            .collect::<Vec<String>>()
                            .join(",")
                    );
                }

                (Method::Post, Path::GuildsIdPrune(guild_id), path.into())
            }
            Self::CreateInvite { channel_id } => (
                Method::Post,
                Path::ChannelsIdInvites(channel_id),
                format!("channels/{}/invites", channel_id).into(),
            ),
            Self::CreateMessage { channel_id } => (
                Method::Post,
                Path::ChannelsIdMessages(channel_id),
                format!("channels/{}/messages", channel_id).into(),
            ),
            Self::CreatePrivateChannel => (
                Method::Post,
                Path::UsersIdChannels,
                "users/@me/channels".into(),
            ),
            Self::CreateReaction {
                channel_id,
                emoji,
                message_id,
            } => (
                Method::Put,
                Path::ChannelsIdMessagesIdReactionsUserIdType(channel_id),
                format!(
                    "channels/{}/messages/{}/reactions/{}/@me",
                    channel_id, message_id, emoji,
                )
                .into(),
            ),
            Self::CreateRole { guild_id } => (
                Method::Post,
                Path::GuildsIdRoles(guild_id),
                format!("guilds/{}/roles", guild_id).into(),
            ),
            Self::CreateTemplate { guild_id } => (
                Method::Post,
                Path::GuildsIdTemplates(guild_id),
                format!("guilds/{}/templates", guild_id).into(),
            ),
            Self::CreateTypingTrigger { channel_id } => (
                Method::Post,
                Path::ChannelsIdTyping(channel_id),
                format!("channels/{}/typing", channel_id).into(),
            ),
            Self::CreateWebhook { channel_id } => (
                Method::Post,
                Path::ChannelsIdWebhooks(channel_id),
                format!("channels/{}/webhooks", channel_id).into(),
            ),
            Self::CrosspostMessage {
                channel_id,
                message_id,
            } => (
                Method::Post,
                Path::ChannelsIdMessagesIdCrosspost(channel_id),
                format!("channels/{}/messages/{}/crosspost", channel_id, message_id).into(),
            ),
            Self::DeleteBan { guild_id, user_id } => (
                Method::Delete,
                Path::GuildsIdBansUserId(guild_id),
                format!("guilds/{}/bans/{}", guild_id, user_id).into(),
            ),
            Self::DeleteChannel { channel_id } => (
                Method::Delete,
                Path::ChannelsId(channel_id),
                format!("channels/{}", channel_id).into(),
            ),
            Self::DeleteEmoji { emoji_id, guild_id } => (
                Method::Delete,
                Path::GuildsIdEmojisId(guild_id),
                format!("guilds/{}/emojis/{}", guild_id, emoji_id).into(),
            ),
            Self::DeleteGuild { guild_id } => (
                Method::Delete,
                Path::GuildsId(guild_id),
                format!("guilds/{}", guild_id).into(),
            ),
            Self::DeleteGuildIntegration {
                guild_id,
                integration_id,
            } => (
                Method::Delete,
                Path::GuildsIdIntegrationsId(guild_id),
                format!("guilds/{}/integrations/{}", guild_id, integration_id).into(),
            ),
            Self::DeleteInvite { code } => (
                Method::Delete,
                Path::InvitesCode,
                format!("invites/{}", code).into(),
            ),
            Self::DeleteMessageReactions {
                channel_id,
                message_id,
            } => (
                Method::Delete,
                Path::ChannelsIdMessagesIdReactions(channel_id),
                format!("channels/{}/messages/{}/reactions", channel_id, message_id).into(),
            ),
            Self::DeleteMessageSpecficReaction {
                channel_id,
                message_id,
                emoji,
            } => (
                Method::Delete,
                Path::ChannelsIdMessagesIdReactions(channel_id),
                format!(
                    "channels/{}/messages/{}/reactions/{}",
                    channel_id, message_id, emoji
                )
                .into(),
            ),
            Self::DeleteMessage {
                channel_id,
                message_id,
            } => (
                Method::Delete,
                Path::ChannelsIdMessagesId(Method::Delete, message_id),
                format!("channels/{}/messages/{}", channel_id, message_id).into(),
            ),
            Self::DeleteMessages { channel_id } => (
                Method::Post,
                Path::ChannelsIdMessagesBulkDelete(channel_id),
                format!("channels/{}/messages/bulk-delete", channel_id).into(),
            ),
            Self::DeletePermissionOverwrite {
                channel_id,
                target_id,
            } => (
                Method::Delete,
                Path::ChannelsIdPermissionsOverwriteId(channel_id),
                format!("channels/{}/permissions/{}", channel_id, target_id).into(),
            ),
            Self::DeleteReaction {
                channel_id,
                emoji,
                message_id,
                user,
            } => (
                Method::Delete,
                Path::ChannelsIdMessagesIdReactionsUserIdType(channel_id),
                format!(
                    "channels/{}/messages/{}/reactions/{}/{}",
                    channel_id, message_id, emoji, user,
                )
                .into(),
            ),
            Self::DeleteRole { guild_id, role_id } => (
                Method::Delete,
                Path::GuildsIdRolesId(guild_id),
                format!("guilds/{}/roles/{}", guild_id, role_id).into(),
            ),
            Self::DeleteTemplate {
                guild_id,
                template_code,
            } => (
                Method::Delete,
                Path::GuildsIdTemplatesCode(guild_id),
                format!("guilds/{}/templates/{}", guild_id, template_code).into(),
            ),
            Self::DeleteWebhookMessage {
                message_id,
                token,
                webhook_id,
            } => (
                Method::Delete,
                Path::WebhooksIdTokenMessagesId(webhook_id),
                format!("webhooks/{}/{}/messages/{}", webhook_id, token, message_id).into(),
            ),
            Self::DeleteWebhook { token, webhook_id } => {
                let mut path = format!("webhooks/{}", webhook_id);

                if let Some(token) = token {
                    path.push('/');
                    path.push_str(&token);
                }

                (Method::Delete, Path::WebhooksId(webhook_id), path.into())
            }
            Self::ExecuteWebhook {
                token,
                wait,
                webhook_id,
            } => {
                let mut path = format!("webhooks/{}/{}", webhook_id, token);

                if let Some(wait) = wait {
                    let _ = write!(path, "?wait={}", wait);
                }

                (Method::Post, Path::WebhooksId(webhook_id), path.into())
            }
            Self::FollowNewsChannel { channel_id } => (
                Method::Post,
                Path::ChannelsIdFollowers(channel_id),
                format!("channels/{}/followers", channel_id).into(),
            ),
            Self::GetAuditLogs {
                action_type,
                before,
                guild_id,
                limit,
                user_id,
            } => {
                let mut path = format!("guilds/{}/audit-logs?", guild_id);

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

                (Method::Get, Path::GuildsIdAuditLogs(guild_id), path.into())
            }
            Self::GetBan { guild_id, user_id } => (
                Method::Get,
                Path::GuildsIdBansId(guild_id),
                format!("guilds/{}/bans/{}", guild_id, user_id).into(),
            ),
            Self::GetBans { guild_id } => (
                Method::Get,
                Path::GuildsIdBans(guild_id),
                format!("guilds/{}/bans", guild_id).into(),
            ),
            Self::GetGatewayBot => (Method::Get, Path::GatewayBot, "gateway/bot".into()),
            Self::GetChannel { channel_id } => (
                Method::Get,
                Path::ChannelsId(channel_id),
                format!("channels/{}", channel_id).into(),
            ),
            Self::GetChannelInvites { channel_id } => (
                Method::Get,
                Path::ChannelsIdInvites(channel_id),
                format!("channels/{}/invites", channel_id).into(),
            ),
            Self::GetChannelWebhooks { channel_id } => (
                Method::Get,
                Path::ChannelsIdWebhooks(channel_id),
                format!("channels/{}/webhooks", channel_id).into(),
            ),
            Self::GetChannels { guild_id } => (
                Method::Get,
                Path::GuildsIdChannels(guild_id),
                format!("guilds/{}/channels", guild_id).into(),
            ),
            Self::GetCurrentUserApplicationInfo => (
                Method::Get,
                Path::OauthApplicationsMe,
                "/oauth2/applications/@me".into(),
            ),
            Self::GetEmoji { emoji_id, guild_id } => (
                Method::Get,
                Path::GuildsIdEmojisId(guild_id),
                format!("guilds/{}/emojis/{}", guild_id, emoji_id).into(),
            ),
            Self::GetEmojis { guild_id } => (
                Method::Get,
                Path::GuildsIdEmojis(guild_id),
                format!("guilds/{}/emojis", guild_id).into(),
            ),
            Self::GetGateway => (Method::Get, Path::Gateway, "gateway".into()),
            Self::GetGuild {
                guild_id,
                with_counts,
            } => {
                let mut path = format!("guilds/{}", guild_id);
                if with_counts {
                    let _ = write!(path, "?with_counts=true");
                }
                (Method::Get, Path::GuildsId(guild_id), path.into())
            }
            Self::GetGuildWidget { guild_id } => (
                Method::Get,
                Path::GuildsIdWidget(guild_id),
                format!("guilds/{}/widget", guild_id).into(),
            ),
            Self::GetGuildIntegrations { guild_id } => {
                (Method::Get, Path::GuildsIdIntegrations(guild_id), {
                    format!("guilds/{}/integrations", guild_id).into()
                })
            }
            Self::GetGuildInvites { guild_id } => (
                Method::Get,
                Path::GuildsIdInvites(guild_id),
                format!("guilds/{}/invites", guild_id).into(),
            ),
            Self::GetGuildMembers {
                after,
                guild_id,
                limit,
                presences,
            } => {
                let mut path = format!("guilds/{}/members?", guild_id);

                if let Some(after) = after {
                    let _ = write!(path, "after={}", after);
                }

                if let Some(limit) = limit {
                    let _ = write!(path, "&limit={}", limit);
                }

                if let Some(presences) = presences {
                    let _ = write!(path, "&presences={}", presences);
                }

                (Method::Get, Path::GuildsIdMembers(guild_id), path.into())
            }
            Self::GetGuildPreview { guild_id } => (
                Method::Get,
                Path::GuildsIdPreview(guild_id),
                format!("guilds/{}/preview", guild_id).into(),
            ),
            Self::GetGuildPruneCount {
                days,
                guild_id,
                include_roles,
            } => {
                let mut path = format!("guilds/{}/prune?", guild_id);

                if let Some(days) = days {
                    let _ = write!(path, "days={}&", days);
                }

                if !include_roles.is_empty() {
                    let _ = write!(
                        path,
                        "include_roles={}",
                        include_roles
                            .iter()
                            .map(ToString::to_string)
                            .collect::<Vec<String>>()
                            .join(",")
                    );
                }

                (Method::Get, Path::GuildsIdPrune(guild_id), path.into())
            }
            Self::GetGuildRoles { guild_id } => (
                Method::Get,
                Path::GuildsIdRoles(guild_id),
                format!("guilds/{}/roles", guild_id).into(),
            ),
            Self::GetGuildVanityUrl { guild_id } => (
                Method::Get,
                Path::GuildsIdVanityUrl(guild_id),
                format!("guilds/{}/vanity-url", guild_id).into(),
            ),
            Self::GetGuildVoiceRegions { guild_id } => (
                Method::Get,
                Path::GuildsIdRegions(guild_id),
                format!("guilds/{}/regions", guild_id).into(),
            ),
            Self::GetGuildWelcomeScreen { guild_id } => (
                Method::Get,
                Path::GuildsIdWelcomeScreen(guild_id),
                format!("guilds/{}/welcome-screen", guild_id).into(),
            ),
            Self::GetGuildWebhooks { guild_id } => (
                Method::Get,
                Path::GuildsIdWebhooks(guild_id),
                format!("guilds/{}/webhooks", guild_id).into(),
            ),
            Self::GetGuilds {
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

                (Method::Get, Path::UsersIdGuilds, path.into())
            }
            Self::GetInvite { code, with_counts } => (
                Method::Get,
                Path::InvitesCode,
                format!("invites/{}?with-counts={}", code, with_counts).into(),
            ),
            Self::GetMember { guild_id, user_id } => (
                Method::Get,
                Path::GuildsIdMembersId(guild_id),
                format!("guilds/{}/members/{}", guild_id, user_id).into(),
            ),
            Self::GetMessage {
                channel_id,
                message_id,
            } => (
                Method::Get,
                Path::ChannelsIdMessagesId(Method::Get, channel_id),
                format!("channels/{}/messages/{}", channel_id, message_id).into(),
            ),
            Self::GetMessages {
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
                    Method::Get,
                    Path::ChannelsIdMessages(channel_id),
                    path.into(),
                )
            }
            Self::GetPins { channel_id } => (
                Method::Get,
                Path::ChannelsIdPins(channel_id),
                format!("channels/{}/pins", channel_id).into(),
            ),
            Self::GetReactionUsers {
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
                    Method::Get,
                    Path::ChannelsIdMessagesIdReactions(channel_id),
                    path.into(),
                )
            }
            Self::GetTemplate { template_code } => (
                Method::Get,
                Path::Guilds,
                format!("guilds/templates/{}", template_code).into(),
            ),
            Self::GetTemplates { guild_id } => (
                Method::Get,
                Path::GuildsIdTemplates(guild_id),
                format!("guilds/{}/templates", guild_id).into(),
            ),
            Self::GetUserConnections => (
                Method::Get,
                Path::UsersIdConnections,
                "users/@me/connections".into(),
            ),
            Self::GetUserPrivateChannels => (
                Method::Get,
                Path::UsersIdChannels,
                "users/@me/channels".into(),
            ),
            Self::GetUser { target_user } => (
                Method::Get,
                Path::UsersId,
                format!("users/{}", target_user).into(),
            ),
            Self::GetVoiceRegions => (Method::Get, Path::VoiceRegions, "voice/regions".into()),
            Self::GetWebhook { token, webhook_id } => {
                let mut path = format!("webhooks/{}", webhook_id);

                if let Some(token) = token {
                    path.push('/');
                    path.push_str(&token);
                }

                (Method::Get, Path::WebhooksId(webhook_id), path.into())
            }
            Self::LeaveGuild { guild_id } => (
                Method::Delete,
                Path::UsersIdGuildsId,
                format!("users/@me/guilds/{}", guild_id).into(),
            ),
            Self::PinMessage {
                channel_id,
                message_id,
            } => (
                Method::Put,
                Path::ChannelsIdPins(channel_id),
                format!("channels/{}/pins/{}", channel_id, message_id).into(),
            ),
            Self::RemoveMember { guild_id, user_id } => (
                Method::Delete,
                Path::GuildsIdMembersId(guild_id),
                format!("guilds/{}/members/{}", guild_id, user_id).into(),
            ),
            Self::RemoveMemberRole {
                guild_id,
                role_id,
                user_id,
            } => (
                Method::Delete,
                Path::GuildsIdMembersIdRolesId(guild_id),
                format!("guilds/{}/members/{}/roles/{}", guild_id, user_id, role_id).into(),
            ),
            Self::SearchGuildMembers {
                guild_id,
                limit,
                query,
            } => {
                let mut path = format!("guilds/{}/members/search?query={}", guild_id, query);

                if let Some(limit) = limit {
                    let _ = write!(path, "&limit={}", limit);
                }

                (
                    Method::Get,
                    Path::GuildsIdMembersSearch(guild_id),
                    path.into(),
                )
            }
            Self::SyncGuildIntegration {
                guild_id,
                integration_id,
            } => (
                Method::Post,
                Path::GuildsIdIntegrationsIdSync(guild_id),
                format!("guilds/{}/integrations/{}/sync", guild_id, integration_id).into(),
            ),
            Self::SyncTemplate {
                guild_id,
                template_code,
            } => (
                Method::Put,
                Path::GuildsIdTemplatesCode(guild_id),
                format!("guilds/{}/templates/{}", guild_id, template_code).into(),
            ),
            Self::UnpinMessage {
                channel_id,
                message_id,
            } => (
                Method::Delete,
                Path::ChannelsIdPinsMessageId(channel_id),
                format!("channels/{}/pins/{}", channel_id, message_id).into(),
            ),
            Self::UpdateChannel { channel_id } => (
                Method::Patch,
                Path::ChannelsId(channel_id),
                format!("channels/{}", channel_id).into(),
            ),
            Self::UpdateCurrentUser => (Method::Patch, Path::UsersId, "users/@me".into()),
            Self::UpdateCurrentUserVoiceState { guild_id } => (
                Method::Patch,
                Path::GuildsIdVoiceStates(guild_id),
                format!("guilds/{}/voice-states/@me", guild_id).into(),
            ),
            Self::UpdateEmoji { emoji_id, guild_id } => (
                Method::Patch,
                Path::GuildsIdEmojisId(guild_id),
                format!("guilds/{}/emojis/{}", guild_id, emoji_id).into(),
            ),
            Self::UpdateGuild { guild_id } => (
                Method::Patch,
                Path::GuildsId(guild_id),
                format!("guilds/{}", guild_id).into(),
            ),
            Self::UpdateGuildChannels { guild_id } => (
                Method::Patch,
                Path::GuildsIdChannels(guild_id),
                format!("guilds/{}/channels", guild_id).into(),
            ),
            Self::UpdateGuildWidget { guild_id } => (
                Method::Patch,
                Path::GuildsIdWidget(guild_id),
                format!("guilds/{}/widget", guild_id).into(),
            ),
            Self::UpdateGuildIntegration {
                guild_id,
                integration_id,
            } => (
                Method::Patch,
                Path::GuildsIdIntegrationsId(guild_id),
                format!("guilds/{}/integrations/{}", guild_id, integration_id,).into(),
            ),
            Self::UpdateGuildWelcomeScreen { guild_id } => (
                Method::Patch,
                Path::GuildsIdWelcomeScreen(guild_id),
                format!("guilds/{}/welcome-screen", guild_id).into(),
            ),
            Self::UpdateMember { guild_id, user_id } => (
                Method::Patch,
                Path::GuildsIdMembersId(guild_id),
                format!("guilds/{}/members/{}", guild_id, user_id).into(),
            ),
            Self::UpdateMessage {
                channel_id,
                message_id,
            } => (
                Method::Patch,
                Path::ChannelsIdMessagesId(Method::Patch, channel_id),
                format!("channels/{}/messages/{}", channel_id, message_id).into(),
            ),
            Self::UpdateNickname { guild_id } => (
                Method::Patch,
                Path::GuildsIdMembersMeNick(guild_id),
                format!("guilds/{}/members/@me/nick", guild_id).into(),
            ),
            Self::UpdatePermissionOverwrite {
                channel_id,
                target_id,
            } => (
                Method::Put,
                Path::ChannelsIdPermissionsOverwriteId(channel_id),
                format!("channels/{}/permissions/{}", channel_id, target_id).into(),
            ),
            Self::UpdateRole { guild_id, role_id } => (
                Method::Patch,
                Path::GuildsIdRolesId(guild_id),
                format!("guilds/{}/roles/{}", guild_id, role_id).into(),
            ),
            Self::UpdateRolePositions { guild_id } => (
                Method::Patch,
                Path::GuildsIdRolesId(guild_id),
                format!("guilds/{}/roles", guild_id).into(),
            ),
            Self::UpdateTemplate {
                guild_id,
                template_code,
            } => (
                Method::Patch,
                Path::GuildsIdTemplatesCode(guild_id),
                format!("guilds/{}/templates/{}", guild_id, template_code).into(),
            ),
            Self::UpdateUserVoiceState { guild_id, user_id } => (
                Method::Patch,
                Path::GuildsIdVoiceStates(guild_id),
                format!("guilds/{}/voice-states/{}", guild_id, user_id).into(),
            ),
            Self::UpdateWebhookMessage {
                message_id,
                token,
                webhook_id,
            } => (
                Method::Patch,
                Path::WebhooksIdTokenMessagesId(webhook_id),
                format!("webhooks/{}/{}/messages/{}", webhook_id, token, message_id).into(),
            ),
            Self::UpdateWebhook { token, webhook_id } => {
                let mut path = format!("webhooks/{}", webhook_id);

                if let Some(token) = token {
                    path.push('/');
                    path.push_str(&token);
                }

                (Method::Patch, Path::WebhooksId(webhook_id), path.into())
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{Path, PathParseErrorType};
    use crate::request::Method;
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
        assert!(matches!(
            Path::from_str("channels/123/messages/456")
                .unwrap_err()
                .kind(),
            PathParseErrorType::MessageIdWithoutMethod { channel_id: 123 },
        ));
        assert_eq!(
            Path::ChannelsIdMessagesId(Method::Get, 123),
            Path::try_from((Method::Get, "/channels/123/messages/456"))?,
        );

        Ok(())
    }
}
