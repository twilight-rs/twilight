use crate::util::known_string::KnownString;
use serde::{Deserialize, Serialize};
use std::{
    fmt::{Debug, Formatter, Result as FmtResult},
    ops::Deref,
    str::FromStr,
};

/// The type of an event.
#[derive(Clone, Copy, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct EventType(KnownString<64>);

impl EventType {
    pub const AUTO_MODERATION_ACTION_EXECUTION: Self =
        Self::from_bytes(b"AUTO_MODERATION_ACTION_EXECUTION");

    pub const AUTO_MODERATION_RULE_CREATE: Self = Self::from_bytes(b"AUTO_MODERATION_RULE_CREATE");

    pub const AUTO_MODERATION_RULE_DELETE: Self = Self::from_bytes(b"AUTO_MODERATION_RULE_DELETE");

    pub const AUTO_MODERATION_RULE_UPDATE: Self = Self::from_bytes(b"AUTO_MODERATION_RULE_UPDATE");

    pub const BAN_ADD: Self = Self::from_bytes(b"GUILD_BAN_ADD");

    pub const BAN_REMOVE: Self = Self::from_bytes(b"GUILD_BAN_REMOVE");

    pub const CHANNEL_CREATE: Self = Self::from_bytes(b"CHANNEL_CREATE");

    pub const CHANNEL_DELETE: Self = Self::from_bytes(b"CHANNEL_DELETE");

    pub const CHANNEL_PINS_UPDATE: Self = Self::from_bytes(b"CHANNEL_PINS_UPDATE");

    pub const CHANNEL_UPDATE: Self = Self::from_bytes(b"CHANNEL_UPDATE");

    pub const COMMAND_PERMISSIONS_UPDATE: Self =
        Self::from_bytes(b"APPLICATION_COMMAND_PERMISSIONS_UPDATE");

    pub const GATEWAY_CLOSE: Self = Self::from_bytes(b"GATEWAY_CLOSE");

    pub const GATEWAY_HEARTBEAT: Self = Self::from_bytes(b"GATEWAY_HEARTBEAT");

    pub const GATEWAY_HEARTBEAT_ACK: Self = Self::from_bytes(b"GATEWAY_HEARTBEAT_ACK");

    pub const GATEWAY_HELLO: Self = Self::from_bytes(b"GATEWAY_HELLO");

    pub const GATEWAY_INVALIDATE_SESSION: Self = Self::from_bytes(b"GATEWAY_INVALIDATE_SESSION");

    pub const GATEWAY_RECONNECT: Self = Self::from_bytes(b"GATEWAY_RECONNECT");

    pub const GIFT_CODE_UPDATE: Self = Self::from_bytes(b"GIFT_CODE_UPDATE");

    pub const GUILD_AUDIT_LOG_ENTRY_CREATE: Self =
        Self::from_bytes(b"GUILD_AUDIT_LOG_ENTRY_CREATE");

    pub const GUILD_CREATE: Self = Self::from_bytes(b"GUILD_CREATE");

    pub const GUILD_DELETE: Self = Self::from_bytes(b"GUILD_DELETE");

    pub const GUILD_EMOJIS_UPDATE: Self = Self::from_bytes(b"GUILD_EMOJIS_UPDATE");

    pub const GUILD_INTEGRATIONS_UPDATE: Self = Self::from_bytes(b"GUILD_INTEGRATIONS_UPDATE");

    pub const GUILD_SCHEDULED_EVENT_CREATE: Self =
        Self::from_bytes(b"GUILD_SCHEDULED_EVENT_CREATE");

    pub const GUILD_SCHEDULED_EVENT_DELETE: Self =
        Self::from_bytes(b"GUILD_SCHEDULED_EVENT_DELETE");

    pub const GUILD_SCHEDULED_EVENT_UPDATE: Self =
        Self::from_bytes(b"GUILD_SCHEDULED_EVENT_UPDATE");

    pub const GUILD_SCHEDULED_EVENT_USER_ADD: Self =
        Self::from_bytes(b"GUILD_SCHEDULED_EVENT_USER_ADD");

    pub const GUILD_SCHEDULED_EVENT_USER_REMOVE: Self =
        Self::from_bytes(b"GUILD_SCHEDULED_EVENT_USER_REMOVE");

    pub const GUILD_STICKERS_UPDATE: Self = Self::from_bytes(b"GUILD_STICKERS_UPDATE");

    pub const GUILD_UPDATE: Self = Self::from_bytes(b"GUILD_UPDATE");

    pub const INTEGRATION_CREATE: Self = Self::from_bytes(b"INTEGRATION_CREATE");

    pub const INTEGRATION_DELETE: Self = Self::from_bytes(b"INTEGRATION_DELETE");

    pub const INTEGRATION_UPDATE: Self = Self::from_bytes(b"INTEGRATION_UPDATE");

    pub const INTERACTION_CREATE: Self = Self::from_bytes(b"INTERACTION_CREATE");

    pub const INVITE_CREATE: Self = Self::from_bytes(b"INVITE_CREATE");

    pub const INVITE_DELETE: Self = Self::from_bytes(b"INVITE_DELETE");

    pub const MEMBER_ADD: Self = Self::from_bytes(b"GUILD_MEMBER_ADD");

    pub const MEMBER_CHUNK: Self = Self::from_bytes(b"GUILD_MEMBERS_CHUNK");

    pub const MEMBER_REMOVE: Self = Self::from_bytes(b"GUILD_MEMBER_REMOVE");

    pub const MEMBER_UPDATE: Self = Self::from_bytes(b"GUILD_MEMBER_UPDATE");

    pub const MESSAGE_CREATE: Self = Self::from_bytes(b"MESSAGE_CREATE");

    pub const MESSAGE_DELETE: Self = Self::from_bytes(b"MESSAGE_DELETE");

    pub const MESSAGE_DELETE_BULK: Self = Self::from_bytes(b"MESSAGE_DELETE_BULK");

    pub const MESSAGE_UPDATE: Self = Self::from_bytes(b"MESSAGE_UPDATE");

    pub const PRESENCE_UPDATE: Self = Self::from_bytes(b"PRESENCE_UPDATE");

    pub const PRESENCES_REPLACE: Self = Self::from_bytes(b"PRESENCES_REPLACE");

    pub const REACTION_ADD: Self = Self::from_bytes(b"MESSAGE_REACTION_ADD");

    pub const REACTION_REMOVE: Self = Self::from_bytes(b"MESSAGE_REACTION_REMOVE");

    pub const REACTION_REMOVE_ALL: Self = Self::from_bytes(b"MESSAGE_REACTION_REMOVE_ALL");

    pub const REACTION_REMOVE_EMOJI: Self = Self::from_bytes(b"MESSAGE_REACTION_REMOVE_EMOJI");

    pub const READY: Self = Self::from_bytes(b"READY");

    pub const RESUMED: Self = Self::from_bytes(b"RESUMED");

    pub const ROLE_CREATE: Self = Self::from_bytes(b"GUILD_ROLE_CREATE");

    pub const ROLE_DELETE: Self = Self::from_bytes(b"GUILD_ROLE_DELETE");

    pub const ROLE_UPDATE: Self = Self::from_bytes(b"GUILD_ROLE_UPDATE");

    pub const STAGE_INSTANCE_CREATE: Self = Self::from_bytes(b"STAGE_INSTANCE_CREATE");

    pub const STAGE_INSTANCE_DELETE: Self = Self::from_bytes(b"STAGE_INSTANCE_DELETE");

    pub const STAGE_INSTANCE_UPDATE: Self = Self::from_bytes(b"STAGE_INSTANCE_UPDATE");

    pub const THREAD_CREATE: Self = Self::from_bytes(b"THREAD_CREATE");

    pub const THREAD_DELETE: Self = Self::from_bytes(b"THREAD_DELETE");

    pub const THREAD_LIST_SYNC: Self = Self::from_bytes(b"THREAD_LIST_SYNC");

    pub const THREAD_MEMBER_UPDATE: Self = Self::from_bytes(b"THREAD_MEMBER_UPDATE");

    pub const THREAD_MEMBERS_UPDATE: Self = Self::from_bytes(b"THREAD_MEMBERS_UPDATE");

    pub const THREAD_UPDATE: Self = Self::from_bytes(b"THREAD_UPDATE");

    pub const TYPING_START: Self = Self::from_bytes(b"TYPING_START");

    pub const UNAVAILABLE_GUILD: Self = Self::from_bytes(b"UNAVAILABLE_GUILD");

    pub const USER_UPDATE: Self = Self::from_bytes(b"USER_UPDATE");

    pub const VOICE_SERVER_UPDATE: Self = Self::from_bytes(b"VOICE_SERVER_UPDATE");

    pub const VOICE_STATE_UPDATE: Self = Self::from_bytes(b"VOICE_STATE_UPDATE");

    pub const WEBHOOKS_UPDATE: Self = Self::from_bytes(b"WEBHOOKS_UPDATE");

    /// Create a event type from a dynamic value.
    ///
    /// The provided event type must be 64 bytes or smaller.
    pub fn new(event_type: &str) -> Option<Self> {
        KnownString::from_str(event_type).map(Self)
    }

    /// Get the value of the event type.
    ///
    /// # Panics
    ///
    /// Panics if the event type isn't valid UTF-8.
    pub fn get(&self) -> &str {
        self.0.get()
    }

    /// Name of the associated constant.
    ///
    /// Returns `None` if the value doesn't have a defined constant.
    pub const fn name(self) -> Option<&'static str> {
        match self {
            Self::AUTO_MODERATION_ACTION_EXECUTION => Some("AUTO_MODERATION_ACTION_EXECUTION"),
            Self::AUTO_MODERATION_RULE_CREATE => Some("AUTO_MODERATION_RULE_CREATE"),
            Self::AUTO_MODERATION_RULE_DELETE => Some("AUTO_MODERATION_RULE_DELETE"),
            Self::AUTO_MODERATION_RULE_UPDATE => Some("AUTO_MODERATION_RULE_UPDATE"),
            Self::BAN_ADD => Some("GUILD_BAN_ADD"),
            Self::BAN_REMOVE => Some("GUILD_BAN_REMOVE"),
            Self::CHANNEL_CREATE => Some("CHANNEL_CREATE"),
            Self::CHANNEL_DELETE => Some("CHANNEL_DELETE"),
            Self::CHANNEL_PINS_UPDATE => Some("CHANNEL_PINS_UPDATE"),
            Self::CHANNEL_UPDATE => Some("CHANNEL_UPDATE"),
            Self::COMMAND_PERMISSIONS_UPDATE => Some("APPLICATION_COMMAND_PERMISSIONS_UPDATE"),
            Self::GIFT_CODE_UPDATE => Some("GIFT_CODE_UPDATE"),
            Self::GUILD_AUDIT_LOG_ENTRY_CREATE => Some("GUILD_AUDIT_LOG_ENTRY_CREATE"),
            Self::GUILD_CREATE => Some("GUILD_CREATE"),
            Self::GUILD_DELETE => Some("GUILD_DELETE"),
            Self::GUILD_EMOJIS_UPDATE => Some("GUILD_EMOJIS_UPDATE"),
            Self::GUILD_INTEGRATIONS_UPDATE => Some("GUILD_INTEGRATIONS_UPDATE"),
            Self::GUILD_SCHEDULED_EVENT_CREATE => Some("GUILD_SCHEDULED_EVENT_CREATE"),
            Self::GUILD_SCHEDULED_EVENT_DELETE => Some("GUILD_SCHEDULED_EVENT_DELETE"),
            Self::GUILD_SCHEDULED_EVENT_UPDATE => Some("GUILD_SCHEDULED_EVENT_UPDATE"),
            Self::GUILD_SCHEDULED_EVENT_USER_ADD => Some("GUILD_SCHEDULED_EVENT_USER_ADD"),
            Self::GUILD_SCHEDULED_EVENT_USER_REMOVE => Some("GUILD_SCHEDULED_EVENT_USER_REMOVE"),
            Self::GUILD_STICKERS_UPDATE => Some("GUILD_STICKERS_UPDATE"),
            Self::GUILD_UPDATE => Some("GUILD_UPDATE"),
            Self::INTEGRATION_CREATE => Some("INTEGRATION_CREATE"),
            Self::INTEGRATION_DELETE => Some("INTEGRATION_DELETE"),
            Self::INTEGRATION_UPDATE => Some("INTEGRATION_UPDATE"),
            Self::INTERACTION_CREATE => Some("INTERACTION_CREATE"),
            Self::INVITE_CREATE => Some("INVITE_CREATE"),
            Self::INVITE_DELETE => Some("INVITE_DELETE"),
            Self::MEMBER_ADD => Some("GUILD_MEMBER_ADD"),
            Self::MEMBER_CHUNK => Some("GUILD_MEMBERS_CHUNK"),
            Self::MEMBER_REMOVE => Some("GUILD_MEMBER_REMOVE"),
            Self::MEMBER_UPDATE => Some("GUILD_MEMBER_UPDATE"),
            Self::MESSAGE_CREATE => Some("MESSAGE_CREATE"),
            Self::MESSAGE_DELETE => Some("MESSAGE_DELETE"),
            Self::MESSAGE_DELETE_BULK => Some("MESSAGE_DELETE_BULK"),
            Self::MESSAGE_UPDATE => Some("MESSAGE_UPDATE"),
            Self::PRESENCES_REPLACE => Some("PRESENCES_REPLACE"),
            Self::PRESENCE_UPDATE => Some("PRESENCE_UPDATE"),
            Self::REACTION_ADD => Some("MESSAGE_REACTION_ADD"),
            Self::REACTION_REMOVE => Some("MESSAGE_REACTION_REMOVE"),
            Self::REACTION_REMOVE_ALL => Some("MESSAGE_REACTION_REMOVE_ALL"),
            Self::REACTION_REMOVE_EMOJI => Some("MESSAGE_REACTION_REMOVE_EMOJI"),
            Self::READY => Some("READY"),
            Self::RESUMED => Some("RESUMED"),
            Self::ROLE_CREATE => Some("GUILD_ROLE_CREATE"),
            Self::ROLE_DELETE => Some("GUILD_ROLE_DELETE"),
            Self::ROLE_UPDATE => Some("GUILD_ROLE_UPDATE"),
            Self::STAGE_INSTANCE_CREATE => Some("STAGE_INSTANCE_CREATE"),
            Self::STAGE_INSTANCE_DELETE => Some("STAGE_INSTANCE_DELETE"),
            Self::STAGE_INSTANCE_UPDATE => Some("STAGE_INSTANCE_UPDATE"),
            Self::THREAD_CREATE => Some("THREAD_CREATE"),
            Self::THREAD_DELETE => Some("THREAD_DELETE"),
            Self::THREAD_LIST_SYNC => Some("THREAD_LIST_SYNC"),
            Self::THREAD_MEMBERS_UPDATE => Some("THREAD_MEMBERS_UPDATE"),
            Self::THREAD_MEMBER_UPDATE => Some("THREAD_MEMBER_UPDATE"),
            Self::THREAD_UPDATE => Some("THREAD_UPDATE"),
            Self::TYPING_START => Some("TYPING_START"),
            Self::UNAVAILABLE_GUILD => Some("UNAVAILABLE_GUILD"),
            Self::USER_UPDATE => Some("USER_UPDATE"),
            Self::VOICE_SERVER_UPDATE => Some("VOICE_SERVER_UPDATE"),
            Self::VOICE_STATE_UPDATE => Some("VOICE_STATE_UPDATE"),
            Self::WEBHOOKS_UPDATE => Some("WEBHOOKS_UPDATE"),
            _ => None,
        }
    }

    /// Create a event type from a set of bytes.
    const fn from_bytes(input: &[u8]) -> Self {
        Self(KnownString::from_bytes(input))
    }
}

impl AsRef<str> for EventType {
    fn as_ref(&self) -> &str {
        self.get()
    }
}

impl Debug for EventType {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        f.write_str(self.name().unwrap_or_else(|| self.get()))
    }
}

impl Deref for EventType {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        self.get()
    }
}

impl FromStr for EventType {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::try_from(s)
    }
}

impl ToString for EventType {
    fn to_string(&self) -> String {
        KnownString::to_string(&self.0)
    }
}

impl TryFrom<&str> for EventType {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::new(value).ok_or(())
    }
}

#[cfg(test)]
mod tests {
    use super::EventType;
    use serde_test::Token;

    fn assert_variant(kind: EventType, name: &'static str) {
        serde_test::assert_tokens(
            &kind,
            &[Token::NewtypeStruct { name: "EventType" }, Token::Str(name)],
        );
    }

    #[allow(clippy::too_many_lines)]
    #[test]
    fn variants() {
        assert_variant(
            EventType::AUTO_MODERATION_ACTION_EXECUTION,
            "AUTO_MODERATION_ACTION_EXECUTION",
        );
        assert_variant(
            EventType::AUTO_MODERATION_RULE_CREATE,
            "AUTO_MODERATION_RULE_CREATE",
        );
        assert_variant(
            EventType::AUTO_MODERATION_RULE_DELETE,
            "AUTO_MODERATION_RULE_DELETE",
        );
        assert_variant(
            EventType::AUTO_MODERATION_RULE_UPDATE,
            "AUTO_MODERATION_RULE_UPDATE",
        );
        assert_variant(EventType::BAN_ADD, "GUILD_BAN_ADD");
        assert_variant(EventType::BAN_REMOVE, "GUILD_BAN_REMOVE");
        assert_variant(EventType::CHANNEL_CREATE, "CHANNEL_CREATE");
        assert_variant(EventType::CHANNEL_DELETE, "CHANNEL_DELETE");
        assert_variant(EventType::CHANNEL_PINS_UPDATE, "CHANNEL_PINS_UPDATE");
        assert_variant(EventType::CHANNEL_UPDATE, "CHANNEL_UPDATE");
        assert_variant(
            EventType::COMMAND_PERMISSIONS_UPDATE,
            "APPLICATION_COMMAND_PERMISSIONS_UPDATE",
        );
        assert_variant(EventType::GATEWAY_CLOSE, "GATEWAY_CLOSE");
        assert_variant(EventType::GATEWAY_HEARTBEAT, "GATEWAY_HEARTBEAT");
        assert_variant(EventType::GATEWAY_HEARTBEAT_ACK, "GATEWAY_HEARTBEAT_ACK");
        assert_variant(EventType::GATEWAY_HELLO, "GATEWAY_HELLO");
        assert_variant(
            EventType::GATEWAY_INVALIDATE_SESSION,
            "GATEWAY_INVALIDATE_SESSION",
        );
        assert_variant(EventType::GATEWAY_RECONNECT, "GATEWAY_RECONNECT");
        assert_variant(EventType::GIFT_CODE_UPDATE, "GIFT_CODE_UPDATE");
        assert_variant(
            EventType::GUILD_AUDIT_LOG_ENTRY_CREATE,
            "GUILD_AUDIT_LOG_ENTRY_CREATE",
        );
        assert_variant(EventType::GUILD_CREATE, "GUILD_CREATE");
        assert_variant(EventType::GUILD_DELETE, "GUILD_DELETE");
        assert_variant(EventType::GUILD_EMOJIS_UPDATE, "GUILD_EMOJIS_UPDATE");
        assert_variant(
            EventType::GUILD_INTEGRATIONS_UPDATE,
            "GUILD_INTEGRATIONS_UPDATE",
        );
        assert_variant(
            EventType::GUILD_SCHEDULED_EVENT_CREATE,
            "GUILD_SCHEDULED_EVENT_CREATE",
        );
        assert_variant(
            EventType::GUILD_SCHEDULED_EVENT_DELETE,
            "GUILD_SCHEDULED_EVENT_DELETE",
        );
        assert_variant(
            EventType::GUILD_SCHEDULED_EVENT_UPDATE,
            "GUILD_SCHEDULED_EVENT_UPDATE",
        );
        assert_variant(
            EventType::GUILD_SCHEDULED_EVENT_USER_ADD,
            "GUILD_SCHEDULED_EVENT_USER_ADD",
        );
        assert_variant(
            EventType::GUILD_SCHEDULED_EVENT_USER_REMOVE,
            "GUILD_SCHEDULED_EVENT_USER_REMOVE",
        );
        assert_variant(EventType::GUILD_UPDATE, "GUILD_UPDATE");
        assert_variant(EventType::INTEGRATION_CREATE, "INTEGRATION_CREATE");
        assert_variant(EventType::INTEGRATION_DELETE, "INTEGRATION_DELETE");
        assert_variant(EventType::INTEGRATION_UPDATE, "INTEGRATION_UPDATE");
        assert_variant(EventType::INTERACTION_CREATE, "INTERACTION_CREATE");
        assert_variant(EventType::INVITE_CREATE, "INVITE_CREATE");
        assert_variant(EventType::INVITE_DELETE, "INVITE_DELETE");
        assert_variant(EventType::MEMBER_ADD, "GUILD_MEMBER_ADD");
        assert_variant(EventType::MEMBER_CHUNK, "GUILD_MEMBERS_CHUNK");
        assert_variant(EventType::MEMBER_REMOVE, "GUILD_MEMBER_REMOVE");
        assert_variant(EventType::MEMBER_UPDATE, "GUILD_MEMBER_UPDATE");
        assert_variant(EventType::MESSAGE_CREATE, "MESSAGE_CREATE");
        assert_variant(EventType::MESSAGE_DELETE, "MESSAGE_DELETE");
        assert_variant(EventType::MESSAGE_DELETE_BULK, "MESSAGE_DELETE_BULK");
        assert_variant(EventType::MESSAGE_UPDATE, "MESSAGE_UPDATE");
        assert_variant(EventType::PRESENCE_UPDATE, "PRESENCE_UPDATE");
        assert_variant(EventType::PRESENCES_REPLACE, "PRESENCES_REPLACE");
        assert_variant(EventType::REACTION_ADD, "MESSAGE_REACTION_ADD");
        assert_variant(EventType::REACTION_REMOVE, "MESSAGE_REACTION_REMOVE");
        assert_variant(
            EventType::REACTION_REMOVE_ALL,
            "MESSAGE_REACTION_REMOVE_ALL",
        );
        assert_variant(
            EventType::REACTION_REMOVE_EMOJI,
            "MESSAGE_REACTION_REMOVE_EMOJI",
        );
        assert_variant(EventType::READY, "READY");
        assert_variant(EventType::RESUMED, "RESUMED");
        assert_variant(EventType::ROLE_CREATE, "GUILD_ROLE_CREATE");
        assert_variant(EventType::ROLE_DELETE, "GUILD_ROLE_DELETE");
        assert_variant(EventType::ROLE_UPDATE, "GUILD_ROLE_UPDATE");
        assert_variant(EventType::STAGE_INSTANCE_CREATE, "STAGE_INSTANCE_CREATE");
        assert_variant(EventType::STAGE_INSTANCE_DELETE, "STAGE_INSTANCE_DELETE");
        assert_variant(EventType::STAGE_INSTANCE_UPDATE, "STAGE_INSTANCE_UPDATE");
        assert_variant(EventType::THREAD_CREATE, "THREAD_CREATE");
        assert_variant(EventType::THREAD_DELETE, "THREAD_DELETE");
        assert_variant(EventType::THREAD_LIST_SYNC, "THREAD_LIST_SYNC");
        assert_variant(EventType::THREAD_MEMBER_UPDATE, "THREAD_MEMBER_UPDATE");
        assert_variant(EventType::THREAD_MEMBERS_UPDATE, "THREAD_MEMBERS_UPDATE");
        assert_variant(EventType::THREAD_UPDATE, "THREAD_UPDATE");
        assert_variant(EventType::TYPING_START, "TYPING_START");
        assert_variant(EventType::UNAVAILABLE_GUILD, "UNAVAILABLE_GUILD");
        assert_variant(EventType::USER_UPDATE, "USER_UPDATE");
        assert_variant(EventType::VOICE_SERVER_UPDATE, "VOICE_SERVER_UPDATE");
        assert_variant(EventType::VOICE_STATE_UPDATE, "VOICE_STATE_UPDATE");
        assert_variant(EventType::WEBHOOKS_UPDATE, "WEBHOOKS_UPDATE");
    }
}
