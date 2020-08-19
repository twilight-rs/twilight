use serde::{Deserialize, Serialize};

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
    ShardPayload,
    ShardReconnecting,
    ShardResuming,
    TypingStart,
    UnavailableGuild,
    UserUpdate,
    VoiceServerUpdate,
    VoiceStateUpdate,
    WebhooksUpdate,
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
