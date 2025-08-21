use serde::{Deserialize, Serialize};

/// Action to cause an [`AuditLogEntry`].
///
/// [`AuditLogEntry`]: super::AuditLogEntry
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
#[serde(from = "u16", into = "u16")]
pub enum AuditLogEventType {
    /// [Guild] was updated.
    ///
    /// [Guild]: crate::guild::Guild
    GuildUpdate,
    /// [Channel] was created.
    ///
    /// [Channel]: crate::channel::Channel
    ChannelCreate,
    /// [Channel] was updated.
    ///
    /// [Channel]: crate::channel::Channel
    ChannelUpdate,
    /// [Channel] was deleted.
    ///
    /// [Channel]: crate::channel::Channel
    ChannelDelete,
    /// [Permission overwrite] for a [channel] was created.
    ///
    /// [channel]: crate::channel::Channel
    /// [Permission overwrite]: crate::channel::permission_overwrite::PermissionOverwrite
    ChannelOverwriteCreate,
    /// [Permission overwrite] for a [channel] was updated.
    ///
    /// [channel]: crate::channel::Channel
    /// [Permission overwrite]: crate::channel::permission_overwrite::PermissionOverwrite
    ChannelOverwriteUpdate,
    /// [Permission overwrite] for a [channel] was deleted.
    ///
    /// [channel]: crate::channel::Channel
    /// [Permission overwrite]: crate::channel::permission_overwrite::PermissionOverwrite
    ChannelOverwriteDelete,
    /// [Member] was kicked.
    ///
    /// [Member]: crate::guild::Member
    MemberKick,
    /// [Member] prune began.
    ///
    /// [Member]: crate::guild::Member
    MemberPrune,
    /// [Member] was banned.
    ///
    /// [Member]: crate::guild::Member
    MemberBanAdd,
    /// [Member]'s [ban] was removed.
    ///
    /// [ban]: crate::guild::Ban
    /// [Member]: crate::guild::Member
    MemberBanRemove,
    /// [Member] was updated.
    ///
    /// [Member]: crate::guild::Member
    MemberUpdate,
    /// [Member] either had a [role] attached or removed.
    ///
    /// [Member]: crate::guild::Member
    /// [role]: crate::guild::Role
    MemberRoleUpdate,
    /// [Member] was moved between voice [channel]s.
    ///
    /// [Member]: crate::guild::Member
    /// [channel]: crate::channel::Channel
    MemberMove,
    /// [Member] was disconnected from a voice [channel].
    ///
    /// [Member]: crate::guild::Member
    /// [channel]: crate::channel::Channel
    MemberDisconnect,
    /// [Bot user] was added to a [guild].
    ///
    /// [Bot user]: crate::user::User::bot
    /// [guild]: crate::guild::Guild
    BotAdd,
    /// [Role] was created.
    ///
    /// [Role]: crate::guild::Role
    RoleCreate,
    /// [Role] was updated.
    ///
    /// [Role]: crate::guild::Role
    RoleUpdate,
    /// [Role] was deleted.
    ///
    /// [Role]: crate::guild::Role
    RoleDelete,
    /// [Invite] was created.
    ///
    /// [Invite]: crate::guild::invite::Invite
    InviteCreate,
    /// [Invite] was updated.
    ///
    /// [Invite]: crate::guild::invite::Invite
    InviteUpdate,
    /// [Invite] was deleted.
    ///
    /// [Invite]: crate::guild::invite::Invite
    InviteDelete,
    /// [Webhook] was created.
    ///
    /// [Webhook]: crate::channel::webhook::Webhook
    WebhookCreate,
    /// [Webhook] was updated.
    ///
    /// [Webhook]: crate::channel::webhook::Webhook
    WebhookUpdate,
    /// [Webhook] was deleted.
    ///
    /// [Webhook]: crate::channel::webhook::Webhook
    WebhookDelete,
    /// [Emoji] was created.
    ///
    /// [Emoji]: crate::guild::Emoji
    EmojiCreate,
    /// [Emoji] was updated.
    ///
    /// [Emoji]: crate::guild::Emoji
    EmojiUpdate,
    /// [Emoji] was deleted.
    ///
    /// [Emoji]: crate::guild::Emoji
    EmojiDelete,
    /// [Message] was deleted.
    ///
    /// [Message]: crate::channel::message::Message
    MessageDelete,
    /// Multiple [messages] were deleted.
    ///
    /// [messages]: crate::channel::message::Message
    MessageBulkDelete,
    /// [Message] was pinned to a [channel].
    ///
    /// [Message]: crate::channel::message::Message
    /// [channel]: crate::channel::Channel
    MessagePin,
    /// [Message] was unpinned from a [channel].
    ///
    /// [Message]: crate::channel::message::Message
    /// [channel]: crate::channel::Channel
    MessageUnpin,
    /// [Integration] was created.
    ///
    /// [Integration]: crate::guild::GuildIntegration
    IntegrationCreate,
    /// [Integration] was updated.
    ///
    /// [Integration]: crate::guild::GuildIntegration
    IntegrationUpdate,
    /// [Integration] was deleted.
    ///
    /// [Integration]: crate::guild::GuildIntegration
    IntegrationDelete,
    /// [Stage instance] was created.
    ///
    /// [Stage instance]: crate::channel::stage_instance::StageInstance
    StageInstanceCreate,
    /// [Stage instance] was updated.
    ///
    /// [Stage instance]: crate::channel::stage_instance::StageInstance
    StageInstanceUpdate,
    /// [Stage instance] was deleted.
    ///
    /// [Stage instance]: crate::channel::stage_instance::StageInstance
    StageInstanceDelete,
    /// [Sticker] was created.
    ///
    /// [Sticker]: crate::channel::message::sticker::Sticker
    StickerCreate,
    /// [Sticker] was updated.
    ///
    /// [Sticker]: crate::channel::message::sticker::Sticker
    StickerUpdate,
    /// [Sticker] was deleted.
    ///
    /// [Sticker]: crate::channel::message::sticker::Sticker
    StickerDelete,
    /// [`GuildScheduledEvent`] was created.
    ///
    /// [`GuildScheduledEvent`]: crate::guild::scheduled_event::GuildScheduledEvent
    GuildScheduledEventCreate,
    /// [`GuildScheduledEvent`] was updated.
    ///
    /// [`GuildScheduledEvent`]: crate::guild::scheduled_event::GuildScheduledEvent
    GuildScheduledEventUpdate,
    /// [`GuildScheduledEvent`] was deleted.
    ///
    /// [`GuildScheduledEvent`]: crate::guild::scheduled_event::GuildScheduledEvent
    GuildScheduledEventDelete,
    /// Thread [channel] was created.
    ///
    /// [channel]: crate::channel::Channel
    ThreadCreate,
    /// Thread [channel] was updated.
    ///
    /// [channel]: crate::channel::Channel
    ThreadUpdate,
    /// Thread [channel] was deleted.
    ///
    /// [channel]: crate::channel::Channel
    ThreadDelete,
    /// A [`GuildCommandPermissions`] was updated.
    ///
    /// [`GuildCommandPermissions`]: crate::application::command::permissions::GuildCommandPermissions
    ApplicationCommandPermissionUpdate,
    /// [`AutoModerationRule`] has been created.
    ///
    /// [`AutoModerationRule`]: crate::guild::auto_moderation::AutoModerationRule
    AutoModerationRuleCreate,
    /// [`AutoModerationRule`] has been updated.
    ///
    /// [`AutoModerationRule`]: crate::guild::auto_moderation::AutoModerationRule
    AutoModerationRuleUpdate,
    /// [`AutoModerationRule`] has been deleted.
    ///
    /// [`AutoModerationRule`]: crate::guild::auto_moderation::AutoModerationRule
    AutoModerationRuleDelete,
    /// Message has been blocked by AutoMod.
    AutoModerationBlockMessage,
    /// Message has been flagged by AutoMod.
    AutoModerationFlagToChannel,
    /// A member has been timed out by AutoMod.
    AutoModerationUserCommunicationDisabled,
    /// Creator monetization request was created.
    CreatorMonetizationRequestCreated,
    /// Creator monetization terms were accepted.
    CreatorMonetizationTermsAccepted,
    /// A voice channel status was updated by a user.
    VoiceChannelStatusUpdate,
    /// A voice channel status was deleted by a user.
    VoiceChannelStatusDelete,
    /// Variant value is unknown to the library.
    Unknown(u16),
}

impl From<u16> for AuditLogEventType {
    fn from(value: u16) -> Self {
        match value {
            1 => AuditLogEventType::GuildUpdate,
            10 => AuditLogEventType::ChannelCreate,
            11 => AuditLogEventType::ChannelUpdate,
            12 => AuditLogEventType::ChannelDelete,
            13 => AuditLogEventType::ChannelOverwriteCreate,
            14 => AuditLogEventType::ChannelOverwriteUpdate,
            15 => AuditLogEventType::ChannelOverwriteDelete,
            20 => AuditLogEventType::MemberKick,
            21 => AuditLogEventType::MemberPrune,
            22 => AuditLogEventType::MemberBanAdd,
            23 => AuditLogEventType::MemberBanRemove,
            24 => AuditLogEventType::MemberUpdate,
            25 => AuditLogEventType::MemberRoleUpdate,
            26 => AuditLogEventType::MemberMove,
            17 => AuditLogEventType::MemberDisconnect,
            28 => AuditLogEventType::BotAdd,
            30 => AuditLogEventType::RoleCreate,
            31 => AuditLogEventType::RoleUpdate,
            32 => AuditLogEventType::RoleDelete,
            40 => AuditLogEventType::InviteCreate,
            41 => AuditLogEventType::InviteUpdate,
            42 => AuditLogEventType::InviteDelete,
            50 => AuditLogEventType::WebhookCreate,
            51 => AuditLogEventType::WebhookUpdate,
            52 => AuditLogEventType::WebhookDelete,
            60 => AuditLogEventType::EmojiCreate,
            61 => AuditLogEventType::EmojiUpdate,
            62 => AuditLogEventType::EmojiDelete,
            72 => AuditLogEventType::MessageDelete,
            73 => AuditLogEventType::MessageBulkDelete,
            74 => AuditLogEventType::MessagePin,
            75 => AuditLogEventType::MessageUnpin,
            80 => AuditLogEventType::IntegrationCreate,
            81 => AuditLogEventType::IntegrationUpdate,
            82 => AuditLogEventType::IntegrationDelete,
            83 => AuditLogEventType::StageInstanceCreate,
            84 => AuditLogEventType::StageInstanceUpdate,
            85 => AuditLogEventType::StageInstanceDelete,
            90 => AuditLogEventType::StickerCreate,
            91 => AuditLogEventType::StickerUpdate,
            92 => AuditLogEventType::StickerDelete,
            100 => AuditLogEventType::GuildScheduledEventCreate,
            101 => AuditLogEventType::GuildScheduledEventUpdate,
            102 => AuditLogEventType::GuildScheduledEventDelete,
            110 => AuditLogEventType::ThreadCreate,
            111 => AuditLogEventType::ThreadUpdate,
            112 => AuditLogEventType::ThreadDelete,
            121 => AuditLogEventType::ApplicationCommandPermissionUpdate,
            140 => AuditLogEventType::AutoModerationRuleCreate,
            141 => AuditLogEventType::AutoModerationRuleUpdate,
            142 => AuditLogEventType::AutoModerationRuleDelete,
            143 => AuditLogEventType::AutoModerationBlockMessage,
            144 => AuditLogEventType::AutoModerationFlagToChannel,
            145 => AuditLogEventType::AutoModerationUserCommunicationDisabled,
            150 => AuditLogEventType::CreatorMonetizationRequestCreated,
            151 => AuditLogEventType::CreatorMonetizationTermsAccepted,
            192 => AuditLogEventType::VoiceChannelStatusUpdate,
            193 => AuditLogEventType::VoiceChannelStatusDelete,
            unknown => AuditLogEventType::Unknown(unknown),
        }
    }
}

impl From<AuditLogEventType> for u16 {
    fn from(value: AuditLogEventType) -> Self {
        match value {
            AuditLogEventType::GuildUpdate => 1,
            AuditLogEventType::ChannelCreate => 10,
            AuditLogEventType::ChannelUpdate => 11,
            AuditLogEventType::ChannelDelete => 12,
            AuditLogEventType::ChannelOverwriteCreate => 13,
            AuditLogEventType::ChannelOverwriteUpdate => 14,
            AuditLogEventType::ChannelOverwriteDelete => 15,
            AuditLogEventType::MemberKick => 20,
            AuditLogEventType::MemberPrune => 21,
            AuditLogEventType::MemberBanAdd => 22,
            AuditLogEventType::MemberBanRemove => 23,
            AuditLogEventType::MemberUpdate => 24,
            AuditLogEventType::MemberRoleUpdate => 25,
            AuditLogEventType::MemberMove => 26,
            AuditLogEventType::MemberDisconnect => 27,
            AuditLogEventType::BotAdd => 28,
            AuditLogEventType::RoleCreate => 30,
            AuditLogEventType::RoleUpdate => 31,
            AuditLogEventType::RoleDelete => 32,
            AuditLogEventType::InviteCreate => 40,
            AuditLogEventType::InviteUpdate => 41,
            AuditLogEventType::InviteDelete => 42,
            AuditLogEventType::WebhookCreate => 50,
            AuditLogEventType::WebhookUpdate => 51,
            AuditLogEventType::WebhookDelete => 52,
            AuditLogEventType::EmojiCreate => 60,
            AuditLogEventType::EmojiUpdate => 61,
            AuditLogEventType::EmojiDelete => 62,
            AuditLogEventType::MessageDelete => 72,
            AuditLogEventType::MessageBulkDelete => 73,
            AuditLogEventType::MessagePin => 74,
            AuditLogEventType::MessageUnpin => 75,
            AuditLogEventType::IntegrationCreate => 80,
            AuditLogEventType::IntegrationUpdate => 81,
            AuditLogEventType::IntegrationDelete => 82,
            AuditLogEventType::StageInstanceCreate => 83,
            AuditLogEventType::StageInstanceUpdate => 84,
            AuditLogEventType::StageInstanceDelete => 85,
            AuditLogEventType::StickerCreate => 90,
            AuditLogEventType::StickerUpdate => 91,
            AuditLogEventType::StickerDelete => 92,
            AuditLogEventType::GuildScheduledEventCreate => 100,
            AuditLogEventType::GuildScheduledEventUpdate => 101,
            AuditLogEventType::GuildScheduledEventDelete => 102,
            AuditLogEventType::ThreadCreate => 110,
            AuditLogEventType::ThreadUpdate => 111,
            AuditLogEventType::ThreadDelete => 112,
            AuditLogEventType::ApplicationCommandPermissionUpdate => 121,
            AuditLogEventType::AutoModerationRuleCreate => 140,
            AuditLogEventType::AutoModerationRuleUpdate => 141,
            AuditLogEventType::AutoModerationRuleDelete => 142,
            AuditLogEventType::AutoModerationBlockMessage => 143,
            AuditLogEventType::AutoModerationFlagToChannel => 144,
            AuditLogEventType::AutoModerationUserCommunicationDisabled => 145,
            AuditLogEventType::CreatorMonetizationRequestCreated => 150,
            AuditLogEventType::CreatorMonetizationTermsAccepted => 151,
            AuditLogEventType::VoiceChannelStatusUpdate => 192,
            AuditLogEventType::VoiceChannelStatusDelete => 193,
            AuditLogEventType::Unknown(unknown) => unknown,
        }
    }
}

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
        assert_eq!(1, u16::from(AuditLogEventType::GuildUpdate));
        assert_eq!(10, u16::from(AuditLogEventType::ChannelCreate));
        assert_eq!(11, u16::from(AuditLogEventType::ChannelUpdate));
        assert_eq!(12, u16::from(AuditLogEventType::ChannelDelete));
        assert_eq!(13, u16::from(AuditLogEventType::ChannelOverwriteCreate));
        assert_eq!(14, u16::from(AuditLogEventType::ChannelOverwriteUpdate));
        assert_eq!(15, u16::from(AuditLogEventType::ChannelOverwriteDelete));
        assert_eq!(20, u16::from(AuditLogEventType::MemberKick));
        assert_eq!(21, u16::from(AuditLogEventType::MemberPrune));
        assert_eq!(22, u16::from(AuditLogEventType::MemberBanAdd));
        assert_eq!(23, u16::from(AuditLogEventType::MemberBanRemove));
        assert_eq!(24, u16::from(AuditLogEventType::MemberUpdate));
        assert_eq!(25, u16::from(AuditLogEventType::MemberRoleUpdate));
        assert_eq!(26, u16::from(AuditLogEventType::MemberMove));
        assert_eq!(27, u16::from(AuditLogEventType::MemberDisconnect));
        assert_eq!(28, u16::from(AuditLogEventType::BotAdd));
        assert_eq!(30, u16::from(AuditLogEventType::RoleCreate));
        assert_eq!(31, u16::from(AuditLogEventType::RoleUpdate));
        assert_eq!(32, u16::from(AuditLogEventType::RoleDelete));
        assert_eq!(40, u16::from(AuditLogEventType::InviteCreate));
        assert_eq!(41, u16::from(AuditLogEventType::InviteUpdate));
        assert_eq!(42, u16::from(AuditLogEventType::InviteDelete));
        assert_eq!(50, u16::from(AuditLogEventType::WebhookCreate));
        assert_eq!(51, u16::from(AuditLogEventType::WebhookUpdate));
        assert_eq!(52, u16::from(AuditLogEventType::WebhookDelete));
        assert_eq!(60, u16::from(AuditLogEventType::EmojiCreate));
        assert_eq!(61, u16::from(AuditLogEventType::EmojiUpdate));
        assert_eq!(62, u16::from(AuditLogEventType::EmojiDelete));
        assert_eq!(72, u16::from(AuditLogEventType::MessageDelete));
        assert_eq!(73, u16::from(AuditLogEventType::MessageBulkDelete));
        assert_eq!(74, u16::from(AuditLogEventType::MessagePin));
        assert_eq!(75, u16::from(AuditLogEventType::MessageUnpin));
        assert_eq!(80, u16::from(AuditLogEventType::IntegrationCreate));
        assert_eq!(81, u16::from(AuditLogEventType::IntegrationUpdate));
        assert_eq!(82, u16::from(AuditLogEventType::IntegrationDelete));
        assert_eq!(83, u16::from(AuditLogEventType::StageInstanceCreate));
        assert_eq!(84, u16::from(AuditLogEventType::StageInstanceUpdate));
        assert_eq!(90, u16::from(AuditLogEventType::StickerCreate));
        assert_eq!(91, u16::from(AuditLogEventType::StickerUpdate));
        assert_eq!(92, u16::from(AuditLogEventType::StickerDelete));
        assert_eq!(100, u16::from(AuditLogEventType::GuildScheduledEventCreate));
        assert_eq!(101, u16::from(AuditLogEventType::GuildScheduledEventUpdate));
        assert_eq!(102, u16::from(AuditLogEventType::GuildScheduledEventDelete));
        assert_eq!(110, u16::from(AuditLogEventType::ThreadCreate));
        assert_eq!(111, u16::from(AuditLogEventType::ThreadUpdate));
        assert_eq!(112, u16::from(AuditLogEventType::ThreadDelete));
        assert_eq!(
            121,
            u16::from(AuditLogEventType::ApplicationCommandPermissionUpdate)
        );
        assert_eq!(140, u16::from(AuditLogEventType::AutoModerationRuleCreate));
        assert_eq!(141, u16::from(AuditLogEventType::AutoModerationRuleUpdate));
        assert_eq!(142, u16::from(AuditLogEventType::AutoModerationRuleDelete));
        assert_eq!(
            143,
            u16::from(AuditLogEventType::AutoModerationBlockMessage)
        );
        assert_eq!(
            144,
            u16::from(AuditLogEventType::AutoModerationFlagToChannel)
        );
        assert_eq!(
            145,
            u16::from(AuditLogEventType::AutoModerationUserCommunicationDisabled)
        );
        assert_eq!(
            150,
            u16::from(AuditLogEventType::CreatorMonetizationRequestCreated)
        );
        assert_eq!(
            151,
            u16::from(AuditLogEventType::CreatorMonetizationTermsAccepted)
        );
        assert_eq!(192, u16::from(AuditLogEventType::VoiceChannelStatusUpdate));
        assert_eq!(193, u16::from(AuditLogEventType::VoiceChannelStatusDelete));
        assert_eq!(250, u16::from(AuditLogEventType::Unknown(250)));
    }
}
