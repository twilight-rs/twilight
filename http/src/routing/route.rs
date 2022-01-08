use super::Path;
use crate::request::{channel::reaction::RequestReactionType, Method};
use percent_encoding::{utf8_percent_encode, NON_ALPHANUMERIC};
use std::fmt::{Display, Formatter, Result as FmtResult};
use twilight_model::id::{marker::RoleMarker, Id};

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
    /// Route information to create a ban on a user in a guild.
    CreateBan {
        /// The number of days' worth of the user's messages to delete in the
        /// guild's channels.
        delete_message_days: Option<u64>,
        /// The ID of the guild.
        guild_id: u64,
        /// The reason for the ban.
        reason: Option<&'a str>,
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
    /// Route information to create a guild from a template.
    CreateGuildFromTemplate {
        /// Code of the template.
        template_code: &'a str,
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
    /// Route information to create a private channel.
    CreatePrivateChannel,
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
    /// Route information to delete a global command.
    DeleteGlobalCommand {
        /// The ID of the owner application.
        application_id: u64,
        /// The ID of the command.
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
    /// Route information to delete the original interaction response.
    DeleteInteractionOriginal {
        /// The ID of the owner application
        application_id: u64,
        /// The token of the interaction.
        interaction_token: &'a str,
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
    DeleteMessageSpecificReaction {
        /// The ID of the channel.
        channel_id: u64,
        /// The URI encoded custom or unicode emoji.
        emoji: &'a RequestReactionType<'a>,
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
    /// Route information to delete the current user's reaction on a message.
    DeleteReactionCurrentUser {
        /// ID of the channel.
        channel_id: u64,
        /// URI encoded custom or unicode emoji.
        emoji: &'a RequestReactionType<'a>,
        /// ID of the message.
        message_id: u64,
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
    /// Route information to delete a guild template.
    DeleteTemplate {
        /// The ID of the guild.
        guild_id: u64,
        /// The target template code.
        template_code: &'a str,
    },
    /// Route information to delete a message created by a webhook.
    DeleteWebhookMessage {
        message_id: u64,
        /// ID of the thread channel, if there is one.
        thread_id: Option<u64>,
        token: &'a str,
        webhook_id: u64,
    },
    /// Route information to delete a webhook.
    DeleteWebhook {
        /// The token of the webhook.
        token: Option<&'a str>,
        /// The ID of the webhook.
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
    /// Route information to get permissions of a specific guild command.
    GetCommandPermissions {
        /// The ID of the application.
        application_id: u64,
        /// The ID of the command.
        command_id: u64,
        /// The ID of the guild.
        guild_id: u64,
    },
    /// Route information to get info about application the current bot user belongs to
    GetCurrentUserApplicationInfo,
    /// Route information to get the current user.
    GetCurrentUser,
    /// Route information to get the current user as a member object within a guild.
    GetCurrentUserGuildMember {
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
    /// Route to get a followup message for an interaction.
    GetFollowupMessage {
        /// ID of the application.
        application_id: u64,
        /// Token of the interaction.
        interaction_token: &'a str,
        /// ID of the thread channel, if there is one.
        thread_id: Option<u64>,
        /// ID of the followup message.
        message_id: u64,
    },
    /// Route information to get basic gateway information.
    GetGateway,
    /// Route information to get gateway information tailored to the current
    /// user.
    GetGatewayBot,
    /// Route information to get a global command for an application.
    GetGlobalCommand {
        /// ID of the owner application.
        application_id: u64,
        /// ID of the command.
        command_id: u64,
    },
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
    /// Route information to get a guild command.
    GetGuildCommand {
        /// ID of the owner application.
        application_id: u64,
        /// ID of the command.
        command_id: u64,
        /// ID of the guild.
        guild_id: u64,
    },
    /// Route information to get permissions of all guild commands.
    GetGuildCommandPermissions {
        /// The ID of the application.
        application_id: u64,
        /// The ID of the guild.
        guild_id: u64,
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
        include_roles: &'a [Id<RoleMarker>],
    },
    /// Route information to get a guild's roles.
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
    /// Route information to get an original interaction response message.
    GetInteractionOriginal {
        /// ID of the owner application.
        application_id: u64,
        /// Token of the interaction.
        interaction_token: &'a str,
    },
    /// Route information to get an invite.
    GetInvite {
        /// The unique invite code.
        code: &'a str,
        /// Whether to retrieve statistics about the invite.
        with_counts: bool,
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
    /// Route information to get a list of sticker packs available to Nitro
    /// subscribers.
    GetNitroStickerPacks,
    /// Route information to get a channel's pins.
    GetPins {
        /// The ID of the channel.
        channel_id: u64,
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
        limit: Option<u64>,
        /// The ID of the message.
        message_id: u64,
    },
    /// Route information to get a stage instance.
    GetStageInstance {
        /// ID of the stage channel.
        channel_id: u64,
    },
    /// Route information to get a sticker.
    GetSticker {
        /// ID of the sticker.
        sticker_id: u64,
    },
    /// Route information to get a template.
    GetTemplate {
        /// The template code.
        template_code: &'a str,
    },
    /// Route information to get a list of templates from a guild.
    GetTemplates {
        /// The ID of the guild.
        guild_id: u64,
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
        /// ID of the thread.
        channel_id: u64,
    },
    /// Route information to get a user.
    GetUser {
        /// ID of the target user.
        user_id: u64,
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
    /// Route information to respond to an interaction.
    InteractionCallback {
        /// The ID of the interaction.
        interaction_id: u64,
        /// The token for the interaction.
        interaction_token: &'a str,
    },
    /// Route information to join a thread as the current user.
    JoinThread {
        /// ID of the thread.
        channel_id: u64,
    },
    /// Route information to leave the guild.
    LeaveGuild {
        /// The ID of the guild.
        guild_id: u64,
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
        limit: Option<u64>,
        /// Query to search by.
        query: &'a str,
    },
    /// Route information to set permissions of commands in a guild.
    SetCommandPermissions {
        /// The ID of the owner application.
        application_id: u64,
        /// The ID of the guild.
        guild_id: u64,
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
    /// Route information to sync a template.
    SyncTemplate {
        /// The ID of the guild.
        guild_id: u64,
        /// The template code.
        template_code: &'a str,
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
    /// Route information to edit permissions of a command in a guild.
    UpdateCommandPermissions {
        /// The ID of the application.
        application_id: u64,
        /// The ID of the command.
        command_id: u64,
        /// The ID of the guild.
        guild_id: u64,
    },
    /// Route information to update the current member.
    UpdateCurrentMember {
        /// ID of the guild.
        guild_id: u64,
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
    /// Update the original interaction response.
    UpdateInteractionOriginal {
        /// The ID of the owner application.
        application_id: u64,
        /// The token for the interaction.
        interaction_token: &'a str,
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
    /// Route information to update an existing stage instance.
    UpdateStageInstance {
        /// ID of the stage channel.
        channel_id: u64,
    },
    /// Route information to update a template.
    UpdateTemplate {
        /// The ID of the guild.
        guild_id: u64,
        /// The template code.
        template_code: &'a str,
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
        /// ID of the thread channel, if there is one.
        thread_id: Option<u64>,
        token: &'a str,
        webhook_id: u64,
    },
    /// Route information to update a webhook.
    UpdateWebhook {
        /// The token of the webhook.
        token: Option<&'a str>,
        /// The ID of the webhook.
        webhook_id: u64,
    },
}

impl<'a> Route<'a> {
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
            Self::DeleteBan { .. }
            | Self::DeleteChannel { .. }
            | Self::DeleteEmoji { .. }
            | Self::DeleteGlobalCommand { .. }
            | Self::DeleteGuild { .. }
            | Self::DeleteGuildCommand { .. }
            | Self::DeleteGuildIntegration { .. }
            | Self::DeleteGuildSticker { .. }
            | Self::DeleteInteractionOriginal { .. }
            | Self::DeleteInvite { .. }
            | Self::DeleteMessageReactions { .. }
            | Self::DeleteMessageSpecificReaction { .. }
            | Self::DeleteMessage { .. }
            | Self::DeletePermissionOverwrite { .. }
            | Self::DeleteReactionCurrentUser { .. }
            | Self::DeleteReaction { .. }
            | Self::DeleteRole { .. }
            | Self::DeleteStageInstance { .. }
            | Self::DeleteTemplate { .. }
            | Self::DeleteWebhookMessage { .. }
            | Self::DeleteWebhook { .. }
            | Self::LeaveGuild { .. }
            | Self::LeaveThread { .. }
            | Self::RemoveMember { .. }
            | Self::RemoveMemberRole { .. }
            | Self::RemoveThreadMember { .. }
            | Self::UnpinMessage { .. } => Method::Delete,
            Self::GetActiveThreads { .. }
            | Self::GetAuditLogs { .. }
            | Self::GetBan { .. }
            | Self::GetBans { .. }
            | Self::GetGatewayBot
            | Self::GetChannel { .. }
            | Self::GetChannelInvites { .. }
            | Self::GetChannelWebhooks { .. }
            | Self::GetChannels { .. }
            | Self::GetCommandPermissions { .. }
            | Self::GetCurrentUserApplicationInfo
            | Self::GetCurrentUser
            | Self::GetCurrentUserGuildMember { .. }
            | Self::GetEmoji { .. }
            | Self::GetEmojis { .. }
            | Self::GetGateway
            | Self::GetFollowupMessage { .. }
            | Self::GetGlobalCommand { .. }
            | Self::GetGlobalCommands { .. }
            | Self::GetGuild { .. }
            | Self::GetGuildCommand { .. }
            | Self::GetGuildCommandPermissions { .. }
            | Self::GetGuildCommands { .. }
            | Self::GetGuildIntegrations { .. }
            | Self::GetGuildInvites { .. }
            | Self::GetGuildMembers { .. }
            | Self::GetGuildPreview { .. }
            | Self::GetGuildPruneCount { .. }
            | Self::GetGuildRoles { .. }
            | Self::GetGuildSticker { .. }
            | Self::GetGuildStickers { .. }
            | Self::GetGuildVanityUrl { .. }
            | Self::GetGuildVoiceRegions { .. }
            | Self::GetGuildWelcomeScreen { .. }
            | Self::GetGuildWebhooks { .. }
            | Self::GetGuildWidget { .. }
            | Self::GetGuilds { .. }
            | Self::GetInteractionOriginal { .. }
            | Self::GetInvite { .. }
            | Self::GetInviteWithExpiration { .. }
            | Self::GetMember { .. }
            | Self::GetMessage { .. }
            | Self::GetMessages { .. }
            | Self::GetNitroStickerPacks { .. }
            | Self::GetPins { .. }
            | Self::GetJoinedPrivateArchivedThreads { .. }
            | Self::GetPrivateArchivedThreads { .. }
            | Self::GetPublicArchivedThreads { .. }
            | Self::GetReactionUsers { .. }
            | Self::GetStageInstance { .. }
            | Self::GetSticker { .. }
            | Self::GetTemplate { .. }
            | Self::GetTemplates { .. }
            | Self::GetThreadMember { .. }
            | Self::GetThreadMembers { .. }
            | Self::GetUserConnections
            | Self::GetUserPrivateChannels
            | Self::GetUser { .. }
            | Self::GetVoiceRegions
            | Self::GetWebhook { .. }
            | Self::GetWebhookMessage { .. }
            | Self::SearchGuildMembers { .. } => Method::Get,
            Self::UpdateChannel { .. }
            | Self::UpdateCurrentMember { .. }
            | Self::UpdateCurrentUser
            | Self::UpdateCurrentUserVoiceState { .. }
            | Self::UpdateEmoji { .. }
            | Self::UpdateGlobalCommand { .. }
            | Self::UpdateGuild { .. }
            | Self::UpdateGuildChannels { .. }
            | Self::UpdateGuildCommand { .. }
            | Self::UpdateGuildWidget { .. }
            | Self::UpdateGuildIntegration { .. }
            | Self::UpdateGuildSticker { .. }
            | Self::UpdateGuildWelcomeScreen { .. }
            | Self::UpdateInteractionOriginal { .. }
            | Self::UpdateMember { .. }
            | Self::UpdateMessage { .. }
            | Self::UpdateNickname { .. }
            | Self::UpdateRole { .. }
            | Self::UpdateRolePositions { .. }
            | Self::UpdateStageInstance { .. }
            | Self::UpdateTemplate { .. }
            | Self::UpdateUserVoiceState { .. }
            | Self::UpdateWebhookMessage { .. }
            | Self::UpdateWebhook { .. } => Method::Patch,
            Self::CreateChannel { .. }
            | Self::CreateGlobalCommand { .. }
            | Self::CreateGuildCommand { .. }
            | Self::CreateEmoji { .. }
            | Self::CreateGuild
            | Self::CreateGuildFromTemplate { .. }
            | Self::CreateGuildIntegration { .. }
            | Self::CreateGuildPrune { .. }
            | Self::CreateGuildSticker { .. }
            | Self::CreateInvite { .. }
            | Self::CreateMessage { .. }
            | Self::CreatePrivateChannel
            | Self::CreateThread { .. }
            | Self::CreateThreadFromMessage { .. }
            | Self::CreateRole { .. }
            | Self::CreateStageInstance { .. }
            | Self::CreateTemplate { .. }
            | Self::CreateTypingTrigger { .. }
            | Self::CreateWebhook { .. }
            | Self::CrosspostMessage { .. }
            | Self::DeleteMessages { .. }
            | Self::ExecuteWebhook { .. }
            | Self::FollowNewsChannel { .. }
            | Self::InteractionCallback { .. }
            | Self::SyncGuildIntegration { .. } => Method::Post,
            Self::AddGuildMember { .. }
            | Self::AddMemberRole { .. }
            | Self::AddThreadMember { .. }
            | Self::CreateBan { .. }
            | Self::CreateReaction { .. }
            | Self::JoinThread { .. }
            | Self::PinMessage { .. }
            | Self::SetCommandPermissions { .. }
            | Self::SetGlobalCommands { .. }
            | Self::SetGuildCommands { .. }
            | Self::SyncTemplate { .. }
            | Self::UpdateCommandPermissions { .. }
            | Self::UpdatePermissionOverwrite { .. } => Method::Put,
        }
    }

    /// Typed path of the route.
    ///
    /// Paths are used with a [`Ratelimiter`].
    ///
    /// # Examples
    ///
    /// Use a route's path to retrieve a ratelimiter ticket:
    ///
    /// ```
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    /// use twilight_http::routing::Route;
    /// use twilight_http_ratelimiting::{InMemoryRatelimiter, Ratelimiter};
    ///
    /// let ratelimiter = InMemoryRatelimiter::new();
    /// let route = Route::CreateMessage {
    ///     channel_id: 123,
    ///  };
    ///
    /// // Take a ticket from the ratelimiter.
    /// let rx = ratelimiter.ticket(route.path()).await?;
    ///
    /// // Wait to be told that a request can be made...
    /// let _tx = rx.await;
    ///
    /// // The request can now be made.
    /// # Ok(()) }
    /// ```
    ///
    /// [`Ratelimiter`]: twilight_http_ratelimiting::Ratelimiter
    #[allow(clippy::too_many_lines)]
    pub fn path(&self) -> Path {
        match self {
            Self::AddGuildMember { guild_id, .. }
            | Self::GetMember { guild_id, .. }
            | Self::RemoveMember { guild_id, .. }
            | Self::UpdateMember { guild_id, .. } => Path::GuildsIdMembersId(*guild_id),
            Self::AddMemberRole { guild_id, .. } | Self::RemoveMemberRole { guild_id, .. } => {
                Path::GuildsIdMembersIdRolesId(*guild_id)
            }
            Self::AddThreadMember { channel_id, .. }
            | Self::GetThreadMember { channel_id, .. }
            | Self::GetThreadMembers { channel_id, .. }
            | Self::JoinThread { channel_id, .. }
            | Self::LeaveThread { channel_id, .. }
            | Self::RemoveThreadMember { channel_id, .. } => {
                Path::ChannelsIdThreadMembers(*channel_id)
            }
            Self::CreateBan { guild_id, .. } | Self::DeleteBan { guild_id, .. } => {
                Path::GuildsIdBansUserId(*guild_id)
            }
            Self::CreateChannel { guild_id } => Path::GuildsIdChannels(*guild_id),
            Self::CreateEmoji { guild_id } | Self::GetEmojis { guild_id } => {
                Path::GuildsIdEmojis(*guild_id)
            }
            Self::CreateGlobalCommand { application_id }
            | Self::GetGlobalCommands { application_id }
            | Self::SetGlobalCommands { application_id } => {
                Path::ApplicationCommand(*application_id)
            }
            Self::CreateGuild => Path::Guilds,
            Self::CreateGuildFromTemplate { template_code, .. }
            | Self::GetTemplate { template_code, .. } => {
                Path::GuildsTemplatesCode((*template_code).to_string().into_boxed_str())
            }
            Self::CreateGuildCommand { application_id, .. }
            | Self::DeleteGuildCommand { application_id, .. }
            | Self::GetGuildCommand { application_id, .. }
            | Self::GetGuildCommandPermissions { application_id, .. }
            | Self::GetGuildCommands { application_id, .. }
            | Self::SetCommandPermissions { application_id, .. }
            | Self::SetGuildCommands { application_id, .. }
            | Self::UpdateGuildCommand { application_id, .. } => {
                Path::ApplicationGuildCommand(*application_id)
            }
            Self::CreateGuildIntegration { guild_id } => Path::GuildsIdIntegrationsId(*guild_id),
            Self::CreateGuildPrune { guild_id, .. } | Self::GetGuildPruneCount { guild_id, .. } => {
                Path::GuildsIdPrune(*guild_id)
            }
            Self::CreateGuildSticker { guild_id, .. }
            | Self::DeleteGuildSticker { guild_id, .. }
            | Self::GetGuildSticker { guild_id, .. }
            | Self::GetGuildStickers { guild_id, .. }
            | Self::UpdateGuildSticker { guild_id, .. } => Path::GuildsIdStickers(*guild_id),
            Self::CreateInvite { channel_id } | Self::GetChannelInvites { channel_id } => {
                Path::ChannelsIdInvites(*channel_id)
            }
            Self::CreateMessage { channel_id } | Self::GetMessages { channel_id, .. } => {
                Path::ChannelsIdMessages(*channel_id)
            }
            Self::CreatePrivateChannel | Self::GetUserPrivateChannels => Path::UsersIdChannels,
            Self::CreateReaction { channel_id, .. }
            | Self::DeleteReactionCurrentUser { channel_id, .. }
            | Self::DeleteReaction { channel_id, .. } => {
                Path::ChannelsIdMessagesIdReactionsUserIdType(*channel_id)
            }
            Self::CreateRole { guild_id } | Self::GetGuildRoles { guild_id } => {
                Path::GuildsIdRoles(*guild_id)
            }
            Self::CreateStageInstance { .. }
            | Self::DeleteStageInstance { .. }
            | Self::GetStageInstance { .. }
            | Self::UpdateStageInstance { .. } => Path::StageInstances,
            Self::CreateTemplate { guild_id } | Self::GetTemplates { guild_id } => {
                Path::GuildsIdTemplates(*guild_id)
            }
            Self::CreateThread { channel_id, .. } => Path::ChannelsIdThreads(*channel_id),
            Self::CreateThreadFromMessage { channel_id, .. } => {
                Path::ChannelsIdMessagesIdThreads(*channel_id)
            }
            Self::CreateTypingTrigger { channel_id } => Path::ChannelsIdTyping(*channel_id),
            Self::CreateWebhook { channel_id } | Self::GetChannelWebhooks { channel_id } => {
                Path::ChannelsIdWebhooks(*channel_id)
            }
            Self::CrosspostMessage { channel_id, .. } => {
                Path::ChannelsIdMessagesIdCrosspost(*channel_id)
            }
            Self::DeleteChannel { channel_id } => Path::ChannelsId(*channel_id),
            Self::DeleteEmoji { guild_id, .. } => Path::GuildsIdEmojisId(*guild_id),
            Self::DeleteGlobalCommand { application_id, .. }
            | Self::GetGlobalCommand { application_id, .. }
            | Self::UpdateGlobalCommand { application_id, .. } => {
                Path::ApplicationCommandId(*application_id)
            }
            Self::DeleteGuild { guild_id } => Path::GuildsId(*guild_id),
            Self::DeleteGuildIntegration { guild_id, .. }
            | Self::UpdateGuildIntegration { guild_id, .. } => {
                Path::GuildsIdIntegrationsId(*guild_id)
            }
            Self::DeleteInteractionOriginal {
                application_id,
                interaction_token,
                ..
            }
            | Self::GetFollowupMessage {
                application_id,
                interaction_token,
                ..
            }
            | Self::GetInteractionOriginal {
                application_id,
                interaction_token,
                ..
            }
            | Self::UpdateInteractionOriginal {
                application_id,
                interaction_token,
                ..
            } => Path::WebhooksIdTokenMessagesId(
                *application_id,
                (*interaction_token).to_string().into_boxed_str(),
            ),
            Self::DeleteInvite { .. }
            | Self::GetInvite { .. }
            | Self::GetInviteWithExpiration { .. } => Path::InvitesCode,
            Self::DeleteMessageReactions { channel_id, .. }
            | Self::DeleteMessageSpecificReaction { channel_id, .. }
            | Self::GetReactionUsers { channel_id, .. } => {
                Path::ChannelsIdMessagesIdReactions(*channel_id)
            }
            Self::DeleteMessage { message_id, .. } => {
                Path::ChannelsIdMessagesId(Method::Delete, *message_id)
            }
            Self::DeleteMessages { channel_id } => Path::ChannelsIdMessagesBulkDelete(*channel_id),
            Self::DeletePermissionOverwrite { channel_id, .. }
            | Self::UpdatePermissionOverwrite { channel_id, .. } => {
                Path::ChannelsIdPermissionsOverwriteId(*channel_id)
            }
            Self::DeleteRole { guild_id, .. }
            | Self::UpdateRole { guild_id, .. }
            | Self::UpdateRolePositions { guild_id } => Path::GuildsIdRolesId(*guild_id),
            Self::DeleteTemplate {
                guild_id,
                template_code,
                ..
            }
            | Self::SyncTemplate {
                guild_id,
                template_code,
                ..
            }
            | Self::UpdateTemplate {
                guild_id,
                template_code,
                ..
            } => Path::GuildsIdTemplatesCode(
                *guild_id,
                (*template_code).to_string().into_boxed_str(),
            ),
            Self::DeleteWebhookMessage {
                webhook_id, token, ..
            }
            | Self::GetWebhookMessage {
                webhook_id, token, ..
            }
            | Self::UpdateWebhookMessage {
                webhook_id, token, ..
            } => {
                Path::WebhooksIdTokenMessagesId(*webhook_id, (*token).to_string().into_boxed_str())
            }
            Self::DeleteWebhook {
                webhook_id,
                token: Some(token),
                ..
            }
            | Self::ExecuteWebhook {
                webhook_id, token, ..
            }
            | Self::GetWebhook {
                webhook_id,
                token: Some(token),
                ..
            }
            | Self::UpdateWebhook {
                webhook_id,
                token: Some(token),
            } => Path::WebhooksIdToken(*webhook_id, (*token).to_string().into_boxed_str()),
            Self::DeleteWebhook { webhook_id, .. }
            | Self::GetWebhook { webhook_id, .. }
            | Self::UpdateWebhook { webhook_id, .. } => (Path::WebhooksId(*webhook_id)),
            Self::FollowNewsChannel { channel_id } => Path::ChannelsIdFollowers(*channel_id),
            Self::GetJoinedPrivateArchivedThreads { channel_id, .. }
            | Self::GetPrivateArchivedThreads { channel_id, .. }
            | Self::GetPublicArchivedThreads { channel_id, .. } => {
                Path::ChannelsIdThreads(*channel_id)
            }
            Self::GetActiveThreads { guild_id, .. } => Path::GuildsIdThreads(*guild_id),
            Self::GetAuditLogs { guild_id, .. } => Path::GuildsIdAuditLogs(*guild_id),
            Self::GetBan { guild_id, .. } => Path::GuildsIdBansId(*guild_id),
            Self::GetBans { guild_id } => Path::GuildsIdBans(*guild_id),
            Self::GetGatewayBot => Path::GatewayBot,
            Self::GetChannel { channel_id } | Self::UpdateChannel { channel_id } => {
                Path::ChannelsId(*channel_id)
            }
            Self::GetChannels { guild_id } | Self::UpdateGuildChannels { guild_id } => {
                Path::GuildsIdChannels(*guild_id)
            }
            Self::GetCommandPermissions { application_id, .. }
            | Self::UpdateCommandPermissions { application_id, .. } => {
                Path::ApplicationGuildCommandId(*application_id)
            }
            Self::GetCurrentUserApplicationInfo => Path::OauthApplicationsMe,
            Self::GetCurrentUser | Self::GetUser { .. } | Self::UpdateCurrentUser => Path::UsersId,
            Self::GetCurrentUserGuildMember { .. } => Path::UsersIdGuildsIdMember,
            Self::GetEmoji { guild_id, .. } | Self::UpdateEmoji { guild_id, .. } => {
                Path::GuildsIdEmojisId(*guild_id)
            }
            Self::GetGateway => Path::Gateway,
            Self::GetGuild { guild_id, .. } | Self::UpdateGuild { guild_id } => {
                Path::GuildsId(*guild_id)
            }
            Self::GetGuildWidget { guild_id } | Self::UpdateGuildWidget { guild_id } => {
                Path::GuildsIdWidget(*guild_id)
            }
            Self::GetGuildIntegrations { guild_id } => Path::GuildsIdIntegrations(*guild_id),
            Self::GetGuildInvites { guild_id } => Path::GuildsIdInvites(*guild_id),
            Self::GetGuildMembers { guild_id, .. } | Self::UpdateCurrentMember { guild_id, .. } => {
                Path::GuildsIdMembers(*guild_id)
            }
            Self::GetGuildPreview { guild_id } => Path::GuildsIdPreview(*guild_id),
            Self::GetGuildVanityUrl { guild_id } => Path::GuildsIdVanityUrl(*guild_id),
            Self::GetGuildVoiceRegions { guild_id } => Path::GuildsIdRegions(*guild_id),
            Self::GetGuildWelcomeScreen { guild_id }
            | Self::UpdateGuildWelcomeScreen { guild_id } => Path::GuildsIdWelcomeScreen(*guild_id),
            Self::GetGuildWebhooks { guild_id } => Path::GuildsIdWebhooks(*guild_id),
            Self::GetGuilds { .. } => Path::UsersIdGuilds,
            Self::GetMessage { channel_id, .. } => {
                Path::ChannelsIdMessagesId(Method::Get, *channel_id)
            }
            Self::GetNitroStickerPacks { .. } => Path::StickerPacks,
            Self::GetPins { channel_id } | Self::PinMessage { channel_id, .. } => {
                Path::ChannelsIdPins(*channel_id)
            }
            Self::GetSticker { .. } => Path::Stickers,
            Self::GetUserConnections => Path::UsersIdConnections,
            Self::GetVoiceRegions => Path::VoiceRegions,
            Self::InteractionCallback { interaction_id, .. } => {
                Path::InteractionCallback(*interaction_id)
            }
            Self::LeaveGuild { .. } => Path::UsersIdGuildsId,
            Self::SearchGuildMembers { guild_id, .. } => Path::GuildsIdMembersSearch(*guild_id),
            Self::SyncGuildIntegration { guild_id, .. } => {
                Path::GuildsIdIntegrationsIdSync(*guild_id)
            }
            Self::UnpinMessage { channel_id, .. } => Path::ChannelsIdPinsMessageId(*channel_id),
            Self::UpdateCurrentUserVoiceState { guild_id }
            | Self::UpdateUserVoiceState { guild_id, .. } => Path::GuildsIdVoiceStates(*guild_id),
            Self::UpdateMessage { channel_id, .. } => {
                Path::ChannelsIdMessagesId(Method::Patch, *channel_id)
            }
            Self::UpdateNickname { guild_id } => Path::GuildsIdMembersMeNick(*guild_id),
        }
    }
}

/// Display formatter of the route portion of a URL.
///
/// # Examples
///
/// Create a formatted representation of the [`GetPins`] route:
///
/// ```
/// use twilight_http::routing::Route;
///
/// let route = Route::GetPins {
///     channel_id: 123,
/// };
/// assert_eq!("channels/123/pins", route.to_string());
/// ```
///
/// Create a formatted representation of the [`GetInvite`] route, which
/// includes a query parameter:
///
/// ```
/// use twilight_http::routing::Route;
///
/// let route = Route::GetInvite {
///     code: "twilight-rs",
///     with_counts: true,
/// };
///
/// assert_eq!(
///     "invites/twilight-rs?with-counts=true",
///     route.to_string(),
/// );
/// ```
///
/// [`GetInvite`]: Self::GetInvite
/// [`GetPins`]: Self::GetPins
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
            Route::CreateBan {
                guild_id,
                delete_message_days,
                reason,
                user_id,
            } => {
                f.write_str("guilds/")?;
                Display::fmt(guild_id, f)?;
                f.write_str("/bans/")?;
                Display::fmt(user_id, f)?;
                f.write_str("?")?;

                if let Some(delete_message_days) = delete_message_days {
                    f.write_str("delete_message_days=")?;
                    Display::fmt(delete_message_days, f)?;

                    if reason.is_some() {
                        f.write_str("&")?;
                    }
                }

                if let Some(reason) = reason {
                    f.write_str("reason=")?;
                    let encoded_reason = utf8_percent_encode(reason, NON_ALPHANUMERIC);

                    Display::fmt(&encoded_reason, f)?;
                }

                Ok(())
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
            Route::CreateGlobalCommand { application_id }
            | Route::GetGlobalCommands { application_id }
            | Route::SetGlobalCommands { application_id } => {
                f.write_str("applications/")?;
                Display::fmt(application_id, f)?;

                f.write_str("/commands")
            }
            Route::CreateGuild => f.write_str("guilds"),
            Route::CreateGuildCommand {
                application_id,
                guild_id,
            }
            | Route::GetGuildCommands {
                application_id,
                guild_id,
            }
            | Route::SetGuildCommands {
                application_id,
                guild_id,
            } => {
                f.write_str("applications/")?;
                Display::fmt(application_id, f)?;
                f.write_str("/guilds/")?;
                Display::fmt(guild_id, f)?;

                f.write_str("/commands")
            }
            Route::CreateGuildFromTemplate { template_code }
            | Route::GetTemplate { template_code } => {
                f.write_str("guilds/templates/")?;

                f.write_str(template_code)
            }
            Route::CreateGuildIntegration { guild_id }
            | Route::GetGuildIntegrations { guild_id } => {
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
                f.write_str("/prune?")?;

                if let Some(compute_prune_count) = compute_prune_count {
                    f.write_str("compute_prune_count=")?;
                    Display::fmt(compute_prune_count, f)?;
                }

                if let Some(days) = days {
                    f.write_str("&days=")?;
                    Display::fmt(days, f)?;
                }

                if !include_roles.is_empty() {
                    let role_count = include_roles.len() - 1;

                    f.write_str("&include_roles=")?;

                    for (idx, role_id) in include_roles.iter().enumerate() {
                        Display::fmt(role_id, f)?;

                        if idx < role_count {
                            f.write_str(",")?;
                        }
                    }
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
            Route::CreatePrivateChannel | Route::GetUserPrivateChannels => {
                f.write_str("users/@me/channels")
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
            Route::CreateStageInstance { .. } => f.write_str("stage-instances"),
            Route::CreateTemplate { guild_id } | Route::GetTemplates { guild_id } => {
                f.write_str("guilds/")?;
                Display::fmt(guild_id, f)?;

                f.write_str("/templates")
            }
            Route::CreateThread { channel_id } => {
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
            Route::DeleteBan { guild_id, user_id } | Route::GetBan { guild_id, user_id } => {
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
            Route::DeleteGlobalCommand {
                application_id,
                command_id,
            }
            | Route::GetGlobalCommand {
                application_id,
                command_id,
            }
            | Route::UpdateGlobalCommand {
                application_id,
                command_id,
            } => {
                f.write_str("applications/")?;
                Display::fmt(application_id, f)?;
                f.write_str("/commands/")?;

                Display::fmt(command_id, f)
            }
            Route::DeleteGuild { guild_id } | Route::UpdateGuild { guild_id } => {
                f.write_str("guilds/")?;

                Display::fmt(guild_id, f)
            }
            Route::DeleteGuildCommand {
                application_id,
                command_id,
                guild_id,
            }
            | Route::GetGuildCommand {
                application_id,
                command_id,
                guild_id,
            }
            | Route::UpdateGuildCommand {
                application_id,
                command_id,
                guild_id,
            } => {
                f.write_str("applications/")?;
                Display::fmt(application_id, f)?;
                f.write_str("/guilds/")?;
                Display::fmt(guild_id, f)?;
                f.write_str("/commands/")?;

                Display::fmt(command_id, f)
            }
            Route::DeleteGuildIntegration {
                guild_id,
                integration_id,
            }
            | Route::UpdateGuildIntegration {
                guild_id,
                integration_id,
            } => {
                f.write_str("guilds/")?;
                Display::fmt(guild_id, f)?;
                f.write_str("/integrations/")?;

                Display::fmt(integration_id, f)
            }
            Route::DeleteInteractionOriginal {
                application_id,
                interaction_token,
            }
            | Route::GetInteractionOriginal {
                application_id,
                interaction_token,
            }
            | Route::UpdateInteractionOriginal {
                application_id,
                interaction_token,
            } => {
                f.write_str("webhooks/")?;
                Display::fmt(application_id, f)?;
                f.write_str("/")?;
                f.write_str(interaction_token)?;

                f.write_str("/messages/@original")
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
            Route::DeleteRole { guild_id, role_id } | Route::UpdateRole { guild_id, role_id } => {
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
            Route::DeleteTemplate {
                guild_id,
                template_code,
            }
            | Route::SyncTemplate {
                guild_id,
                template_code,
            }
            | Route::UpdateTemplate {
                guild_id,
                template_code,
            } => {
                f.write_str("guilds/")?;
                Display::fmt(guild_id, f)?;
                f.write_str("/templates/")?;

                f.write_str(template_code)
            }
            Route::DeleteWebhookMessage {
                message_id,
                thread_id,
                token,
                webhook_id,
            }
            | Route::GetFollowupMessage {
                application_id: webhook_id,
                interaction_token: token,
                thread_id,
                message_id,
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

                if let Some(thread_id) = thread_id {
                    f.write_str("?thread_id=")?;
                    Display::fmt(thread_id, f)?;
                }

                Ok(())
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
                webhook_id,
            } => {
                f.write_str("webhooks/")?;
                Display::fmt(webhook_id, f)?;
                f.write_str("/")?;
                f.write_str(token)?;
                f.write_str("?")?;

                if let Some(thread_id) = thread_id {
                    f.write_str("thread_id=")?;
                    Display::fmt(thread_id, f)?;
                    f.write_str("&")?;
                }

                if let Some(wait) = wait {
                    f.write_str("wait=")?;
                    f.write_str(if *wait { "true" } else { "false" })?;
                }

                Ok(())
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
                before,
                guild_id,
                limit,
                user_id,
            } => {
                f.write_str("guilds/")?;
                Display::fmt(guild_id, f)?;
                f.write_str("/audit-logs?")?;

                if let Some(action_type) = action_type {
                    f.write_str("action_type=")?;
                    Display::fmt(action_type, f)?;
                }

                if let Some(before) = before {
                    f.write_str("&before=")?;
                    Display::fmt(before, f)?;
                }

                if let Some(limit) = limit {
                    f.write_str("&limit=")?;
                    Display::fmt(limit, f)?;
                }

                if let Some(user_id) = user_id {
                    f.write_str("&user_id=")?;
                    Display::fmt(user_id, f)?;
                }

                Ok(())
            }
            Route::GetBans { guild_id } => {
                f.write_str("guilds/")?;
                Display::fmt(guild_id, f)?;

                f.write_str("/bans")
            }
            Route::GetGatewayBot => f.write_str("gateway/bot"),
            Route::GetCommandPermissions {
                application_id,
                command_id,
                guild_id,
            }
            | Route::UpdateCommandPermissions {
                application_id,
                command_id,
                guild_id,
            } => {
                f.write_str("applications/")?;
                Display::fmt(application_id, f)?;
                f.write_str("/guilds/")?;
                Display::fmt(guild_id, f)?;
                f.write_str("/commands/")?;
                Display::fmt(command_id, f)?;

                f.write_str("/permissions")
            }
            Route::GetCurrentUserApplicationInfo => f.write_str("oauth2/applications/@me"),
            Route::GetCurrentUser | Route::UpdateCurrentUser => f.write_str("users/@me"),
            Route::GetCurrentUserGuildMember { guild_id } => {
                f.write_str("users/@me/guilds/")?;
                Display::fmt(guild_id, f)?;

                f.write_str("/member")
            }
            Route::GetGateway => f.write_str("gateway"),
            Route::GetGuild {
                guild_id,
                with_counts,
            } => {
                f.write_str("guilds/")?;
                Display::fmt(guild_id, f)?;

                if *with_counts {
                    f.write_str("?with_counts=true")?;
                }

                Ok(())
            }
            Route::GetGuildCommandPermissions {
                application_id,
                guild_id,
            }
            | Route::SetCommandPermissions {
                application_id,
                guild_id,
            } => {
                f.write_str("applications/")?;
                Display::fmt(application_id, f)?;
                f.write_str("/guilds/")?;
                Display::fmt(guild_id, f)?;

                f.write_str("/commands/permissions")
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
                presences,
            } => {
                f.write_str("guilds/")?;
                Display::fmt(guild_id, f)?;
                f.write_str("/members?")?;

                if let Some(after) = after {
                    f.write_str("after=")?;
                    Display::fmt(after, f)?;
                }

                if let Some(limit) = limit {
                    f.write_str("&limit=")?;
                    Display::fmt(limit, f)?;
                }

                if let Some(presences) = presences {
                    f.write_str("&presences=")?;
                    Display::fmt(presences, f)?;
                }

                Ok(())
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
                f.write_str("/prune?")?;

                if let Some(days) = days {
                    f.write_str("days=")?;
                    Display::fmt(days, f)?;
                }

                if !include_roles.is_empty() {
                    f.write_str("&include_roles=")?;

                    let role_count = include_roles.len() - 1;

                    for (idx, role_id) in include_roles.iter().enumerate() {
                        Display::fmt(role_id, f)?;

                        if idx < role_count {
                            f.write_str(",")?;
                        }
                    }
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
            Route::GetGuildWidget { guild_id } | Route::UpdateGuildWidget { guild_id } => {
                f.write_str("guilds/")?;
                Display::fmt(guild_id, f)?;

                f.write_str("/widget")
            }
            Route::GetGuilds {
                after,
                before,
                limit,
            } => {
                f.write_str("users/@me/guilds?")?;

                if let Some(after) = after {
                    f.write_str("after=")?;
                    Display::fmt(after, f)?;
                }

                if let Some(before) = before {
                    f.write_str("&before=")?;
                    Display::fmt(before, f)?;
                }

                if let Some(limit) = limit {
                    f.write_str("&limit=")?;
                    Display::fmt(limit, f)?;
                }

                Ok(())
            }
            Route::GetInvite { code, with_counts } => {
                f.write_str("invites/")?;
                f.write_str(code)?;

                if *with_counts {
                    f.write_str("?with-counts=true")?;
                }

                Ok(())
            }
            Route::GetInviteWithExpiration {
                code,
                with_counts,
                with_expiration,
            } => {
                f.write_str("invites/")?;
                f.write_str(code)?;
                f.write_str("?")?;

                if *with_counts {
                    f.write_str("with-counts=true")?;
                }

                if *with_expiration {
                    f.write_str("with-expiration=true")?;
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
                f.write_str("/messages?")?;

                if let Some(after) = after {
                    f.write_str("after=")?;
                    Display::fmt(after, f)?;
                }

                if let Some(around) = around {
                    f.write_str("&around=")?;
                    Display::fmt(around, f)?;
                }

                if let Some(before) = before {
                    f.write_str("&before=")?;
                    Display::fmt(before, f)?;
                }

                if let Some(limit) = limit {
                    f.write_str("&limit=")?;
                    Display::fmt(limit, f)?;
                }

                Ok(())
            }
            Route::GetNitroStickerPacks { .. } => f.write_str("sticker-packs"),
            Route::GetPins { channel_id } => {
                f.write_str("channels/")?;
                Display::fmt(channel_id, f)?;

                f.write_str("/pins")
            }
            Route::GetJoinedPrivateArchivedThreads {
                before,
                channel_id,
                limit,
            } => {
                f.write_str("channels/")?;
                Display::fmt(channel_id, f)?;
                f.write_str("/users/@me/threads/archived/private?")?;

                if let Some(before) = before {
                    f.write_str("before=")?;
                    Display::fmt(before, f)?;
                }

                if let Some(limit) = limit {
                    f.write_str("&limit=")?;
                    Display::fmt(limit, f)?;
                }

                Ok(())
            }
            Route::GetPrivateArchivedThreads {
                before,
                channel_id,
                limit,
            } => {
                f.write_str("channels/")?;
                Display::fmt(channel_id, f)?;
                f.write_str("/threads/archived/private?")?;

                if let Some(before) = before {
                    f.write_str("before=")?;
                    Display::fmt(before, f)?;
                }

                if let Some(limit) = limit {
                    f.write_str("&limit=")?;
                    Display::fmt(limit, f)?;
                }

                Ok(())
            }
            Route::GetPublicArchivedThreads {
                before,
                channel_id,
                limit,
            } => {
                f.write_str("channels/")?;
                Display::fmt(channel_id, f)?;
                f.write_str("/threads/archived/public?")?;

                if let Some(before) = before {
                    f.write_str("before=")?;
                    Display::fmt(before, f)?;
                }

                if let Some(limit) = limit {
                    f.write_str("&limit=")?;
                    Display::fmt(limit, f)?;
                }

                Ok(())
            }
            Route::GetReactionUsers {
                after,
                channel_id,
                emoji,
                limit,
                message_id,
            } => {
                f.write_str("channels/")?;
                Display::fmt(channel_id, f)?;
                f.write_str("/messages/")?;
                Display::fmt(message_id, f)?;
                f.write_str("/reactions/")?;
                Display::fmt(&emoji, f)?;
                f.write_str("?")?;

                if let Some(after) = after {
                    f.write_str("after=")?;
                    Display::fmt(after, f)?;
                }

                if let Some(limit) = limit {
                    f.write_str("&limit=")?;
                    Display::fmt(limit, f)?;
                }

                Ok(())
            }
            Route::GetSticker { sticker_id } => {
                f.write_str("stickers/")?;

                Display::fmt(sticker_id, f)
            }
            Route::GetThreadMembers { channel_id } => {
                f.write_str("channels/")?;
                Display::fmt(channel_id, f)?;

                f.write_str("/thread-members")
            }
            Route::GetUserConnections => f.write_str("users/@me/connections"),
            Route::GetUser { user_id } => {
                f.write_str("users/")?;

                Display::fmt(user_id, f)
            }
            Route::GetVoiceRegions => f.write_str("voice/regions"),
            Route::InteractionCallback {
                interaction_id,
                interaction_token,
            } => {
                f.write_str("interactions/")?;
                Display::fmt(interaction_id, f)?;
                f.write_str("/")?;
                f.write_str(interaction_token)?;

                f.write_str("/callback")
            }
            Route::JoinThread { channel_id } | Route::LeaveThread { channel_id } => {
                f.write_str("channels/")?;
                Display::fmt(channel_id, f)?;

                f.write_str("/thread-members/@me")
            }
            Route::LeaveGuild { guild_id } => {
                f.write_str("users/@me/guilds/")?;

                Display::fmt(guild_id, f)
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
                f.write_str("/pins/")?;

                Display::fmt(message_id, f)
            }
            Route::SearchGuildMembers {
                guild_id,
                limit,
                query,
            } => {
                f.write_str("guilds/")?;
                Display::fmt(guild_id, f)?;
                f.write_str("/members/search?query=")?;
                f.write_str(query)?;

                if let Some(limit) = limit {
                    f.write_str("&limit=")?;
                    Display::fmt(limit, f)?;
                }

                Ok(())
            }
            Route::SyncGuildIntegration {
                guild_id,
                integration_id,
            } => {
                f.write_str("guilds/")?;
                Display::fmt(guild_id, f)?;
                f.write_str("/integrations/")?;
                Display::fmt(integration_id, f)?;

                f.write_str("/sync")
            }
            Route::UpdateCurrentMember { guild_id } => {
                f.write_str("guilds/")?;
                Display::fmt(guild_id, f)?;

                f.write_str("/members/@me")
            }
            Route::UpdateCurrentUserVoiceState { guild_id } => {
                f.write_str("guilds/")?;
                Display::fmt(guild_id, f)?;

                f.write_str("/voice-states/@me")
            }
            Route::UpdateNickname { guild_id } => {
                f.write_str("guilds/")?;
                Display::fmt(guild_id, f)?;

                f.write_str("/members/@me/nick")
            }
            Route::UpdateUserVoiceState { guild_id, user_id } => {
                f.write_str("guilds/")?;
                Display::fmt(guild_id, f)?;
                f.write_str("/voice-states/")?;

                Display::fmt(user_id, f)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Route;
    use crate::request::{channel::reaction::RequestReactionType, Method};
    use twilight_model::id::Id;

    /// Test a route for each method.
    #[test]
    fn test_methods() {
        assert_eq!(
            Method::Delete,
            Route::DeleteInvite {
                code: "twilight-rs",
            }
            .method()
        );
        assert_eq!(
            Method::Get,
            Route::GetInvite {
                code: "twilight-rs",
                with_counts: false,
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
        assert_eq!(Method::Post, Route::CreateGuild.method());
        assert_eq!(
            Method::Put,
            Route::SyncTemplate {
                guild_id: 123,
                template_code: "abc",
            }
            .method()
        );
    }

    // Test display implementation

    const APPLICATION_ID: u64 = 1;
    const CHANNEL_ID: u64 = 2;
    const CODE: &str = "invitecode";
    const COMMAND_ID: u64 = 3;
    const EMOJI_ID: u64 = 4;
    const GUILD_ID: u64 = 5;
    const INTERACTION_ID: u64 = 6;
    const INTERACTION_TOKEN: &str = "interactiontoken";
    const INTEGRATION_ID: u64 = 7;
    const MESSAGE_ID: u64 = 8;
    const ROLE_ID: u64 = 9;
    const STICKER_ID: u64 = 10;
    const TEMPLATE_CODE: &str = "templatecode";
    const USER_ID: u64 = 11;

    const fn emoji() -> RequestReactionType<'static> {
        RequestReactionType::Custom {
            id: Id::new(EMOJI_ID),
            name: None,
        }
    }

    #[test]
    fn test_get_public_archived_threads() {
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
    fn test_update_webhook_message_thread_id() {
        let route = Route::UpdateWebhookMessage {
            message_id: 1,
            thread_id: Some(2),
            token: "token",
            webhook_id: 3,
        };

        assert_eq!("webhooks/3/token/messages/1?thread_id=2", route.to_string())
    }

    #[test]
    fn test_add_guild_member() {
        let route = Route::AddGuildMember {
            guild_id: GUILD_ID,
            user_id: USER_ID,
        };
        assert_eq!(
            route.to_string(),
            format!(
                "guilds/{guild_id}/members/{user_id}",
                guild_id = GUILD_ID,
                user_id = USER_ID
            )
        );
    }

    #[test]
    fn test_get_member() {
        let route = Route::GetMember {
            guild_id: GUILD_ID,
            user_id: USER_ID,
        };
        assert_eq!(
            route.to_string(),
            format!(
                "guilds/{guild_id}/members/{user_id}",
                guild_id = GUILD_ID,
                user_id = USER_ID
            )
        );
    }

    #[test]
    fn test_remove_member() {
        let route = Route::RemoveMember {
            guild_id: GUILD_ID,
            user_id: USER_ID,
        };
        assert_eq!(
            route.to_string(),
            format!(
                "guilds/{guild_id}/members/{user_id}",
                guild_id = GUILD_ID,
                user_id = USER_ID
            )
        );
    }

    #[test]
    fn test_update_member() {
        let route = Route::UpdateMember {
            guild_id: GUILD_ID,
            user_id: USER_ID,
        };
        assert_eq!(
            route.to_string(),
            format!(
                "guilds/{guild_id}/members/{user_id}",
                guild_id = GUILD_ID,
                user_id = USER_ID
            )
        );
    }

    #[test]
    fn test_add_member_role() {
        let route = Route::AddMemberRole {
            guild_id: GUILD_ID,
            role_id: ROLE_ID,
            user_id: USER_ID,
        };
        assert_eq!(
            route.to_string(),
            format!(
                "guilds/{guild_id}/members/{user_id}/roles/{role_id}",
                guild_id = GUILD_ID,
                role_id = ROLE_ID,
                user_id = USER_ID
            )
        );
    }

    #[test]
    fn test_remove_member_role() {
        let route = Route::RemoveMemberRole {
            guild_id: GUILD_ID,
            role_id: ROLE_ID,
            user_id: USER_ID,
        };
        assert_eq!(
            route.to_string(),
            format!(
                "guilds/{guild_id}/members/{user_id}/roles/{role_id}",
                guild_id = GUILD_ID,
                role_id = ROLE_ID,
                user_id = USER_ID
            )
        );
    }

    #[test]
    fn test_add_thread_member() {
        let route = Route::AddThreadMember {
            channel_id: CHANNEL_ID,
            user_id: USER_ID,
        };
        assert_eq!(
            route.to_string(),
            format!(
                "channels/{channel_id}/thread-members/{user_id}",
                channel_id = CHANNEL_ID,
                user_id = USER_ID
            )
        );
    }

    #[test]
    fn test_get_thread_member() {
        let route = Route::GetThreadMember {
            channel_id: CHANNEL_ID,
            user_id: USER_ID,
        };
        assert_eq!(
            route.to_string(),
            format!(
                "channels/{channel_id}/thread-members/{user_id}",
                channel_id = CHANNEL_ID,
                user_id = USER_ID
            )
        );
    }

    #[test]
    fn test_remove_thread_member() {
        let route = Route::RemoveThreadMember {
            channel_id: CHANNEL_ID,
            user_id: USER_ID,
        };
        assert_eq!(
            route.to_string(),
            format!(
                "channels/{channel_id}/thread-members/{user_id}",
                channel_id = CHANNEL_ID,
                user_id = USER_ID
            )
        );
    }

    #[test]
    fn test_create_channel() {
        let route = Route::CreateChannel { guild_id: GUILD_ID };
        assert_eq!(
            route.to_string(),
            format!("guilds/{guild_id}/channels", guild_id = GUILD_ID)
        );
    }

    #[test]
    fn test_get_channels() {
        let route = Route::GetChannels { guild_id: GUILD_ID };
        assert_eq!(
            route.to_string(),
            format!("guilds/{guild_id}/channels", guild_id = GUILD_ID)
        );
    }

    #[test]
    fn test_update_guild_channels() {
        let route = Route::UpdateGuildChannels { guild_id: GUILD_ID };
        assert_eq!(
            route.to_string(),
            format!("guilds/{guild_id}/channels", guild_id = GUILD_ID)
        );
    }

    #[test]
    fn test_create_emoji() {
        let route = Route::CreateEmoji { guild_id: GUILD_ID };
        assert_eq!(
            route.to_string(),
            format!("guilds/{guild_id}/emojis", guild_id = GUILD_ID)
        );
    }

    #[test]
    fn test_get_emojis() {
        let route = Route::GetEmojis { guild_id: GUILD_ID };
        assert_eq!(
            route.to_string(),
            format!("guilds/{guild_id}/emojis", guild_id = GUILD_ID)
        );
    }

    #[test]
    fn test_create_global_command() {
        let route = Route::CreateGlobalCommand {
            application_id: APPLICATION_ID,
        };
        assert_eq!(
            route.to_string(),
            format!(
                "applications/{application_id}/commands",
                application_id = APPLICATION_ID
            )
        );
    }

    #[test]
    fn test_get_global_commands() {
        let route = Route::GetGlobalCommands {
            application_id: APPLICATION_ID,
        };
        assert_eq!(
            route.to_string(),
            format!(
                "applications/{application_id}/commands",
                application_id = APPLICATION_ID
            )
        );
    }

    #[test]
    fn test_set_global_commands() {
        let route = Route::SetGlobalCommands {
            application_id: APPLICATION_ID,
        };
        assert_eq!(
            route.to_string(),
            format!(
                "applications/{application_id}/commands",
                application_id = APPLICATION_ID
            )
        );
    }

    #[test]
    fn test_create_guild() {
        let route = Route::CreateGuild;
        assert_eq!(route.to_string(), "guilds");
    }

    #[test]
    fn test_create_guild_command() {
        let route = Route::CreateGuildCommand {
            application_id: APPLICATION_ID,
            guild_id: GUILD_ID,
        };
        assert_eq!(
            route.to_string(),
            format!(
                "applications/{application_id}/guilds/{guild_id}/commands",
                application_id = APPLICATION_ID,
                guild_id = GUILD_ID
            )
        );
    }

    #[test]
    fn test_get_guild_commands() {
        let route = Route::GetGuildCommands {
            application_id: APPLICATION_ID,
            guild_id: GUILD_ID,
        };
        assert_eq!(
            route.to_string(),
            format!(
                "applications/{application_id}/guilds/{guild_id}/commands",
                application_id = APPLICATION_ID,
                guild_id = GUILD_ID
            )
        );
    }

    #[test]
    fn test_set_guild_commands() {
        let route = Route::SetGuildCommands {
            application_id: APPLICATION_ID,
            guild_id: GUILD_ID,
        };
        assert_eq!(
            route.to_string(),
            format!(
                "applications/{application_id}/guilds/{guild_id}/commands",
                application_id = APPLICATION_ID,
                guild_id = GUILD_ID
            )
        );
    }

    #[test]
    fn test_create_guild_from_template() {
        let route = Route::CreateGuildFromTemplate {
            template_code: TEMPLATE_CODE,
        };
        assert_eq!(
            route.to_string(),
            format!(
                "guilds/templates/{template_code}",
                template_code = TEMPLATE_CODE
            )
        );
    }

    #[test]
    fn test_get_template() {
        let route = Route::GetTemplate {
            template_code: TEMPLATE_CODE,
        };
        assert_eq!(
            route.to_string(),
            format!(
                "guilds/templates/{template_code}",
                template_code = TEMPLATE_CODE
            )
        );
    }

    #[test]
    fn test_create_guild_integration() {
        let route = Route::CreateGuildIntegration { guild_id: GUILD_ID };
        assert_eq!(
            route.to_string(),
            format!("guilds/{guild_id}/integrations", guild_id = GUILD_ID)
        );
    }

    #[test]
    fn test_get_guild_integrations() {
        let route = Route::GetGuildIntegrations { guild_id: GUILD_ID };
        assert_eq!(
            route.to_string(),
            format!("guilds/{guild_id}/integrations", guild_id = GUILD_ID)
        );
    }

    #[test]
    fn test_create_guild_sticker() {
        let route = Route::CreateGuildSticker { guild_id: GUILD_ID };
        assert_eq!(
            route.to_string(),
            format!("guilds/{guild_id}/stickers", guild_id = GUILD_ID)
        );
    }

    #[test]
    fn test_get_guild_stickers() {
        let route = Route::GetGuildStickers { guild_id: GUILD_ID };
        assert_eq!(
            route.to_string(),
            format!("guilds/{guild_id}/stickers", guild_id = GUILD_ID)
        );
    }

    #[test]
    fn test_create_invite() {
        let route = Route::CreateInvite {
            channel_id: CHANNEL_ID,
        };
        assert_eq!(
            route.to_string(),
            format!("channels/{channel_id}/invites", channel_id = CHANNEL_ID)
        );
    }

    #[test]
    fn test_get_channel_invites() {
        let route = Route::GetChannelInvites {
            channel_id: CHANNEL_ID,
        };
        assert_eq!(
            route.to_string(),
            format!("channels/{channel_id}/invites", channel_id = CHANNEL_ID)
        );
    }

    #[test]
    fn test_create_message() {
        let route = Route::CreateMessage {
            channel_id: CHANNEL_ID,
        };
        assert_eq!(
            route.to_string(),
            format!("channels/{channel_id}/messages", channel_id = CHANNEL_ID)
        );
    }

    #[test]
    fn test_create_private_channel() {
        let route = Route::CreatePrivateChannel;
        assert_eq!(route.to_string(), "users/@me/channels");
    }

    #[test]
    fn test_get_user_private_channels() {
        let route = Route::GetUserPrivateChannels;
        assert_eq!(route.to_string(), "users/@me/channels");
    }

    #[test]
    fn test_create_reaction() {
        let emoji = emoji();

        let route = Route::CreateReaction {
            channel_id: CHANNEL_ID,
            emoji: &emoji,
            message_id: MESSAGE_ID,
        };
        assert_eq!(
            route.to_string(),
            format!(
                "channels/{channel_id}/messages/{message_id}/reactions/{emoji}/@me",
                channel_id = CHANNEL_ID,
                emoji = emoji,
                message_id = MESSAGE_ID
            )
        );
    }

    #[test]
    fn test_delete_reaction_current_user() {
        let emoji = emoji();

        let route = Route::DeleteReactionCurrentUser {
            channel_id: CHANNEL_ID,
            emoji: &emoji,
            message_id: MESSAGE_ID,
        };
        assert_eq!(
            route.to_string(),
            format!(
                "channels/{channel_id}/messages/{message_id}/reactions/{emoji}/@me",
                channel_id = CHANNEL_ID,
                emoji = emoji,
                message_id = MESSAGE_ID
            )
        );
    }

    #[test]
    fn test_create_role() {
        let route = Route::CreateRole { guild_id: GUILD_ID };
        assert_eq!(
            route.to_string(),
            format!("guilds/{guild_id}/roles", guild_id = GUILD_ID)
        );
    }

    #[test]
    fn test_get_guild_roles() {
        let route = Route::GetGuildRoles { guild_id: GUILD_ID };
        assert_eq!(
            route.to_string(),
            format!("guilds/{guild_id}/roles", guild_id = GUILD_ID)
        );
    }

    #[test]
    fn test_update_role_positions() {
        let route = Route::UpdateRolePositions { guild_id: GUILD_ID };
        assert_eq!(
            route.to_string(),
            format!("guilds/{guild_id}/roles", guild_id = GUILD_ID)
        );
    }

    #[test]
    fn test_create_stage_instance() {
        let route = Route::CreateStageInstance;
        assert_eq!(route.to_string(), "stage-instances");
    }

    #[test]
    fn test_create_template() {
        let route = Route::CreateTemplate { guild_id: GUILD_ID };
        assert_eq!(
            route.to_string(),
            format!("guilds/{guild_id}/templates", guild_id = GUILD_ID)
        );
    }

    #[test]
    fn test_get_templates() {
        let route = Route::GetTemplates { guild_id: GUILD_ID };
        assert_eq!(
            route.to_string(),
            format!("guilds/{guild_id}/templates", guild_id = GUILD_ID)
        );
    }

    #[test]
    fn test_create_thread() {
        let route = Route::CreateThread {
            channel_id: CHANNEL_ID,
        };
        assert_eq!(
            route.to_string(),
            format!("channels/{channel_id}/threads", channel_id = CHANNEL_ID)
        );
    }

    #[test]
    fn test_create_thread_from_message() {
        let route = Route::CreateThreadFromMessage {
            channel_id: CHANNEL_ID,
            message_id: MESSAGE_ID,
        };
        assert_eq!(
            route.to_string(),
            format!(
                "channels/{channel_id}/messages/{message_id}/threads",
                channel_id = CHANNEL_ID,
                message_id = MESSAGE_ID
            )
        );
    }

    #[test]
    fn test_create_typing_trigger() {
        let route = Route::CreateTypingTrigger {
            channel_id: CHANNEL_ID,
        };
        assert_eq!(
            route.to_string(),
            format!("channels/{channel_id}/typing", channel_id = CHANNEL_ID)
        );
    }

    #[test]
    fn test_create_webhook() {
        let route = Route::CreateWebhook {
            channel_id: CHANNEL_ID,
        };
        assert_eq!(
            route.to_string(),
            format!("channels/{channel_id}/webhooks", channel_id = CHANNEL_ID)
        );
    }

    #[test]
    fn test_get_channel_webhooks() {
        let route = Route::GetChannelWebhooks {
            channel_id: CHANNEL_ID,
        };
        assert_eq!(
            route.to_string(),
            format!("channels/{channel_id}/webhooks", channel_id = CHANNEL_ID)
        );
    }

    #[test]
    fn test_crosspost_message() {
        let route = Route::CrosspostMessage {
            channel_id: CHANNEL_ID,
            message_id: MESSAGE_ID,
        };
        assert_eq!(
            route.to_string(),
            format!(
                "channels/{channel_id}/messages/{message_id}/crosspost",
                channel_id = CHANNEL_ID,
                message_id = MESSAGE_ID
            )
        );
    }

    #[test]
    fn test_delete_ban() {
        let route = Route::DeleteBan {
            guild_id: GUILD_ID,
            user_id: USER_ID,
        };
        assert_eq!(
            route.to_string(),
            format!(
                "guilds/{guild_id}/bans/{user_id}",
                guild_id = GUILD_ID,
                user_id = USER_ID
            )
        );
    }

    #[test]
    fn test_get_ban() {
        let route = Route::GetBan {
            guild_id: GUILD_ID,
            user_id: USER_ID,
        };
        assert_eq!(
            route.to_string(),
            format!(
                "guilds/{guild_id}/bans/{user_id}",
                guild_id = GUILD_ID,
                user_id = USER_ID
            )
        );
    }

    #[test]
    fn test_delete_channel() {
        let route = Route::DeleteChannel {
            channel_id: CHANNEL_ID,
        };
        assert_eq!(
            route.to_string(),
            format!("channels/{channel_id}", channel_id = CHANNEL_ID)
        );
    }

    #[test]
    fn test_get_channel() {
        let route = Route::GetChannel {
            channel_id: CHANNEL_ID,
        };
        assert_eq!(
            route.to_string(),
            format!("channels/{channel_id}", channel_id = CHANNEL_ID)
        );
    }

    #[test]
    fn test_update_channel() {
        let route = Route::UpdateChannel {
            channel_id: CHANNEL_ID,
        };
        assert_eq!(
            route.to_string(),
            format!("channels/{channel_id}", channel_id = CHANNEL_ID)
        );
    }

    #[test]
    fn test_delete_emoji() {
        let route = Route::DeleteEmoji {
            emoji_id: EMOJI_ID,
            guild_id: GUILD_ID,
        };
        assert_eq!(
            route.to_string(),
            format!(
                "guilds/{guild_id}/emojis/{emoji_id}",
                emoji_id = EMOJI_ID,
                guild_id = GUILD_ID
            )
        );
    }

    #[test]
    fn test_get_emoji() {
        let route = Route::GetEmoji {
            emoji_id: EMOJI_ID,
            guild_id: GUILD_ID,
        };
        assert_eq!(
            route.to_string(),
            format!(
                "guilds/{guild_id}/emojis/{emoji_id}",
                emoji_id = EMOJI_ID,
                guild_id = GUILD_ID
            )
        );
    }

    #[test]
    fn test_update_emoji() {
        let route = Route::UpdateEmoji {
            emoji_id: EMOJI_ID,
            guild_id: GUILD_ID,
        };
        assert_eq!(
            route.to_string(),
            format!(
                "guilds/{guild_id}/emojis/{emoji_id}",
                emoji_id = EMOJI_ID,
                guild_id = GUILD_ID
            )
        );
    }

    #[test]
    fn test_delete_global_command() {
        let route = Route::DeleteGlobalCommand {
            application_id: APPLICATION_ID,
            command_id: COMMAND_ID,
        };
        assert_eq!(
            route.to_string(),
            format!(
                "applications/{application_id}/commands/{command_id}",
                application_id = APPLICATION_ID,
                command_id = COMMAND_ID
            )
        );
    }

    #[test]
    fn test_get_global_command() {
        let route = Route::GetGlobalCommand {
            application_id: APPLICATION_ID,
            command_id: COMMAND_ID,
        };
        assert_eq!(
            route.to_string(),
            format!(
                "applications/{application_id}/commands/{command_id}",
                application_id = APPLICATION_ID,
                command_id = COMMAND_ID
            )
        );
    }

    #[test]
    fn test_update_global_command() {
        let route = Route::UpdateGlobalCommand {
            application_id: APPLICATION_ID,
            command_id: COMMAND_ID,
        };
        assert_eq!(
            route.to_string(),
            format!(
                "applications/{application_id}/commands/{command_id}",
                application_id = APPLICATION_ID,
                command_id = COMMAND_ID
            )
        );
    }

    #[test]
    fn test_delete_guild() {
        let route = Route::DeleteGuild { guild_id: GUILD_ID };
        assert_eq!(
            route.to_string(),
            format!("guilds/{guild_id}", guild_id = GUILD_ID)
        );
    }

    #[test]
    fn test_update_guild() {
        let route = Route::UpdateGuild { guild_id: GUILD_ID };
        assert_eq!(
            route.to_string(),
            format!("guilds/{guild_id}", guild_id = GUILD_ID)
        );
    }

    #[test]
    fn test_delete_guild_command() {
        let route = Route::DeleteGuildCommand {
            application_id: APPLICATION_ID,
            command_id: COMMAND_ID,
            guild_id: GUILD_ID,
        };
        assert_eq!(
            route.to_string(),
            format!(
                "applications/{application_id}/guilds/{guild_id}/commands/{command_id}",
                application_id = APPLICATION_ID,
                command_id = COMMAND_ID,
                guild_id = GUILD_ID
            )
        );
    }

    #[test]
    fn test_get_guild_command() {
        let route = Route::GetGuildCommand {
            application_id: APPLICATION_ID,
            command_id: COMMAND_ID,
            guild_id: GUILD_ID,
        };
        assert_eq!(
            route.to_string(),
            format!(
                "applications/{application_id}/guilds/{guild_id}/commands/{command_id}",
                application_id = APPLICATION_ID,
                command_id = COMMAND_ID,
                guild_id = GUILD_ID
            )
        );
    }

    #[test]
    fn test_update_guild_command() {
        let route = Route::UpdateGuildCommand {
            application_id: APPLICATION_ID,
            command_id: COMMAND_ID,
            guild_id: GUILD_ID,
        };
        assert_eq!(
            route.to_string(),
            format!(
                "applications/{application_id}/guilds/{guild_id}/commands/{command_id}",
                application_id = APPLICATION_ID,
                command_id = COMMAND_ID,
                guild_id = GUILD_ID
            )
        );
    }

    #[test]
    fn test_delete_guild_integration() {
        let route = Route::DeleteGuildIntegration {
            guild_id: GUILD_ID,
            integration_id: INTEGRATION_ID,
        };
        assert_eq!(
            route.to_string(),
            format!(
                "guilds/{guild_id}/integrations/{integration_id}",
                guild_id = GUILD_ID,
                integration_id = INTEGRATION_ID
            )
        );
    }

    #[test]
    fn test_update_guild_integration() {
        let route = Route::UpdateGuildIntegration {
            guild_id: GUILD_ID,
            integration_id: INTEGRATION_ID,
        };
        assert_eq!(
            route.to_string(),
            format!(
                "guilds/{guild_id}/integrations/{integration_id}",
                guild_id = GUILD_ID,
                integration_id = INTEGRATION_ID
            )
        );
    }

    #[test]
    fn test_delete_interaction_original() {
        let route = Route::DeleteInteractionOriginal {
            application_id: APPLICATION_ID,
            interaction_token: INTERACTION_TOKEN,
        };
        assert_eq!(
            route.to_string(),
            format!(
                "webhooks/{application_id}/{interaction_token}/messages/@original",
                application_id = APPLICATION_ID,
                interaction_token = INTERACTION_TOKEN
            )
        );
    }

    #[test]
    fn test_get_interaction_original() {
        let route = Route::GetInteractionOriginal {
            application_id: APPLICATION_ID,
            interaction_token: INTERACTION_TOKEN,
        };
        assert_eq!(
            route.to_string(),
            format!(
                "webhooks/{application_id}/{interaction_token}/messages/@original",
                application_id = APPLICATION_ID,
                interaction_token = INTERACTION_TOKEN
            )
        );
    }

    #[test]
    fn test_update_interaction_original() {
        let route = Route::UpdateInteractionOriginal {
            application_id: APPLICATION_ID,
            interaction_token: INTERACTION_TOKEN,
        };
        assert_eq!(
            route.to_string(),
            format!(
                "webhooks/{application_id}/{interaction_token}/messages/@original",
                application_id = APPLICATION_ID,
                interaction_token = INTERACTION_TOKEN
            )
        );
    }

    #[test]
    fn test_delete_invite() {
        let route = Route::DeleteInvite { code: CODE };
        assert_eq!(route.to_string(), format!("invites/{code}", code = CODE));
    }

    #[test]
    fn test_delete_message_reactions() {
        let route = Route::DeleteMessageReactions {
            channel_id: CHANNEL_ID,
            message_id: MESSAGE_ID,
        };
        assert_eq!(
            route.to_string(),
            format!(
                "channels/{channel_id}/messages/{message_id}/reactions",
                channel_id = CHANNEL_ID,
                message_id = MESSAGE_ID
            )
        );
    }

    #[test]
    fn test_delete_message_specific_reaction() {
        let emoji = emoji();

        let route = Route::DeleteMessageSpecificReaction {
            channel_id: CHANNEL_ID,
            message_id: MESSAGE_ID,
            emoji: &emoji,
        };
        assert_eq!(
            route.to_string(),
            format!(
                "channels/{channel_id}/messages/{message_id}/reactions/{emoji}",
                channel_id = CHANNEL_ID,
                emoji = emoji,
                message_id = MESSAGE_ID
            )
        );
    }

    #[test]
    fn test_delete_message() {
        let route = Route::DeleteMessage {
            channel_id: CHANNEL_ID,
            message_id: MESSAGE_ID,
        };
        assert_eq!(
            route.to_string(),
            format!(
                "channels/{channel_id}/messages/{message_id}",
                channel_id = CHANNEL_ID,
                message_id = MESSAGE_ID
            )
        );
    }

    #[test]
    fn test_get_message() {
        let route = Route::GetMessage {
            channel_id: CHANNEL_ID,
            message_id: MESSAGE_ID,
        };
        assert_eq!(
            route.to_string(),
            format!(
                "channels/{channel_id}/messages/{message_id}",
                channel_id = CHANNEL_ID,
                message_id = MESSAGE_ID
            )
        );
    }

    #[test]
    fn test_update_message() {
        let route = Route::UpdateMessage {
            channel_id: CHANNEL_ID,
            message_id: MESSAGE_ID,
        };
        assert_eq!(
            route.to_string(),
            format!(
                "channels/{channel_id}/messages/{message_id}",
                channel_id = CHANNEL_ID,
                message_id = MESSAGE_ID
            )
        );
    }

    #[test]
    fn test_delete_messages() {
        let route = Route::DeleteMessages {
            channel_id: CHANNEL_ID,
        };
        assert_eq!(
            route.to_string(),
            format!(
                "channels/{channel_id}/messages/bulk-delete",
                channel_id = CHANNEL_ID
            )
        );
    }

    #[test]
    fn test_delete_permission_overwrite() {
        let route = Route::DeletePermissionOverwrite {
            channel_id: CHANNEL_ID,
            target_id: ROLE_ID,
        };
        assert_eq!(
            route.to_string(),
            format!(
                "channels/{channel_id}/permissions/{target_id}",
                channel_id = CHANNEL_ID,
                target_id = ROLE_ID
            )
        );
    }

    #[test]
    fn test_update_permission_overwrite() {
        let route = Route::UpdatePermissionOverwrite {
            channel_id: CHANNEL_ID,
            target_id: USER_ID,
        };
        assert_eq!(
            route.to_string(),
            format!(
                "channels/{channel_id}/permissions/{target_id}",
                channel_id = CHANNEL_ID,
                target_id = USER_ID
            )
        );
    }

    #[test]
    fn test_delete_reaction() {
        let emoji = emoji();

        let route = Route::DeleteReaction {
            channel_id: CHANNEL_ID,
            emoji: &emoji,
            message_id: MESSAGE_ID,
            user_id: USER_ID,
        };
        assert_eq!(
            route.to_string(),
            format!(
                "channels/{channel_id}/messages/{message_id}/reactions/{emoji}/{user_id}",
                channel_id = CHANNEL_ID,
                emoji = emoji,
                message_id = MESSAGE_ID,
                user_id = USER_ID
            )
        );
    }

    #[test]
    fn test_delete_role() {
        let route = Route::DeleteRole {
            guild_id: GUILD_ID,
            role_id: ROLE_ID,
        };
        assert_eq!(
            route.to_string(),
            format!(
                "guilds/{guild_id}/roles/{role_id}",
                guild_id = GUILD_ID,
                role_id = ROLE_ID
            )
        );
    }

    #[test]
    fn test_update_role() {
        let route = Route::UpdateRole {
            guild_id: GUILD_ID,
            role_id: ROLE_ID,
        };
        assert_eq!(
            route.to_string(),
            format!(
                "guilds/{guild_id}/roles/{role_id}",
                guild_id = GUILD_ID,
                role_id = ROLE_ID
            )
        );
    }

    #[test]
    fn test_delete_stage_instance() {
        let route = Route::DeleteStageInstance {
            channel_id: CHANNEL_ID,
        };
        assert_eq!(
            route.to_string(),
            format!("stage-instances/{channel_id}", channel_id = CHANNEL_ID)
        );
    }

    #[test]
    fn test_get_stage_instance() {
        let route = Route::GetStageInstance {
            channel_id: CHANNEL_ID,
        };
        assert_eq!(
            route.to_string(),
            format!("stage-instances/{channel_id}", channel_id = CHANNEL_ID)
        );
    }

    #[test]
    fn test_update_stage_instance() {
        let route = Route::UpdateStageInstance {
            channel_id: CHANNEL_ID,
        };
        assert_eq!(
            route.to_string(),
            format!("stage-instances/{channel_id}", channel_id = CHANNEL_ID)
        );
    }

    #[test]
    fn test_delete_template() {
        let route = Route::DeleteTemplate {
            guild_id: GUILD_ID,
            template_code: TEMPLATE_CODE,
        };
        assert_eq!(
            route.to_string(),
            format!(
                "guilds/{guild_id}/templates/{template_code}",
                guild_id = GUILD_ID,
                template_code = TEMPLATE_CODE
            )
        );
    }

    #[test]
    fn test_sync_template() {
        let route = Route::SyncTemplate {
            guild_id: GUILD_ID,
            template_code: TEMPLATE_CODE,
        };
        assert_eq!(
            route.to_string(),
            format!(
                "guilds/{guild_id}/templates/{template_code}",
                guild_id = GUILD_ID,
                template_code = TEMPLATE_CODE
            )
        );
    }

    #[test]
    fn test_update_template() {
        let route = Route::UpdateTemplate {
            guild_id: GUILD_ID,
            template_code: TEMPLATE_CODE,
        };
        assert_eq!(
            route.to_string(),
            format!(
                "guilds/{guild_id}/templates/{template_code}",
                guild_id = GUILD_ID,
                template_code = TEMPLATE_CODE
            )
        );
    }

    #[test]
    fn test_follow_news_channel() {
        let route = Route::FollowNewsChannel {
            channel_id: CHANNEL_ID,
        };
        assert_eq!(
            route.to_string(),
            format!("channels/{channel_id}/followers", channel_id = CHANNEL_ID)
        );
    }

    #[test]
    fn test_get_active_threads() {
        let route = Route::GetActiveThreads { guild_id: GUILD_ID };
        assert_eq!(
            route.to_string(),
            format!("guilds/{guild_id}/threads/active", guild_id = GUILD_ID)
        );
    }

    #[test]
    fn test_get_bans() {
        let route = Route::GetBans { guild_id: GUILD_ID };
        assert_eq!(
            route.to_string(),
            format!("guilds/{guild_id}/bans", guild_id = GUILD_ID)
        );
    }

    #[test]
    fn test_get_gateway_bot() {
        let route = Route::GetGatewayBot;
        assert_eq!(route.to_string(), "gateway/bot");
    }

    #[test]
    fn test_get_command_permissions() {
        let route = Route::GetCommandPermissions {
            application_id: APPLICATION_ID,
            command_id: COMMAND_ID,
            guild_id: GUILD_ID,
        };
        assert_eq!(
            route.to_string(),
            format!(
                "applications/{application_id}/guilds/{guild_id}/commands/{command_id}/permissions",
                application_id = APPLICATION_ID,
                command_id = COMMAND_ID,
                guild_id = GUILD_ID
            )
        );
    }

    #[test]
    fn test_update_command_permissions() {
        let route = Route::UpdateCommandPermissions {
            application_id: APPLICATION_ID,
            command_id: COMMAND_ID,
            guild_id: GUILD_ID,
        };
        assert_eq!(
            route.to_string(),
            format!(
                "applications/{application_id}/guilds/{guild_id}/commands/{command_id}/permissions",
                application_id = APPLICATION_ID,
                command_id = COMMAND_ID,
                guild_id = GUILD_ID
            )
        );
    }

    #[test]
    fn test_get_current_user_application_info() {
        let route = Route::GetCurrentUserApplicationInfo;
        assert_eq!(route.to_string(), "oauth2/applications/@me");
    }

    #[test]
    fn test_get_current_user() {
        let route = Route::GetCurrentUser;
        assert_eq!(route.to_string(), "users/@me");
    }

    #[test]
    fn test_get_current_user_guild_member() {
        let route = Route::GetCurrentUserGuildMember { guild_id: GUILD_ID };
        assert_eq!(
            route.to_string(),
            format!("users/@me/guilds/{guild_id}/member", guild_id = GUILD_ID)
        )
    }

    #[test]
    fn test_update_current_user() {
        let route = Route::UpdateCurrentUser;
        assert_eq!(route.to_string(), "users/@me");
    }

    #[test]
    fn test_get_gateway() {
        let route = Route::GetGateway;
        assert_eq!(route.to_string(), "gateway");
    }

    #[test]
    fn test_get_guild_command_permissions() {
        let route = Route::GetGuildCommandPermissions {
            application_id: APPLICATION_ID,
            guild_id: GUILD_ID,
        };
        assert_eq!(
            route.to_string(),
            format!(
                "applications/{application_id}/guilds/{guild_id}/commands/permissions",
                application_id = APPLICATION_ID,
                guild_id = GUILD_ID
            )
        );
    }

    #[test]
    fn test_set_command_permissions() {
        let route = Route::SetCommandPermissions {
            application_id: APPLICATION_ID,
            guild_id: GUILD_ID,
        };
        assert_eq!(
            route.to_string(),
            format!(
                "applications/{application_id}/guilds/{guild_id}/commands/permissions",
                application_id = APPLICATION_ID,
                guild_id = GUILD_ID
            )
        );
    }

    #[test]
    fn test_get_guild_invites() {
        let route = Route::GetGuildInvites { guild_id: GUILD_ID };
        assert_eq!(
            route.to_string(),
            format!("guilds/{guild_id}/invites", guild_id = GUILD_ID)
        );
    }

    #[test]
    fn test_get_guild_preview() {
        let route = Route::GetGuildPreview { guild_id: GUILD_ID };
        assert_eq!(
            route.to_string(),
            format!("guilds/{guild_id}/preview", guild_id = GUILD_ID)
        );
    }

    #[test]
    fn test_get_guild_sticker() {
        let route = Route::GetGuildSticker {
            guild_id: GUILD_ID,
            sticker_id: STICKER_ID,
        };
        assert_eq!(
            route.to_string(),
            format!(
                "guilds/{guild_id}/stickers/{sticker_id}",
                guild_id = GUILD_ID,
                sticker_id = STICKER_ID
            )
        );
    }

    #[test]
    fn test_delete_guild_sticker() {
        let route = Route::DeleteGuildSticker {
            guild_id: GUILD_ID,
            sticker_id: STICKER_ID,
        };
        assert_eq!(
            route.to_string(),
            format!(
                "guilds/{guild_id}/stickers/{sticker_id}",
                guild_id = GUILD_ID,
                sticker_id = STICKER_ID
            )
        );
    }

    #[test]
    fn test_update_guild_sticker() {
        let route = Route::UpdateGuildSticker {
            guild_id: GUILD_ID,
            sticker_id: STICKER_ID,
        };
        assert_eq!(
            route.to_string(),
            format!(
                "guilds/{guild_id}/stickers/{sticker_id}",
                guild_id = GUILD_ID,
                sticker_id = STICKER_ID
            )
        );
    }

    #[test]
    fn test_get_guild_vanity_url() {
        let route = Route::GetGuildVanityUrl { guild_id: GUILD_ID };
        assert_eq!(
            route.to_string(),
            format!("guilds/{guild_id}/vanity-url", guild_id = GUILD_ID)
        );
    }

    #[test]
    fn test_get_guild_voice_regions() {
        let route = Route::GetGuildVoiceRegions { guild_id: GUILD_ID };
        assert_eq!(
            route.to_string(),
            format!("guilds/{guild_id}/regions", guild_id = GUILD_ID)
        );
    }

    #[test]
    fn test_get_guild_welcome_screen() {
        let route = Route::GetGuildWelcomeScreen { guild_id: GUILD_ID };
        assert_eq!(
            route.to_string(),
            format!("guilds/{guild_id}/welcome-screen", guild_id = GUILD_ID)
        );
    }

    #[test]
    fn test_update_guild_welcome_screen() {
        let route = Route::UpdateGuildWelcomeScreen { guild_id: GUILD_ID };
        assert_eq!(
            route.to_string(),
            format!("guilds/{guild_id}/welcome-screen", guild_id = GUILD_ID)
        );
    }

    #[test]
    fn test_get_guild_webhooks() {
        let route = Route::GetGuildWebhooks { guild_id: GUILD_ID };
        assert_eq!(
            route.to_string(),
            format!("guilds/{guild_id}/webhooks", guild_id = GUILD_ID)
        );
    }

    #[test]
    fn test_get_guild_widget() {
        let route = Route::GetGuildWidget { guild_id: GUILD_ID };
        assert_eq!(
            route.to_string(),
            format!("guilds/{guild_id}/widget", guild_id = GUILD_ID)
        );
    }

    #[test]
    fn test_update_guild_widget() {
        let route = Route::UpdateGuildWidget { guild_id: GUILD_ID };
        assert_eq!(
            route.to_string(),
            format!("guilds/{guild_id}/widget", guild_id = GUILD_ID)
        );
    }

    #[test]
    fn test_get_nitro_sticker_packs() {
        let route = Route::GetNitroStickerPacks;

        assert_eq!(route.to_string(), "sticker-packs");
    }

    #[test]
    fn test_get_pins() {
        let route = Route::GetPins {
            channel_id: CHANNEL_ID,
        };
        assert_eq!(
            route.to_string(),
            format!("channels/{channel_id}/pins", channel_id = CHANNEL_ID)
        );
    }

    #[test]
    fn test_get_sticker() {
        let route = Route::GetSticker {
            sticker_id: STICKER_ID,
        };
        assert_eq!(
            route.to_string(),
            format!("stickers/{sticker_id}", sticker_id = STICKER_ID)
        );
    }

    #[test]
    fn test_get_thread_members() {
        let route = Route::GetThreadMembers {
            channel_id: CHANNEL_ID,
        };
        assert_eq!(
            route.to_string(),
            format!(
                "channels/{channel_id}/thread-members",
                channel_id = CHANNEL_ID
            )
        );
    }

    #[test]
    fn test_get_user_connections() {
        let route = Route::GetUserConnections;
        assert_eq!(route.to_string(), "users/@me/connections");
    }

    #[test]
    fn test_get_user() {
        let route = Route::GetUser { user_id: USER_ID };
        assert_eq!(
            route.to_string(),
            format!("users/{user_id}", user_id = USER_ID)
        );
    }

    #[test]
    fn test_get_voice_regions() {
        let route = Route::GetVoiceRegions;
        assert_eq!(route.to_string(), "voice/regions");
    }

    #[test]
    fn test_interaction_callback() {
        let route = Route::InteractionCallback {
            interaction_id: INTERACTION_ID,
            interaction_token: INTERACTION_TOKEN,
        };
        assert_eq!(
            route.to_string(),
            format!(
                "interactions/{interaction_id}/{interaction_token}/callback",
                interaction_id = INTERACTION_ID,
                interaction_token = INTERACTION_TOKEN
            )
        );
    }

    #[test]
    fn test_join_thread() {
        let route = Route::JoinThread {
            channel_id: CHANNEL_ID,
        };
        assert_eq!(
            route.to_string(),
            format!(
                "channels/{channel_id}/thread-members/@me",
                channel_id = CHANNEL_ID
            )
        );
    }

    #[test]
    fn test_leave_thread() {
        let route = Route::LeaveThread {
            channel_id: CHANNEL_ID,
        };
        assert_eq!(
            route.to_string(),
            format!(
                "channels/{channel_id}/thread-members/@me",
                channel_id = CHANNEL_ID
            )
        );
    }

    #[test]
    fn test_leave_guild() {
        let route = Route::LeaveGuild { guild_id: GUILD_ID };
        assert_eq!(
            route.to_string(),
            format!("users/@me/guilds/{guild_id}", guild_id = GUILD_ID)
        );
    }

    #[test]
    fn test_pin_message() {
        let route = Route::PinMessage {
            channel_id: CHANNEL_ID,
            message_id: MESSAGE_ID,
        };
        assert_eq!(
            route.to_string(),
            format!(
                "channels/{channel_id}/pins/{message_id}",
                channel_id = CHANNEL_ID,
                message_id = MESSAGE_ID
            )
        );
    }

    #[test]
    fn test_unpin_message() {
        let route = Route::UnpinMessage {
            channel_id: CHANNEL_ID,
            message_id: MESSAGE_ID,
        };
        assert_eq!(
            route.to_string(),
            format!(
                "channels/{channel_id}/pins/{message_id}",
                channel_id = CHANNEL_ID,
                message_id = MESSAGE_ID
            )
        );
    }

    #[test]
    fn test_sync_guild_integration() {
        let route = Route::SyncGuildIntegration {
            guild_id: GUILD_ID,
            integration_id: INTEGRATION_ID,
        };
        assert_eq!(
            route.to_string(),
            format!(
                "guilds/{guild_id}/integrations/{integration_id}/sync",
                guild_id = GUILD_ID,
                integration_id = INTEGRATION_ID
            )
        );
    }

    #[test]
    fn test_update_current_member() {
        let route = Route::UpdateCurrentMember { guild_id: GUILD_ID };
        assert_eq!(
            route.to_string(),
            format!("guilds/{guild_id}/members/@me", guild_id = GUILD_ID)
        );
    }

    #[test]
    fn test_update_current_user_voice_state() {
        let route = Route::UpdateCurrentUserVoiceState { guild_id: GUILD_ID };
        assert_eq!(
            route.to_string(),
            format!("guilds/{guild_id}/voice-states/@me", guild_id = GUILD_ID)
        );
    }

    #[test]
    fn test_update_nickname() {
        let route = Route::UpdateNickname { guild_id: GUILD_ID };
        assert_eq!(
            route.to_string(),
            format!("guilds/{guild_id}/members/@me/nick", guild_id = GUILD_ID)
        );
    }

    #[test]
    fn test_update_user_voice_state() {
        let route = Route::UpdateUserVoiceState {
            guild_id: GUILD_ID,
            user_id: USER_ID,
        };
        assert_eq!(
            route.to_string(),
            format!(
                "guilds/{guild_id}/voice-states/{user_id}",
                guild_id = GUILD_ID,
                user_id = USER_ID
            )
        );
    }

    #[test]
    fn test_create_ban() {
        let mut route = Route::CreateBan {
            guild_id: GUILD_ID,
            delete_message_days: None,
            reason: None,
            user_id: USER_ID,
        };
        assert_eq!(
            route.to_string(),
            format!(
                "guilds/{guild_id}/bans/{user_id}?",
                guild_id = GUILD_ID,
                user_id = USER_ID
            )
        );

        route = Route::CreateBan {
            guild_id: GUILD_ID,
            delete_message_days: Some(3),
            reason: None,
            user_id: USER_ID,
        };
        assert_eq!(
            route.to_string(),
            format!(
                "guilds/{guild_id}/bans/{user_id}?delete_message_days=3",
                guild_id = GUILD_ID,
                user_id = USER_ID
            )
        );

        route = Route::CreateBan {
            guild_id: GUILD_ID,
            delete_message_days: None,
            reason: Some("test"),
            user_id: USER_ID,
        };
        assert_eq!(
            route.to_string(),
            format!(
                "guilds/{guild_id}/bans/{user_id}?reason=test",
                guild_id = GUILD_ID,
                user_id = USER_ID
            )
        );

        route = Route::CreateBan {
            guild_id: GUILD_ID,
            delete_message_days: Some(3),
            reason: Some("test"),
            user_id: USER_ID,
        };
        assert_eq!(
            route.to_string(),
            format!(
                "guilds/{guild_id}/bans/{user_id}?delete_message_days=3&reason=test",
                guild_id = GUILD_ID,
                user_id = USER_ID
            )
        );
    }

    #[test]
    fn test_create_guild_prune_none() {
        let route = Route::CreateGuildPrune {
            compute_prune_count: None,
            days: None,
            guild_id: GUILD_ID,
            include_roles: &[],
        };
        assert_eq!(
            route.to_string(),
            format!("guilds/{guild_id}/prune?", guild_id = GUILD_ID)
        );
    }

    #[test]
    fn test_create_guild_prune_compute_prune_count_true() {
        let route = Route::CreateGuildPrune {
            compute_prune_count: Some(true),
            days: None,
            guild_id: GUILD_ID,
            include_roles: &[],
        };
        assert_eq!(
            route.to_string(),
            format!(
                "guilds/{guild_id}/prune?compute_prune_count=true",
                guild_id = GUILD_ID
            )
        );
    }

    #[test]
    fn test_create_guild_prune_compute_prune_count_false() {
        let route = Route::CreateGuildPrune {
            compute_prune_count: Some(false),
            days: None,
            guild_id: GUILD_ID,
            include_roles: &[],
        };
        assert_eq!(
            route.to_string(),
            format!(
                "guilds/{guild_id}/prune?compute_prune_count=false",
                guild_id = GUILD_ID
            )
        );
    }

    #[test]
    fn test_create_guild_prune_days() {
        let route = Route::CreateGuildPrune {
            compute_prune_count: None,
            days: Some(4),
            guild_id: GUILD_ID,
            include_roles: &[],
        };
        assert_eq!(
            route.to_string(),
            format!("guilds/{guild_id}/prune?&days=4", guild_id = GUILD_ID)
        );
    }

    #[test]
    fn test_create_guild_prune_include_one_role() {
        let include_roles = [Id::new(1)];

        let route = Route::CreateGuildPrune {
            compute_prune_count: None,
            days: None,
            guild_id: GUILD_ID,
            include_roles: &include_roles,
        };
        assert_eq!(
            route.to_string(),
            format!(
                "guilds/{guild_id}/prune?&include_roles=1",
                guild_id = GUILD_ID
            )
        );
    }

    #[test]
    fn test_create_guild_prune_include_two_roles() {
        let include_roles = [Id::new(1), Id::new(2)];

        let route = Route::CreateGuildPrune {
            compute_prune_count: None,
            days: None,
            guild_id: GUILD_ID,
            include_roles: &include_roles,
        };
        assert_eq!(
            route.to_string(),
            format!(
                "guilds/{guild_id}/prune?&include_roles=1,2",
                guild_id = GUILD_ID
            )
        );
    }

    #[test]
    fn test_create_guild_prune_all() {
        let include_roles = [Id::new(1), Id::new(2)];

        let route = Route::CreateGuildPrune {
            compute_prune_count: Some(true),
            days: Some(4),
            guild_id: GUILD_ID,
            include_roles: &include_roles,
        };
        assert_eq!(
            route.to_string(),
            format!(
                "guilds/{guild_id}/prune?compute_prune_count=true&days=4&include_roles=1,2",
                guild_id = GUILD_ID
            )
        );
    }
}
