use super::DispatchEventType;
use serde::{Deserialize, Serialize};

/// Type of an [`Event`].
///
/// The variants starting with `Gateway` are Twilight-specific event types and
/// therefore lack a name and can not be (de)serialized.
///
/// [`Event`]: super::Event
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum EventType {
    AutoModerationActionExecution,
    AutoModerationRuleCreate,
    AutoModerationRuleDelete,
    AutoModerationRuleUpdate,
    #[serde(rename = "GUILD_BAN_ADD")]
    BanAdd,
    #[serde(rename = "GUILD_BAN_REMOVE")]
    BanRemove,
    ChannelCreate,
    ChannelDelete,
    ChannelPinsUpdate,
    ChannelUpdate,
    #[serde(rename = "APPLICATION_COMMAND_PERMISSIONS_UPDATE")]
    CommandPermissionsUpdate,
    #[serde(skip)]
    GatewayClose,
    #[serde(skip)]
    GatewayHeartbeat,
    #[serde(skip)]
    GatewayHeartbeatAck,
    #[serde(skip)]
    GatewayHello,
    #[serde(skip)]
    GatewayInvalidateSession,
    #[serde(skip)]
    GatewayReconnect,
    GiftCodeUpdate,
    GuildAuditLogEntryCreate,
    GuildCreate,
    GuildDelete,
    GuildEmojisUpdate,
    GuildIntegrationsUpdate,
    GuildScheduledEventCreate,
    GuildScheduledEventDelete,
    GuildScheduledEventUpdate,
    GuildScheduledEventUserAdd,
    GuildScheduledEventUserRemove,
    GuildStickersUpdate,
    GuildUpdate,
    IntegrationCreate,
    IntegrationDelete,
    IntegrationUpdate,
    InteractionCreate,
    InviteCreate,
    InviteDelete,
    #[serde(rename = "GUILD_MEMBER_ADD")]
    MemberAdd,
    #[serde(rename = "GUILD_MEMBERS_CHUNK")]
    MemberChunk,
    #[serde(rename = "GUILD_MEMBER_REMOVE")]
    MemberRemove,
    #[serde(rename = "GUILD_MEMBER_UPDATE")]
    MemberUpdate,
    MessageCreate,
    MessageDelete,
    MessageDeleteBulk,
    MessageUpdate,
    PresenceUpdate,
    PresencesReplace,
    #[serde(rename = "MESSAGE_REACTION_ADD")]
    ReactionAdd,
    #[serde(rename = "MESSAGE_REACTION_REMOVE")]
    ReactionRemove,
    #[serde(rename = "MESSAGE_REACTION_REMOVE_ALL")]
    ReactionRemoveAll,
    #[serde(rename = "MESSAGE_REACTION_REMOVE_EMOJI")]
    ReactionRemoveEmoji,
    Ready,
    Resumed,
    #[serde(rename = "GUILD_ROLE_CREATE")]
    RoleCreate,
    #[serde(rename = "GUILD_ROLE_DELETE")]
    RoleDelete,
    #[serde(rename = "GUILD_ROLE_UPDATE")]
    RoleUpdate,
    StageInstanceCreate,
    StageInstanceDelete,
    StageInstanceUpdate,
    ThreadCreate,
    ThreadDelete,
    ThreadListSync,
    ThreadMemberUpdate,
    ThreadMembersUpdate,
    ThreadUpdate,
    TypingStart,
    UnavailableGuild,
    UserUpdate,
    VoiceServerUpdate,
    VoiceStateUpdate,
    WebhooksUpdate,
}

impl EventType {
    /// Discord event name.
    ///
    /// Returns [`None`] for the variants starting with `Gateway`.
    pub const fn name(self) -> Option<&'static str> {
        match self {
            Self::AutoModerationActionExecution => Some("AUTO_MODERATION_ACTION_EXECUTION"),
            Self::AutoModerationRuleCreate => Some("AUTO_MODERATION_RULE_CREATE"),
            Self::AutoModerationRuleDelete => Some("AUTO_MODERATION_RULE_DELETE"),
            Self::AutoModerationRuleUpdate => Some("AUTO_MODERATION_RULE_UPDATE"),
            Self::BanAdd => Some("GUILD_BAN_ADD"),
            Self::BanRemove => Some("GUILD_BAN_REMOVE"),
            Self::ChannelCreate => Some("CHANNEL_CREATE"),
            Self::ChannelDelete => Some("CHANNEL_DELETE"),
            Self::ChannelPinsUpdate => Some("CHANNEL_PINS_UPDATE"),
            Self::ChannelUpdate => Some("CHANNEL_UPDATE"),
            Self::CommandPermissionsUpdate => Some("APPLICATION_COMMAND_PERMISSIONS_UPDATE"),
            Self::GatewayClose
            | Self::GatewayHeartbeat
            | Self::GatewayHeartbeatAck
            | Self::GatewayHello
            | Self::GatewayInvalidateSession
            | Self::GatewayReconnect => None,
            Self::GiftCodeUpdate => Some("GIFT_CODE_UPDATE"),
            Self::GuildAuditLogEntryCreate => Some("GUILD_AUDIT_LOG_ENTRY_CREATE"),
            Self::GuildCreate => Some("GUILD_CREATE"),
            Self::GuildDelete => Some("GUILD_DELETE"),
            Self::GuildEmojisUpdate => Some("GUILD_EMOJIS_UPDATE"),
            Self::GuildIntegrationsUpdate => Some("GUILD_INTEGRATIONS_UPDATE"),
            Self::GuildScheduledEventCreate => Some("GUILD_SCHEDULED_EVENT_CREATE"),
            Self::GuildScheduledEventDelete => Some("GUILD_SCHEDULED_EVENT_DELETE"),
            Self::GuildScheduledEventUpdate => Some("GUILD_SCHEDULED_EVENT_UPDATE"),
            Self::GuildScheduledEventUserAdd => Some("GUILD_SCHEDULED_EVENT_USER_ADD"),
            Self::GuildScheduledEventUserRemove => Some("GUILD_SCHEDULED_EVENT_USER_REMOVE"),
            Self::GuildStickersUpdate => Some("GUILD_STICKERS_UPDATE"),
            Self::GuildUpdate => Some("GUILD_UPDATE"),
            Self::IntegrationCreate => Some("INTEGRATION_CREATE"),
            Self::IntegrationDelete => Some("INTEGRATION_DELETE"),
            Self::IntegrationUpdate => Some("INTEGRATION_UPDATE"),
            Self::InteractionCreate => Some("INTERACTION_CREATE"),
            Self::InviteCreate => Some("INVITE_CREATE"),
            Self::InviteDelete => Some("INVITE_DELETE"),
            Self::MemberAdd => Some("GUILD_MEMBER_ADD"),
            Self::MemberChunk => Some("GUILD_MEMBERS_CHUNK"),
            Self::MemberRemove => Some("GUILD_MEMBER_REMOVE"),
            Self::MemberUpdate => Some("GUILD_MEMBER_UPDATE"),
            Self::MessageCreate => Some("MESSAGE_CREATE"),
            Self::MessageDelete => Some("MESSAGE_DELETE"),
            Self::MessageDeleteBulk => Some("MESSAGE_DELETE_BULK"),
            Self::MessageUpdate => Some("MESSAGE_UPDATE"),
            Self::PresencesReplace => Some("PRESENCES_REPLACE"),
            Self::PresenceUpdate => Some("PRESENCE_UPDATE"),
            Self::ReactionAdd => Some("MESSAGE_REACTION_ADD"),
            Self::ReactionRemove => Some("MESSAGE_REACTION_REMOVE"),
            Self::ReactionRemoveAll => Some("MESSAGE_REACTION_REMOVE_ALL"),
            Self::ReactionRemoveEmoji => Some("MESSAGE_REACTION_REMOVE_EMOJI"),
            Self::Ready => Some("READY"),
            Self::Resumed => Some("RESUMED"),
            Self::RoleCreate => Some("GUILD_ROLE_CREATE"),
            Self::RoleDelete => Some("GUILD_ROLE_DELETE"),
            Self::RoleUpdate => Some("GUILD_ROLE_UPDATE"),
            Self::StageInstanceCreate => Some("STAGE_INSTANCE_CREATE"),
            Self::StageInstanceDelete => Some("STAGE_INSTANCE_DELETE"),
            Self::StageInstanceUpdate => Some("STAGE_INSTANCE_UPDATE"),
            Self::ThreadCreate => Some("THREAD_CREATE"),
            Self::ThreadDelete => Some("THREAD_DELETE"),
            Self::ThreadListSync => Some("THREAD_LIST_SYNC"),
            Self::ThreadMembersUpdate => Some("THREAD_MEMBERS_UPDATE"),
            Self::ThreadMemberUpdate => Some("THREAD_MEMBER_UPDATE"),
            Self::ThreadUpdate => Some("THREAD_UPDATE"),
            Self::TypingStart => Some("TYPING_START"),
            Self::UnavailableGuild => Some("UNAVAILABLE_GUILD"),
            Self::UserUpdate => Some("USER_UPDATE"),
            Self::VoiceServerUpdate => Some("VOICE_SERVER_UPDATE"),
            Self::VoiceStateUpdate => Some("VOICE_STATE_UPDATE"),
            Self::WebhooksUpdate => Some("WEBHOOKS_UPDATE"),
        }
    }
}

impl From<DispatchEventType> for EventType {
    fn from(event_type: DispatchEventType) -> Self {
        match event_type {
            DispatchEventType::AutoModerationActionExecution => Self::AutoModerationActionExecution,
            DispatchEventType::AutoModerationRuleCreate => Self::AutoModerationRuleCreate,
            DispatchEventType::AutoModerationRuleDelete => Self::AutoModerationRuleDelete,
            DispatchEventType::AutoModerationRuleUpdate => Self::AutoModerationRuleUpdate,
            DispatchEventType::BanAdd => Self::BanAdd,
            DispatchEventType::BanRemove => Self::BanRemove,
            DispatchEventType::ChannelCreate => Self::ChannelCreate,
            DispatchEventType::ChannelDelete => Self::ChannelDelete,
            DispatchEventType::ChannelPinsUpdate => Self::ChannelPinsUpdate,
            DispatchEventType::ChannelUpdate => Self::ChannelUpdate,
            DispatchEventType::CommandPermissionsUpdate => Self::CommandPermissionsUpdate,
            DispatchEventType::GiftCodeUpdate => Self::GiftCodeUpdate,
            DispatchEventType::GuildAuditLogEntryCreate => Self::GuildAuditLogEntryCreate,
            DispatchEventType::GuildCreate => Self::GuildCreate,
            DispatchEventType::GuildDelete => Self::GuildDelete,
            DispatchEventType::GuildEmojisUpdate => Self::GuildEmojisUpdate,
            DispatchEventType::GuildIntegrationsUpdate => Self::GuildIntegrationsUpdate,
            DispatchEventType::GuildScheduledEventCreate => Self::GuildScheduledEventCreate,
            DispatchEventType::GuildScheduledEventDelete => Self::GuildScheduledEventDelete,
            DispatchEventType::GuildScheduledEventUpdate => Self::GuildScheduledEventUpdate,
            DispatchEventType::GuildScheduledEventUserAdd => Self::GuildScheduledEventUserAdd,
            DispatchEventType::GuildScheduledEventUserRemove => Self::GuildScheduledEventUserRemove,
            DispatchEventType::GuildStickersUpdate => Self::GuildStickersUpdate,
            DispatchEventType::GuildUpdate => Self::GuildUpdate,
            DispatchEventType::IntegrationCreate => Self::IntegrationCreate,
            DispatchEventType::IntegrationDelete => Self::IntegrationDelete,
            DispatchEventType::IntegrationUpdate => Self::IntegrationUpdate,
            DispatchEventType::InteractionCreate => Self::InteractionCreate,
            DispatchEventType::InviteCreate => Self::InviteCreate,
            DispatchEventType::InviteDelete => Self::InviteDelete,
            DispatchEventType::MemberAdd => Self::MemberAdd,
            DispatchEventType::MemberChunk => Self::MemberChunk,
            DispatchEventType::MemberRemove => Self::MemberRemove,
            DispatchEventType::MemberUpdate => Self::MemberUpdate,
            DispatchEventType::MessageCreate => Self::MessageCreate,
            DispatchEventType::MessageDelete => Self::MessageDelete,
            DispatchEventType::MessageDeleteBulk => Self::MessageDeleteBulk,
            DispatchEventType::MessageUpdate => Self::MessageUpdate,
            DispatchEventType::PresenceUpdate => Self::PresenceUpdate,
            DispatchEventType::PresencesReplace => Self::PresencesReplace,
            DispatchEventType::ReactionAdd => Self::ReactionAdd,
            DispatchEventType::ReactionRemove => Self::ReactionRemove,
            DispatchEventType::ReactionRemoveAll => Self::ReactionRemoveAll,
            DispatchEventType::ReactionRemoveEmoji => Self::ReactionRemoveEmoji,
            DispatchEventType::Ready => Self::Ready,
            DispatchEventType::Resumed => Self::Resumed,
            DispatchEventType::RoleCreate => Self::RoleCreate,
            DispatchEventType::RoleDelete => Self::RoleDelete,
            DispatchEventType::RoleUpdate => Self::RoleUpdate,
            DispatchEventType::StageInstanceCreate => Self::StageInstanceCreate,
            DispatchEventType::StageInstanceDelete => Self::StageInstanceDelete,
            DispatchEventType::StageInstanceUpdate => Self::StageInstanceUpdate,
            DispatchEventType::ThreadCreate => Self::ThreadCreate,
            DispatchEventType::ThreadDelete => Self::ThreadDelete,
            DispatchEventType::ThreadListSync => Self::ThreadListSync,
            DispatchEventType::ThreadMemberUpdate => Self::ThreadMemberUpdate,
            DispatchEventType::ThreadMembersUpdate => Self::ThreadMembersUpdate,
            DispatchEventType::ThreadUpdate => Self::ThreadUpdate,
            DispatchEventType::TypingStart => Self::TypingStart,
            DispatchEventType::UnavailableGuild => Self::UnavailableGuild,
            DispatchEventType::UserUpdate => Self::UserUpdate,
            DispatchEventType::VoiceServerUpdate => Self::VoiceServerUpdate,
            DispatchEventType::VoiceStateUpdate => Self::VoiceStateUpdate,
            DispatchEventType::WebhooksUpdate => Self::WebhooksUpdate,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{super::DispatchEventType, EventType};
    use serde::{de::DeserializeOwned, Serialize};
    use serde_test::Token;
    use static_assertions::assert_impl_all;
    use std::{fmt::Debug, hash::Hash};

    assert_impl_all!(
        EventType: Clone,
        Copy,
        Debug,
        DeserializeOwned,
        Eq,
        From<DispatchEventType>,
        Hash,
        PartialEq,
        Send,
        Serialize,
        Sync
    );

    const MAP: &[(EventType, &str)] = &[
        (
            EventType::AutoModerationActionExecution,
            "AUTO_MODERATION_ACTION_EXECUTION",
        ),
        (
            EventType::AutoModerationRuleCreate,
            "AUTO_MODERATION_RULE_CREATE",
        ),
        (
            EventType::AutoModerationRuleDelete,
            "AUTO_MODERATION_RULE_DELETE",
        ),
        (
            EventType::AutoModerationRuleUpdate,
            "AUTO_MODERATION_RULE_UPDATE",
        ),
        (EventType::BanAdd, "GUILD_BAN_ADD"),
        (EventType::BanRemove, "GUILD_BAN_REMOVE"),
        (EventType::ChannelCreate, "CHANNEL_CREATE"),
        (EventType::ChannelDelete, "CHANNEL_DELETE"),
        (EventType::ChannelPinsUpdate, "CHANNEL_PINS_UPDATE"),
        (EventType::ChannelUpdate, "CHANNEL_UPDATE"),
        (
            EventType::CommandPermissionsUpdate,
            "APPLICATION_COMMAND_PERMISSIONS_UPDATE",
        ),
        (EventType::GiftCodeUpdate, "GIFT_CODE_UPDATE"),
        (
            EventType::GuildAuditLogEntryCreate,
            "GUILD_AUDIT_LOG_ENTRY_CREATE",
        ),
        (EventType::GuildCreate, "GUILD_CREATE"),
        (EventType::GuildDelete, "GUILD_DELETE"),
        (EventType::GuildEmojisUpdate, "GUILD_EMOJIS_UPDATE"),
        (
            EventType::GuildIntegrationsUpdate,
            "GUILD_INTEGRATIONS_UPDATE",
        ),
        (
            EventType::GuildScheduledEventCreate,
            "GUILD_SCHEDULED_EVENT_CREATE",
        ),
        (
            EventType::GuildScheduledEventDelete,
            "GUILD_SCHEDULED_EVENT_DELETE",
        ),
        (
            EventType::GuildScheduledEventUpdate,
            "GUILD_SCHEDULED_EVENT_UPDATE",
        ),
        (
            EventType::GuildScheduledEventUserAdd,
            "GUILD_SCHEDULED_EVENT_USER_ADD",
        ),
        (
            EventType::GuildScheduledEventUserRemove,
            "GUILD_SCHEDULED_EVENT_USER_REMOVE",
        ),
        (EventType::GuildUpdate, "GUILD_UPDATE"),
        (EventType::IntegrationCreate, "INTEGRATION_CREATE"),
        (EventType::IntegrationDelete, "INTEGRATION_DELETE"),
        (EventType::IntegrationUpdate, "INTEGRATION_UPDATE"),
        (EventType::InteractionCreate, "INTERACTION_CREATE"),
        (EventType::InviteCreate, "INVITE_CREATE"),
        (EventType::InviteDelete, "INVITE_DELETE"),
        (EventType::MemberAdd, "GUILD_MEMBER_ADD"),
        (EventType::MemberChunk, "GUILD_MEMBERS_CHUNK"),
        (EventType::MemberRemove, "GUILD_MEMBER_REMOVE"),
        (EventType::MemberUpdate, "GUILD_MEMBER_UPDATE"),
        (EventType::MessageCreate, "MESSAGE_CREATE"),
        (EventType::MessageDelete, "MESSAGE_DELETE"),
        (EventType::MessageDeleteBulk, "MESSAGE_DELETE_BULK"),
        (EventType::MessageUpdate, "MESSAGE_UPDATE"),
        (EventType::PresenceUpdate, "PRESENCE_UPDATE"),
        (EventType::PresencesReplace, "PRESENCES_REPLACE"),
        (EventType::ReactionAdd, "MESSAGE_REACTION_ADD"),
        (EventType::ReactionRemove, "MESSAGE_REACTION_REMOVE"),
        (EventType::ReactionRemoveAll, "MESSAGE_REACTION_REMOVE_ALL"),
        (
            EventType::ReactionRemoveEmoji,
            "MESSAGE_REACTION_REMOVE_EMOJI",
        ),
        (EventType::Ready, "READY"),
        (EventType::Resumed, "RESUMED"),
        (EventType::RoleCreate, "GUILD_ROLE_CREATE"),
        (EventType::RoleDelete, "GUILD_ROLE_DELETE"),
        (EventType::RoleUpdate, "GUILD_ROLE_UPDATE"),
        (EventType::StageInstanceCreate, "STAGE_INSTANCE_CREATE"),
        (EventType::StageInstanceDelete, "STAGE_INSTANCE_DELETE"),
        (EventType::StageInstanceUpdate, "STAGE_INSTANCE_UPDATE"),
        (EventType::ThreadCreate, "THREAD_CREATE"),
        (EventType::ThreadDelete, "THREAD_DELETE"),
        (EventType::ThreadListSync, "THREAD_LIST_SYNC"),
        (EventType::ThreadMemberUpdate, "THREAD_MEMBER_UPDATE"),
        (EventType::ThreadMembersUpdate, "THREAD_MEMBERS_UPDATE"),
        (EventType::ThreadUpdate, "THREAD_UPDATE"),
        (EventType::TypingStart, "TYPING_START"),
        (EventType::UnavailableGuild, "UNAVAILABLE_GUILD"),
        (EventType::UserUpdate, "USER_UPDATE"),
        (EventType::VoiceServerUpdate, "VOICE_SERVER_UPDATE"),
        (EventType::VoiceStateUpdate, "VOICE_STATE_UPDATE"),
        (EventType::WebhooksUpdate, "WEBHOOKS_UPDATE"),
    ];

    #[test]
    fn serde() {
        for (value, name) in MAP {
            serde_test::assert_tokens(
                value,
                &[Token::UnitVariant {
                    name: "EventType",
                    variant: name,
                }],
            );
            assert_eq!(value.name().unwrap(), *name);
        }
    }
}
