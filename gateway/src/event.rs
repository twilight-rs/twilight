use bitflags::bitflags;
use std::convert::TryFrom;
use twilight_model::gateway::event::EventType;

bitflags! {
    /// Bitflags representing all of the possible types of events.
    #[non_exhaustive]
    pub struct EventTypeFlags: u64 {
        /// User has been banned from a guild.
        const BAN_ADD = 1;
        /// User has been unbanned from a guild.
        const BAN_REMOVE = 1 << 1;
        /// Channel has been created.
        const CHANNEL_CREATE = 1 << 2;
        /// Channel has been deleted.
        const CHANNEL_DELETE = 1 << 3;
        /// Channel's pins have been updated.
        const CHANNEL_PINS_UPDATE = 1 << 4;
        /// Channel has been updated.
        const CHANNEL_UPDATE = 1 << 5;
        /// Heartbeat has been created.
        const GATEWAY_HEARTBEAT = 1 << 6;
        /// Heartbeat has been acknowledged.
        const GATEWAY_HEARTBEAT_ACK = 1 << 7;
        /// A "hello" packet has been received from the gateway.
        const GATEWAY_HELLO = 1 << 8;
        /// Shard's session has been invalidated.
        ///
        /// A payload containing a boolean is included. If `true` the session is
        /// resumable. If not, then the shard must initialize a new session.
        const GATEWAY_INVALIDATE_SESSION = 1 << 8;
        /// Gateway is indicating that a shard should perform a reconnect.
        const GATEWAY_RECONNECT = 1 << 9;
        /// Gift code sent in a channel has been updated.
        const GIFT_CODE_UPDATE = 1 << 49;
        /// A guild has been created.
        const GUILD_CREATE = 1 << 10;
        /// A guild has been deleted or the current user has been removed from a guild.
        const GUILD_DELETE = 1 << 11;
        /// A guild's emojis have been updated.
        const GUILD_EMOJIS_UPDATE = 1 << 12;
        /// A guild's integrations have been updated.
        const GUILD_INTEGRATIONS_UPDATE = 1 << 13;
        /// A guild has been updated.
        const GUILD_UPDATE = 1 << 14;
        /// Invite for a channel has been created.
        const INVITE_CREATE = 1 << 46;
        /// Invite for a channel has been deleted.
        const INVITE_DELETE = 1 << 47;
        /// A user used a interaction registrered with the bot
        const INTERACTION_CREATE = 1 << 50;
        /// Member has been added to a guild.
        const MEMBER_ADD = 1 << 15;
        /// Member has been removed from a guild.
        const MEMBER_REMOVE = 1 << 16;
        /// Member in a guild has been updated.
        const MEMBER_UPDATE = 1 << 17;
        /// Group of members from a guild.
        ///
        /// This may be all of the remaining members or not; the chunk index in
        /// the event payload needs to be confirmed.
        const MEMBER_CHUNK = 1 << 18;
        /// Message created in a channel.
        const MESSAGE_CREATE = 1 << 19;
        /// Message deleted in a channel.
        const MESSAGE_DELETE = 1 << 20;
        /// Multiple messages have been deleted in a channel.
        const MESSAGE_DELETE_BULK = 1 << 21;
        /// Message in a channel has been updated.
        const MESSAGE_UPDATE = 1 << 22;
        /// User's presence details are updated.
        const PRESENCE_UPDATE = 1 << 23;
        /// Group of presences are replaced.
        ///
        /// This is a placeholder as it *can* happen for bots but has no real
        /// meaning.
        const PRESENCES_REPLACE = 1 << 24;
        /// Reaction has been added to a message.
        const REACTION_ADD = 1 << 25;
        /// Reaction has been removed from a message.
        const REACTION_REMOVE = 1 << 26;
        /// All of the reactions for a message have been removed.
        const REACTION_REMOVE_ALL = 1 << 27;
        /// All of a given emoji's reactions for a message have been removed.
        const REACTION_REMOVE_EMOJI = 1 << 48;
        /// Session is initialized.
        const READY = 1 << 28;
        /// Session is resumed.
        const RESUMED = 1 << 29;
        /// Role has been created in a guild.
        const ROLE_CREATE = 1 << 30;
        /// Role has been deleted in a guild.
        const ROLE_DELETE = 1 << 31;
        /// Role has been updated in a guild.
        const ROLE_UPDATE = 1 << 32;
        /// Shard has finalized a session with the gateway.
        const SHARD_CONNECTED = 1 << 33;
        /// Shard has begun connecting to the gateway.
        const SHARD_CONNECTING = 1 << 34;
        /// Shard has disconnected from the gateway.
        const SHARD_DISCONNECTED = 1 << 35;
        /// Shard is identifying to create a session with the gateway.
        const SHARD_IDENTIFYING = 1 << 36;
        /// Incoming message has been received from the gateway.
        const SHARD_PAYLOAD = 1 << 45;
        /// Shard is reconnecting to the gateway.
        const SHARD_RECONNECTING = 1 << 37;
        /// Shard is resuming a session with the gateway.
        const SHARD_RESUMING = 1 << 38;
        /// User has begun typing in a channel.
        const TYPING_START = 1 << 39;
        /// Guild is unavailable, potentially due to an outage.
        const UNAVAILABLE_GUILD = 1 << 40;
        /// Current user's profile has been updated.
        const USER_UPDATE = 1 << 41;
        /// Voice server has provided an update with voice session details.
        const VOICE_SERVER_UPDATE = 1 << 42;
        /// User's state in a voice channel has been updated.
        const VOICE_STATE_UPDATE = 1 << 43;
        /// Webhook in a guild has been updated.
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
            EventType::GiftCodeUpdate => EventTypeFlags::GIFT_CODE_UPDATE,
            EventType::GuildCreate => EventTypeFlags::GUILD_CREATE,
            EventType::GuildDelete => EventTypeFlags::GUILD_DELETE,
            EventType::GuildEmojisUpdate => EventTypeFlags::GUILD_EMOJIS_UPDATE,
            EventType::GuildIntegrationsUpdate => EventTypeFlags::GUILD_INTEGRATIONS_UPDATE,
            EventType::GuildUpdate => EventTypeFlags::GUILD_UPDATE,
            EventType::InteractionCreate => EventTypeFlags::INTERACTION_CREATE,
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

impl<'a> TryFrom<(u8, Option<&'a str>)> for EventTypeFlags {
    type Error = (u8, Option<&'a str>);

    fn try_from((op, event_type): (u8, Option<&'a str>)) -> Result<Self, Self::Error> {
        match (op, event_type) {
            (1, _) => Ok(EventTypeFlags::GATEWAY_HEARTBEAT),
            (7, _) => Ok(EventTypeFlags::GATEWAY_RECONNECT),
            (9, _) => Ok(EventTypeFlags::GATEWAY_INVALIDATE_SESSION),
            (10, _) => Ok(EventTypeFlags::GATEWAY_HELLO),
            (11, _) => Ok(EventTypeFlags::GATEWAY_HEARTBEAT_ACK),
            (_, Some(event_type)) => {
                let flag = EventType::try_from(event_type).map_err(|kind| (op, Some(kind)))?;

                Ok(Self::from(flag))
            }
            (_, None) => Err((op, event_type)),
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

#[cfg(test)]
mod tests {
    use super::{EventType, EventTypeFlags};
    use static_assertions::assert_impl_all;
    use std::{convert::TryFrom, fmt::Debug, hash::Hash};

    assert_impl_all!(
        EventTypeFlags: Copy,
        Clone,
        Debug,
        Eq,
        From<EventType>,
        Hash,
        PartialEq,
        Send,
        Sync,
        TryFrom<(u8, Option<&'static str>)>
    );
}
