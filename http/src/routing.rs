use hyper::Method;
use std::{
    borrow::Cow,
    convert::TryFrom,
    error::Error as StdError,
    fmt::{Display, Formatter, Result as FmtResult, Write},
    num::ParseIntError,
    str::FromStr,
};

#[derive(Clone, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum PathParseError {
    /// The ID couldn't be parsed as an integer.
    IntegerParsing {
        /// Additional information about the parsing failure.
        source: ParseIntError,
    },
    /// When parsing into a [`Path::ChannelsIdMessagesId`] variant, the method
    /// must also be specified via its `TryFrom` impl.
    MessageIdWithoutMethod {
        /// The ID of the channel.
        channel_id: u64,
    },
    /// A static path for the provided path string wasn't found.
    NoMatch,
}

impl From<ParseIntError> for PathParseError {
    fn from(source: ParseIntError) -> Self {
        Self::IntegerParsing { source }
    }
}

impl Display for PathParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            Self::IntegerParsing { .. } => f.write_str("An ID in a segment was invalid"),
            Self::MessageIdWithoutMethod { .. } => {
                f.write_str("A message path was detected but the method wasn't given")
            }
            Self::NoMatch => f.write_str("There was no matched path"),
        }
    }
}

impl StdError for PathParseError {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        match self {
            Self::IntegerParsing { source } => Some(source),
            Self::MessageIdWithoutMethod { .. } | Self::NoMatch => None,
        }
    }
}

/// An enum representing a path, most useful for ratelimiting implementations.
// If adding to this enum, be sure to add to the `TryFrom` impl.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
#[non_exhaustive]
pub enum Path {
    /// Operating on global commands.
    ApplicationCommand(u64),
    /// Operating on a specific command.
    ApplicationCommandId(u64),
    /// Operating on commands in a guild.
    ApplicationGuildCommand(u64),
    /// Operating on a specific command in a guild.
    ApplicationGuildCommandId(u64),
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
    GuildsIdPreview(u64),
    GuildsIdPrune(u64),
    GuildsIdRegions(u64),
    GuildsIdRoles(u64),
    GuildsIdRolesId(u64),
    GuildsIdVanityUrl(u64),
    GuildsIdWebhooks(u64),
    InvitesCode,
    /// Operating on an interaction's callback.
    InteractionCallback(u64),
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
    WebhooksIdTokenMessageId(u64),
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

        let skip = usize::from(s.starts_with('/'));

        let parts = s.split('/').skip(skip).collect::<Vec<&str>>();

        Ok(match parts.as_slice() {
            ["applications", id, "commands"] => ApplicationCommand(id.parse()?),
            ["applications", id, "commands", _] => ApplicationCommandId(id.parse()?),
            ["applications", id, "guilds", _, "commands"] => ApplicationGuildCommand(id.parse()?),
            ["applications", id, "guilds", _, "commands", _] => {
                ApplicationGuildCommandId(id.parse()?)
            }
            ["channels", id] => ChannelsId(id.parse()?),
            ["channels", id, "followers"] => ChannelsIdFollowers(id.parse()?),
            ["channels", id, "invites"] => ChannelsIdInvites(id.parse()?),
            ["channels", id, "messages"] => ChannelsIdMessages(id.parse()?),
            ["channels", id, "messages", _] => {
                return Err(PathParseError::MessageIdWithoutMethod {
                    channel_id: id.parse()?,
                });
            }
            ["channels", id, "messages", _, "crosspost"] => {
                ChannelsIdMessagesIdCrosspost(id.parse()?)
            }
            ["channels", id, "messages", _, "reactions"] => {
                ChannelsIdMessagesIdReactions(id.parse()?)
            }
            ["channels", id, "messages", _, "reactions", _, _] => {
                ChannelsIdMessagesIdReactionsUserIdType(id.parse()?)
            }
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
            ["guilds", id, "widget"] => GuildsIdWidget(id.parse()?),
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
            ["guilds", id, "preview"] => GuildsIdPreview(id.parse()?),
            ["guilds", id, "prune"] => GuildsIdPrune(id.parse()?),
            ["guilds", id, "regions"] => GuildsIdRegions(id.parse()?),
            ["guilds", id, "roles"] => GuildsIdRoles(id.parse()?),
            ["guilds", id, "roles", _] => GuildsIdRolesId(id.parse()?),
            ["guilds", id, "vanity-url"] => GuildsIdVanityUrl(id.parse()?),
            ["guilds", id, "webhooks"] => GuildsIdWebhooks(id.parse()?),
            ["invites", _] => InvitesCode,
            ["interactions", id, _, "callback"] => InteractionCallback(id.parse()?),
            ["oauth2", "applications", "@me"] => OauthApplicationsMe,
            ["users", _] => UsersId,
            ["users", _, "connections"] => UsersIdConnections,
            ["users", _, "channels"] => UsersIdChannels,
            ["users", _, "guilds"] => UsersIdGuilds,
            ["users", _, "guilds", _] => UsersIdGuildsId,
            ["voice", "regions"] => VoiceRegions,
            ["webhooks", id] | ["webhooks", id, _] => WebhooksId(id.parse()?),
            _ => return Err(PathParseError::NoMatch),
        })
    }
}

impl TryFrom<(Method, &str)> for Path {
    type Error = PathParseError;

    fn try_from((method, s): (Method, &str)) -> Result<Self, Self::Error> {
        match Self::from_str(s) {
            Ok(v) => Ok(v),
            Err(PathParseError::MessageIdWithoutMethod { channel_id }) => {
                Ok(Self::ChannelsIdMessagesId(method, channel_id))
            }
            Err(why) => Err(why),
        }
    }
}

#[derive(Clone, Debug)]
#[non_exhaustive]
pub enum Route {
    /// Route information to add a user to a guild.
    AddGuildMember {
        /// The ID of the guild.
        guild_id: u64,
        /// The ID of the user.
        user_id: u64,
    },
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
    /// Route information to create a global command.
    CreateGlobalCommand {
        /// The ID of the owner application.
        application_id: u64,
    },
    /// Route information to create a guild.
    CreateGuild,
    /// Route information to create a guild command.
    CreateGuildCommand {
        /// The ID of the owner application.
        application_id: u64,
        /// The ID of the guild.
        guild_id: u64,
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
    /// Route information to delete a global command.
    DeleteGlobalCommand {
        /// The ID of the owner application
        application_id: u64,
        /// The ID of the command
        command_id: u64,
    },
    /// Route information to delete a guild.
    DeleteGuild {
        /// The ID of the guild.
        guild_id: u64,
    },
    /// Route information to delete a guild command.
    DeleteGuildCommand {
        /// The ID of the owner application.
        application_id: u64,
        /// The ID of the command.
        command_id: u64,
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
    /// Route information to delete the original interaction response.
    DeleteInteractionOriginal {
        /// The ID of the owner application
        application_id: u64,
        /// The token of the interaction.
        interaction_token: String,
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
    /// Route information to get global commands.
    GetGlobalCommands {
        /// The ID of the owner application.
        application_id: u64,
    },
    /// Route information to get a guild.
    GetGuild {
        /// The ID of the guild.
        guild_id: u64,
        /// Whether to include approximate member and presence counts for the
        /// guild.
        with_counts: bool,
    },
    /// Route information to get guild commands.
    GetGuildCommands {
        /// The ID of the owner application.
        application_id: u64,
        /// The ID of the guild.
        guild_id: u64,
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
    /// Route information to callback of a interaction.
    InteractionCallback {
        /// The ID of the interaction.
        interaction_id: u64,
        /// The token for the interaction.
        interaction_token: String,
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
    /// Route information to set global commands.
    SetGlobalCommands {
        /// The ID of the owner application.
        application_id: u64,
    },
    /// Route information to set guild commands.
    SetGuildCommands {
        /// The ID of the owner application.
        application_id: u64,
        /// The ID of the guild.
        guild_id: u64,
    },
    /// Route information to sync a guild's integration.
    SyncGuildIntegration {
        /// The ID of the guild.
        guild_id: u64,
        /// The ID of the integration.
        integration_id: u64,
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
    /// Route information to update an emoji.
    UpdateEmoji {
        /// The ID of the emoji.
        emoji_id: u64,
        /// The ID of the guild.
        guild_id: u64,
    },
    /// Route information to update a global command.
    UpdateGlobalCommand {
        /// The ID of the owner application.
        application_id: u64,
        /// The ID of the command.
        command_id: u64,
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
    /// Route information to update a guild command.
    UpdateGuildCommand {
        /// The ID of the owner application.
        application_id: u64,
        /// The ID of the command.
        command_id: u64,
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
    /// Update the original interaction response.
    UpdateInteractionOriginal {
        /// The ID of the owner application.
        application_id: u64,
        /// The token for the interaction.
        interaction_token: String,
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
                Method::PUT,
                Path::GuildsIdMembersId(guild_id),
                format!("guilds/{}/members/{}", guild_id, user_id).into(),
            ),
            Self::AddMemberRole {
                guild_id,
                role_id,
                user_id,
            } => (
                Method::PUT,
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
                    let _ = write!(path, "reason={}", reason);
                }

                (Method::PUT, Path::GuildsIdBansUserId(guild_id), path.into())
            }
            Self::CreateChannel { guild_id } => (
                Method::POST,
                Path::GuildsIdChannels(guild_id),
                format!("guilds/{}/channels", guild_id).into(),
            ),
            Self::CreateEmoji { guild_id } => (
                Method::POST,
                Path::GuildsIdEmojis(guild_id),
                format!("guilds/{}/emojis", guild_id).into(),
            ),
            Self::CreateGlobalCommand { application_id } => (
                Method::POST,
                Path::ApplicationCommand(application_id),
                format!("applications/{}/commands", application_id).into(),
            ),
            Self::CreateGuild => (Method::POST, Path::Guilds, "guilds".into()),
            Self::CreateGuildCommand {
                application_id,
                guild_id,
            } => (
                Method::POST,
                Path::ApplicationGuildCommand(application_id),
                format!(
                    "applications/{}/guilds/{}/commands",
                    application_id, guild_id
                )
                .into(),
            ),
            Self::CreateGuildIntegration { guild_id } => (
                Method::POST,
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

                (Method::POST, Path::GuildsIdPrune(guild_id), path.into())
            }
            Self::CreateInvite { channel_id } => (
                Method::POST,
                Path::ChannelsIdInvites(channel_id),
                format!("channels/{}/invites", channel_id).into(),
            ),
            Self::CreateMessage { channel_id } => (
                Method::POST,
                Path::ChannelsIdMessages(channel_id),
                format!("channels/{}/messages", channel_id).into(),
            ),
            Self::CreatePrivateChannel => (
                Method::POST,
                Path::UsersIdChannels,
                "users/@me/channels".into(),
            ),
            Self::CreateReaction {
                channel_id,
                emoji,
                message_id,
            } => (
                Method::PUT,
                Path::ChannelsIdMessagesIdReactionsUserIdType(channel_id),
                format!(
                    "channels/{}/messages/{}/reactions/{}/@me",
                    channel_id, message_id, emoji,
                )
                .into(),
            ),
            Self::CreateRole { guild_id } => (
                Method::POST,
                Path::GuildsIdRoles(guild_id),
                format!("guilds/{}/roles", guild_id).into(),
            ),
            Self::CreateTypingTrigger { channel_id } => (
                Method::POST,
                Path::ChannelsIdTyping(channel_id),
                format!("channels/{}/typing", channel_id).into(),
            ),
            Self::CreateWebhook { channel_id } => (
                Method::POST,
                Path::ChannelsIdWebhooks(channel_id),
                format!("channels/{}/webhooks", channel_id).into(),
            ),
            Self::CrosspostMessage {
                channel_id,
                message_id,
            } => (
                Method::POST,
                Path::ChannelsIdMessagesIdCrosspost(channel_id),
                format!("channels/{}/messages/{}/crosspost", channel_id, message_id).into(),
            ),
            Self::DeleteBan { guild_id, user_id } => (
                Method::DELETE,
                Path::GuildsIdBansUserId(guild_id),
                format!("guilds/{}/bans/{}", guild_id, user_id).into(),
            ),
            Self::DeleteChannel { channel_id } => (
                Method::DELETE,
                Path::ChannelsId(channel_id),
                format!("channels/{}", channel_id).into(),
            ),
            Self::DeleteEmoji { emoji_id, guild_id } => (
                Method::DELETE,
                Path::GuildsIdEmojisId(guild_id),
                format!("guilds/{}/emojis/{}", guild_id, emoji_id).into(),
            ),
            Self::DeleteGlobalCommand {
                application_id,
                command_id,
            } => (
                Method::DELETE,
                Path::ApplicationCommandId(application_id),
                format!("applications/{}/commands/{}", application_id, command_id).into(),
            ),
            Self::DeleteGuild { guild_id } => (
                Method::DELETE,
                Path::GuildsId(guild_id),
                format!("guilds/{}", guild_id).into(),
            ),
            Self::DeleteGuildCommand {
                application_id,
                command_id,
                guild_id,
            } => (
                Method::DELETE,
                Path::ApplicationGuildCommand(application_id),
                format!(
                    "applications/{}/guilds/{}/commands/{}",
                    application_id, guild_id, command_id
                )
                .into(),
            ),
            Self::DeleteGuildIntegration {
                guild_id,
                integration_id,
            } => (
                Method::DELETE,
                Path::GuildsIdIntegrationsId(guild_id),
                format!("guilds/{}/integrations/{}", guild_id, integration_id).into(),
            ),
            Self::DeleteInvite { code } => (
                Method::DELETE,
                Path::InvitesCode,
                format!("invites/{}", code).into(),
            ),
            Self::DeleteInteractionOriginal {
                application_id,
                interaction_token,
            } => (
                Method::DELETE,
                Path::WebhooksIdTokenMessageId(application_id),
                format!(
                    "webhooks/{}/{}/messages/@original",
                    application_id, interaction_token
                )
                .into(),
            ),
            Self::DeleteMessageReactions {
                channel_id,
                message_id,
            } => (
                Method::DELETE,
                Path::ChannelsIdMessagesIdReactions(channel_id),
                format!("channels/{}/messages/{}/reactions", channel_id, message_id).into(),
            ),
            Self::DeleteMessageSpecficReaction {
                channel_id,
                message_id,
                emoji,
            } => (
                Method::DELETE,
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
                Method::DELETE,
                Path::ChannelsIdMessagesId(Method::DELETE, message_id),
                format!("channels/{}/messages/{}", channel_id, message_id).into(),
            ),
            Self::DeleteMessages { channel_id } => (
                Method::POST,
                Path::ChannelsIdMessagesBulkDelete(channel_id),
                format!("channels/{}/messages/bulk-delete", channel_id).into(),
            ),
            Self::DeletePermissionOverwrite {
                channel_id,
                target_id,
            } => (
                Method::DELETE,
                Path::ChannelsIdPermissionsOverwriteId(channel_id),
                format!("channels/{}/permissions/{}", channel_id, target_id).into(),
            ),
            Self::DeleteReaction {
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
            Self::DeleteRole { guild_id, role_id } => (
                Method::DELETE,
                Path::GuildsIdRolesId(guild_id),
                format!("guilds/{}/roles/{}", guild_id, role_id).into(),
            ),
            Self::DeleteWebhookMessage {
                message_id,
                token,
                webhook_id,
            } => (
                Method::DELETE,
                Path::WebhooksIdTokenMessageId(webhook_id),
                format!("webhooks/{}/{}/messages/{}", webhook_id, token, message_id).into(),
            ),
            Self::DeleteWebhook { token, webhook_id } => {
                let mut path = format!("webhooks/{}", webhook_id);

                if let Some(token) = token {
                    path.push('/');
                    path.push_str(&token);
                }

                (Method::DELETE, Path::WebhooksId(webhook_id), path.into())
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

                (Method::POST, Path::WebhooksId(webhook_id), path.into())
            }
            Self::FollowNewsChannel { channel_id } => (
                Method::POST,
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

                (Method::GET, Path::GuildsIdAuditLogs(guild_id), path.into())
            }
            Self::GetBan { guild_id, user_id } => (
                Method::GET,
                Path::GuildsIdBansId(guild_id),
                format!("guilds/{}/bans/{}", guild_id, user_id).into(),
            ),
            Self::GetBans { guild_id } => (
                Method::GET,
                Path::GuildsIdBans(guild_id),
                format!("guilds/{}/bans", guild_id).into(),
            ),
            Self::GetGatewayBot => (Method::GET, Path::GatewayBot, "gateway/bot".into()),
            Self::GetChannel { channel_id } => (
                Method::GET,
                Path::ChannelsId(channel_id),
                format!("channels/{}", channel_id).into(),
            ),
            Self::GetChannelInvites { channel_id } => (
                Method::GET,
                Path::ChannelsIdInvites(channel_id),
                format!("channels/{}/invites", channel_id).into(),
            ),
            Self::GetChannelWebhooks { channel_id } => (
                Method::GET,
                Path::ChannelsIdWebhooks(channel_id),
                format!("channels/{}/webhooks", channel_id).into(),
            ),
            Self::GetChannels { guild_id } => (
                Method::GET,
                Path::GuildsIdChannels(guild_id),
                format!("guilds/{}/channels", guild_id).into(),
            ),
            Self::GetCurrentUserApplicationInfo => (
                Method::GET,
                Path::OauthApplicationsMe,
                "/oauth2/applications/@me".into(),
            ),
            Self::GetEmoji { emoji_id, guild_id } => (
                Method::GET,
                Path::GuildsIdEmojisId(guild_id),
                format!("guilds/{}/emojis/{}", guild_id, emoji_id).into(),
            ),
            Self::GetEmojis { guild_id } => (
                Method::GET,
                Path::GuildsIdEmojis(guild_id),
                format!("guilds/{}/emojis", guild_id).into(),
            ),
            Self::GetGateway => (Method::GET, Path::Gateway, "gateway".into()),
            Self::GetGlobalCommands { application_id } => (
                Method::GET,
                Path::ApplicationCommand(application_id),
                format!("applications/{}/commands", application_id).into(),
            ),
            Self::GetGuild {
                guild_id,
                with_counts,
            } => {
                let mut path = format!("guilds/{}", guild_id);
                if with_counts {
                    let _ = write!(path, "?with_counts=true");
                }
                (Method::GET, Path::GuildsId(guild_id), path.into())
            }
            Self::GetGuildCommands {
                application_id,
                guild_id,
            } => (
                Method::GET,
                Path::ApplicationGuildCommand(application_id),
                format!(
                    "applications/{}/guilds/{}/commands",
                    application_id, guild_id
                )
                .into(),
            ),
            Self::GetGuildWidget { guild_id } => (
                Method::GET,
                Path::GuildsIdWidget(guild_id),
                format!("guilds/{}/widget", guild_id).into(),
            ),
            Self::GetGuildIntegrations { guild_id } => {
                (Method::GET, Path::GuildsIdIntegrations(guild_id), {
                    format!("guilds/{}/integrations", guild_id).into()
                })
            }
            Self::GetGuildInvites { guild_id } => (
                Method::GET,
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

                (Method::GET, Path::GuildsIdMembers(guild_id), path.into())
            }
            Self::GetGuildPreview { guild_id } => (
                Method::GET,
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

                (Method::GET, Path::GuildsIdPrune(guild_id), path.into())
            }
            Self::GetGuildRoles { guild_id } => (
                Method::GET,
                Path::GuildsIdRoles(guild_id),
                format!("guilds/{}/roles", guild_id).into(),
            ),
            Self::GetGuildVanityUrl { guild_id } => (
                Method::GET,
                Path::GuildsIdVanityUrl(guild_id),
                format!("guilds/{}/vanity-url", guild_id).into(),
            ),
            Self::GetGuildVoiceRegions { guild_id } => (
                Method::GET,
                Path::GuildsIdRegions(guild_id),
                format!("guilds/{}/regions", guild_id).into(),
            ),
            Self::GetGuildWebhooks { guild_id } => (
                Method::GET,
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

                (Method::GET, Path::UsersIdGuilds, path.into())
            }
            Self::GetInvite { code, with_counts } => (
                Method::GET,
                Path::InvitesCode,
                format!("invites/{}?with-counts={}", code, with_counts).into(),
            ),
            Self::GetMember { guild_id, user_id } => (
                Method::GET,
                Path::GuildsIdMembersId(guild_id),
                format!("guilds/{}/members/{}", guild_id, user_id).into(),
            ),
            Self::GetMessage {
                channel_id,
                message_id,
            } => (
                Method::GET,
                Path::ChannelsIdMessagesId(Method::GET, channel_id),
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
                    Method::GET,
                    Path::ChannelsIdMessages(channel_id),
                    path.into(),
                )
            }
            Self::GetPins { channel_id } => (
                Method::GET,
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
                    Method::GET,
                    Path::ChannelsIdMessagesIdReactions(channel_id),
                    path.into(),
                )
            }
            Self::GetUserConnections => (
                Method::GET,
                Path::UsersIdConnections,
                "users/@me/connections".into(),
            ),
            Self::GetUserPrivateChannels => (
                Method::GET,
                Path::UsersIdChannels,
                "users/@me/channels".into(),
            ),
            Self::GetUser { target_user } => (
                Method::GET,
                Path::UsersId,
                format!("users/{}", target_user).into(),
            ),
            Self::GetVoiceRegions => (Method::GET, Path::VoiceRegions, "voice/regions".into()),
            Self::GetWebhook { token, webhook_id } => {
                let mut path = format!("webhooks/{}", webhook_id);

                if let Some(token) = token {
                    path.push('/');
                    path.push_str(&token);
                }

                (Method::GET, Path::WebhooksId(webhook_id), path.into())
            }
            Self::InteractionCallback {
                interaction_id,
                interaction_token,
            } => (
                Method::POST,
                Path::InteractionCallback(interaction_id),
                format!(
                    "interactions/{}/{}/callback",
                    interaction_id, interaction_token
                )
                .into(),
            ),
            Self::LeaveGuild { guild_id } => (
                Method::DELETE,
                Path::UsersIdGuildsId,
                format!("users/@me/guilds/{}", guild_id).into(),
            ),
            Self::PinMessage {
                channel_id,
                message_id,
            } => (
                Method::PUT,
                Path::ChannelsIdPins(channel_id),
                format!("channels/{}/pins/{}", channel_id, message_id).into(),
            ),
            Self::RemoveMember { guild_id, user_id } => (
                Method::DELETE,
                Path::GuildsIdMembersId(guild_id),
                format!("guilds/{}/members/{}", guild_id, user_id).into(),
            ),
            Self::RemoveMemberRole {
                guild_id,
                role_id,
                user_id,
            } => (
                Method::DELETE,
                Path::GuildsIdMembersIdRolesId(guild_id),
                format!("guilds/{}/members/{}/roles/{}", guild_id, user_id, role_id).into(),
            ),
            Self::SetGlobalCommands { application_id } => (
                Method::PUT,
                Path::ApplicationCommand(application_id),
                format!("applications/{}/commands", application_id).into(),
            ),
            Self::SetGuildCommands {
                application_id,
                guild_id,
            } => (
                Method::PUT,
                Path::ApplicationGuildCommand(application_id),
                format!(
                    "applications/{}/guilds/{}/commands",
                    application_id, guild_id
                )
                .into(),
            ),
            Self::SyncGuildIntegration {
                guild_id,
                integration_id,
            } => (
                Method::POST,
                Path::GuildsIdIntegrationsIdSync(guild_id),
                format!("guilds/{}/integrations/{}/sync", guild_id, integration_id).into(),
            ),
            Self::UnpinMessage {
                channel_id,
                message_id,
            } => (
                Method::DELETE,
                Path::ChannelsIdPinsMessageId(channel_id),
                format!("channels/{}/pins/{}", channel_id, message_id).into(),
            ),
            Self::UpdateChannel { channel_id } => (
                Method::PATCH,
                Path::ChannelsId(channel_id),
                format!("channels/{}", channel_id).into(),
            ),
            Self::UpdateCurrentUser => (Method::PATCH, Path::UsersId, "users/@me".into()),
            Self::UpdateEmoji { emoji_id, guild_id } => (
                Method::PATCH,
                Path::GuildsIdEmojisId(guild_id),
                format!("guilds/{}/emojis/{}", guild_id, emoji_id).into(),
            ),
            Self::UpdateGlobalCommand {
                application_id,
                command_id,
            } => (
                Method::PATCH,
                Path::ApplicationCommandId(application_id),
                format!("applications/{}/commands/{}", application_id, command_id).into(),
            ),
            Self::UpdateGuild { guild_id } => (
                Method::PATCH,
                Path::GuildsId(guild_id),
                format!("guilds/{}", guild_id).into(),
            ),
            Self::UpdateGuildChannels { guild_id } => (
                Method::PATCH,
                Path::GuildsIdChannels(guild_id),
                format!("guilds/{}/channels", guild_id).into(),
            ),
            Self::UpdateGuildCommand {
                application_id,
                command_id,
                guild_id,
            } => (
                Method::PATCH,
                Path::ApplicationGuildCommand(application_id),
                format!(
                    "applications/{}/guilds/{}/commands/{}",
                    application_id, guild_id, command_id
                )
                .into(),
            ),
            Self::UpdateGuildWidget { guild_id } => (
                Method::PATCH,
                Path::GuildsIdWidget(guild_id),
                format!("guilds/{}/widget", guild_id).into(),
            ),
            Self::UpdateGuildIntegration {
                guild_id,
                integration_id,
            } => (
                Method::PATCH,
                Path::GuildsIdIntegrationsId(guild_id),
                format!("guilds/{}/integrations/{}", guild_id, integration_id,).into(),
            ),
            Self::UpdateInteractionOriginal {
                application_id,
                interaction_token,
            } => (
                Method::PATCH,
                Path::WebhooksIdTokenMessageId(application_id),
                format!(
                    "webhooks/{}/{}/messages/@original",
                    application_id, interaction_token
                )
                .into(),
            ),
            Self::UpdateMember { guild_id, user_id } => (
                Method::PATCH,
                Path::GuildsIdMembersId(guild_id),
                format!("guilds/{}/members/{}", guild_id, user_id).into(),
            ),
            Self::UpdateMessage {
                channel_id,
                message_id,
            } => (
                Method::PATCH,
                Path::ChannelsIdMessagesId(Method::PATCH, channel_id),
                format!("channels/{}/messages/{}", channel_id, message_id).into(),
            ),
            Self::UpdateNickname { guild_id } => (
                Method::PATCH,
                Path::GuildsIdMembersMeNick(guild_id),
                format!("guilds/{}/members/@me/nick", guild_id).into(),
            ),
            Self::UpdatePermissionOverwrite {
                channel_id,
                target_id,
            } => (
                Method::PUT,
                Path::ChannelsIdPermissionsOverwriteId(channel_id),
                format!("channels/{}/permissions/{}", channel_id, target_id).into(),
            ),
            Self::UpdateRole { guild_id, role_id } => (
                Method::PATCH,
                Path::GuildsIdRolesId(guild_id),
                format!("guilds/{}/roles/{}", guild_id, role_id).into(),
            ),
            Self::UpdateRolePositions { guild_id } => (
                Method::PATCH,
                Path::GuildsIdRolesId(guild_id),
                format!("guilds/{}/roles", guild_id).into(),
            ),
            Self::UpdateWebhookMessage {
                message_id,
                token,
                webhook_id,
            } => (
                Method::PATCH,
                Path::WebhooksIdTokenMessageId(webhook_id),
                format!("webhooks/{}/{}/messages/{}", webhook_id, token, message_id).into(),
            ),
            Self::UpdateWebhook { token, webhook_id } => {
                let mut path = format!("webhooks/{}", webhook_id);

                if let Some(token) = token {
                    path.push('/');
                    path.push_str(&token);
                }

                (Method::PATCH, Path::WebhooksId(webhook_id), path.into())
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{Path, PathParseError};
    use hyper::Method;
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
            PathParseError::MessageIdWithoutMethod { channel_id: 123 },
            Path::from_str("channels/123/messages/456").unwrap_err()
        );
        assert_eq!(
            Path::ChannelsIdMessagesId(Method::GET, 123),
            Path::try_from((Method::GET, "/channels/123/messages/456"))?,
        );

        Ok(())
    }
}
