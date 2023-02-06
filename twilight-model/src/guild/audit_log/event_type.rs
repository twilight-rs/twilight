use serde::{Deserialize, Serialize};

/// Action to cause an [`AuditLogEntry`].
///
/// [`AuditLogEntry`]: super::AuditLogEntry
#[derive(Clone, Copy, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct AuditLogEventType(u16);

impl AuditLogEventType {
    /// [Guild] was updated.
    ///
    /// [Guild]: crate::guild::Guild
    pub const GUILD_UPDATE: Self = Self::new(1);

    /// [Channel] was created.
    ///
    /// [Channel]: crate::channel::Channel
    pub const CHANNEL_CREATE: Self = Self::new(10);

    /// [Channel] was updated.
    ///
    /// [Channel]: crate::channel::Channel
    pub const CHANNEL_UPDATE: Self = Self::new(11);

    /// [Channel] was deleted.
    ///
    /// [Channel]: crate::channel::Channel
    pub const CHANNEL_DELETE: Self = Self::new(12);

    /// [Permission overwrite] for a [channel] was created.
    ///
    /// [channel]: crate::channel::Channel
    /// [Permission overwrite]: crate::channel::permission_overwrite::PermissionOverwrite
    pub const CHANNEL_OVERWRITE_CREATE: Self = Self::new(13);

    /// [Permission overwrite] for a [channel] was updated.
    ///
    /// [channel]: crate::channel::Channel
    /// [Permission overwrite]: crate::channel::permission_overwrite::PermissionOverwrite
    pub const CHANNEL_OVERWRITE_UPDATE: Self = Self::new(14);

    /// [Permission overwrite] for a [channel] was deleted.
    ///
    /// [channel]: crate::channel::Channel
    /// [Permission overwrite]: crate::channel::permission_overwrite::PermissionOverwrite
    pub const CHANNEL_OVERWRITE_DELETE: Self = Self::new(15);

    /// [Member] was kicked.
    ///
    /// [Member]: crate::guild::Member
    pub const MEMBER_KICK: Self = Self::new(20);

    /// [Member] prune began.
    ///
    /// [Member]: crate::guild::Member
    pub const MEMBER_PRUNE: Self = Self::new(21);

    /// [Member] was banned.
    ///
    /// [Member]: crate::guild::Member
    pub const MEMBER_BAN_ADD: Self = Self::new(22);

    /// [Member]'s [ban] was removed.
    ///
    /// [ban]: crate::guild::Ban
    /// [Member]: crate::guild::Member
    pub const MEMBER_BAN_REMOVE: Self = Self::new(23);

    /// [Member] was updated.
    ///
    /// [Member]: crate::guild::Member
    pub const MEMBER_UPDATE: Self = Self::new(24);

    /// [Member] either had a [role] attached or removed.
    ///
    /// [Member]: crate::guild::Member
    /// [role]: crate::guild::Role
    pub const MEMBER_ROLE_UPDATE: Self = Self::new(25);

    /// [Member] was moved between voice [channel]s.
    ///
    /// [Member]: crate::guild::Member
    /// [channel]: crate::channel::Channel
    pub const MEMBER_MOVE: Self = Self::new(26);

    /// [Member] was disconnected from a voice [channel].
    ///
    /// [Member]: crate::guild::Member
    /// [channel]: crate::channel::Channel
    pub const MEMBER_DISCONNECT: Self = Self::new(27);

    /// [Bot user] was added to a [guild].
    ///
    /// [Bot user]: crate::user::User::bot
    /// [guild]: crate::guild::Guild
    pub const BOT_ADD: Self = Self::new(28);

    /// [Role] was created.
    ///
    /// [Role]: crate::guild::Role
    pub const ROLE_CREATE: Self = Self::new(30);

    /// [Role] was updated.
    ///
    /// [Role]: crate::guild::Role
    pub const ROLE_UPDATE: Self = Self::new(31);

    /// [Role] was deleted.
    ///
    /// [Role]: crate::guild::Role
    pub const ROLE_DELETE: Self = Self::new(32);

    /// [Invite] was created.
    ///
    /// [Invite]: crate::guild::invite::Invite
    pub const INVITE_CREATE: Self = Self::new(40);

    /// [Invite] was updated.
    ///
    /// [Invite]: crate::guild::invite::Invite
    pub const INVITE_UPDATE: Self = Self::new(41);

    /// [Invite] was deleted.
    ///
    /// [Invite]: crate::guild::invite::Invite
    pub const INVITE_DELETE: Self = Self::new(42);

    /// [Webhook] was created.
    ///
    /// [Webhook]: crate::channel::webhook::Webhook
    pub const WEBHOOK_CREATE: Self = Self::new(50);

    /// [Webhook] was updated.
    ///
    /// [Webhook]: crate::channel::webhook::Webhook
    pub const WEBHOOK_UPDATE: Self = Self::new(51);

    /// [Webhook] was deleted.
    ///
    /// [Webhook]: crate::channel::webhook::Webhook
    pub const WEBHOOK_DELETE: Self = Self::new(52);

    /// [Emoji] was created.
    ///
    /// [Emoji]: crate::guild::Emoji
    pub const EMOJI_CREATE: Self = Self::new(60);

    /// [Emoji] was updated.
    ///
    /// [Emoji]: crate::guild::Emoji
    pub const EMOJI_UPDATE: Self = Self::new(61);

    /// [Emoji] was deleted.
    ///
    /// [Emoji]: crate::guild::Emoji
    pub const EMOJI_DELETE: Self = Self::new(62);

    /// [Message] was deleted.
    ///
    /// [Message]: crate::channel::message::Message
    pub const MESSAGE_DELETE: Self = Self::new(72);

    /// Multiple [messages] were deleted.
    ///
    /// [messages]: crate::channel::message::Message
    pub const MESSAGE_BULK_DELETE: Self = Self::new(73);

    /// [Message] was pinned to a [channel].
    ///
    /// [Message]: crate::channel::message::Message
    /// [channel]: crate::channel::Channel
    pub const MESSAGE_PIN: Self = Self::new(74);

    /// [Message] was unpinned from a [channel].
    ///
    /// [Message]: crate::channel::message::Message
    /// [channel]: crate::channel::Channel
    pub const MESSAGE_UNPIN: Self = Self::new(75);

    /// [Integration] was created.
    ///
    /// [Integration]: crate::guild::GuildIntegration
    pub const INTEGRATION_CREATE: Self = Self::new(80);

    /// [Integration] was updated.
    ///
    /// [Integration]: crate::guild::GuildIntegration
    pub const INTEGRATION_UPDATE: Self = Self::new(81);

    /// [Integration] was deleted.
    ///
    /// [Integration]: crate::guild::GuildIntegration
    pub const INTEGRATION_DELETE: Self = Self::new(82);

    /// [Stage instance] was created.
    ///
    /// [Stage instance]: crate::channel::stage_instance::StageInstance
    pub const STAGE_INSTANCE_CREATE: Self = Self::new(83);

    /// [Stage instance] was updated.
    ///
    /// [Stage instance]: crate::channel::stage_instance::StageInstance
    pub const STAGE_INSTANCE_UPDATE: Self = Self::new(84);

    /// [Stage instance] was deleted.
    ///
    /// [Stage instance]: crate::channel::stage_instance::StageInstance
    pub const STAGE_INSTANCE_DELETE: Self = Self::new(85);

    /// [Sticker] was created.
    ///
    /// [Sticker]: crate::channel::message::sticker::Sticker
    pub const STICKER_CREATE: Self = Self::new(90);

    /// [Sticker] was updated.
    ///
    /// [Sticker]: crate::channel::message::sticker::Sticker
    pub const STICKER_UPDATE: Self = Self::new(91);

    /// [Sticker] was deleted.
    ///
    /// [Sticker]: crate::channel::message::sticker::Sticker
    pub const STICKER_DELETE: Self = Self::new(92);

    /// [`GuildScheduledEvent`] was created.
    ///
    /// [`GuildScheduledEvent`]: crate::guild::scheduled_event::GuildScheduledEvent
    pub const GUILD_SCHEDULED_EVENT_CREATE: Self = Self::new(100);

    /// [`GuildScheduledEvent`] was updated.
    ///
    /// [`GuildScheduledEvent`]: crate::guild::scheduled_event::GuildScheduledEvent
    pub const GUILD_SCHEDULED_EVENT_UPDATE: Self = Self::new(101);

    /// [`GuildScheduledEvent`] was deleted.
    ///
    /// [`GuildScheduledEvent`]: crate::guild::scheduled_event::GuildScheduledEvent
    pub const GUILD_SCHEDULED_EVENT_DELETE: Self = Self::new(102);

    /// Thread [channel] was created.
    ///
    /// [channel]: crate::channel::Channel
    pub const THREAD_CREATE: Self = Self::new(110);

    /// Thread [channel] was updated.
    ///
    /// [channel]: crate::channel::Channel
    pub const THREAD_UPDATE: Self = Self::new(111);

    /// Thread [channel] was deleted.
    ///
    /// [channel]: crate::channel::Channel
    pub const THREAD_DELETE: Self = Self::new(112);

    /// A [`GuildCommandPermissions`] was updated.
    ///
    /// [`GuildCommandPermissions`]: crate::application::command::permissions::GuildCommandPermissions
    pub const APPLICATION_COMMAND_PERMISSION_UPDATE: Self = Self::new(121);

    /// [`AutoModerationRule`] has been created.
    ///
    /// [`AutoModerationRule`]: crate::guild::auto_moderation::AutoModerationRule
    pub const AUTO_MODERATION_RULE_CREATE: Self = Self::new(140);

    /// [`AutoModerationRule`] has been updated.
    ///
    /// [`AutoModerationRule`]: crate::guild::auto_moderation::AutoModerationRule
    pub const AUTO_MODERATION_RULE_UPDATE: Self = Self::new(141);

    /// [`AutoModerationRule`] has been deleted.
    ///
    /// [`AutoModerationRule`]: crate::guild::auto_moderation::AutoModerationRule
    pub const AUTO_MODERATION_RULE_DELETE: Self = Self::new(142);

    /// Message has been blocked by Automod.
    pub const AUTO_MODERATION_BLOCK_MESSAGE: Self = Self::new(143);

    /// Message has been flagged by Automod.
    pub const AUTO_MODERATION_FLAG_TO_CHANNEL: Self = Self::new(144);

    /// A member has been timed out by Automod.
    pub const AUTO_MODERATION_USER_COMMUNICATION_DISABLED: Self = Self::new(145);

    /// Name of the associated constant.
    ///
    /// Returns `None` if the value doesn't have a defined constant.
    pub const fn name(self) -> Option<&'static str> {
        Some(match self {
            Self::GUILD_UPDATE => "GUILD_UPDATE",
            Self::CHANNEL_CREATE => "CHANNEL_CREATE",
            Self::CHANNEL_UPDATE => "CHANNEL_UPDATE",
            Self::CHANNEL_DELETE => "CHANNEL_DELETE",
            Self::CHANNEL_OVERWRITE_CREATE => "CHANNEL_OVERWRITE_CREATE",
            Self::CHANNEL_OVERWRITE_UPDATE => "CHANNEL_OVERWRITE_UPDATE",
            Self::CHANNEL_OVERWRITE_DELETE => "CHANNEL_OVERWRITE_DELETE",
            Self::MEMBER_KICK => "MEMBER_KICK",
            Self::MEMBER_PRUNE => "MEMBER_PRUNE",
            Self::MEMBER_BAN_ADD => "MEMBER_BAN_ADD",
            Self::MEMBER_BAN_REMOVE => "MEMBER_BAN_REMOVE",
            Self::MEMBER_UPDATE => "MEMBER_UPDATE",
            Self::MEMBER_ROLE_UPDATE => "MEMBER_ROLE_UPDATE",
            Self::MEMBER_MOVE => "MEMBER_MOVE",
            Self::MEMBER_DISCONNECT => "MEMBER_DISCONNECT",
            Self::BOT_ADD => "BOT_ADD",
            Self::ROLE_CREATE => "ROLE_CREATE",
            Self::ROLE_UPDATE => "ROLE_UPDATE",
            Self::ROLE_DELETE => "ROLE_DELETE",
            Self::INVITE_CREATE => "INVITE_CREATE",
            Self::INVITE_UPDATE => "INVITE_UPDATE",
            Self::INVITE_DELETE => "INVITE_DELETE",
            Self::WEBHOOK_CREATE => "WEBHOOK_CREATE",
            Self::WEBHOOK_UPDATE => "WEBHOOK_UPDATE",
            Self::WEBHOOK_DELETE => "WEBHOOK_DELETE",
            Self::EMOJI_CREATE => "EMOJI_CREATE",
            Self::EMOJI_UPDATE => "EMOJI_UPDATE",
            Self::EMOJI_DELETE => "EMOJI_DELETE",
            Self::MESSAGE_DELETE => "MESSAGE_DELETE",
            Self::MESSAGE_BULK_DELETE => "MESSAGE_BULK_DELETE",
            Self::MESSAGE_PIN => "MESSAGE_PIN",
            Self::MESSAGE_UNPIN => "MESSAGE_UNPIN",
            Self::INTEGRATION_CREATE => "INTEGRATION_CREATE",
            Self::INTEGRATION_UPDATE => "INTEGRATION_UPDATE",
            Self::INTEGRATION_DELETE => "INTEGRATION_DELETE",
            Self::STAGE_INSTANCE_CREATE => "STAGE_INSTANCE_CREATE",
            Self::STAGE_INSTANCE_UPDATE => "STAGE_INSTANCE_UPDATE",
            Self::STAGE_INSTANCE_DELETE => "STAGE_INSTANCE_DELETE",
            Self::STICKER_CREATE => "STICKER_CREATE",
            Self::STICKER_UPDATE => "STICKER_UPDATE",
            Self::STICKER_DELETE => "STICKER_DELETE",
            Self::GUILD_SCHEDULED_EVENT_CREATE => "GUILD_SCHEDULED_EVENT_CREATE",
            Self::GUILD_SCHEDULED_EVENT_UPDATE => "GUILD_SCHEDULED_EVENT_UPDATE",
            Self::GUILD_SCHEDULED_EVENT_DELETE => "GUILD_SCHEDULED_EVENT_DELETE",
            Self::THREAD_CREATE => "THREAD_CREATE",
            Self::THREAD_UPDATE => "THREAD_UPDATE",
            Self::THREAD_DELETE => "THREAD_DELETE",
            Self::APPLICATION_COMMAND_PERMISSION_UPDATE => "APPLICATION_COMMAND_PERMISSION_UPDATE",
            Self::AUTO_MODERATION_RULE_CREATE => "AUTO_MODERATION_RULE_CREATE",
            Self::AUTO_MODERATION_RULE_UPDATE => "AUTO_MODERATION_RULE_UPDATE",
            Self::AUTO_MODERATION_RULE_DELETE => "AUTO_MODERATION_RULE_DELETE",
            Self::AUTO_MODERATION_BLOCK_MESSAGE => "AUTO_MODERATION_BLOCK_MESSAGE",
            Self::AUTO_MODERATION_FLAG_TO_CHANNEL => "AUTO_MODERATION_FLAG_TO_CHANNEL",
            Self::AUTO_MODERATION_USER_COMMUNICATION_DISABLED => {
                "AUTO_MODERATION_USER_COMMUNICATION_DISABLED"
            }
            _ => return None,
        })
    }
}

impl_typed!(AuditLogEventType, u16);

#[cfg(test)]
mod tests {
    use super::AuditLogEventType;
    use serde::{Deserialize, Serialize};
    use static_assertions::assert_impl_all;
    use std::{fmt::Debug, hash::Hash};

    assert_impl_all!(
        AuditLogEventType: Clone,
        Copy,
        Debug,
        Deserialize<'static>,
        Eq,
        Hash,
        PartialEq,
        Send,
        Serialize,
        Sync,
    );

    #[test]
    fn test_values() {
        assert_eq!(1, u16::from(AuditLogEventType::GUILD_UPDATE));
        assert_eq!(10, u16::from(AuditLogEventType::CHANNEL_CREATE));
        assert_eq!(11, u16::from(AuditLogEventType::CHANNEL_UPDATE));
        assert_eq!(12, u16::from(AuditLogEventType::CHANNEL_DELETE));
        assert_eq!(13, u16::from(AuditLogEventType::CHANNEL_OVERWRITE_CREATE));
        assert_eq!(14, u16::from(AuditLogEventType::CHANNEL_OVERWRITE_UPDATE));
        assert_eq!(15, u16::from(AuditLogEventType::CHANNEL_OVERWRITE_DELETE));
        assert_eq!(20, u16::from(AuditLogEventType::MEMBER_KICK));
        assert_eq!(21, u16::from(AuditLogEventType::MEMBER_PRUNE));
        assert_eq!(22, u16::from(AuditLogEventType::MEMBER_BAN_ADD));
        assert_eq!(23, u16::from(AuditLogEventType::MEMBER_BAN_REMOVE));
        assert_eq!(24, u16::from(AuditLogEventType::MEMBER_UPDATE));
        assert_eq!(25, u16::from(AuditLogEventType::MEMBER_ROLE_UPDATE));
        assert_eq!(26, u16::from(AuditLogEventType::MEMBER_MOVE));
        assert_eq!(27, u16::from(AuditLogEventType::MEMBER_DISCONNECT));
        assert_eq!(28, u16::from(AuditLogEventType::BOT_ADD));
        assert_eq!(30, u16::from(AuditLogEventType::ROLE_CREATE));
        assert_eq!(31, u16::from(AuditLogEventType::ROLE_UPDATE));
        assert_eq!(32, u16::from(AuditLogEventType::ROLE_DELETE));
        assert_eq!(40, u16::from(AuditLogEventType::INVITE_CREATE));
        assert_eq!(41, u16::from(AuditLogEventType::INVITE_UPDATE));
        assert_eq!(42, u16::from(AuditLogEventType::INVITE_DELETE));
        assert_eq!(50, u16::from(AuditLogEventType::WEBHOOK_CREATE));
        assert_eq!(51, u16::from(AuditLogEventType::WEBHOOK_UPDATE));
        assert_eq!(52, u16::from(AuditLogEventType::WEBHOOK_DELETE));
        assert_eq!(60, u16::from(AuditLogEventType::EMOJI_CREATE));
        assert_eq!(61, u16::from(AuditLogEventType::EMOJI_UPDATE));
        assert_eq!(62, u16::from(AuditLogEventType::EMOJI_DELETE));
        assert_eq!(72, u16::from(AuditLogEventType::MESSAGE_DELETE));
        assert_eq!(73, u16::from(AuditLogEventType::MESSAGE_BULK_DELETE));
        assert_eq!(74, u16::from(AuditLogEventType::MESSAGE_PIN));
        assert_eq!(75, u16::from(AuditLogEventType::MESSAGE_UNPIN));
        assert_eq!(80, u16::from(AuditLogEventType::INTEGRATION_CREATE));
        assert_eq!(81, u16::from(AuditLogEventType::INTEGRATION_UPDATE));
        assert_eq!(82, u16::from(AuditLogEventType::INTEGRATION_DELETE));
        assert_eq!(83, u16::from(AuditLogEventType::STAGE_INSTANCE_CREATE));
        assert_eq!(84, u16::from(AuditLogEventType::STAGE_INSTANCE_UPDATE));
        assert_eq!(90, u16::from(AuditLogEventType::STICKER_CREATE));
        assert_eq!(91, u16::from(AuditLogEventType::STICKER_UPDATE));
        assert_eq!(92, u16::from(AuditLogEventType::STICKER_DELETE));
        assert_eq!(
            100,
            u16::from(AuditLogEventType::GUILD_SCHEDULED_EVENT_CREATE)
        );
        assert_eq!(
            101,
            u16::from(AuditLogEventType::GUILD_SCHEDULED_EVENT_UPDATE)
        );
        assert_eq!(
            102,
            u16::from(AuditLogEventType::GUILD_SCHEDULED_EVENT_DELETE)
        );
        assert_eq!(110, u16::from(AuditLogEventType::THREAD_CREATE));
        assert_eq!(111, u16::from(AuditLogEventType::THREAD_UPDATE));
        assert_eq!(112, u16::from(AuditLogEventType::THREAD_DELETE));
        assert_eq!(
            121,
            u16::from(AuditLogEventType::APPLICATION_COMMAND_PERMISSION_UPDATE)
        );
        assert_eq!(
            140,
            u16::from(AuditLogEventType::AUTO_MODERATION_RULE_CREATE)
        );
        assert_eq!(
            141,
            u16::from(AuditLogEventType::AUTO_MODERATION_RULE_UPDATE)
        );
        assert_eq!(
            142,
            u16::from(AuditLogEventType::AUTO_MODERATION_RULE_DELETE)
        );
        assert_eq!(
            143,
            u16::from(AuditLogEventType::AUTO_MODERATION_BLOCK_MESSAGE)
        );
        assert_eq!(
            144,
            u16::from(AuditLogEventType::AUTO_MODERATION_FLAG_TO_CHANNEL)
        );
        assert_eq!(
            145,
            u16::from(AuditLogEventType::AUTO_MODERATION_USER_COMMUNICATION_DISABLED)
        );
        assert_eq!(250, u16::from(AuditLogEventType::new(250)));
    }
}
