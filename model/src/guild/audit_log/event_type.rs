use serde_repr::{Deserialize_repr, Serialize_repr};

/// Action to cause an [`AuditLogEntry`].
///
/// [`AuditLogEntry`]: super::AuditLogEntry
#[derive(
    Clone, Copy, Debug, Deserialize_repr, Eq, Hash, PartialEq, PartialOrd, Ord, Serialize_repr,
)]
#[non_exhaustive]
#[repr(u8)]
pub enum AuditLogEventType {
    /// [Guild] was updated.
    ///
    /// [Guild]: super::super::Guild
    GuildUpdate = 1,
    /// [Guild channel] was created.
    ///
    /// [Guild channel]: crate::channel::GuildChannel
    ChannelCreate = 10,
    /// [Guild channel] was updated.
    ///
    /// [Guild channel]: crate::channel::GuildChannel
    ChannelUpdate = 11,
    /// [Guild channel] was deleted.
    ///
    /// [Guild channel]: crate::channel::GuildChannel
    ChannelDelete = 12,
    /// [Permission overwrite] for a [guild channel] was created.
    ///
    /// [guild channel]: crate::channel::GuildChannel
    /// [Permission overwrite]: crate::channel::permission_overwrite::PermissionOverwrite
    ChannelOverwriteCreate = 13,
    /// [Permission overwrite] for a [guild channel] was updated.
    ///
    /// [guild channel]: crate::channel::GuildChannel
    /// [Permission overwrite]: crate::channel::permission_overwrite::PermissionOverwrite
    ChannelOverwriteUpdate = 14,
    /// [Permission overwrite] for a [guild channel] was deleted.
    ///
    /// [guild channel]: crate::channel::GuildChannel
    /// [Permission overwrite]: crate::channel::permission_overwrite::PermissionOverwrite
    ChannelOverwriteDelete = 15,
    /// [Member] was kicked.
    ///
    /// [Member]: super::super::Member
    MemberKick = 20,
    /// [Member] prune began.
    ///
    /// [Member]: super::super::Member
    MemberPrune = 21,
    /// [Member] was banned.
    ///
    /// [Member]: super::super::Member
    MemberBanAdd = 22,
    /// [Member]'s [ban] was removed.
    ///
    /// [ban]: super::super::Ban
    /// [Member]: super::super::Member
    MemberBanRemove = 23,
    /// [Member] was updated.
    ///
    /// [Member]: super::super::Member
    MemberUpdate = 24,
    /// [Member] either had a [role] attached or removed.
    ///
    /// [Member]: super::super::Member
    /// [role]: super::super::Role
    MemberRoleUpdate = 25,
    /// [Member] was moved to a [voice channel].
    ///
    /// [Member]: super::super::Member
    /// [voice channel]: crate::channel::VoiceChannel
    MemberMove = 26,
    /// [Member] was disconnected from a [voice channel].
    ///
    /// [Member]: super::super::Member
    /// [voice channel]: crate::channel::VoiceChannel
    MemberDisconnect = 27,
    /// [Bot user] was added to a [guild].
    ///
    /// [Bot user]: crate::user::User::bot
    /// [guild]: super::super::Guild
    BotAdd = 28,
    /// [Role] was created.
    ///
    /// [Role]: super::super::Role
    RoleCreate = 30,
    /// [Role] was updated.
    ///
    /// [Role]: super::super::Role
    RoleUpdate = 31,
    /// [Role] was deleted.
    ///
    /// [Role]: super::super::Role
    RoleDelete = 32,
    /// [Invite] was created.
    ///
    /// [Invite]: crate::invite::Invite
    InviteCreate = 40,
    /// [Invite] was updated.
    ///
    /// [Invite]: crate::invite::Invite
    InviteUpdate = 41,
    /// [Invite] was deleted.
    ///
    /// [Invite]: crate::invite::Invite
    InviteDelete = 42,
    /// [Webhook] was created.
    ///
    /// [Webhook]: crate::channel::webhook::Webhook
    WebhookCreate = 50,
    /// [Webhook] was updated.
    ///
    /// [Webhook]: crate::channel::webhook::Webhook
    WebhookUpdate = 51,
    /// [Webhook] was deleted.
    ///
    /// [Webhook]: crate::channel::webhook::Webhook
    WebhookDelete = 52,
    /// [Emoji] was created.
    ///
    /// [Emoji]: super::super::Emoji
    EmojiCreate = 60,
    /// [Emoji] was updated.
    ///
    /// [Emoji]: super::super::Emoji
    EmojiUpdate = 61,
    /// [Emoji] was deleted.
    ///
    /// [Emoji]: super::super::Emoji
    EmojiDelete = 62,
    /// [Message] was deleted.
    ///
    /// [Message]: crate::channel::message::Message
    MessageDelete = 72,
    /// Multiple [messages] were deleted.
    ///
    /// [messages]: crate::channel::message::Message
    MessageBulkDelete = 73,
    /// [Message] was pinned to a [guild channel].
    ///
    /// [Message]: crate::channel::message::Message
    /// [guild channel]: crate::channel::GuildChannel
    MessagePin = 74,
    /// [Message] was unpinned from a [guild channel].
    ///
    /// [Message]: crate::channel::message::Message
    /// [guild channel]: crate::channel::GuildChannel
    MessageUnpin = 75,
    /// [Integration] was created.
    ///
    /// [Integration]: super::super::GuildIntegration
    IntegrationCreate = 80,
    /// [Integration] was updated.
    ///
    /// [Integration]: super::super::GuildIntegration
    IntegrationUpdate = 81,
    /// [Integration] was deleted.
    ///
    /// [Integration]: super::super::GuildIntegration
    IntegrationDelete = 82,
    /// [`NewsThread`], [`PrivateThread`], or [`PublicThread]` was created.
    ///
    /// [`NewsThread`]: crate::channel::thread::NewsThread
    /// [`PrivateThread`]: crate::channel::thread::PrivateThread
    /// [`PublicThread`]: crate::channel::thread::PublicThread
    ThreadCreate = 110,
    /// [`NewsThread`], [`PrivateThread`], or [`PublicThread]` was deleted.
    ///
    /// [`NewsThread`]: crate::channel::thread::NewsThread
    /// [`PrivateThread`]: crate::channel::thread::PrivateThread
    /// [`PublicThread`]: crate::channel::thread::PublicThread
    ThreadDelete = 112,
    /// [`NewsThread`], [`PrivateThread`], or [`PublicThread]` was updated.
    ///
    /// [`NewsThread`]: crate::channel::thread::NewsThread
    /// [`PrivateThread`]: crate::channel::thread::PrivateThread
    /// [`PublicThread`]: crate::channel::thread::PublicThread
    ThreadUpdate = 111,
    /// [Stage instance] was created.
    ///
    /// [Stage instance]: crate::channel::stage_instance::StageInstance
    StageInstanceCreate = 83,
    /// [Stage instance] was updated.
    ///
    /// [Stage instance]: crate::channel::stage_instance::StageInstance
    StageInstanceUpdate = 84,
    /// [Stage instance] was deleted.
    ///
    /// [Stage instance]: crate::channel::stage_instance::StageInstance
    StageInstanceDelete = 85,
    /// [Sticker] was created.
    ///
    /// [Sticker]: crate::channel::message::sticker::Sticker
    StickerCreate = 90,
    /// [Sticker] was updated.
    ///
    /// [Sticker]: crate::channel::message::sticker::Sticker
    StickerUpdate = 91,
    /// [Sticker] was deleted.
    ///
    /// [Sticker]: crate::channel::message::sticker::Sticker
    StickerDelete = 92,
    /// [`GuildScheduledEvent`] was created.
    ///
    /// [`GuildScheduledEvent`]: crate::scheduled_event::GuildScheduledEvent
    GuildScheduledEventCreate = 100,
    /// [`GuildScheduledEvent`] was deleted.
    ///
    /// [`GuildScheduledEvent`]: crate::scheduled_event::GuildScheduledEvent
    GuildScheduledEventDelete = 102,
    /// [`GuildScheduledEvent`] was updated.
    ///
    /// [`GuildScheduledEvent`]: crate::scheduled_event::GuildScheduledEvent
    GuildScheduledEventUpdate = 101,
}

#[cfg(test)]
mod tests {
    use super::AuditLogEventType;
    use serde::{Deserialize, Serialize};
    use static_assertions::{assert_impl_all, const_assert_eq};
    use std::{fmt::Debug, hash::Hash};

    assert_impl_all!(
        AuditLogEventType: Clone,
        Copy,
        Debug,
        Deserialize<'static>,
        Eq,
        Hash,
        PartialEq,
        PartialOrd,
        Ord,
        Send,
        Serialize,
        Sync,
    );
    const_assert_eq!(1, AuditLogEventType::GuildUpdate as u8);
    const_assert_eq!(10, AuditLogEventType::ChannelCreate as u8);
    const_assert_eq!(11, AuditLogEventType::ChannelUpdate as u8);
    const_assert_eq!(12, AuditLogEventType::ChannelDelete as u8);
    const_assert_eq!(13, AuditLogEventType::ChannelOverwriteCreate as u8);
    const_assert_eq!(14, AuditLogEventType::ChannelOverwriteUpdate as u8);
    const_assert_eq!(15, AuditLogEventType::ChannelOverwriteDelete as u8);
    const_assert_eq!(20, AuditLogEventType::MemberKick as u8);
    const_assert_eq!(21, AuditLogEventType::MemberPrune as u8);
    const_assert_eq!(22, AuditLogEventType::MemberBanAdd as u8);
    const_assert_eq!(23, AuditLogEventType::MemberBanRemove as u8);
    const_assert_eq!(24, AuditLogEventType::MemberUpdate as u8);
    const_assert_eq!(25, AuditLogEventType::MemberRoleUpdate as u8);
    const_assert_eq!(26, AuditLogEventType::MemberMove as u8);
    const_assert_eq!(27, AuditLogEventType::MemberDisconnect as u8);
    const_assert_eq!(28, AuditLogEventType::BotAdd as u8);
    const_assert_eq!(30, AuditLogEventType::RoleCreate as u8);
    const_assert_eq!(31, AuditLogEventType::RoleUpdate as u8);
    const_assert_eq!(32, AuditLogEventType::RoleDelete as u8);
    const_assert_eq!(40, AuditLogEventType::InviteCreate as u8);
    const_assert_eq!(41, AuditLogEventType::InviteUpdate as u8);
    const_assert_eq!(42, AuditLogEventType::InviteDelete as u8);
    const_assert_eq!(50, AuditLogEventType::WebhookCreate as u8);
    const_assert_eq!(51, AuditLogEventType::WebhookUpdate as u8);
    const_assert_eq!(52, AuditLogEventType::WebhookDelete as u8);
    const_assert_eq!(60, AuditLogEventType::EmojiCreate as u8);
    const_assert_eq!(61, AuditLogEventType::EmojiUpdate as u8);
    const_assert_eq!(62, AuditLogEventType::EmojiDelete as u8);
    const_assert_eq!(72, AuditLogEventType::MessageDelete as u8);
    const_assert_eq!(73, AuditLogEventType::MessageBulkDelete as u8);
    const_assert_eq!(74, AuditLogEventType::MessagePin as u8);
    const_assert_eq!(75, AuditLogEventType::MessageUnpin as u8);
    const_assert_eq!(80, AuditLogEventType::IntegrationCreate as u8);
    const_assert_eq!(81, AuditLogEventType::IntegrationUpdate as u8);
    const_assert_eq!(82, AuditLogEventType::IntegrationDelete as u8);
    const_assert_eq!(83, AuditLogEventType::StageInstanceCreate as u8);
    const_assert_eq!(84, AuditLogEventType::StageInstanceUpdate as u8);
    const_assert_eq!(85, AuditLogEventType::StageInstanceDelete as u8);
}
