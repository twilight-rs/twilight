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
    #[serde(rename = "REACTION_REMOVE")]
    ReactionRemove,
    #[serde(rename = "REACTION_REMOVE_ALL")]
    ReactionRemoveAll,
    #[serde(rename = "REACTION_REMOVE_EMOJI")]
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
