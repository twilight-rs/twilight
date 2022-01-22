use super::route::Route;
use percent_encoding::{utf8_percent_encode, NON_ALPHANUMERIC};
use std::fmt::{Display, Formatter, Result as FmtResult};

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct RouteDisplay<'a>(&'a Route<'a>);

impl<'a> RouteDisplay<'a> {
    /// Create a display formatter for a route.
    ///
    /// This is equivalent to [`Route::display`].
    pub(super) const fn new(route: &'a Route<'a>) -> Self {
        Self(route)
    }

    /// Immutable reference to the underlying route.
    ///
    /// ```
    /// use twilight_http::routing::Route;
    ///
    /// let route = Route::GetMessage {
    ///     channel_id: 123,
    ///     message_id: 456,
    /// };
    /// let display = route.display();
    ///
    /// assert_eq!(display.route_ref(), &route);
    /// ```
    pub const fn route_ref(&self) -> &'a Route<'a> {
        self.0
    }
}

impl Display for RouteDisplay<'_> {
    // Notably, we don't use macros like `write!` or `format_args!` due to them
    // both compiling slowly and performing slowly during runtime.
    //
    // See:
    // <https://github.com/rust-lang/rust/issues/76490>
    // <https://github.com/rust-lang/rust/issues/10761>
    #[allow(clippy::too_many_lines)]
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self.0 {
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
                Display::fmt(&emoji.display(), f)?;

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

                Display::fmt(&emoji.display(), f)
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
                Display::fmt(&emoji.display(), f)?;
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
            Route::GetGuildWidgetSettings { guild_id } | Route::UpdateGuildWidgetSettings { guild_id } => {
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
                Display::fmt(&emoji.display(), f)?;
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
    use crate::request::channel::reaction::RequestReactionType;

    use super::{super::Route, RouteDisplay};
    use static_assertions::assert_impl_all;
    use std::{
        fmt::{Debug, Display},
        hash::Hash,
    };
    use twilight_model::id::{EmojiId, RoleId};

    assert_impl_all!(RouteDisplay<'_>: Clone, Debug, Display, Eq, Hash, PartialEq, Send, Sync);

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

    fn emoji() -> RequestReactionType<'static> {
        RequestReactionType::Custom {
            id: EmojiId::new(EMOJI_ID).expect("non zero id"),
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
            route.display().to_string()
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

        assert_eq!(
            "webhooks/3/token/messages/1?thread_id=2",
            route.display().to_string()
        )
    }

    #[test]
    fn test_add_guild_member() {
        let route = Route::AddGuildMember {
            guild_id: GUILD_ID,
            user_id: USER_ID,
        };
        assert_eq!(
            route.display().to_string(),
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
            route.display().to_string(),
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
            route.display().to_string(),
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
            route.display().to_string(),
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
            route.display().to_string(),
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
            route.display().to_string(),
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
            route.display().to_string(),
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
            route.display().to_string(),
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
            route.display().to_string(),
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
            route.display().to_string(),
            format!("guilds/{guild_id}/channels", guild_id = GUILD_ID)
        );
    }

    #[test]
    fn test_get_channels() {
        let route = Route::GetChannels { guild_id: GUILD_ID };
        assert_eq!(
            route.display().to_string(),
            format!("guilds/{guild_id}/channels", guild_id = GUILD_ID)
        );
    }

    #[test]
    fn test_update_guild_channels() {
        let route = Route::UpdateGuildChannels { guild_id: GUILD_ID };
        assert_eq!(
            route.display().to_string(),
            format!("guilds/{guild_id}/channels", guild_id = GUILD_ID)
        );
    }

    #[test]
    fn test_create_emoji() {
        let route = Route::CreateEmoji { guild_id: GUILD_ID };
        assert_eq!(
            route.display().to_string(),
            format!("guilds/{guild_id}/emojis", guild_id = GUILD_ID)
        );
    }

    #[test]
    fn test_get_emojis() {
        let route = Route::GetEmojis { guild_id: GUILD_ID };
        assert_eq!(
            route.display().to_string(),
            format!("guilds/{guild_id}/emojis", guild_id = GUILD_ID)
        );
    }

    #[test]
    fn test_create_global_command() {
        let route = Route::CreateGlobalCommand {
            application_id: APPLICATION_ID,
        };
        assert_eq!(
            route.display().to_string(),
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
            route.display().to_string(),
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
            route.display().to_string(),
            format!(
                "applications/{application_id}/commands",
                application_id = APPLICATION_ID
            )
        );
    }

    #[test]
    fn test_create_guild() {
        let route = Route::CreateGuild;
        assert_eq!(route.display().to_string(), "guilds");
    }

    #[test]
    fn test_create_guild_command() {
        let route = Route::CreateGuildCommand {
            application_id: APPLICATION_ID,
            guild_id: GUILD_ID,
        };
        assert_eq!(
            route.display().to_string(),
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
            route.display().to_string(),
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
            route.display().to_string(),
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
            route.display().to_string(),
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
            route.display().to_string(),
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
            route.display().to_string(),
            format!("guilds/{guild_id}/integrations", guild_id = GUILD_ID)
        );
    }

    #[test]
    fn test_get_guild_integrations() {
        let route = Route::GetGuildIntegrations { guild_id: GUILD_ID };
        assert_eq!(
            route.display().to_string(),
            format!("guilds/{guild_id}/integrations", guild_id = GUILD_ID)
        );
    }

    #[test]
    fn test_create_guild_sticker() {
        let route = Route::CreateGuildSticker { guild_id: GUILD_ID };
        assert_eq!(
            route.display().to_string(),
            format!("guilds/{guild_id}/stickers", guild_id = GUILD_ID)
        );
    }

    #[test]
    fn test_get_guild_stickers() {
        let route = Route::GetGuildStickers { guild_id: GUILD_ID };
        assert_eq!(
            route.display().to_string(),
            format!("guilds/{guild_id}/stickers", guild_id = GUILD_ID)
        );
    }

    #[test]
    fn test_create_invite() {
        let route = Route::CreateInvite {
            channel_id: CHANNEL_ID,
        };
        assert_eq!(
            route.display().to_string(),
            format!("channels/{channel_id}/invites", channel_id = CHANNEL_ID)
        );
    }

    #[test]
    fn test_get_channel_invites() {
        let route = Route::GetChannelInvites {
            channel_id: CHANNEL_ID,
        };
        assert_eq!(
            route.display().to_string(),
            format!("channels/{channel_id}/invites", channel_id = CHANNEL_ID)
        );
    }

    #[test]
    fn test_create_message() {
        let route = Route::CreateMessage {
            channel_id: CHANNEL_ID,
        };
        assert_eq!(
            route.display().to_string(),
            format!("channels/{channel_id}/messages", channel_id = CHANNEL_ID)
        );
    }

    #[test]
    fn test_create_private_channel() {
        let route = Route::CreatePrivateChannel;
        assert_eq!(route.display().to_string(), "users/@me/channels");
    }

    #[test]
    fn test_get_user_private_channels() {
        let route = Route::GetUserPrivateChannels;
        assert_eq!(route.display().to_string(), "users/@me/channels");
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
            route.display().to_string(),
            format!(
                "channels/{channel_id}/messages/{message_id}/reactions/{emoji}/@me",
                channel_id = CHANNEL_ID,
                emoji = emoji.display(),
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
            route.display().to_string(),
            format!(
                "channels/{channel_id}/messages/{message_id}/reactions/{emoji}/@me",
                channel_id = CHANNEL_ID,
                emoji = emoji.display(),
                message_id = MESSAGE_ID
            )
        );
    }

    #[test]
    fn test_create_role() {
        let route = Route::CreateRole { guild_id: GUILD_ID };
        assert_eq!(
            route.display().to_string(),
            format!("guilds/{guild_id}/roles", guild_id = GUILD_ID)
        );
    }

    #[test]
    fn test_get_guild_roles() {
        let route = Route::GetGuildRoles { guild_id: GUILD_ID };
        assert_eq!(
            route.display().to_string(),
            format!("guilds/{guild_id}/roles", guild_id = GUILD_ID)
        );
    }

    #[test]
    fn test_update_role_positions() {
        let route = Route::UpdateRolePositions { guild_id: GUILD_ID };
        assert_eq!(
            route.display().to_string(),
            format!("guilds/{guild_id}/roles", guild_id = GUILD_ID)
        );
    }

    #[test]
    fn test_create_stage_instance() {
        let route = Route::CreateStageInstance;
        assert_eq!(route.display().to_string(), "stage-instances");
    }

    #[test]
    fn test_create_template() {
        let route = Route::CreateTemplate { guild_id: GUILD_ID };
        assert_eq!(
            route.display().to_string(),
            format!("guilds/{guild_id}/templates", guild_id = GUILD_ID)
        );
    }

    #[test]
    fn test_get_templates() {
        let route = Route::GetTemplates { guild_id: GUILD_ID };
        assert_eq!(
            route.display().to_string(),
            format!("guilds/{guild_id}/templates", guild_id = GUILD_ID)
        );
    }

    #[test]
    fn test_create_thread() {
        let route = Route::CreateThread {
            channel_id: CHANNEL_ID,
        };
        assert_eq!(
            route.display().to_string(),
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
            route.display().to_string(),
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
            route.display().to_string(),
            format!("channels/{channel_id}/typing", channel_id = CHANNEL_ID)
        );
    }

    #[test]
    fn test_create_webhook() {
        let route = Route::CreateWebhook {
            channel_id: CHANNEL_ID,
        };
        assert_eq!(
            route.display().to_string(),
            format!("channels/{channel_id}/webhooks", channel_id = CHANNEL_ID)
        );
    }

    #[test]
    fn test_get_channel_webhooks() {
        let route = Route::GetChannelWebhooks {
            channel_id: CHANNEL_ID,
        };
        assert_eq!(
            route.display().to_string(),
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
            route.display().to_string(),
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
            route.display().to_string(),
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
            route.display().to_string(),
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
            route.display().to_string(),
            format!("channels/{channel_id}", channel_id = CHANNEL_ID)
        );
    }

    #[test]
    fn test_get_channel() {
        let route = Route::GetChannel {
            channel_id: CHANNEL_ID,
        };
        assert_eq!(
            route.display().to_string(),
            format!("channels/{channel_id}", channel_id = CHANNEL_ID)
        );
    }

    #[test]
    fn test_update_channel() {
        let route = Route::UpdateChannel {
            channel_id: CHANNEL_ID,
        };
        assert_eq!(
            route.display().to_string(),
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
            route.display().to_string(),
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
            route.display().to_string(),
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
            route.display().to_string(),
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
            route.display().to_string(),
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
            route.display().to_string(),
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
            route.display().to_string(),
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
            route.display().to_string(),
            format!("guilds/{guild_id}", guild_id = GUILD_ID)
        );
    }

    #[test]
    fn test_update_guild() {
        let route = Route::UpdateGuild { guild_id: GUILD_ID };
        assert_eq!(
            route.display().to_string(),
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
            route.display().to_string(),
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
            route.display().to_string(),
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
            route.display().to_string(),
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
            route.display().to_string(),
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
            route.display().to_string(),
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
            route.display().to_string(),
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
            route.display().to_string(),
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
            route.display().to_string(),
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
        assert_eq!(
            route.display().to_string(),
            format!("invites/{code}", code = CODE)
        );
    }

    #[test]
    fn test_delete_message_reactions() {
        let route = Route::DeleteMessageReactions {
            channel_id: CHANNEL_ID,
            message_id: MESSAGE_ID,
        };
        assert_eq!(
            route.display().to_string(),
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
            route.display().to_string(),
            format!(
                "channels/{channel_id}/messages/{message_id}/reactions/{emoji}",
                channel_id = CHANNEL_ID,
                emoji = emoji.display(),
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
            route.display().to_string(),
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
            route.display().to_string(),
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
            route.display().to_string(),
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
            route.display().to_string(),
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
            route.display().to_string(),
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
            route.display().to_string(),
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
            route.display().to_string(),
            format!(
                "channels/{channel_id}/messages/{message_id}/reactions/{emoji}/{user_id}",
                channel_id = CHANNEL_ID,
                emoji = emoji.display(),
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
            route.display().to_string(),
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
            route.display().to_string(),
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
            route.display().to_string(),
            format!("stage-instances/{channel_id}", channel_id = CHANNEL_ID)
        );
    }

    #[test]
    fn test_get_stage_instance() {
        let route = Route::GetStageInstance {
            channel_id: CHANNEL_ID,
        };
        assert_eq!(
            route.display().to_string(),
            format!("stage-instances/{channel_id}", channel_id = CHANNEL_ID)
        );
    }

    #[test]
    fn test_update_stage_instance() {
        let route = Route::UpdateStageInstance {
            channel_id: CHANNEL_ID,
        };
        assert_eq!(
            route.display().to_string(),
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
            route.display().to_string(),
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
            route.display().to_string(),
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
            route.display().to_string(),
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
            route.display().to_string(),
            format!("channels/{channel_id}/followers", channel_id = CHANNEL_ID)
        );
    }

    #[test]
    fn test_get_active_threads() {
        let route = Route::GetActiveThreads { guild_id: GUILD_ID };
        assert_eq!(
            route.display().to_string(),
            format!("guilds/{guild_id}/threads/active", guild_id = GUILD_ID)
        );
    }

    #[test]
    fn test_get_bans() {
        let route = Route::GetBans { guild_id: GUILD_ID };
        assert_eq!(
            route.display().to_string(),
            format!("guilds/{guild_id}/bans", guild_id = GUILD_ID)
        );
    }

    #[test]
    fn test_get_gateway_bot() {
        let route = Route::GetGatewayBot;
        assert_eq!(route.display().to_string(), "gateway/bot");
    }

    #[test]
    fn test_get_command_permissions() {
        let route = Route::GetCommandPermissions {
            application_id: APPLICATION_ID,
            command_id: COMMAND_ID,
            guild_id: GUILD_ID,
        };
        assert_eq!(
            route.display().to_string(),
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
            route.display().to_string(),
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
        assert_eq!(route.display().to_string(), "oauth2/applications/@me");
    }

    #[test]
    fn test_get_current_user() {
        let route = Route::GetCurrentUser;
        assert_eq!(route.display().to_string(), "users/@me");
    }

    #[test]
    fn test_get_current_user_guild_member() {
        let route = Route::GetCurrentUserGuildMember { guild_id: GUILD_ID };
        assert_eq!(
            route.display().to_string(),
            format!("users/@me/guilds/{guild_id}/member", guild_id = GUILD_ID)
        )
    }

    #[test]
    fn test_update_current_user() {
        let route = Route::UpdateCurrentUser;
        assert_eq!(route.display().to_string(), "users/@me");
    }

    #[test]
    fn test_get_gateway() {
        let route = Route::GetGateway;
        assert_eq!(route.display().to_string(), "gateway");
    }

    #[test]
    fn test_get_guild_command_permissions() {
        let route = Route::GetGuildCommandPermissions {
            application_id: APPLICATION_ID,
            guild_id: GUILD_ID,
        };
        assert_eq!(
            route.display().to_string(),
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
            route.display().to_string(),
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
            route.display().to_string(),
            format!("guilds/{guild_id}/invites", guild_id = GUILD_ID)
        );
    }

    #[test]
    fn test_get_guild_preview() {
        let route = Route::GetGuildPreview { guild_id: GUILD_ID };
        assert_eq!(
            route.display().to_string(),
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
            route.display().to_string(),
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
            route.display().to_string(),
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
            route.display().to_string(),
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
            route.display().to_string(),
            format!("guilds/{guild_id}/vanity-url", guild_id = GUILD_ID)
        );
    }

    #[test]
    fn test_get_guild_voice_regions() {
        let route = Route::GetGuildVoiceRegions { guild_id: GUILD_ID };
        assert_eq!(
            route.display().to_string(),
            format!("guilds/{guild_id}/regions", guild_id = GUILD_ID)
        );
    }

    #[test]
    fn test_get_guild_welcome_screen() {
        let route = Route::GetGuildWelcomeScreen { guild_id: GUILD_ID };
        assert_eq!(
            route.display().to_string(),
            format!("guilds/{guild_id}/welcome-screen", guild_id = GUILD_ID)
        );
    }

    #[test]
    fn test_update_guild_welcome_screen() {
        let route = Route::UpdateGuildWelcomeScreen { guild_id: GUILD_ID };
        assert_eq!(
            route.display().to_string(),
            format!("guilds/{guild_id}/welcome-screen", guild_id = GUILD_ID)
        );
    }

    #[test]
    fn test_get_guild_webhooks() {
        let route = Route::GetGuildWebhooks { guild_id: GUILD_ID };
        assert_eq!(
            route.display().to_string(),
            format!("guilds/{guild_id}/webhooks", guild_id = GUILD_ID)
        );
    }

    #[test]
    fn test_get_guild_widget() {
        let route = Route::GetGuildWidgetSettings { guild_id: GUILD_ID };
        assert_eq!(
            route.display().to_string(),
            format!("guilds/{guild_id}/widget", guild_id = GUILD_ID)
        );
    }

    #[test]
    fn test_update_guild_widget() {
        let route = Route::UpdateGuildWidgetSettings { guild_id: GUILD_ID };
        assert_eq!(
            route.display().to_string(),
            format!("guilds/{guild_id}/widget", guild_id = GUILD_ID)
        );
    }

    #[test]
    fn test_get_nitro_sticker_packs() {
        let route = Route::GetNitroStickerPacks;

        assert_eq!(route.display().to_string(), "sticker-packs");
    }

    #[test]
    fn test_get_pins() {
        let route = Route::GetPins {
            channel_id: CHANNEL_ID,
        };
        assert_eq!(
            route.display().to_string(),
            format!("channels/{channel_id}/pins", channel_id = CHANNEL_ID)
        );
    }

    #[test]
    fn test_get_sticker() {
        let route = Route::GetSticker {
            sticker_id: STICKER_ID,
        };
        assert_eq!(
            route.display().to_string(),
            format!("stickers/{sticker_id}", sticker_id = STICKER_ID)
        );
    }

    #[test]
    fn test_get_thread_members() {
        let route = Route::GetThreadMembers {
            channel_id: CHANNEL_ID,
        };
        assert_eq!(
            route.display().to_string(),
            format!(
                "channels/{channel_id}/thread-members",
                channel_id = CHANNEL_ID
            )
        );
    }

    #[test]
    fn test_get_user_connections() {
        let route = Route::GetUserConnections;
        assert_eq!(route.display().to_string(), "users/@me/connections");
    }

    #[test]
    fn test_get_user() {
        let route = Route::GetUser { user_id: USER_ID };
        assert_eq!(
            route.display().to_string(),
            format!("users/{user_id}", user_id = USER_ID)
        );
    }

    #[test]
    fn test_get_voice_regions() {
        let route = Route::GetVoiceRegions;
        assert_eq!(route.display().to_string(), "voice/regions");
    }

    #[test]
    fn test_interaction_callback() {
        let route = Route::InteractionCallback {
            interaction_id: INTERACTION_ID,
            interaction_token: INTERACTION_TOKEN,
        };
        assert_eq!(
            route.display().to_string(),
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
            route.display().to_string(),
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
            route.display().to_string(),
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
            route.display().to_string(),
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
            route.display().to_string(),
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
            route.display().to_string(),
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
            route.display().to_string(),
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
            route.display().to_string(),
            format!("guilds/{guild_id}/members/@me", guild_id = GUILD_ID)
        );
    }

    #[test]
    fn test_update_current_user_voice_state() {
        let route = Route::UpdateCurrentUserVoiceState { guild_id: GUILD_ID };
        assert_eq!(
            route.display().to_string(),
            format!("guilds/{guild_id}/voice-states/@me", guild_id = GUILD_ID)
        );
    }

    #[test]
    fn test_update_nickname() {
        let route = Route::UpdateNickname { guild_id: GUILD_ID };
        assert_eq!(
            route.display().to_string(),
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
            route.display().to_string(),
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
            route.display().to_string(),
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
            route.display().to_string(),
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
            route.display().to_string(),
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
            route.display().to_string(),
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
            route.display().to_string(),
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
            route.display().to_string(),
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
            route.display().to_string(),
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
            route.display().to_string(),
            format!("guilds/{guild_id}/prune?&days=4", guild_id = GUILD_ID)
        );
    }

    #[test]
    fn test_create_guild_prune_include_one_role() {
        let include_roles = [RoleId::new(1).expect("non zero id")];

        let route = Route::CreateGuildPrune {
            compute_prune_count: None,
            days: None,
            guild_id: GUILD_ID,
            include_roles: &include_roles,
        };
        assert_eq!(
            route.display().to_string(),
            format!(
                "guilds/{guild_id}/prune?&include_roles=1",
                guild_id = GUILD_ID
            )
        );
    }

    #[test]
    fn test_create_guild_prune_include_two_roles() {
        let include_roles = [
            RoleId::new(1).expect("non zero id"),
            RoleId::new(2).expect("non zero id"),
        ];

        let route = Route::CreateGuildPrune {
            compute_prune_count: None,
            days: None,
            guild_id: GUILD_ID,
            include_roles: &include_roles,
        };
        assert_eq!(
            route.display().to_string(),
            format!(
                "guilds/{guild_id}/prune?&include_roles=1,2",
                guild_id = GUILD_ID
            )
        );
    }

    #[test]
    fn test_create_guild_prune_all() {
        let include_roles = [
            RoleId::new(1).expect("non zero id"),
            RoleId::new(2).expect("non zero id"),
        ];

        let route = Route::CreateGuildPrune {
            compute_prune_count: Some(true),
            days: Some(4),
            guild_id: GUILD_ID,
            include_roles: &include_roles,
        };
        assert_eq!(
            route.display().to_string(),
            format!(
                "guilds/{guild_id}/prune?compute_prune_count=true&days=4&include_roles=1,2",
                guild_id = GUILD_ID
            )
        );
    }
}
