use bitflags::bitflags;
use twilight_model::gateway::event::EventType;

bitflags! {
    /// Bitflags representing all of the possible types of events.
    pub struct EventTypeFlags: u64 {
        /// A user was banned from a guild.
        const BAN_ADD = 1;
        /// A user was unbanned from a guild.
        const BAN_REMOVE = 1 << 1;
        /// A channel was created.
        const CHANNEL_CREATE = 1 << 2;
        /// A channel was deleted.
        const CHANNEL_DELETE = 1 << 3;
        /// A channel's pins were updated.
        const CHANNEL_PINS_UPDATE = 1 << 4;
        /// A channel was updated.
        const CHANNEL_UPDATE = 1 << 5;
        /// A heartbeat was created.
        const GATEWAY_HEARTBEAT = 1 << 6;
        /// A heartbeat was acknowledged.
        const GATEWAY_HEARTBEAT_ACK = 1 << 7;
        /// A "hello" packet was received from the gateway.
        const GATEWAY_HELLO = 1 << 8;
        /// A shard's session was invalidated.
        ///
        /// `true` if resumeable. If not, then the shard must do a full
        /// reconnect.
        const GATEWAY_INVALIDATE_SESSION = 1 << 8;
        /// The gateway is indicating to perform a reconnect.
        const GATEWAY_RECONNECT = 1 << 9;
        /// A guild was created.
        const GUILD_CREATE = 1 << 10;
        /// A guild was deleted or the current user was removed from a guild.
        const GUILD_DELETE = 1 << 11;
        /// A guild's emojis were updated.
        const GUILD_EMOJIS_UPDATE = 1 << 12;
        /// A guild's integrations were updated.
        const GUILD_INTEGRATIONS_UPDATE = 1 << 13;
        /// A guild was updated.
        const GUILD_UPDATE = 1 << 14;
        const INVITE_CREATE = 1 << 46;
        const INVITE_DELETE = 1 << 47;
        const MEMBER_ADD = 1 << 15;
        const MEMBER_REMOVE = 1 << 16;
        const MEMBER_UPDATE = 1 << 17;
        const MEMBER_CHUNK = 1 << 18;
        const MESSAGE_CREATE = 1 << 19;
        const MESSAGE_DELETE = 1 << 20;
        const MESSAGE_DELETE_BULK = 1 << 21;
        const MESSAGE_UPDATE = 1 << 22;
        const PRESENCE_UPDATE = 1 << 23;
        const PRESENCES_REPLACE = 1 << 24;
        const REACTION_ADD = 1 << 25;
        const REACTION_REMOVE = 1 << 26;
        const REACTION_REMOVE_ALL = 1 << 27;
        const REACTION_REMOVE_EMOJI = 1 << 48;
        const READY = 1 << 28;
        const RESUMED = 1 << 29;
        const ROLE_CREATE = 1 << 30;
        const ROLE_DELETE = 1 << 31;
        const ROLE_UPDATE = 1 << 32;
        const SHARD_CONNECTED = 1 << 33;
        const SHARD_CONNECTING = 1 << 34;
        const SHARD_DISCONNECTED = 1 << 35;
        const SHARD_IDENTIFYING = 1 << 36;
        const SHARD_PAYLOAD = 1 << 45;
        const SHARD_RECONNECTING = 1 << 37;
        const SHARD_RESUMING = 1 << 38;
        const TYPING_START = 1 << 39;
        const UNAVAILABLE_GUILD = 1 << 40;
        const USER_UPDATE = 1 << 41;
        const VOICE_SERVER_UPDATE = 1 << 42;
        const VOICE_STATE_UPDATE = 1 << 43;
        const WEBHOOKS_UPDATE = 1 << 44;
    }
}

impl From<EventType> for EventTypeFlags {
    fn from(event_type: EventType) -> Self {
        match event_type {
            EventType::BanAdd => EventTypeFlags::BAN_ADD,
            EventType::BanRemove => EventTypeFlags::BAN_REMOVE,
            EventType::ChannelCreate => EventTypeFlags::CHANNEL_CREATE,
            EventType::ChannelDelete => EventTypeFlags::CHANNEL_DELETE,
            EventType::ChannelPinsUpdate => EventTypeFlags::CHANNEL_PINS_UPDATE,
            EventType::ChannelUpdate => EventTypeFlags::CHANNEL_UPDATE,
            EventType::GatewayHeartbeat => EventTypeFlags::GATEWAY_HEARTBEAT,
            EventType::GatewayHeartbeatAck => EventTypeFlags::GATEWAY_HEARTBEAT_ACK,
            EventType::GatewayHello => EventTypeFlags::GATEWAY_HELLO,
            EventType::GatewayInvalidateSession => EventTypeFlags::GATEWAY_INVALIDATE_SESSION,
            EventType::GatewayReconnect => EventTypeFlags::GATEWAY_RECONNECT,
            EventType::GuildCreate => EventTypeFlags::GUILD_CREATE,
            EventType::GuildDelete => EventTypeFlags::GUILD_DELETE,
            EventType::GuildEmojisUpdate => EventTypeFlags::GUILD_EMOJIS_UPDATE,
            EventType::GuildIntegrationsUpdate => EventTypeFlags::GUILD_INTEGRATIONS_UPDATE,
            EventType::GuildUpdate => EventTypeFlags::GUILD_UPDATE,
            EventType::InviteCreate => EventTypeFlags::INVITE_CREATE,
            EventType::InviteDelete => EventTypeFlags::INVITE_DELETE,
            EventType::MemberAdd => EventTypeFlags::MEMBER_ADD,
            EventType::MemberRemove => EventTypeFlags::MEMBER_REMOVE,
            EventType::MemberUpdate => EventTypeFlags::MEMBER_UPDATE,
            EventType::MemberChunk => EventTypeFlags::MEMBER_CHUNK,
            EventType::MessageCreate => EventTypeFlags::MESSAGE_CREATE,
            EventType::MessageDelete => EventTypeFlags::MESSAGE_DELETE,
            EventType::MessageDeleteBulk => EventTypeFlags::MESSAGE_DELETE_BULK,
            EventType::MessageUpdate => EventTypeFlags::MESSAGE_UPDATE,
            EventType::PresenceUpdate => EventTypeFlags::PRESENCE_UPDATE,
            EventType::PresencesReplace => EventTypeFlags::PRESENCES_REPLACE,
            EventType::ReactionAdd => EventTypeFlags::REACTION_ADD,
            EventType::ReactionRemove => EventTypeFlags::REACTION_REMOVE,
            EventType::ReactionRemoveAll => EventTypeFlags::REACTION_REMOVE_ALL,
            EventType::ReactionRemoveEmoji => EventTypeFlags::REACTION_REMOVE_EMOJI,
            EventType::Ready => EventTypeFlags::READY,
            EventType::Resumed => EventTypeFlags::RESUMED,
            EventType::RoleCreate => EventTypeFlags::ROLE_CREATE,
            EventType::RoleDelete => EventTypeFlags::ROLE_DELETE,
            EventType::RoleUpdate => EventTypeFlags::ROLE_UPDATE,
            EventType::ShardConnected => EventTypeFlags::SHARD_CONNECTED,
            EventType::ShardConnecting => EventTypeFlags::SHARD_CONNECTING,
            EventType::ShardDisconnected => EventTypeFlags::SHARD_DISCONNECTED,
            EventType::ShardIdentifying => EventTypeFlags::SHARD_IDENTIFYING,
            EventType::ShardReconnecting => EventTypeFlags::SHARD_RECONNECTING,
            EventType::ShardPayload => EventTypeFlags::SHARD_PAYLOAD,
            EventType::ShardResuming => EventTypeFlags::SHARD_RESUMING,
            EventType::TypingStart => EventTypeFlags::TYPING_START,
            EventType::UnavailableGuild => EventTypeFlags::UNAVAILABLE_GUILD,
            EventType::UserUpdate => EventTypeFlags::USER_UPDATE,
            EventType::VoiceServerUpdate => EventTypeFlags::VOICE_SERVER_UPDATE,
            EventType::VoiceStateUpdate => EventTypeFlags::VOICE_STATE_UPDATE,
            EventType::WebhooksUpdate => EventTypeFlags::WEBHOOKS_UPDATE,
        }
    }
}

impl Default for EventTypeFlags {
    fn default() -> Self {
        let mut flags = Self::all();
        flags.remove(Self::SHARD_PAYLOAD);

        flags
    }
}
