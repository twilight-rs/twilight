use serde::{Deserialize, Serialize};
use std::convert::TryFrom;

/// The type of an event.
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum EventType {
    #[serde(rename = "GUILD_BAN_ADD")]
    BanAdd,
    #[serde(rename = "GUILD_BAN_REMOVE")]
    BanRemove,
    ChannelCreate,
    ChannelDelete,
    ChannelPinsUpdate,
    ChannelUpdate,
    GatewayHeartbeat,
    GatewayHeartbeatAck,
    GatewayHello,
    GatewayInvalidateSession,
    GatewayReconnect,
    GiftCodeUpdate,
    GuildCreate,
    GuildDelete,
    GuildEmojisUpdate,
    GuildIntegrationsUpdate,
    GuildUpdate,
    InviteCreate,
    InviteDelete,
    InteractionCreate,
    #[serde(rename = "GUILD_MEMBER_ADD")]
    MemberAdd,
    #[serde(rename = "GUILD_MEMBER_REMOVE")]
    MemberRemove,
    #[serde(rename = "GUILD_MEMBER_UPDATE")]
    MemberUpdate,
    #[serde(rename = "GUILD_MEMBERS_CHUNK")]
    MemberChunk,
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
    ShardConnected,
    ShardConnecting,
    ShardDisconnected,
    ShardIdentifying,
    ShardReconnecting,
    ShardPayload,
    ShardResuming,
    TypingStart,
    UnavailableGuild,
    UserUpdate,
    VoiceServerUpdate,
    VoiceStateUpdate,
    WebhooksUpdate,
}

impl EventType {
    pub fn name(self) -> Option<&'static str> {
        match self {
            Self::BanAdd => Some("GUILD_BAN_ADD"),
            Self::BanRemove => Some("GUILD_BAN_REMOVE"),
            Self::ChannelCreate => Some("CHANNEL_CREATE"),
            Self::ChannelDelete => Some("CHANNEL_DELETE"),
            Self::ChannelPinsUpdate => Some("CHANNEL_PINS_UPDATE"),
            Self::ChannelUpdate => Some("CHANNEL_UPDATE"),
            Self::GiftCodeUpdate => Some("GIFT_CODE_UPDATE"),
            Self::GuildCreate => Some("GUILD_CREATE"),
            Self::GuildDelete => Some("GUILD_DELETE"),
            Self::GuildEmojisUpdate => Some("GUILD_EMOJIS_UPDATE"),
            Self::GuildIntegrationsUpdate => Some("GUILD_INTEGRATIONS_UPDATE"),
            Self::GuildUpdate => Some("GUILD_UPDATE"),
            Self::InviteCreate => Some("INVITE_CREATE"),
            Self::InviteDelete => Some("INVITE_DELETE"),
            Self::InteractionCreate => Some("INTERACTION_CREATE"),
            Self::MemberAdd => Some("GUILD_MEMBER_ADD"),
            Self::MemberRemove => Some("GUILD_MEMBER_REMOVE"),
            Self::MemberUpdate => Some("GUILD_MEMBER_UPDATE"),
            Self::MemberChunk => Some("GUILD_MEMBERS_CHUNK"),
            Self::MessageCreate => Some("MESSAGE_CREATE"),
            Self::MessageDelete => Some("MESSAGE_DELETE"),
            Self::MessageDeleteBulk => Some("MESSAGE_DELETE_BULK"),
            Self::MessageUpdate => Some("MESSAGE_UPDATE"),
            Self::PresenceUpdate => Some("PRESENCE_UPDATE"),
            Self::PresencesReplace => Some("PRESENCES_REPLACE"),
            Self::ReactionAdd => Some("MESSAGE_REACTION_ADD"),
            Self::ReactionRemove => Some("MESSAGE_REACTION_REMOVE"),
            Self::ReactionRemoveAll => Some("MESSAGE_REACTION_REMOVE_ALL"),
            Self::ReactionRemoveEmoji => Some("MESSAGE_REACTION_REMOVE_EMOJI"),
            Self::Ready => Some("READY"),
            Self::Resumed => Some("RESUMED"),
            Self::RoleCreate => Some("GUILD_ROLE_CREATE"),
            Self::RoleDelete => Some("GUILD_ROLE_DELETE"),
            Self::RoleUpdate => Some("GUILD_ROLE_UPDATE"),
            Self::TypingStart => Some("TYPING_START"),
            Self::UnavailableGuild => Some("UNAVAILABLE_GUILD"),
            Self::UserUpdate => Some("USER_UPDATE"),
            Self::VoiceServerUpdate => Some("VOICE_SERVER_UPDATE"),
            Self::VoiceStateUpdate => Some("VOICE_STATE_UPDATE"),
            Self::WebhooksUpdate => Some("WEBHOOKS_UPDATE"),
            Self::GatewayHeartbeat
            | Self::GatewayHeartbeatAck
            | Self::GatewayHello
            | Self::GatewayInvalidateSession
            | Self::GatewayReconnect
            | Self::ShardConnected
            | Self::ShardConnecting
            | Self::ShardDisconnected
            | Self::ShardIdentifying
            | Self::ShardReconnecting
            | Self::ShardPayload
            | Self::ShardResuming => None,
        }
    }
}

impl<'a> TryFrom<&'a str> for EventType {
    type Error = &'a str;

    fn try_from(event_type: &'a str) -> Result<Self, Self::Error> {
        match event_type {
            "GUILD_BAN_ADD" => Ok(Self::BanAdd),
            "GUILD_BAN_REMOVE" => Ok(Self::BanRemove),
            "CHANNEL_CREATE" => Ok(Self::ChannelCreate),
            "CHANNEL_DELETE" => Ok(Self::ChannelDelete),
            "CHANNEL_PINS_UPDATE" => Ok(Self::ChannelPinsUpdate),
            "CHANNEL_UPDATE" => Ok(Self::ChannelUpdate),
            "GIFT_CODE_UPDATE" => Ok(Self::GiftCodeUpdate),
            "GUILD_CREATE" => Ok(Self::GuildCreate),
            "GUILD_DELETE" => Ok(Self::GuildDelete),
            "GUILD_EMOJIS_UPDATE" => Ok(Self::GuildEmojisUpdate),
            "GUILD_INTEGRATIONS_UPDATE" => Ok(Self::GuildIntegrationsUpdate),
            "GUILD_UPDATE" => Ok(Self::GuildUpdate),
            "INVITE_CREATE" => Ok(Self::InviteCreate),
            "INVITE_DELETE" => Ok(Self::InviteDelete),
            "INTERACTION_CREATE" => Ok(Self::InteractionCreate),
            "GUILD_MEMBER_ADD" => Ok(Self::MemberAdd),
            "GUILD_MEMBER_REMOVE" => Ok(Self::MemberRemove),
            "GUILD_MEMBER_UPDATE" => Ok(Self::MemberUpdate),
            "GUILD_MEMBERS_CHUNK" => Ok(Self::MemberChunk),
            "MESSAGE_CREATE" => Ok(Self::MessageCreate),
            "MESSAGE_DELETE" => Ok(Self::MessageDelete),
            "MESSAGE_DELETE_BULK" => Ok(Self::MessageDeleteBulk),
            "MESSAGE_UPDATE" => Ok(Self::MessageUpdate),
            "PRESENCE_UPDATE" => Ok(Self::PresenceUpdate),
            "PRESENCES_REPLACE" => Ok(Self::PresencesReplace),
            "MESSAGE_REACTION_ADD" => Ok(Self::ReactionAdd),
            "MESSAGE_REACTION_REMOVE" => Ok(Self::ReactionRemove),
            "MESSAGE_REACTION_REMOVE_ALL" => Ok(Self::ReactionRemoveAll),
            "MESSAGE_REACTION_REMOVE_EMOJI" => Ok(Self::ReactionRemoveEmoji),
            "READY" => Ok(Self::Ready),
            "RESUMED" => Ok(Self::Resumed),
            "GUILD_ROLE_CREATE" => Ok(Self::RoleCreate),
            "GUILD_ROLE_DELETE" => Ok(Self::RoleDelete),
            "GUILD_ROLE_UPDATE" => Ok(Self::RoleUpdate),
            "TYPING_START" => Ok(Self::TypingStart),
            "UNAVAILABLE_GUILD" => Ok(Self::UnavailableGuild),
            "USER_UPDATE" => Ok(Self::UserUpdate),
            "VOICE_SERVER_UPDATE" => Ok(Self::VoiceServerUpdate),
            "VOICE_STATE_UPDATE" => Ok(Self::VoiceStateUpdate),
            "WEBHOOKS_UPDATE" => Ok(Self::WebhooksUpdate),
            _ => Err(event_type),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::EventType;
    use serde_test::Token;

    fn assert_variant(kind: EventType, name: &'static str) {
        serde_test::assert_tokens(
            &kind,
            &[Token::UnitVariant {
                name: "EventType",
                variant: name,
            }],
        );
    }

    #[test]
    fn test_variants() {
        assert_variant(EventType::BanAdd, "GUILD_BAN_ADD");
        assert_variant(EventType::BanRemove, "GUILD_BAN_REMOVE");
        assert_variant(EventType::ChannelCreate, "CHANNEL_CREATE");
        assert_variant(EventType::ChannelDelete, "CHANNEL_DELETE");
        assert_variant(EventType::ChannelPinsUpdate, "CHANNEL_PINS_UPDATE");
        assert_variant(EventType::ChannelUpdate, "CHANNEL_UPDATE");
        assert_variant(EventType::GatewayHeartbeat, "GATEWAY_HEARTBEAT");
        assert_variant(EventType::GatewayHeartbeatAck, "GATEWAY_HEARTBEAT_ACK");
        assert_variant(EventType::GatewayHello, "GATEWAY_HELLO");
        assert_variant(
            EventType::GatewayInvalidateSession,
            "GATEWAY_INVALIDATE_SESSION",
        );
        assert_variant(EventType::GatewayReconnect, "GATEWAY_RECONNECT");
        assert_variant(EventType::GiftCodeUpdate, "GIFT_CODE_UPDATE");
        assert_variant(EventType::GuildCreate, "GUILD_CREATE");
        assert_variant(EventType::GuildDelete, "GUILD_DELETE");
        assert_variant(EventType::GuildEmojisUpdate, "GUILD_EMOJIS_UPDATE");
        assert_variant(
            EventType::GuildIntegrationsUpdate,
            "GUILD_INTEGRATIONS_UPDATE",
        );
        assert_variant(EventType::GuildUpdate, "GUILD_UPDATE");
        assert_variant(EventType::InviteCreate, "INVITE_CREATE");
        assert_variant(EventType::InviteDelete, "INVITE_DELETE");
        assert_variant(EventType::InteractionCreate, "INTERACTION_CREATE");
        assert_variant(EventType::MemberAdd, "GUILD_MEMBER_ADD");
        assert_variant(EventType::MemberRemove, "GUILD_MEMBER_REMOVE");
        assert_variant(EventType::MemberUpdate, "GUILD_MEMBER_UPDATE");
        assert_variant(EventType::MemberChunk, "GUILD_MEMBERS_CHUNK");
        assert_variant(EventType::MessageCreate, "MESSAGE_CREATE");
        assert_variant(EventType::MessageDelete, "MESSAGE_DELETE");
        assert_variant(EventType::MessageDeleteBulk, "MESSAGE_DELETE_BULK");
        assert_variant(EventType::MessageUpdate, "MESSAGE_UPDATE");
        assert_variant(EventType::PresenceUpdate, "PRESENCE_UPDATE");
        assert_variant(EventType::PresencesReplace, "PRESENCES_REPLACE");
        assert_variant(EventType::ReactionAdd, "MESSAGE_REACTION_ADD");
        assert_variant(EventType::ReactionRemove, "MESSAGE_REACTION_REMOVE");
        assert_variant(EventType::ReactionRemoveAll, "MESSAGE_REACTION_REMOVE_ALL");
        assert_variant(
            EventType::ReactionRemoveEmoji,
            "MESSAGE_REACTION_REMOVE_EMOJI",
        );
        assert_variant(EventType::Ready, "READY");
        assert_variant(EventType::Resumed, "RESUMED");
        assert_variant(EventType::RoleCreate, "GUILD_ROLE_CREATE");
        assert_variant(EventType::RoleDelete, "GUILD_ROLE_DELETE");
        assert_variant(EventType::RoleUpdate, "GUILD_ROLE_UPDATE");
        assert_variant(EventType::ShardConnected, "SHARD_CONNECTED");
        assert_variant(EventType::ShardConnecting, "SHARD_CONNECTING");
        assert_variant(EventType::ShardDisconnected, "SHARD_DISCONNECTED");
        assert_variant(EventType::ShardIdentifying, "SHARD_IDENTIFYING");
        assert_variant(EventType::ShardPayload, "SHARD_PAYLOAD");
        assert_variant(EventType::ShardReconnecting, "SHARD_RECONNECTING");
        assert_variant(EventType::ShardResuming, "SHARD_RESUMING");
        assert_variant(EventType::TypingStart, "TYPING_START");
        assert_variant(EventType::UnavailableGuild, "UNAVAILABLE_GUILD");
        assert_variant(EventType::UserUpdate, "USER_UPDATE");
        assert_variant(EventType::VoiceServerUpdate, "VOICE_SERVER_UPDATE");
        assert_variant(EventType::VoiceStateUpdate, "VOICE_STATE_UPDATE");
        assert_variant(EventType::WebhooksUpdate, "WEBHOOKS_UPDATE");
    }
}
