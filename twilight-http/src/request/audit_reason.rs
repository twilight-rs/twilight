/// Attach a reason for a request.
///
/// Reasons are associated with the audit log entries that are automatically
/// created for certain requests.
pub trait AuditLogReason<'a>: private::Sealed {
    /// Attach an audit log reason to the request.
    ///
    /// # Errors
    ///
    /// Returns an error of type [`AuditReason`] if the length is invalid.
    ///
    /// [`AuditReason`]: twilight_validate::request::ValidationErrorType::AuditReason
    #[must_use]
    fn reason(self, reason: &'a str) -> Self
    where
        Self: Sized;
}

mod private {
    use crate::request::{
        channel::{
            invite::{CreateInvite, DeleteInvite},
            message::{DeleteMessage, DeleteMessages},
            thread::UpdateThread,
            webhook::{CreateWebhook, DeleteWebhook, DeleteWebhookMessage, UpdateWebhook},
            CreatePin, DeleteChannel, DeleteChannelPermissionConfigured, DeletePin, UpdateChannel,
            UpdateChannelPermission,
        },
        guild::{
            auto_moderation::{
                CreateAutoModerationRule, DeleteAutoModerationRule, UpdateAutoModerationRule,
            },
            ban::{CreateBan, DeleteBan},
            emoji::{CreateEmoji, DeleteEmoji, UpdateEmoji},
            integration::DeleteGuildIntegration,
            member::{AddRoleToMember, RemoveMember, RemoveRoleFromMember, UpdateGuildMember},
            role::{CreateRole, DeleteRole, UpdateRole, UpdateRolePositions},
            sticker::{CreateGuildSticker, UpdateGuildSticker},
            update_guild_onboarding::UpdateGuildOnboarding,
            CreateGuildChannel, CreateGuildPrune, UpdateCurrentMember, UpdateGuild, UpdateGuildMfa,
            UpdateGuildWidgetSettings,
        },
        scheduled_event::{
            CreateGuildExternalScheduledEvent, CreateGuildScheduledEvent,
            CreateGuildStageInstanceScheduledEvent, CreateGuildVoiceScheduledEvent,
            UpdateGuildScheduledEvent,
        },
        user::UpdateCurrentUser,
    };

    /// Sealed stops crates other crates implementing the trait.
    pub trait Sealed {}

    impl Sealed for AddRoleToMember<'_> {}
    impl Sealed for CreateAutoModerationRule<'_> {}
    impl Sealed for CreateBan<'_> {}
    impl Sealed for CreateEmoji<'_> {}
    impl Sealed for CreateGuildChannel<'_> {}
    impl Sealed for CreateGuildExternalScheduledEvent<'_> {}
    impl Sealed for CreateGuildPrune<'_> {}
    impl Sealed for CreateGuildScheduledEvent<'_> {}
    impl Sealed for CreateGuildStageInstanceScheduledEvent<'_> {}
    impl Sealed for CreateGuildSticker<'_> {}
    impl Sealed for CreateGuildVoiceScheduledEvent<'_> {}
    impl Sealed for CreateInvite<'_> {}
    impl Sealed for CreatePin<'_> {}
    impl Sealed for CreateRole<'_> {}
    impl Sealed for CreateWebhook<'_> {}
    impl Sealed for DeleteAutoModerationRule<'_> {}
    impl Sealed for DeleteBan<'_> {}
    impl Sealed for DeleteChannel<'_> {}
    impl Sealed for DeleteChannelPermissionConfigured<'_> {}
    impl Sealed for DeleteEmoji<'_> {}
    impl Sealed for DeleteGuildIntegration<'_> {}
    impl Sealed for DeleteInvite<'_> {}
    impl Sealed for DeleteMessage<'_> {}
    impl Sealed for DeleteMessages<'_> {}
    impl Sealed for DeletePin<'_> {}
    impl Sealed for DeleteRole<'_> {}
    impl Sealed for DeleteWebhook<'_> {}
    impl Sealed for DeleteWebhookMessage<'_> {}
    impl Sealed for RemoveMember<'_> {}
    impl Sealed for RemoveRoleFromMember<'_> {}
    impl Sealed for UpdateAutoModerationRule<'_> {}
    impl Sealed for UpdateChannel<'_> {}
    impl Sealed for UpdateChannelPermission<'_> {}
    impl Sealed for UpdateCurrentMember<'_> {}
    impl Sealed for UpdateCurrentUser<'_> {}
    impl Sealed for UpdateEmoji<'_> {}
    impl Sealed for UpdateGuild<'_> {}
    impl Sealed for UpdateGuildMember<'_> {}
    impl Sealed for UpdateGuildMfa<'_> {}
    impl Sealed for UpdateGuildOnboarding<'_> {}
    impl Sealed for UpdateGuildScheduledEvent<'_> {}
    impl Sealed for UpdateGuildSticker<'_> {}
    impl Sealed for UpdateGuildWidgetSettings<'_> {}
    impl Sealed for UpdateRole<'_> {}
    impl Sealed for UpdateRolePositions<'_> {}
    impl Sealed for UpdateThread<'_> {}
    impl Sealed for UpdateWebhook<'_> {}
}

#[cfg(test)]
mod tests {
    use super::AuditLogReason;
    use crate::request::{
        channel::{
            invite::{CreateInvite, DeleteInvite},
            message::{DeleteMessage, DeleteMessages},
            webhook::{CreateWebhook, DeleteWebhook, UpdateWebhook},
            CreatePin, DeleteChannel, DeleteChannelPermissionConfigured, DeletePin, UpdateChannel,
            UpdateChannelPermission,
        },
        guild::{
            ban::{CreateBan, DeleteBan},
            emoji::{CreateEmoji, DeleteEmoji, UpdateEmoji},
            integration::DeleteGuildIntegration,
            member::{AddRoleToMember, RemoveMember, RemoveRoleFromMember, UpdateGuildMember},
            role::{CreateRole, DeleteRole, UpdateRole, UpdateRolePositions},
            sticker::{CreateGuildSticker, UpdateGuildSticker},
            CreateGuildChannel, CreateGuildPrune, UpdateCurrentMember, UpdateGuild,
        },
        user::UpdateCurrentUser,
    };
    use static_assertions::{assert_impl_all, assert_obj_safe};

    assert_obj_safe!(AuditLogReason<'_>);

    assert_impl_all!(AddRoleToMember<'_>: AuditLogReason<'static>);
    assert_impl_all!(CreateBan<'_>: AuditLogReason<'static>);
    assert_impl_all!(CreateEmoji<'_>: AuditLogReason<'static>);
    assert_impl_all!(CreateGuildChannel<'_>: AuditLogReason<'static>);
    assert_impl_all!(CreateGuildPrune<'_>: AuditLogReason<'static>);
    assert_impl_all!(CreateGuildSticker<'_>: AuditLogReason<'static>);
    assert_impl_all!(CreateInvite<'_>: AuditLogReason<'static>);
    assert_impl_all!(CreatePin<'_>: AuditLogReason<'static>);
    assert_impl_all!(CreateRole<'_>: AuditLogReason<'static>);
    assert_impl_all!(CreateWebhook<'_>: AuditLogReason<'static>);
    assert_impl_all!(DeleteBan<'_>: AuditLogReason<'static>);
    assert_impl_all!(DeleteChannel<'_>: AuditLogReason<'static>);
    assert_impl_all!(DeleteChannelPermissionConfigured<'_>: AuditLogReason<'static>);
    assert_impl_all!(DeleteEmoji<'_>: AuditLogReason<'static>);
    assert_impl_all!(DeleteGuildIntegration<'_>: AuditLogReason<'static>);
    assert_impl_all!(DeleteInvite<'_>: AuditLogReason<'static>);
    assert_impl_all!(DeleteMessage<'_>: AuditLogReason<'static>);
    assert_impl_all!(DeleteMessages<'_>: AuditLogReason<'static>);
    assert_impl_all!(DeletePin<'_>: AuditLogReason<'static>);
    assert_impl_all!(DeleteRole<'_>: AuditLogReason<'static>);
    assert_impl_all!(DeleteWebhook<'_>: AuditLogReason<'static>);
    assert_impl_all!(RemoveMember<'_>: AuditLogReason<'static>);
    assert_impl_all!(RemoveRoleFromMember<'_>: AuditLogReason<'static>);
    assert_impl_all!(UpdateChannel<'_>: AuditLogReason<'static>);
    assert_impl_all!(UpdateChannelPermission<'_>: AuditLogReason<'static>);
    assert_impl_all!(UpdateCurrentMember<'_>: AuditLogReason<'static>);
    assert_impl_all!(UpdateCurrentUser<'_>: AuditLogReason<'static>);
    assert_impl_all!(UpdateEmoji<'_>: AuditLogReason<'static>);
    assert_impl_all!(UpdateGuild<'_>: AuditLogReason<'static>);
    assert_impl_all!(UpdateGuildMember<'_>: AuditLogReason<'static>);
    assert_impl_all!(UpdateGuildSticker<'_>: AuditLogReason<'static>);
    assert_impl_all!(UpdateRole<'_>: AuditLogReason<'static>);
    assert_impl_all!(UpdateRolePositions<'_>: AuditLogReason<'static>);
    assert_impl_all!(UpdateWebhook<'_>: AuditLogReason<'static>);
}
