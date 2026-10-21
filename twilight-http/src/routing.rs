use percent_encoding::{NON_ALPHANUMERIC, utf8_percent_encode};

use crate::{
    query_formatter::{QueryArray, QueryStringFormatter},
    request::{Method, channel::reaction::RequestReactionType},
};
use std::fmt::{Display, Formatter, Result as FmtResult};
use twilight_model::id::{Id, marker::RoleMarker};

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
#[non_exhaustive]
pub enum Route<'a> {
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
    /// Route information to add a member to a thread.
    AddThreadMember {
        /// ID of the thread.
        channel_id: u64,
        /// ID of the member.
        user_id: u64,
    },
    /// Route information to create an auto moderation rule.
    CreateAutoModerationRule {
        /// ID of the guild.
        guild_id: u64,
    },
    /// Route information to create a ban on a user in a guild.
    CreateBan {
        /// The ID of the guild.
        guild_id: u64,
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
    /// Route information to create a thread in a forum channel.
    CreateForumThread {
        /// The ID of the channel.
        channel_id: u64,
    },
    /// Route information to create a prune in a guild.
    CreateGuildPrune {
        /// Whether to compute the number of pruned users.
        compute_prune_count: Option<bool>,
        /// The number of days that a user must be offline before being able to
        /// be pruned.
        days: Option<u16>,
        /// The ID of the guild.
        guild_id: u64,
        /// The roles to filter the prune by.
        ///
        /// A user must have at least one of these roles to be able to be
        /// pruned.
        include_roles: &'a [Id<RoleMarker>],
    },
    /// Route information to create a sticker in a guild.
    CreateGuildSticker {
        /// ID of the guild.
        guild_id: u64,
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
    /// Route information to create a reaction on a message.
    CreateReaction {
        /// The ID of the channel.
        channel_id: u64,
        /// The URI encoded custom or unicode emoji.
        emoji: &'a RequestReactionType<'a>,
        /// The ID of the message.
        message_id: u64,
    },
    /// Route information to create a role in a guild.
    CreateRole {
        /// The ID of the guild.
        guild_id: u64,
    },
    /// Route information to create a stage instance.
    CreateStageInstance,
    /// Route information to create a thread in a channel.
    CreateThread {
        /// ID of the channel.
        channel_id: u64,
    },
    /// Route information to create a thread from a message.
    CreateThreadFromMessage {
        /// ID of the channel.
        channel_id: u64,
        /// ID of the message.
        message_id: u64,
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
    /// Route information to delete an auto moderation rule for a guild.
    DeleteAutoModerationRule {
        /// ID of the guild.
        guild_id: u64,
        /// ID of the auto moderation rule.
        auto_moderation_rule_id: u64,
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
    /// Route information to delete a guild sticker.
    DeleteGuildSticker {
        /// ID of the guild.
        guild_id: u64,
        /// ID of the sticker.
        sticker_id: u64,
    },
    /// Route information to delete an invite.
    DeleteInvite {
        /// The unique invite code.
        code: &'a str,
    },
    /// Route information to delete a channel's message.
    DeleteMessage {
        /// The ID of the channel.
        channel_id: u64,
        /// The ID of the message.
        message_id: u64,
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
    DeleteMessageSpecificReaction {
        /// The ID of the channel.
        channel_id: u64,
        /// The URI encoded custom or unicode emoji.
        emoji: &'a RequestReactionType<'a>,
        /// The ID of the message.
        message_id: u64,
    },
    /// Route information to bulk delete messages in a channel.
    DeleteMessages {
        /// The ID of the channel.
        channel_id: u64,
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
        emoji: &'a RequestReactionType<'a>,
        /// The ID of the message.
        message_id: u64,
        /// The ID of the user.
        user_id: u64,
    },
    /// Route information to delete the current user's reaction on a message.
    DeleteReactionCurrentUser {
        /// ID of the channel.
        channel_id: u64,
        /// URI encoded custom or unicode emoji.
        emoji: &'a RequestReactionType<'a>,
        /// ID of the message.
        message_id: u64,
    },
    /// Route information to delete a guild's role.
    DeleteRole {
        /// The ID of the guild.
        guild_id: u64,
        /// The ID of the role.
        role_id: u64,
    },
    /// Route information to delete a stage instance.
    DeleteStageInstance {
        /// ID of the stage channel.
        channel_id: u64,
    },
    /// Route information to delete a webhook.
    DeleteWebhook {
        /// The token of the webhook.
        token: Option<&'a str>,
        /// The ID of the webhook.
        webhook_id: u64,
    },
    /// Route information to delete a message created by a webhook.
    DeleteWebhookMessage {
        message_id: u64,
        /// ID of the thread channel, if there is one.
        thread_id: Option<u64>,
        token: &'a str,
        webhook_id: u64,
    },
    /// Route information to execute a webhook by ID and token.
    ExecuteWebhook {
        /// ID of the thread channel, if there is one.
        thread_id: Option<u64>,
        /// The token of the webhook.
        token: &'a str,
        /// Whether to wait for a message response.
        wait: Option<bool>,
        /// Whether the message includes Components V2.
        with_components: Option<bool>,
        /// The ID of the webhook.
        webhook_id: u64,
    },
    /// Route information to follow a news channel.
    FollowNewsChannel {
        /// The ID of the channel to follow.
        channel_id: u64,
    },
    /// Route information to get active threads in a channel.
    GetActiveThreads {
        /// ID of the guild.
        guild_id: u64,
    },
    /// Route information to get a paginated list of audit logs in a guild.
    GetAuditLogs {
        /// The type of action to get audit logs for.
        action_type: Option<u64>,
        /// The minimum ID of audit logs to get.
        after: Option<u64>,
        /// The maximum ID of audit logs to get.
        before: Option<u64>,
        /// The ID of the guild.
        guild_id: u64,
        /// The maximum number of audit logs to get.
        limit: Option<u16>,
        /// The ID of the user, if specified.
        user_id: Option<u64>,
    },
    /// Route information to get an auto moderation rule for a guild.
    GetAutoModerationRule {
        /// ID of the guild.
        guild_id: u64,
        /// ID of the auto moderation rule.
        auto_moderation_rule_id: u64,
    },
    /// Route information to get information about a single ban in a guild.
    GetBan {
        /// The ID of the guild.
        guild_id: u64,
        /// The ID of the user.
        user_id: u64,
    },
    /// Route information to get a guild's bans with parameters.
    GetBansWithParameters {
        /// User ID after which to retrieve bans.
        after: Option<u64>,
        /// User ID before which to retrieve bans.
        before: Option<u64>,
        /// Maximum number of bans to retrieve.
        limit: Option<u16>,
        /// ID of the guild.
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
    /// Route information to get the current user's voice state.
    GetCurrentUserVoiceState {
        /// ID of the guild.
        guild_id: u64,
    },
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
    /// Route information to get a guild.
    GetGuild {
        /// The ID of the guild.
        guild_id: u64,
        /// Whether to include approximate member and presence counts for the
        /// guild.
        with_counts: bool,
    },
    /// Route information to get a list of automation rules for a guild.
    GetGuildAutoModerationRules {
        /// ID of the guild.
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
        limit: Option<u16>,
    },
    /// Route information to get a guild's onboarding information.
    GetGuildOnboarding {
        /// The ID of the guild to get onboarding information for.
        guild_id: u64,
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
        days: Option<u16>,
        /// The ID of the guild.
        guild_id: u64,
        /// The roles to filter the prune by.
        ///
        /// A user must have at least one of these roles to be able to be
        /// pruned.
        include_roles: &'a [Id<RoleMarker>],
    },
    /// Route information to get guild's roles' member counts.
    GetGuildRoleMemberCounts { guild_id: u64 },
    /// Route information to get guild's roles.
    GetGuildRoles {
        /// The ID of the guild.
        guild_id: u64,
    },
    /// Route information to get a guild's sticker.
    GetGuildSticker {
        /// ID of the guild.
        guild_id: u64,
        /// ID of the stickers.
        sticker_id: u64,
    },
    /// Route information to get a guild's stickers.
    GetGuildStickers {
        /// ID of the guild.
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
    /// Route information to get a guild's welcome screen.
    GetGuildWelcomeScreen {
        /// ID of the guild.
        guild_id: u64,
    },
    /// Route information to get a guild's widget.
    GetGuildWidget {
        /// ID of the guild.
        guild_id: u64,
    },
    /// Route information to get a guild's widget settings.
    GetGuildWidgetSettings {
        /// ID of the guild.
        guild_id: u64,
    },
    /// Route information to get an invite with an expiration.
    GetInviteWithExpiration {
        /// The unique invite code.
        code: &'a str,
        /// Whether to retrieve statistics about the invite.
        with_counts: bool,
        /// Whether to retrieve the expiration date of the invite.
        with_expiration: bool,
    },
    /// Route information to get joined private archived threads in a channel.
    GetJoinedPrivateArchivedThreads {
        /// Optional timestamp to return threads before.
        before: Option<u64>,
        /// ID of the channel.
        channel_id: u64,
        /// Optional maximum number of threads to return.
        limit: Option<u64>,
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
        limit: Option<u16>,
    },
    /// Route information to get a channel's pins.
    GetPins {
        /// The ID of the channel.
        channel_id: u64,
        /// Optionally the limit of pins to show in the response. (1-50) (default: 50)
        limit: Option<u16>,
        /// Optionally the timestamp as a filter to only show pins before the provided timestamp.
        before: Option<String>,
    },
    /// Route information to get private archived threads in a channel.
    GetPrivateArchivedThreads {
        /// Optional timestamp to return threads before.
        before: Option<&'a str>,
        /// ID of the channel.
        channel_id: u64,
        /// Optional maximum number of threads to return.
        limit: Option<u64>,
    },
    /// Route information to get public archived threads in a channel.
    GetPublicArchivedThreads {
        /// Optional timestamp to return threads before.
        before: Option<&'a str>,
        /// ID of the channel.
        channel_id: u64,
        /// Optional maximum number of threads to return.
        limit: Option<u64>,
    },
    /// Route information to get the users who reacted to a message with a
    /// specified emoji.
    GetReactionUsers {
        /// The minimum ID of users to get.
        after: Option<u64>,
        /// The ID of the channel.
        channel_id: u64,
        /// The URI encoded custom or unicode emoji.
        emoji: &'a RequestReactionType<'a>,
        /// The maximum number of users to retrieve.
        limit: Option<u16>,
        /// The ID of the message.
        message_id: u64,
        /// The type of reactions to fetch.
        kind: Option<u8>,
    },
    /// Route information to get a guild's role.
    GetRole {
        /// The ID of the guild.
        guild_id: u64,
        /// The ID of the role.
        role_id: u64,
    },
    /// Route information to get a stage instance.
    GetStageInstance {
        /// ID of the stage channel.
        channel_id: u64,
    },
    /// Route information to get a member of a thread.
    GetThreadMember {
        /// ID of the thread.
        channel_id: u64,
        /// ID of the member.
        user_id: u64,
    },
    /// Route information to get members of a thread.
    GetThreadMembers {
        /// Fetch thread members after this user ID.
        after: Option<u64>,
        /// ID of the thread.
        channel_id: u64,
        /// Maximum number of thread members to return.
        ///
        /// Must be between 1 and 100. Defaults to 100.
        limit: Option<u32>,
        /// Whether to include associated member objects.
        with_member: Option<bool>,
    },
    /// Route information to get a user's voice state.
    GetUserVoiceState {
        /// The ID of the guild.
        guild_id: u64,
        /// ID of the target user.
        user_id: u64,
    },
    /// Route information to get a webhook by ID, optionally with a token if the
    /// current user doesn't have access to it.
    GetWebhook {
        /// The token of the webhook.
        token: Option<&'a str>,
        /// The ID of the webhook.
        webhook_id: u64,
    },
    /// Route information to get a previously-sent webhook message.
    GetWebhookMessage {
        /// ID of the message.
        message_id: u64,
        /// ID of the thread channel, if there is one.
        thread_id: Option<u64>,
        /// Token of the webhook.
        token: &'a str,
        /// ID of the webhook.
        webhook_id: u64,
    },
    /// Route information to join a thread as the current user.
    JoinThread {
        /// ID of the thread.
        channel_id: u64,
    },
    /// Route information to leave a thread as the current user.
    LeaveThread {
        /// ID of the thread.
        channel_id: u64,
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
    /// Route information to remove a member from a thread.
    RemoveThreadMember {
        /// ID of the thread.
        channel_id: u64,
        /// ID of the member.
        user_id: u64,
    },
    /// Route information to search for members in a guild.
    SearchGuildMembers {
        /// ID of the guild to search in.
        guild_id: u64,
        /// Upper limit of members to query for.
        limit: Option<u16>,
        /// Query to search by.
        query: &'a str,
    },
    /// Route information to unpin a message from a channel.
    UnpinMessage {
        /// The ID of the channel.
        channel_id: u64,
        /// The ID of the message.
        message_id: u64,
    },
    /// Route information to update an auto moderation rule for a guild.
    UpdateAutoModerationRule {
        /// ID of the auto moderation rule.
        auto_moderation_rule_id: u64,
        /// ID of the guild.
        guild_id: u64,
    },
    /// Route information to update a channel, such as a guild channel or group.
    UpdateChannel {
        /// The ID of the channel.
        channel_id: u64,
    },
    /// Route information to update the current member.
    UpdateCurrentMember {
        /// ID of the guild.
        guild_id: u64,
    },
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
    /// Route information to update a guild's MFA level.
    UpdateGuildMfa {
        /// ID of the guild.
        guild_id: u64,
    },
    UpdateGuildOnboarding {
        /// The ID of the guild to update onboarding information for.
        guild_id: u64,
    },
    /// Route information to update a guild sticker.
    UpdateGuildSticker {
        /// ID of the guild.
        guild_id: u64,
        /// ID of the sticker.
        sticker_id: u64,
    },
    /// Route information to update a guild's welcome screen.
    UpdateGuildWelcomeScreen {
        /// ID of the guild.
        guild_id: u64,
    },
    /// Route information to update a guild's widget settings.
    UpdateGuildWidgetSettings {
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
    /// Route information to update an existing stage instance.
    UpdateStageInstance {
        /// ID of the stage channel.
        channel_id: u64,
    },
    /// Route information to update a user's voice state.
    UpdateUserVoiceState {
        /// ID of the guild.
        guild_id: u64,
        /// ID of the user.
        user_id: u64,
    },
    /// Route information to update a webhook.
    UpdateWebhook {
        /// The token of the webhook.
        token: Option<&'a str>,
        /// The ID of the webhook.
        webhook_id: u64,
    },
    /// Route information to update a message created by a webhook.
    UpdateWebhookMessage {
        message_id: u64,
        /// ID of the thread channel, if there is one.
        thread_id: Option<u64>,
        token: &'a str,
        webhook_id: u64,
    },
}

impl Route<'_> {
    /// HTTP method of the route.
    ///
    /// # Examples
    ///
    /// Assert that the [`GetGuild`] route returns [`Method::Get`]:
    ///
    /// ```
    /// use twilight_http::{request::Method, routing::Route};
    ///
    /// let route = Route::GetGuild {
    ///     guild_id: 123,
    ///     with_counts: false,
    /// };
    ///
    /// assert_eq!(Method::Get, route.method());
    /// ```
    ///
    /// [`GetGuild`]: Self::GetGuild
    #[allow(clippy::too_many_lines)]
    pub const fn method(&self) -> Method {
        match self {
            Self::DeleteAutoModerationRule { .. }
            | Self::DeleteBan { .. }
            | Self::DeleteChannel { .. }
            | Self::DeleteEmoji { .. }
            | Self::DeleteGuild { .. }
            | Self::DeleteGuildIntegration { .. }
            | Self::DeleteGuildSticker { .. }
            | Self::DeleteInvite { .. }
            | Self::DeleteMessageReactions { .. }
            | Self::DeleteMessageSpecificReaction { .. }
            | Self::DeleteMessage { .. }
            | Self::DeletePermissionOverwrite { .. }
            | Self::DeleteReactionCurrentUser { .. }
            | Self::DeleteReaction { .. }
            | Self::DeleteRole { .. }
            | Self::DeleteStageInstance { .. }
            | Self::DeleteWebhookMessage { .. }
            | Self::DeleteWebhook { .. }
            | Self::LeaveThread { .. }
            | Self::RemoveMember { .. }
            | Self::RemoveMemberRole { .. }
            | Self::RemoveThreadMember { .. }
            | Self::UnpinMessage { .. } => Method::Delete,
            Self::GetActiveThreads { .. }
            | Self::GetAuditLogs { .. }
            | Self::GetAutoModerationRule { .. }
            | Self::GetBan { .. }
            | Self::GetBansWithParameters { .. }
            | Self::GetChannel { .. }
            | Self::GetChannelInvites { .. }
            | Self::GetChannelWebhooks { .. }
            | Self::GetChannels { .. }
            | Self::GetCurrentUserVoiceState { .. }
            | Self::GetEmoji { .. }
            | Self::GetEmojis { .. }
            | Self::GetGuild { .. }
            | Self::GetGuildAutoModerationRules { .. }
            | Self::GetGuildIntegrations { .. }
            | Self::GetGuildInvites { .. }
            | Self::GetGuildMembers { .. }
            | Self::GetGuildOnboarding { .. }
            | Self::GetGuildPreview { .. }
            | Self::GetGuildPruneCount { .. }
            | Self::GetGuildRoles { .. }
            | Self::GetGuildRoleMemberCounts { .. }
            | Self::GetGuildSticker { .. }
            | Self::GetGuildStickers { .. }
            | Self::GetGuildVanityUrl { .. }
            | Self::GetGuildVoiceRegions { .. }
            | Self::GetGuildWelcomeScreen { .. }
            | Self::GetGuildWebhooks { .. }
            | Self::GetGuildWidget { .. }
            | Self::GetGuildWidgetSettings { .. }
            | Self::GetInviteWithExpiration { .. }
            | Self::GetMember { .. }
            | Self::GetMessage { .. }
            | Self::GetMessages { .. }
            | Self::GetPins { .. }
            | Self::GetJoinedPrivateArchivedThreads { .. }
            | Self::GetPrivateArchivedThreads { .. }
            | Self::GetPublicArchivedThreads { .. }
            | Self::GetReactionUsers { .. }
            | Self::GetRole { .. }
            | Self::GetStageInstance { .. }
            | Self::GetThreadMember { .. }
            | Self::GetThreadMembers { .. }
            | Self::GetUserVoiceState { .. }
            | Self::GetWebhook { .. }
            | Self::GetWebhookMessage { .. }
            | Self::SearchGuildMembers { .. } => Method::Get,
            Self::UpdateAutoModerationRule { .. }
            | Self::UpdateChannel { .. }
            | Self::UpdateCurrentMember { .. }
            | Self::UpdateCurrentUserVoiceState { .. }
            | Self::UpdateEmoji { .. }
            | Self::UpdateGuild { .. }
            | Self::UpdateGuildChannels { .. }
            | Self::UpdateGuildMfa { .. }
            | Self::UpdateGuildWidgetSettings { .. }
            | Self::UpdateGuildSticker { .. }
            | Self::UpdateGuildWelcomeScreen { .. }
            | Self::UpdateMember { .. }
            | Self::UpdateMessage { .. }
            | Self::UpdateRole { .. }
            | Self::UpdateRolePositions { .. }
            | Self::UpdateStageInstance { .. }
            | Self::UpdateUserVoiceState { .. }
            | Self::UpdateWebhookMessage { .. }
            | Self::UpdateWebhook { .. } => Method::Patch,
            Self::CreateChannel { .. }
            | Self::CreateEmoji { .. }
            | Self::CreateForumThread { .. }
            | Self::CreateAutoModerationRule { .. }
            | Self::CreateGuildPrune { .. }
            | Self::CreateGuildSticker { .. }
            | Self::CreateInvite { .. }
            | Self::CreateMessage { .. }
            | Self::CreateThread { .. }
            | Self::CreateThreadFromMessage { .. }
            | Self::CreateRole { .. }
            | Self::CreateStageInstance { .. }
            | Self::CreateTypingTrigger { .. }
            | Self::CreateWebhook { .. }
            | Self::CrosspostMessage { .. }
            | Self::DeleteMessages { .. }
            | Self::ExecuteWebhook { .. }
            | Self::FollowNewsChannel { .. } => Method::Post,
            Self::AddGuildMember { .. }
            | Self::AddMemberRole { .. }
            | Self::AddThreadMember { .. }
            | Self::CreateBan { .. }
            | Self::CreateReaction { .. }
            | Self::JoinThread { .. }
            | Self::PinMessage { .. }
            | Self::UpdateGuildOnboarding { .. }
            | Self::UpdatePermissionOverwrite { .. } => Method::Put,
        }
    }
}

impl Display for Route<'_> {
    // Notably, we don't use macros like `write!` or `format_args!` due to them
    // both compiling slowly and performing slowly during runtime.
    //
    // See:
    // <https://github.com/rust-lang/rust/issues/76490>
    // <https://github.com/rust-lang/rust/issues/10761>
    #[allow(clippy::too_many_lines)]
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            Route::AddGuildMember { guild_id, user_id }
            | Route::GetMember { guild_id, user_id }
            | Route::RemoveMember { guild_id, user_id }
            | Route::UpdateMember { guild_id, user_id } => {
                f.write_str("guilds/")?;
                Display::fmt(guild_id, f)?;
                f.write_str("/members/")?;

                Display::fmt(user_id, f)
            }
            Route::AddMemberRole {
                guild_id,
                role_id,
                user_id,
            }
            | Route::RemoveMemberRole {
                guild_id,
                role_id,
                user_id,
            } => {
                f.write_str("guilds/")?;
                Display::fmt(guild_id, f)?;
                f.write_str("/members/")?;
                Display::fmt(user_id, f)?;
                f.write_str("/roles/")?;

                Display::fmt(role_id, f)
            }
            Route::AddThreadMember {
                channel_id,
                user_id,
            }
            | Route::GetThreadMember {
                channel_id,
                user_id,
            }
            | Route::RemoveThreadMember {
                channel_id,
                user_id,
            } => {
                f.write_str("channels/")?;
                Display::fmt(channel_id, f)?;
                f.write_str("/thread-members/")?;

                Display::fmt(user_id, f)
            }
            Route::CreateAutoModerationRule { guild_id, .. }
            | Route::GetGuildAutoModerationRules { guild_id, .. } => {
                f.write_str("guilds/")?;
                Display::fmt(guild_id, f)?;

                f.write_str("/auto-moderation/rules")
            }
            Route::CreateChannel { guild_id }
            | Route::GetChannels { guild_id }
            | Route::UpdateGuildChannels { guild_id } => {
                f.write_str("guilds/")?;
                Display::fmt(guild_id, f)?;

                f.write_str("/channels")
            }
            Route::CreateEmoji { guild_id } | Route::GetEmojis { guild_id } => {
                f.write_str("guilds/")?;
                Display::fmt(guild_id, f)?;

                f.write_str("/emojis")
            }
            Route::DeleteAutoModerationRule {
                auto_moderation_rule_id,
                guild_id,
                ..
            }
            | Route::GetAutoModerationRule {
                auto_moderation_rule_id,
                guild_id,
                ..
            }
            | Route::UpdateAutoModerationRule {
                auto_moderation_rule_id,
                guild_id,
                ..
            } => {
                f.write_str("guilds/")?;
                Display::fmt(guild_id, f)?;
                f.write_str("/auto-moderation/rules/")?;

                Display::fmt(auto_moderation_rule_id, f)
            }
            Route::GetGuildIntegrations { guild_id } => {
                f.write_str("guilds/")?;
                Display::fmt(guild_id, f)?;

                f.write_str("/integrations")
            }
            Route::CreateGuildPrune {
                compute_prune_count,
                days,
                guild_id,
                include_roles,
            } => {
                f.write_str("guilds/")?;
                Display::fmt(guild_id, f)?;
                f.write_str("/prune")?;

                let mut writer = QueryStringFormatter::new(f);

                writer.write_opt_param("compute_prune_count", compute_prune_count.as_ref())?;
                writer.write_opt_param("days", days.as_ref())?;

                if !include_roles.is_empty() {
                    writer.write_param("include_roles", &QueryArray(*include_roles))?;
                }

                Ok(())
            }
            Route::CreateGuildSticker { guild_id, .. }
            | Route::GetGuildStickers { guild_id, .. } => {
                f.write_str("guilds/")?;
                Display::fmt(guild_id, f)?;

                f.write_str("/stickers")
            }
            Route::CreateInvite { channel_id } | Route::GetChannelInvites { channel_id } => {
                f.write_str("channels/")?;
                Display::fmt(channel_id, f)?;

                f.write_str("/invites")
            }
            Route::CreateMessage { channel_id } => {
                f.write_str("channels/")?;
                Display::fmt(channel_id, f)?;

                f.write_str("/messages")
            }
            Route::CreateReaction {
                channel_id,
                emoji,
                message_id,
            }
            | Route::DeleteReactionCurrentUser {
                channel_id,
                emoji,
                message_id,
            } => {
                f.write_str("channels/")?;
                Display::fmt(channel_id, f)?;
                f.write_str("/messages/")?;
                Display::fmt(message_id, f)?;
                f.write_str("/reactions/")?;
                Display::fmt(&emoji, f)?;

                f.write_str("/@me")
            }
            Route::CreateRole { guild_id }
            | Route::GetGuildRoles { guild_id }
            | Route::UpdateRolePositions { guild_id } => {
                f.write_str("guilds/")?;
                Display::fmt(guild_id, f)?;

                f.write_str("/roles")
            }
            Route::GetGuildRoleMemberCounts { guild_id } => {
                f.write_str("guilds/")?;
                Display::fmt(guild_id, f)?;

                f.write_str("/roles/member-counts")
            }
            Route::CreateStageInstance { .. } => f.write_str("stage-instances"),
            Route::CreateForumThread { channel_id } | Route::CreateThread { channel_id } => {
                f.write_str("channels/")?;
                Display::fmt(channel_id, f)?;

                f.write_str("/threads")
            }
            Route::CreateThreadFromMessage {
                channel_id,
                message_id,
            } => {
                f.write_str("channels/")?;
                Display::fmt(channel_id, f)?;
                f.write_str("/messages/")?;
                Display::fmt(message_id, f)?;

                f.write_str("/threads")
            }
            Route::CreateTypingTrigger { channel_id } => {
                f.write_str("channels/")?;
                Display::fmt(channel_id, f)?;

                f.write_str("/typing")
            }
            Route::CreateWebhook { channel_id } | Route::GetChannelWebhooks { channel_id } => {
                f.write_str("channels/")?;
                Display::fmt(channel_id, f)?;

                f.write_str("/webhooks")
            }
            Route::CrosspostMessage {
                channel_id,
                message_id,
            } => {
                f.write_str("channels/")?;
                Display::fmt(channel_id, f)?;
                f.write_str("/messages/")?;
                Display::fmt(message_id, f)?;

                f.write_str("/crosspost")
            }
            Route::DeleteBan { guild_id, user_id }
            | Route::GetBan { guild_id, user_id }
            | Route::CreateBan { guild_id, user_id } => {
                f.write_str("guilds/")?;
                Display::fmt(guild_id, f)?;
                f.write_str("/bans/")?;

                Display::fmt(user_id, f)
            }
            Route::DeleteChannel { channel_id }
            | Route::GetChannel { channel_id }
            | Route::UpdateChannel { channel_id } => {
                f.write_str("channels/")?;

                Display::fmt(channel_id, f)
            }
            Route::DeleteEmoji { emoji_id, guild_id }
            | Route::GetEmoji { emoji_id, guild_id }
            | Route::UpdateEmoji { emoji_id, guild_id } => {
                f.write_str("guilds/")?;
                Display::fmt(guild_id, f)?;
                f.write_str("/emojis/")?;

                Display::fmt(emoji_id, f)
            }
            Route::DeleteGuild { guild_id } | Route::UpdateGuild { guild_id } => {
                f.write_str("guilds/")?;

                Display::fmt(guild_id, f)
            }
            Route::DeleteGuildIntegration {
                guild_id,
                integration_id,
            } => {
                f.write_str("guilds/")?;
                Display::fmt(guild_id, f)?;
                f.write_str("/integrations/")?;

                Display::fmt(integration_id, f)
            }
            Route::DeleteInvite { code } => {
                f.write_str("invites/")?;

                f.write_str(code)
            }
            Route::DeleteMessageReactions {
                channel_id,
                message_id,
            } => {
                f.write_str("channels/")?;
                Display::fmt(channel_id, f)?;
                f.write_str("/messages/")?;
                Display::fmt(message_id, f)?;

                f.write_str("/reactions")
            }
            Route::DeleteMessageSpecificReaction {
                channel_id,
                message_id,
                emoji,
            } => {
                f.write_str("channels/")?;
                Display::fmt(channel_id, f)?;
                f.write_str("/messages/")?;
                Display::fmt(message_id, f)?;
                f.write_str("/reactions/")?;

                Display::fmt(&emoji, f)
            }
            Route::DeleteMessage {
                channel_id,
                message_id,
            }
            | Route::GetMessage {
                channel_id,
                message_id,
            }
            | Route::UpdateMessage {
                channel_id,
                message_id,
            } => {
                f.write_str("channels/")?;
                Display::fmt(channel_id, f)?;
                f.write_str("/messages/")?;

                Display::fmt(message_id, f)
            }
            Route::DeleteMessages { channel_id } => {
                f.write_str("channels/")?;
                Display::fmt(channel_id, f)?;

                f.write_str("/messages/bulk-delete")
            }
            Route::DeletePermissionOverwrite {
                channel_id,
                target_id,
            }
            | Route::UpdatePermissionOverwrite {
                channel_id,
                target_id,
            } => {
                f.write_str("channels/")?;
                Display::fmt(channel_id, f)?;
                f.write_str("/permissions/")?;

                Display::fmt(target_id, f)
            }
            Route::DeleteReaction {
                channel_id,
                emoji,
                message_id,
                user_id,
            } => {
                f.write_str("channels/")?;
                Display::fmt(channel_id, f)?;
                f.write_str("/messages/")?;
                Display::fmt(message_id, f)?;
                f.write_str("/reactions/")?;
                Display::fmt(&emoji, f)?;
                f.write_str("/")?;

                Display::fmt(user_id, f)
            }
            Route::DeleteRole { guild_id, role_id }
            | Route::GetRole { guild_id, role_id }
            | Route::UpdateRole { guild_id, role_id } => {
                f.write_str("guilds/")?;
                Display::fmt(guild_id, f)?;
                f.write_str("/roles/")?;

                Display::fmt(role_id, f)
            }
            Route::DeleteStageInstance { channel_id }
            | Route::GetStageInstance { channel_id }
            | Route::UpdateStageInstance { channel_id } => {
                f.write_str("stage-instances/")?;

                Display::fmt(channel_id, f)
            }
            Route::DeleteWebhookMessage {
                message_id,
                thread_id,
                token,
                webhook_id,
            }
            | Route::GetWebhookMessage {
                message_id,
                token,
                thread_id,
                webhook_id,
            }
            | Route::UpdateWebhookMessage {
                message_id,
                thread_id,
                token,
                webhook_id,
            } => {
                f.write_str("webhooks/")?;
                Display::fmt(webhook_id, f)?;
                f.write_str("/")?;
                f.write_str(token)?;
                f.write_str("/messages/")?;
                Display::fmt(message_id, f)?;

                let mut query_formatter = QueryStringFormatter::new(f);

                query_formatter.write_opt_param("thread_id", thread_id.as_ref())
            }
            Route::DeleteWebhook { token, webhook_id }
            | Route::GetWebhook { token, webhook_id }
            | Route::UpdateWebhook { token, webhook_id } => {
                f.write_str("webhooks/")?;
                Display::fmt(webhook_id, f)?;

                if let Some(token) = token {
                    f.write_str("/")?;
                    f.write_str(token)?;
                }

                Ok(())
            }
            Route::ExecuteWebhook {
                thread_id,
                token,
                wait,
                with_components,
                webhook_id,
            } => {
                f.write_str("webhooks/")?;
                Display::fmt(webhook_id, f)?;
                f.write_str("/")?;
                f.write_str(token)?;

                let mut query_formatter = QueryStringFormatter::new(f);

                query_formatter.write_opt_param("thread_id", thread_id.as_ref())?;
                query_formatter.write_opt_param("wait", wait.as_ref())?;
                query_formatter.write_opt_param("with_components", with_components.as_ref())
            }
            Route::FollowNewsChannel { channel_id } => {
                f.write_str("channels/")?;
                Display::fmt(channel_id, f)?;

                f.write_str("/followers")
            }
            Route::GetActiveThreads { guild_id } => {
                f.write_str("guilds/")?;
                Display::fmt(guild_id, f)?;

                f.write_str("/threads/active")
            }
            Route::GetAuditLogs {
                action_type,
                after,
                before,
                guild_id,
                limit,
                user_id,
            } => {
                f.write_str("guilds/")?;
                Display::fmt(guild_id, f)?;
                f.write_str("/audit-logs")?;

                let mut query_formatter = QueryStringFormatter::new(f);

                query_formatter.write_opt_param("action_type", action_type.as_ref())?;
                query_formatter.write_opt_param("after", after.as_ref())?;
                query_formatter.write_opt_param("before", before.as_ref())?;
                query_formatter.write_opt_param("limit", limit.as_ref())?;
                query_formatter.write_opt_param("user_id", user_id.as_ref())
            }
            Route::GetBansWithParameters {
                after,
                before,
                guild_id,
                limit,
            } => {
                f.write_str("guilds/")?;
                Display::fmt(guild_id, f)?;
                f.write_str("/bans")?;

                let mut query_formatter = QueryStringFormatter::new(f);

                query_formatter.write_opt_param("after", after.as_ref())?;
                query_formatter.write_opt_param("before", before.as_ref())?;
                query_formatter.write_opt_param("limit", limit.as_ref())
            }
            Route::GetGuild {
                guild_id,
                with_counts,
            } => {
                f.write_str("guilds/")?;
                Display::fmt(guild_id, f)?;

                let mut query_formatter = QueryStringFormatter::new(f);

                if *with_counts {
                    query_formatter.write_param("with_counts", &true)?;
                }

                Ok(())
            }
            Route::GetGuildInvites { guild_id } => {
                f.write_str("guilds/")?;
                Display::fmt(guild_id, f)?;

                f.write_str("/invites")
            }
            Route::GetGuildMembers {
                after,
                guild_id,
                limit,
            } => {
                f.write_str("guilds/")?;
                Display::fmt(guild_id, f)?;
                f.write_str("/members")?;

                let mut query_formatter = QueryStringFormatter::new(f);

                query_formatter.write_opt_param("after", after.as_ref())?;
                query_formatter.write_opt_param("limit", limit.as_ref())
            }
            Route::GetGuildOnboarding { guild_id } | Route::UpdateGuildOnboarding { guild_id } => {
                f.write_str("guilds/")?;
                Display::fmt(guild_id, f)?;

                f.write_str("/onboarding")
            }
            Route::GetGuildPreview { guild_id } => {
                f.write_str("guilds/")?;
                Display::fmt(guild_id, f)?;

                f.write_str("/preview")
            }
            Route::GetGuildPruneCount {
                days,
                guild_id,
                include_roles,
            } => {
                f.write_str("guilds/")?;
                Display::fmt(guild_id, f)?;
                f.write_str("/prune")?;

                let mut query_formatter = QueryStringFormatter::new(f);

                query_formatter.write_opt_param("days", days.as_ref())?;

                if !include_roles.is_empty() {
                    query_formatter.write_param("include_roles", &QueryArray(*include_roles))?;
                }

                Ok(())
            }
            Route::GetGuildSticker {
                guild_id,
                sticker_id,
                ..
            }
            | Route::DeleteGuildSticker {
                guild_id,
                sticker_id,
                ..
            }
            | Route::UpdateGuildSticker {
                guild_id,
                sticker_id,
                ..
            } => {
                f.write_str("guilds/")?;
                Display::fmt(guild_id, f)?;
                f.write_str("/stickers/")?;

                Display::fmt(sticker_id, f)
            }
            Route::GetGuildVanityUrl { guild_id } => {
                f.write_str("guilds/")?;
                Display::fmt(guild_id, f)?;

                f.write_str("/vanity-url")
            }
            Route::GetGuildVoiceRegions { guild_id } => {
                f.write_str("guilds/")?;
                Display::fmt(guild_id, f)?;

                f.write_str("/regions")
            }
            Route::GetGuildWelcomeScreen { guild_id }
            | Route::UpdateGuildWelcomeScreen { guild_id } => {
                f.write_str("guilds/")?;
                Display::fmt(guild_id, f)?;

                f.write_str("/welcome-screen")
            }
            Route::GetGuildWebhooks { guild_id } => {
                f.write_str("guilds/")?;
                Display::fmt(guild_id, f)?;

                f.write_str("/webhooks")
            }
            Route::GetGuildWidget { guild_id } => {
                f.write_str("guilds/")?;
                Display::fmt(guild_id, f)?;

                f.write_str("/widget.json")
            }
            Route::GetGuildWidgetSettings { guild_id }
            | Route::UpdateGuildWidgetSettings { guild_id } => {
                f.write_str("guilds/")?;
                Display::fmt(guild_id, f)?;

                f.write_str("/widget")
            }
            Route::GetInviteWithExpiration {
                code,
                with_counts,
                with_expiration,
            } => {
                f.write_str("invites/")?;
                f.write_str(code)?;

                let mut query_formatter = QueryStringFormatter::new(f);

                if *with_counts {
                    query_formatter.write_param("with_counts", &true)?;
                }

                if *with_expiration {
                    query_formatter.write_param("with_expiration", &true)?;
                }

                Ok(())
            }
            Route::GetMessages {
                channel_id,
                after,
                around,
                before,
                limit,
            } => {
                f.write_str("channels/")?;
                Display::fmt(channel_id, f)?;
                f.write_str("/messages")?;

                let mut query_formatter = QueryStringFormatter::new(f);

                query_formatter.write_opt_param("after", after.as_ref())?;
                query_formatter.write_opt_param("around", around.as_ref())?;
                query_formatter.write_opt_param("before", before.as_ref())?;
                query_formatter.write_opt_param("limit", limit.as_ref())
            }
            Route::GetPins {
                channel_id,
                limit,
                before,
            } => {
                f.write_str("channels/")?;
                Display::fmt(channel_id, f)?;

                f.write_str("/messages/pins")?;

                let mut query_formatter = QueryStringFormatter::new(f);

                query_formatter.write_opt_param("before", before.as_ref())?;
                query_formatter.write_opt_param("limit", limit.as_ref())
            }
            Route::GetJoinedPrivateArchivedThreads {
                before,
                channel_id,
                limit,
            } => {
                f.write_str("channels/")?;
                Display::fmt(channel_id, f)?;
                f.write_str("/users/@me/threads/archived/private")?;

                let mut query_formatter = QueryStringFormatter::new(f);

                query_formatter.write_opt_param("before", before.as_ref())?;
                query_formatter.write_opt_param("limit", limit.as_ref())
            }
            Route::GetPrivateArchivedThreads {
                before,
                channel_id,
                limit,
            } => {
                f.write_str("channels/")?;
                Display::fmt(channel_id, f)?;
                f.write_str("/threads/archived/private")?;

                let mut query_formatter = QueryStringFormatter::new(f);

                query_formatter.write_opt_param("before", before.as_ref())?;
                query_formatter.write_opt_param("limit", limit.as_ref())
            }
            Route::GetPublicArchivedThreads {
                before,
                channel_id,
                limit,
            } => {
                f.write_str("channels/")?;
                Display::fmt(channel_id, f)?;
                f.write_str("/threads/archived/public")?;

                let mut query_formatter = QueryStringFormatter::new(f);

                query_formatter.write_opt_param("before", before.as_ref())?;
                query_formatter.write_opt_param("limit", limit.as_ref())
            }
            Route::GetReactionUsers {
                after,
                channel_id,
                emoji,
                limit,
                message_id,
                kind,
            } => {
                f.write_str("channels/")?;
                Display::fmt(channel_id, f)?;
                f.write_str("/messages/")?;
                Display::fmt(message_id, f)?;
                f.write_str("/reactions/")?;
                Display::fmt(&emoji, f)?;

                let mut query_formatter = QueryStringFormatter::new(f);

                query_formatter.write_opt_param("after", after.as_ref())?;
                query_formatter.write_opt_param("limit", limit.as_ref())?;
                query_formatter.write_opt_param("type", kind.as_ref())
            }
            Route::GetThreadMembers {
                after,
                channel_id,
                limit,
                with_member,
            } => {
                f.write_str("channels/")?;
                Display::fmt(channel_id, f)?;
                f.write_str("/thread-members")?;

                let mut query_formatter = QueryStringFormatter::new(f);

                query_formatter.write_opt_param("after", after.as_ref())?;
                query_formatter.write_opt_param("limit", limit.as_ref())?;
                query_formatter.write_opt_param("with_member", with_member.as_ref())
            }
            Route::JoinThread { channel_id } | Route::LeaveThread { channel_id } => {
                f.write_str("channels/")?;
                Display::fmt(channel_id, f)?;

                f.write_str("/thread-members/@me")
            }
            Route::PinMessage {
                channel_id,
                message_id,
            }
            | Route::UnpinMessage {
                channel_id,
                message_id,
            } => {
                f.write_str("channels/")?;
                Display::fmt(channel_id, f)?;
                f.write_str("/messages/pins/")?;

                Display::fmt(message_id, f)
            }
            Route::SearchGuildMembers {
                guild_id,
                limit,
                query,
            } => {
                f.write_str("guilds/")?;
                Display::fmt(guild_id, f)?;
                f.write_str("/members/search")?;

                let mut query_formatter = QueryStringFormatter::new(f);

                query_formatter
                    .write_param("query", &utf8_percent_encode(query, NON_ALPHANUMERIC))?;
                query_formatter.write_opt_param("limit", limit.as_ref())
            }
            Route::UpdateCurrentMember { guild_id } => {
                f.write_str("guilds/")?;
                Display::fmt(guild_id, f)?;

                f.write_str("/members/@me")
            }
            Route::GetCurrentUserVoiceState { guild_id }
            | Route::UpdateCurrentUserVoiceState { guild_id } => {
                f.write_str("guilds/")?;
                Display::fmt(guild_id, f)?;

                f.write_str("/voice-states/@me")
            }
            Route::GetUserVoiceState { guild_id, user_id }
            | Route::UpdateUserVoiceState { guild_id, user_id } => {
                f.write_str("guilds/")?;
                Display::fmt(guild_id, f)?;
                f.write_str("/voice-states/")?;

                Display::fmt(user_id, f)
            }
            Route::UpdateGuildMfa { guild_id, .. } => {
                f.write_str("guilds/")?;
                Display::fmt(guild_id, f)?;

                f.write_str("/mfa")
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Route;
    use crate::request::{Method, channel::reaction::RequestReactionType};
    use twilight_model::id::Id;

    /// Test a route for each method.
    #[test]
    fn methods() {
        assert_eq!(
            Method::Delete,
            Route::DeleteInvite {
                code: "twilight-rs",
            }
            .method()
        );
        assert_eq!(
            Method::Patch,
            Route::UpdateMessage {
                channel_id: 123,
                message_id: 456,
            }
            .method()
        );
    }

    // Test display implementation

    const CHANNEL_ID: u64 = 2;
    const CODE: &str = "invitecode";
    const EMOJI_ID: u64 = 4;
    const GUILD_ID: u64 = 5;
    const INTEGRATION_ID: u64 = 7;
    const MESSAGE_ID: u64 = 8;
    const ROLE_ID: u64 = 9;
    const STICKER_ID: u64 = 10;
    const USER_ID: u64 = 11;
    const AUTO_MODERATION_RULE_ID: u64 = 13;

    const fn emoji() -> RequestReactionType<'static> {
        RequestReactionType::Custom {
            id: Id::new(EMOJI_ID),
            name: None,
        }
    }

    #[test]
    fn get_public_archived_threads() {
        let route = Route::GetPublicArchivedThreads {
            channel_id: 1,
            before: Some("2021-01-01T00:00:00Z"),
            limit: None,
        };

        assert_eq!(
            "channels/1/threads/archived/public?before=2021-01-01T00:00:00Z",
            route.to_string()
        );
    }

    #[test]
    fn update_webhook_message_thread_id() {
        let route = Route::UpdateWebhookMessage {
            message_id: 1,
            thread_id: Some(2),
            token: "token",
            webhook_id: 3,
        };

        assert_eq!("webhooks/3/token/messages/1?thread_id=2", route.to_string());
    }

    #[test]
    fn add_guild_member() {
        let route = Route::AddGuildMember {
            guild_id: GUILD_ID,
            user_id: USER_ID,
        };
        assert_eq!(
            route.to_string(),
            format!("guilds/{GUILD_ID}/members/{USER_ID}")
        );
    }

    #[test]
    fn get_member() {
        let route = Route::GetMember {
            guild_id: GUILD_ID,
            user_id: USER_ID,
        };
        assert_eq!(
            route.to_string(),
            format!("guilds/{GUILD_ID}/members/{USER_ID}")
        );
    }

    #[test]
    fn remove_member() {
        let route = Route::RemoveMember {
            guild_id: GUILD_ID,
            user_id: USER_ID,
        };
        assert_eq!(
            route.to_string(),
            format!("guilds/{GUILD_ID}/members/{USER_ID}")
        );
    }

    #[test]
    fn update_member() {
        let route = Route::UpdateMember {
            guild_id: GUILD_ID,
            user_id: USER_ID,
        };
        assert_eq!(
            route.to_string(),
            format!("guilds/{GUILD_ID}/members/{USER_ID}")
        );
    }

    #[test]
    fn add_member_role() {
        let route = Route::AddMemberRole {
            guild_id: GUILD_ID,
            role_id: ROLE_ID,
            user_id: USER_ID,
        };
        assert_eq!(
            route.to_string(),
            format!("guilds/{GUILD_ID}/members/{USER_ID}/roles/{ROLE_ID}")
        );
    }

    #[test]
    fn remove_member_role() {
        let route = Route::RemoveMemberRole {
            guild_id: GUILD_ID,
            role_id: ROLE_ID,
            user_id: USER_ID,
        };
        assert_eq!(
            route.to_string(),
            format!("guilds/{GUILD_ID}/members/{USER_ID}/roles/{ROLE_ID}")
        );
    }

    #[test]
    fn add_thread_member() {
        let route = Route::AddThreadMember {
            channel_id: CHANNEL_ID,
            user_id: USER_ID,
        };
        assert_eq!(
            route.to_string(),
            format!("channels/{CHANNEL_ID}/thread-members/{USER_ID}")
        );
    }

    #[test]
    fn get_thread_member() {
        let route = Route::GetThreadMember {
            channel_id: CHANNEL_ID,
            user_id: USER_ID,
        };
        assert_eq!(
            route.to_string(),
            format!("channels/{CHANNEL_ID}/thread-members/{USER_ID}")
        );
    }

    #[test]
    fn remove_thread_member() {
        let route = Route::RemoveThreadMember {
            channel_id: CHANNEL_ID,
            user_id: USER_ID,
        };
        assert_eq!(
            route.to_string(),
            format!("channels/{CHANNEL_ID}/thread-members/{USER_ID}")
        );
    }

    #[test]
    fn create_channel() {
        let route = Route::CreateChannel { guild_id: GUILD_ID };
        assert_eq!(route.to_string(), format!("guilds/{GUILD_ID}/channels"));
    }

    #[test]
    fn get_channels() {
        let route = Route::GetChannels { guild_id: GUILD_ID };
        assert_eq!(route.to_string(), format!("guilds/{GUILD_ID}/channels"));
    }

    #[test]
    fn update_guild_channels() {
        let route = Route::UpdateGuildChannels { guild_id: GUILD_ID };
        assert_eq!(route.to_string(), format!("guilds/{GUILD_ID}/channels"));
    }

    #[test]
    fn create_emoji() {
        let route = Route::CreateEmoji { guild_id: GUILD_ID };
        assert_eq!(route.to_string(), format!("guilds/{GUILD_ID}/emojis"));
    }

    #[test]
    fn get_emojis() {
        let route = Route::GetEmojis { guild_id: GUILD_ID };
        assert_eq!(route.to_string(), format!("guilds/{GUILD_ID}/emojis"));
    }

    #[test]
    fn get_guild_integrations() {
        let route = Route::GetGuildIntegrations { guild_id: GUILD_ID };
        assert_eq!(route.to_string(), format!("guilds/{GUILD_ID}/integrations"));
    }

    #[test]
    fn create_guild_sticker() {
        let route = Route::CreateGuildSticker { guild_id: GUILD_ID };
        assert_eq!(route.to_string(), format!("guilds/{GUILD_ID}/stickers"));
    }

    #[test]
    fn get_guild_stickers() {
        let route = Route::GetGuildStickers { guild_id: GUILD_ID };
        assert_eq!(route.to_string(), format!("guilds/{GUILD_ID}/stickers"));
    }

    #[test]
    fn create_invite() {
        let route = Route::CreateInvite {
            channel_id: CHANNEL_ID,
        };
        assert_eq!(route.to_string(), format!("channels/{CHANNEL_ID}/invites"));
    }

    #[test]
    fn get_channel_invites() {
        let route = Route::GetChannelInvites {
            channel_id: CHANNEL_ID,
        };
        assert_eq!(route.to_string(), format!("channels/{CHANNEL_ID}/invites"));
    }

    #[test]
    fn create_message() {
        let route = Route::CreateMessage {
            channel_id: CHANNEL_ID,
        };
        assert_eq!(route.to_string(), format!("channels/{CHANNEL_ID}/messages"));
    }

    #[test]
    fn create_reaction() {
        let emoji = emoji();

        let route = Route::CreateReaction {
            channel_id: CHANNEL_ID,
            emoji: &emoji,
            message_id: MESSAGE_ID,
        };
        assert_eq!(
            route.to_string(),
            format!("channels/{CHANNEL_ID}/messages/{MESSAGE_ID}/reactions/{emoji}/@me")
        );
    }

    #[test]
    fn delete_reaction_current_user() {
        let emoji = emoji();

        let route = Route::DeleteReactionCurrentUser {
            channel_id: CHANNEL_ID,
            emoji: &emoji,
            message_id: MESSAGE_ID,
        };
        assert_eq!(
            route.to_string(),
            format!("channels/{CHANNEL_ID}/messages/{MESSAGE_ID}/reactions/{emoji}/@me")
        );
    }

    #[test]
    fn create_role() {
        let route = Route::CreateRole { guild_id: GUILD_ID };
        assert_eq!(route.to_string(), format!("guilds/{GUILD_ID}/roles"));
    }

    #[test]
    fn get_guild_roles() {
        let route = Route::GetGuildRoles { guild_id: GUILD_ID };
        assert_eq!(route.to_string(), format!("guilds/{GUILD_ID}/roles"));
    }

    #[test]
    fn update_role_positions() {
        let route = Route::UpdateRolePositions { guild_id: GUILD_ID };
        assert_eq!(route.to_string(), format!("guilds/{GUILD_ID}/roles"));
    }

    #[test]
    fn create_stage_instance() {
        let route = Route::CreateStageInstance;
        assert_eq!(route.to_string(), "stage-instances");
    }

    #[test]
    fn create_thread() {
        let route = Route::CreateThread {
            channel_id: CHANNEL_ID,
        };
        assert_eq!(route.to_string(), format!("channels/{CHANNEL_ID}/threads"));
    }

    #[test]
    fn create_thread_from_message() {
        let route = Route::CreateThreadFromMessage {
            channel_id: CHANNEL_ID,
            message_id: MESSAGE_ID,
        };
        assert_eq!(
            route.to_string(),
            format!("channels/{CHANNEL_ID}/messages/{MESSAGE_ID}/threads")
        );
    }

    #[test]
    fn create_typing_trigger() {
        let route = Route::CreateTypingTrigger {
            channel_id: CHANNEL_ID,
        };
        assert_eq!(route.to_string(), format!("channels/{CHANNEL_ID}/typing"));
    }

    #[test]
    fn create_webhook() {
        let route = Route::CreateWebhook {
            channel_id: CHANNEL_ID,
        };
        assert_eq!(route.to_string(), format!("channels/{CHANNEL_ID}/webhooks"));
    }

    #[test]
    fn get_channel_webhooks() {
        let route = Route::GetChannelWebhooks {
            channel_id: CHANNEL_ID,
        };
        assert_eq!(route.to_string(), format!("channels/{CHANNEL_ID}/webhooks"));
    }

    #[test]
    fn crosspost_message() {
        let route = Route::CrosspostMessage {
            channel_id: CHANNEL_ID,
            message_id: MESSAGE_ID,
        };
        assert_eq!(
            route.to_string(),
            format!("channels/{CHANNEL_ID}/messages/{MESSAGE_ID}/crosspost")
        );
    }

    #[test]
    fn delete_ban() {
        let route = Route::DeleteBan {
            guild_id: GUILD_ID,
            user_id: USER_ID,
        };
        assert_eq!(
            route.to_string(),
            format!("guilds/{GUILD_ID}/bans/{USER_ID}")
        );
    }

    #[test]
    fn get_ban() {
        let route = Route::GetBan {
            guild_id: GUILD_ID,
            user_id: USER_ID,
        };
        assert_eq!(
            route.to_string(),
            format!("guilds/{GUILD_ID}/bans/{USER_ID}")
        );
    }

    #[test]
    fn delete_channel() {
        let route = Route::DeleteChannel {
            channel_id: CHANNEL_ID,
        };
        assert_eq!(route.to_string(), format!("channels/{CHANNEL_ID}"));
    }

    #[test]
    fn get_channel() {
        let route = Route::GetChannel {
            channel_id: CHANNEL_ID,
        };
        assert_eq!(route.to_string(), format!("channels/{CHANNEL_ID}"));
    }

    #[test]
    fn update_channel() {
        let route = Route::UpdateChannel {
            channel_id: CHANNEL_ID,
        };
        assert_eq!(route.to_string(), format!("channels/{CHANNEL_ID}"));
    }

    #[test]
    fn delete_emoji() {
        let route = Route::DeleteEmoji {
            emoji_id: EMOJI_ID,
            guild_id: GUILD_ID,
        };
        assert_eq!(
            route.to_string(),
            format!("guilds/{GUILD_ID}/emojis/{EMOJI_ID}")
        );
    }

    #[test]
    fn get_emoji() {
        let route = Route::GetEmoji {
            emoji_id: EMOJI_ID,
            guild_id: GUILD_ID,
        };
        assert_eq!(
            route.to_string(),
            format!("guilds/{GUILD_ID}/emojis/{EMOJI_ID}")
        );
    }

    #[test]
    fn update_emoji() {
        let route = Route::UpdateEmoji {
            emoji_id: EMOJI_ID,
            guild_id: GUILD_ID,
        };
        assert_eq!(
            route.to_string(),
            format!("guilds/{GUILD_ID}/emojis/{EMOJI_ID}")
        );
    }

    #[test]
    fn delete_guild() {
        let route = Route::DeleteGuild { guild_id: GUILD_ID };
        assert_eq!(route.to_string(), format!("guilds/{GUILD_ID}"));
    }

    #[test]
    fn update_guild() {
        let route = Route::UpdateGuild { guild_id: GUILD_ID };
        assert_eq!(route.to_string(), format!("guilds/{GUILD_ID}"));
    }

    #[test]
    fn delete_guild_integration() {
        let route = Route::DeleteGuildIntegration {
            guild_id: GUILD_ID,
            integration_id: INTEGRATION_ID,
        };
        assert_eq!(
            route.to_string(),
            format!("guilds/{GUILD_ID}/integrations/{INTEGRATION_ID}")
        );
    }

    #[test]
    fn delete_invite() {
        let route = Route::DeleteInvite { code: CODE };
        assert_eq!(route.to_string(), format!("invites/{CODE}"));
    }

    #[test]
    fn delete_message_reactions() {
        let route = Route::DeleteMessageReactions {
            channel_id: CHANNEL_ID,
            message_id: MESSAGE_ID,
        };
        assert_eq!(
            route.to_string(),
            format!("channels/{CHANNEL_ID}/messages/{MESSAGE_ID}/reactions")
        );
    }

    #[test]
    fn delete_message_specific_reaction() {
        let emoji = emoji();

        let route = Route::DeleteMessageSpecificReaction {
            channel_id: CHANNEL_ID,
            message_id: MESSAGE_ID,
            emoji: &emoji,
        };
        assert_eq!(
            route.to_string(),
            format!("channels/{CHANNEL_ID}/messages/{MESSAGE_ID}/reactions/{emoji}")
        );
    }

    #[test]
    fn delete_message() {
        let route = Route::DeleteMessage {
            channel_id: CHANNEL_ID,
            message_id: MESSAGE_ID,
        };
        assert_eq!(
            route.to_string(),
            format!("channels/{CHANNEL_ID}/messages/{MESSAGE_ID}")
        );
    }

    #[test]
    fn get_message() {
        let route = Route::GetMessage {
            channel_id: CHANNEL_ID,
            message_id: MESSAGE_ID,
        };
        assert_eq!(
            route.to_string(),
            format!("channels/{CHANNEL_ID}/messages/{MESSAGE_ID}")
        );
    }

    #[test]
    fn update_message() {
        let route = Route::UpdateMessage {
            channel_id: CHANNEL_ID,
            message_id: MESSAGE_ID,
        };
        assert_eq!(
            route.to_string(),
            format!("channels/{CHANNEL_ID}/messages/{MESSAGE_ID}")
        );
    }

    #[test]
    fn delete_messages() {
        let route = Route::DeleteMessages {
            channel_id: CHANNEL_ID,
        };
        assert_eq!(
            route.to_string(),
            format!("channels/{CHANNEL_ID}/messages/bulk-delete")
        );
    }

    #[test]
    fn delete_permission_overwrite() {
        let route = Route::DeletePermissionOverwrite {
            channel_id: CHANNEL_ID,
            target_id: ROLE_ID,
        };
        assert_eq!(
            route.to_string(),
            format!("channels/{CHANNEL_ID}/permissions/{ROLE_ID}")
        );
    }

    #[test]
    fn update_permission_overwrite() {
        let route = Route::UpdatePermissionOverwrite {
            channel_id: CHANNEL_ID,
            target_id: USER_ID,
        };
        assert_eq!(
            route.to_string(),
            format!("channels/{CHANNEL_ID}/permissions/{USER_ID}")
        );
    }

    #[test]
    fn delete_reaction() {
        let emoji = emoji();

        let route = Route::DeleteReaction {
            channel_id: CHANNEL_ID,
            emoji: &emoji,
            message_id: MESSAGE_ID,
            user_id: USER_ID,
        };
        assert_eq!(
            route.to_string(),
            format!("channels/{CHANNEL_ID}/messages/{MESSAGE_ID}/reactions/{emoji}/{USER_ID}")
        );
    }

    #[test]
    fn delete_role() {
        let route = Route::DeleteRole {
            guild_id: GUILD_ID,
            role_id: ROLE_ID,
        };
        assert_eq!(
            route.to_string(),
            format!("guilds/{GUILD_ID}/roles/{ROLE_ID}")
        );
    }

    #[test]
    fn update_role() {
        let route = Route::UpdateRole {
            guild_id: GUILD_ID,
            role_id: ROLE_ID,
        };
        assert_eq!(
            route.to_string(),
            format!("guilds/{GUILD_ID}/roles/{ROLE_ID}")
        );
    }

    #[test]
    fn delete_stage_instance() {
        let route = Route::DeleteStageInstance {
            channel_id: CHANNEL_ID,
        };
        assert_eq!(route.to_string(), format!("stage-instances/{CHANNEL_ID}"));
    }

    #[test]
    fn get_stage_instance() {
        let route = Route::GetStageInstance {
            channel_id: CHANNEL_ID,
        };
        assert_eq!(route.to_string(), format!("stage-instances/{CHANNEL_ID}"));
    }

    #[test]
    fn update_stage_instance() {
        let route = Route::UpdateStageInstance {
            channel_id: CHANNEL_ID,
        };
        assert_eq!(route.to_string(), format!("stage-instances/{CHANNEL_ID}"));
    }

    #[test]
    fn follow_news_channel() {
        let route = Route::FollowNewsChannel {
            channel_id: CHANNEL_ID,
        };
        assert_eq!(
            route.to_string(),
            format!("channels/{CHANNEL_ID}/followers")
        );
    }

    #[test]
    fn get_active_threads() {
        let route = Route::GetActiveThreads { guild_id: GUILD_ID };
        assert_eq!(
            route.to_string(),
            format!("guilds/{GUILD_ID}/threads/active")
        );
    }

    #[test]
    fn get_bans_with_parameters() {
        let route = Route::GetBansWithParameters {
            after: None,
            before: None,
            guild_id: GUILD_ID,
            limit: None,
        };
        assert_eq!(route.to_string(), format!("guilds/{GUILD_ID}/bans"));

        let route = Route::GetBansWithParameters {
            after: Some(USER_ID),
            before: None,
            guild_id: GUILD_ID,
            limit: None,
        };
        assert_eq!(
            route.to_string(),
            format!("guilds/{GUILD_ID}/bans?after={USER_ID}")
        );

        let route = Route::GetBansWithParameters {
            after: None,
            before: Some(USER_ID),
            guild_id: GUILD_ID,
            limit: None,
        };
        assert_eq!(
            route.to_string(),
            format!("guilds/{GUILD_ID}/bans?before={USER_ID}")
        );

        let route = Route::GetBansWithParameters {
            after: None,
            before: None,
            guild_id: GUILD_ID,
            limit: Some(100),
        };
        assert_eq!(
            route.to_string(),
            format!("guilds/{GUILD_ID}/bans?limit={limit}", limit = 100)
        );

        let route = Route::GetBansWithParameters {
            after: Some(USER_ID),
            before: Some(USER_ID + 100),
            guild_id: GUILD_ID,
            limit: Some(25),
        };
        assert_eq!(
            route.to_string(),
            format!(
                "guilds/{GUILD_ID}/bans?after={USER_ID}&before={before}&limit={limit}",
                before = USER_ID + 100,
                limit = 25,
            )
        );
    }

    #[test]
    fn get_guild_invites() {
        let route = Route::GetGuildInvites { guild_id: GUILD_ID };
        assert_eq!(route.to_string(), format!("guilds/{GUILD_ID}/invites"));
    }

    #[test]
    fn get_guild_preview() {
        let route = Route::GetGuildPreview { guild_id: GUILD_ID };
        assert_eq!(route.to_string(), format!("guilds/{GUILD_ID}/preview"));
    }

    #[test]
    fn get_guild_sticker() {
        let route = Route::GetGuildSticker {
            guild_id: GUILD_ID,
            sticker_id: STICKER_ID,
        };
        assert_eq!(
            route.to_string(),
            format!("guilds/{GUILD_ID}/stickers/{STICKER_ID}")
        );
    }

    #[test]
    fn delete_guild_sticker() {
        let route = Route::DeleteGuildSticker {
            guild_id: GUILD_ID,
            sticker_id: STICKER_ID,
        };
        assert_eq!(
            route.to_string(),
            format!("guilds/{GUILD_ID}/stickers/{STICKER_ID}")
        );
    }

    #[test]
    fn update_guild_sticker() {
        let route = Route::UpdateGuildSticker {
            guild_id: GUILD_ID,
            sticker_id: STICKER_ID,
        };
        assert_eq!(
            route.to_string(),
            format!("guilds/{GUILD_ID}/stickers/{STICKER_ID}")
        );
    }

    #[test]
    fn get_guild_vanity_url() {
        let route = Route::GetGuildVanityUrl { guild_id: GUILD_ID };
        assert_eq!(route.to_string(), format!("guilds/{GUILD_ID}/vanity-url"));
    }

    #[test]
    fn get_guild_voice_regions() {
        let route = Route::GetGuildVoiceRegions { guild_id: GUILD_ID };
        assert_eq!(route.to_string(), format!("guilds/{GUILD_ID}/regions"));
    }

    #[test]
    fn get_guild_welcome_screen() {
        let route = Route::GetGuildWelcomeScreen { guild_id: GUILD_ID };
        assert_eq!(
            route.to_string(),
            format!("guilds/{GUILD_ID}/welcome-screen")
        );
    }

    #[test]
    fn update_guild_welcome_screen() {
        let route = Route::UpdateGuildWelcomeScreen { guild_id: GUILD_ID };
        assert_eq!(
            route.to_string(),
            format!("guilds/{GUILD_ID}/welcome-screen")
        );
    }

    #[test]
    fn get_guild_webhooks() {
        let route = Route::GetGuildWebhooks { guild_id: GUILD_ID };
        assert_eq!(route.to_string(), format!("guilds/{GUILD_ID}/webhooks"));
    }

    #[test]
    fn get_guild_widget() {
        let route = Route::GetGuildWidget { guild_id: GUILD_ID };
        assert_eq!(route.to_string(), format!("guilds/{GUILD_ID}/widget.json"));
    }

    #[test]
    fn get_guild_widget_settings() {
        let route = Route::GetGuildWidgetSettings { guild_id: GUILD_ID };
        assert_eq!(route.to_string(), format!("guilds/{GUILD_ID}/widget"));
    }

    #[test]
    fn update_guild_widget_settings() {
        let route = Route::UpdateGuildWidgetSettings { guild_id: GUILD_ID };
        assert_eq!(route.to_string(), format!("guilds/{GUILD_ID}/widget"));
    }

    #[test]
    fn get_pins() {
        let route = Route::GetPins {
            channel_id: CHANNEL_ID,
            limit: None,
            before: None,
        };
        assert_eq!(
            route.to_string(),
            format!("channels/{CHANNEL_ID}/messages/pins")
        );
    }

    #[test]
    fn get_thread_members() {
        let route = Route::GetThreadMembers {
            after: None,
            channel_id: CHANNEL_ID,
            limit: None,
            with_member: None,
        };
        assert_eq!(
            route.to_string(),
            format!("channels/{CHANNEL_ID}/thread-members")
        );

        let route = Route::GetThreadMembers {
            after: Some(USER_ID),
            channel_id: CHANNEL_ID,
            limit: Some(1),
            with_member: Some(true),
        };

        assert_eq!(
            route.to_string(),
            format!(
                "channels/{CHANNEL_ID}/thread-members?after={USER_ID}&limit=1&with_member=true"
            )
        );
    }

    #[test]
    fn join_thread() {
        let route = Route::JoinThread {
            channel_id: CHANNEL_ID,
        };
        assert_eq!(
            route.to_string(),
            format!("channels/{CHANNEL_ID}/thread-members/@me")
        );
    }

    #[test]
    fn leave_thread() {
        let route = Route::LeaveThread {
            channel_id: CHANNEL_ID,
        };
        assert_eq!(
            route.to_string(),
            format!("channels/{CHANNEL_ID}/thread-members/@me")
        );
    }

    #[test]
    fn pin_message() {
        let route = Route::PinMessage {
            channel_id: CHANNEL_ID,
            message_id: MESSAGE_ID,
        };
        assert_eq!(
            route.to_string(),
            format!("channels/{CHANNEL_ID}/messages/pins/{MESSAGE_ID}")
        );
    }

    #[test]
    fn unpin_message() {
        let route = Route::UnpinMessage {
            channel_id: CHANNEL_ID,
            message_id: MESSAGE_ID,
        };
        assert_eq!(
            route.to_string(),
            format!("channels/{CHANNEL_ID}/messages/pins/{MESSAGE_ID}")
        );
    }

    #[test]
    fn update_current_member() {
        let route = Route::UpdateCurrentMember { guild_id: GUILD_ID };
        assert_eq!(route.to_string(), format!("guilds/{GUILD_ID}/members/@me"));
    }

    #[test]
    fn get_current_user_voice_state() {
        let route = Route::GetCurrentUserVoiceState { guild_id: GUILD_ID };
        assert_eq!(
            route.to_string(),
            format!("guilds/{GUILD_ID}/voice-states/@me")
        );
    }

    #[test]
    fn update_current_user_voice_state() {
        let route = Route::UpdateCurrentUserVoiceState { guild_id: GUILD_ID };
        assert_eq!(
            route.to_string(),
            format!("guilds/{GUILD_ID}/voice-states/@me")
        );
    }

    #[test]
    fn get_user_voice_state() {
        let route = Route::GetUserVoiceState {
            guild_id: GUILD_ID,
            user_id: USER_ID,
        };
        assert_eq!(
            route.to_string(),
            format!("guilds/{GUILD_ID}/voice-states/{USER_ID}")
        );
    }

    #[test]
    fn update_user_voice_state() {
        let route = Route::UpdateUserVoiceState {
            guild_id: GUILD_ID,
            user_id: USER_ID,
        };
        assert_eq!(
            route.to_string(),
            format!("guilds/{GUILD_ID}/voice-states/{USER_ID}")
        );
    }

    #[test]
    fn create_ban() {
        let mut route = Route::CreateBan {
            guild_id: GUILD_ID,
            user_id: USER_ID,
        };
        assert_eq!(
            route.to_string(),
            format!("guilds/{GUILD_ID}/bans/{USER_ID}")
        );

        route = Route::CreateBan {
            guild_id: GUILD_ID,
            user_id: USER_ID,
        };
        assert_eq!(
            route.to_string(),
            format!("guilds/{GUILD_ID}/bans/{USER_ID}")
        );
    }

    #[test]
    fn create_guild_prune_none() {
        let route = Route::CreateGuildPrune {
            compute_prune_count: None,
            days: None,
            guild_id: GUILD_ID,
            include_roles: &[],
        };
        assert_eq!(route.to_string(), format!("guilds/{GUILD_ID}/prune"));
    }

    #[test]
    fn create_guild_prune_compute_prune_count_true() {
        let route = Route::CreateGuildPrune {
            compute_prune_count: Some(true),
            days: None,
            guild_id: GUILD_ID,
            include_roles: &[],
        };
        assert_eq!(
            route.to_string(),
            format!("guilds/{GUILD_ID}/prune?compute_prune_count=true")
        );
    }

    #[test]
    fn create_guild_prune_compute_prune_count_false() {
        let route = Route::CreateGuildPrune {
            compute_prune_count: Some(false),
            days: None,
            guild_id: GUILD_ID,
            include_roles: &[],
        };
        assert_eq!(
            route.to_string(),
            format!("guilds/{GUILD_ID}/prune?compute_prune_count=false")
        );
    }

    #[test]
    fn create_guild_prune_days() {
        let route = Route::CreateGuildPrune {
            compute_prune_count: None,
            days: Some(4),
            guild_id: GUILD_ID,
            include_roles: &[],
        };
        assert_eq!(route.to_string(), format!("guilds/{GUILD_ID}/prune?days=4"));
    }

    #[test]
    fn create_guild_prune_include_one_role() {
        let include_roles = [Id::new(1)];

        let route = Route::CreateGuildPrune {
            compute_prune_count: None,
            days: None,
            guild_id: GUILD_ID,
            include_roles: &include_roles,
        };
        assert_eq!(
            route.to_string(),
            format!("guilds/{GUILD_ID}/prune?include_roles=1")
        );
    }

    #[test]
    fn create_guild_prune_include_two_roles() {
        let include_roles = [Id::new(1), Id::new(2)];

        let route = Route::CreateGuildPrune {
            compute_prune_count: None,
            days: None,
            guild_id: GUILD_ID,
            include_roles: &include_roles,
        };
        assert_eq!(
            route.to_string(),
            format!("guilds/{GUILD_ID}/prune?include_roles=1,2")
        );
    }

    #[test]
    fn create_guild_prune_all() {
        let include_roles = [Id::new(1), Id::new(2)];

        let route = Route::CreateGuildPrune {
            compute_prune_count: Some(true),
            days: Some(4),
            guild_id: GUILD_ID,
            include_roles: &include_roles,
        };
        assert_eq!(
            route.to_string(),
            format!("guilds/{GUILD_ID}/prune?compute_prune_count=true&days=4&include_roles=1,2")
        );
    }

    #[test]
    fn search_guild_members() {
        let route = Route::SearchGuildMembers {
            guild_id: GUILD_ID,
            limit: Some(99),
            query: "foo bar",
        };

        assert_eq!(
            route.to_string(),
            format!("guilds/{GUILD_ID}/members/search?query=foo%20bar&limit=99")
        );

        let route = Route::SearchGuildMembers {
            guild_id: GUILD_ID,
            limit: Some(99),
            query: "foo/bar",
        };

        assert_eq!(
            route.to_string(),
            format!("guilds/{GUILD_ID}/members/search?query=foo%2Fbar&limit=99")
        );
    }

    #[test]
    fn update_guild_mfa() {
        let route = Route::UpdateGuildMfa { guild_id: GUILD_ID };
        assert_eq!(route.to_string(), format!("guilds/{GUILD_ID}/mfa"));
    }

    #[test]
    fn create_auto_moderation_rule() {
        let route = Route::CreateAutoModerationRule { guild_id: GUILD_ID };
        assert_eq!(
            route.to_string(),
            format!("guilds/{GUILD_ID}/auto-moderation/rules")
        );
    }

    #[test]
    fn delete_auto_moderation_rule() {
        let route = Route::DeleteAutoModerationRule {
            guild_id: GUILD_ID,
            auto_moderation_rule_id: AUTO_MODERATION_RULE_ID,
        };
        assert_eq!(
            route.to_string(),
            format!("guilds/{GUILD_ID}/auto-moderation/rules/{AUTO_MODERATION_RULE_ID}")
        );
    }

    #[test]
    fn get_auto_moderation_rule() {
        let route = Route::GetAutoModerationRule {
            guild_id: GUILD_ID,
            auto_moderation_rule_id: AUTO_MODERATION_RULE_ID,
        };
        assert_eq!(
            route.to_string(),
            format!("guilds/{GUILD_ID}/auto-moderation/rules/{AUTO_MODERATION_RULE_ID}")
        );
    }

    #[test]
    fn get_guild_auto_moderation_rules() {
        let route = Route::GetGuildAutoModerationRules { guild_id: GUILD_ID };
        assert_eq!(
            route.to_string(),
            format!("guilds/{GUILD_ID}/auto-moderation/rules")
        );
    }

    #[test]
    fn update_auto_moderation_rule() {
        let route = Route::UpdateAutoModerationRule {
            guild_id: GUILD_ID,
            auto_moderation_rule_id: AUTO_MODERATION_RULE_ID,
        };
        assert_eq!(
            route.to_string(),
            format!("guilds/{GUILD_ID}/auto-moderation/rules/{AUTO_MODERATION_RULE_ID}")
        );
    }

    #[test]
    fn get_guild_onboarding() {
        let route = Route::GetGuildOnboarding { guild_id: GUILD_ID };
        assert_eq!(route.to_string(), format!("guilds/{GUILD_ID}/onboarding"));
    }
}
