//! Home of [`EventTypeFlags`], and optimization technique for skipping gateway
//! event deserialization.

use bitflags::bitflags;
use twilight_model::gateway::event::EventType;

bitflags! {
    /// Bitflags representing all of the possible types of events.
    ///
    /// Specifying event types is an important optimization technique on top of
    /// [intents], which can dramatically decrease processor usage in many
    /// circumstances. While specifying intents are required by Discord and
    /// allow filtering groups of [events], event type flags are a
    /// Twilight-specific technique to filter out individual events from being
    /// deserialized at all, effectively discarding those events.
    ///
    /// For example, [`Intents::GUILDS`] includes a wide range of events from
    /// [`GuildCreate`] to [`GuildRoleUpdate`] to [`ChannelPinsUpdate`]. If the
    /// only events used in this group of events is, say, [`ChannelCreate`] and
    /// [`GuildRoleCreate`], then the [`CHANNEL_CREATE`] and
    /// [`GUILD_ROLE_CREATE`] event type flags can be specified in combination
    /// with that intent. This reduces the events deserialized and received to
    /// only those events.
    ///
    /// [`CHANNEL_CREATE`]: Self::CHANNEL_CREATE
    /// [`GUILD_ROLE_CREATE`]: Self::GUILD_ROLE_CREATE
    /// [`ChannelCreate`]: twilight_model::gateway::payload::incoming::ChannelCreate
    /// [`ChannelPinsUpdate`]: twilight_model::gateway::payload::incoming::ChannelPinsUpdate
    /// [`GuildRoleCreate`]: twilight_model::gateway::payload::incoming::GuildRoleCreate
    pub struct EventTypeFlags: u128 {
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
        /// A command's permissions has been updated.
        const COMMAND_PERMISSIONS_UPDATE = 1 << 70;
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
        const GATEWAY_INVALIDATE_SESSION = 1 << 69;
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
        /// A guild's integrations have been updated.
        const GUILD_SCHEDULED_EVENT_CREATE = 1 << 64;
        /// A guild's integrations have been updated.
        const GUILD_SCHEDULED_EVENT_DELETE = 1 << 65;
        /// A guild's integrations have been updated.
        const GUILD_SCHEDULED_EVENT_UPDATE = 1 << 66;
        /// A guild's integrations have been updated.
        const GUILD_SCHEDULED_EVENT_USER_ADD = 1 << 67;
        /// A guild's integrations have been updated.
        const GUILD_SCHEDULED_EVENT_USER_REMOVE = 1 << 68;
        /// A guild's stickers have been updated.
        const GUILD_STICKERS_UPDATE = 1 << 63;
        /// A guild has been updated.
        const GUILD_UPDATE = 1 << 14;
        /// A guild integration was created.
        const INTEGRATION_CREATE = 1 << 60;
        /// A guild integration was deleted.
        const INTEGRATION_DELETE = 1 << 61;
        /// A guild integration was updated.
        const INTEGRATION_UPDATE = 1 << 62;
        /// An interaction was invoked by a user.
        const INTERACTION_CREATE = 1 << 56;
        /// Invite for a channel has been created.
        const INVITE_CREATE = 1 << 46;
        /// Invite for a channel has been deleted.
        const INVITE_DELETE = 1 << 47;
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
        /// Stage instance was created in a stage channel.
        const STAGE_INSTANCE_CREATE = 1 << 57;
        /// Stage instance was deleted in a stage channel.
        const STAGE_INSTANCE_DELETE = 1 << 58;
        /// Stage instance was updated in a stage channel.
        const STAGE_INSTANCE_UPDATE = 1 << 59;
        /// A thread has been created, relevant to the current user,
        /// or the current user has been added to a thread.
        const THREAD_CREATE = 1 << 50;
        /// A thread, relevant to the current user, has been deleted.
        const THREAD_DELETE = 1 << 52;
        /// The current user has gained access to a thread.
        const THREAD_LIST_SYNC = 1 << 53;
        /// A user has been added to or removed from a thread.
        const THREAD_MEMBERS_UPDATE = 1 << 55;
        /// The thread member object for the current user has been updated.
        const THREAD_MEMBER_UPDATE = 1 << 54;
        /// A thread has been updated.
        const THREAD_UPDATE = 1 << 51;
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

impl EventTypeFlags {
    /// All [`EventTypeFlags`] in [`Intents::DIRECT_MESSAGES`].
    ///
    /// [`Intents::DIRECT_MESSAGES`]: crate::Intents::DIRECT_MESSAGES
    pub const DIRECT_MESSAGES: EventTypeFlags = EventTypeFlags::from_bits_truncate(
        EventTypeFlags::MESSAGE_CREATE.bits()
            | EventTypeFlags::MESSAGE_DELETE.bits()
            | EventTypeFlags::MESSAGE_DELETE_BULK.bits()
            | EventTypeFlags::MESSAGE_UPDATE.bits(),
    );

    /// All [`EventTypeFlags`] in [`Intents::DIRECT_MESSAGE_REACTIONS`].
    ///
    /// [`Intents::DIRECT_MESSAGE_REACTIONS`]: crate::Intents::DIRECT_MESSAGE_REACTIONS
    pub const DIRECT_MESSAGE_REACTIONS: EventTypeFlags = EventTypeFlags::from_bits_truncate(
        EventTypeFlags::REACTION_ADD.bits()
            | EventTypeFlags::REACTION_REMOVE.bits()
            | EventTypeFlags::REACTION_REMOVE_ALL.bits()
            | EventTypeFlags::REACTION_REMOVE_EMOJI.bits(),
    );

    /// All [`EventTypeFlags`] in [`Intents::DIRECT_MESSAGE_TYPING`].
    ///
    /// [`Intents::DIRECT_MESSAGE_TYPING`]: crate::Intents::DIRECT_MESSAGE_TYPING
    pub const DIRECT_MESSAGE_TYPING: EventTypeFlags =
        EventTypeFlags::from_bits_truncate(EventTypeFlags::TYPING_START.bits());

    /// All [`EventTypeFlags`] in [`Intents::GUILDS`].
    ///
    /// [`Intents::GUILDS`]: crate::Intents::GUILDS
    pub const GUILDS: EventTypeFlags = EventTypeFlags::from_bits_truncate(
        EventTypeFlags::CHANNEL_CREATE.bits()
            | EventTypeFlags::CHANNEL_DELETE.bits()
            | EventTypeFlags::CHANNEL_PINS_UPDATE.bits()
            | EventTypeFlags::CHANNEL_UPDATE.bits()
            | EventTypeFlags::GUILD_CREATE.bits()
            | EventTypeFlags::GUILD_DELETE.bits()
            | EventTypeFlags::GUILD_UPDATE.bits()
            | EventTypeFlags::ROLE_CREATE.bits()
            | EventTypeFlags::ROLE_DELETE.bits()
            | EventTypeFlags::ROLE_UPDATE.bits()
            | EventTypeFlags::STAGE_INSTANCE_CREATE.bits()
            | EventTypeFlags::STAGE_INSTANCE_UPDATE.bits()
            | EventTypeFlags::STAGE_INSTANCE_DELETE.bits()
            | EventTypeFlags::THREAD_CREATE.bits()
            | EventTypeFlags::THREAD_UPDATE.bits()
            | EventTypeFlags::THREAD_DELETE.bits()
            | EventTypeFlags::THREAD_LIST_SYNC.bits()
            | EventTypeFlags::THREAD_MEMBER_UPDATE.bits()
            | EventTypeFlags::THREAD_MEMBERS_UPDATE.bits(),
    );

    /// All [`EventTypeFlags`] in [`Intents::GUILD_BANS`].
    ///
    /// [`Intents::GUILD_BANS`]: crate::Intents::GUILD_BANS
    pub const GUILD_BANS: EventTypeFlags = EventTypeFlags::from_bits_truncate(
        EventTypeFlags::BAN_ADD.bits() | EventTypeFlags::BAN_REMOVE.bits(),
    );

    /// All [`EventTypeFlags`] in [`Intents::GUILD_EMOJIS_AND_STICKERS`].
    ///
    /// [`Intents::GUILD_EMOJIS_AND_STICKERS`]: crate::Intents::GUILD_EMOJIS_AND_STICKERS
    pub const GUILD_EMOJIS_AND_STICKERS: EventTypeFlags = EventTypeFlags::from_bits_truncate(
        EventTypeFlags::GUILD_EMOJIS_UPDATE.bits() | EventTypeFlags::GUILD_STICKERS_UPDATE.bits(),
    );

    /// All [`EventTypeFlags`] in [`Intents::GUILD_INTEGRATIONS`].
    ///
    /// [`Intents::GUILD_INTEGRATIONS`]: crate::Intents::GUILD_INTEGRATIONS
    pub const GUILD_INTEGRATIONS: EventTypeFlags = EventTypeFlags::from_bits_truncate(
        EventTypeFlags::GUILD_INTEGRATIONS_UPDATE.bits()
            | EventTypeFlags::INTEGRATION_CREATE.bits()
            | EventTypeFlags::INTEGRATION_UPDATE.bits()
            | EventTypeFlags::INTEGRATION_DELETE.bits(),
    );

    /// All [`EventTypeFlags`] in [`Intents::GUILD_INVITES`].
    ///
    /// [`Intents::GUILD_INVITES`]: crate::Intents::GUILD_INVITES
    pub const GUILD_INVITES: EventTypeFlags = EventTypeFlags::from_bits_truncate(
        EventTypeFlags::INVITE_CREATE.bits() | EventTypeFlags::INVITE_DELETE.bits(),
    );

    /// All [`EventTypeFlags`] in [`Intents::GUILD_MEMBERS`].
    ///
    /// [`Intents::GUILD_MEMBERS`]: crate::Intents::GUILD_MEMBERS
    pub const GUILD_MEMBERS: EventTypeFlags = EventTypeFlags::from_bits_truncate(
        EventTypeFlags::MEMBER_ADD.bits()
            | EventTypeFlags::MEMBER_REMOVE.bits()
            | EventTypeFlags::MEMBER_UPDATE.bits()
            | EventTypeFlags::THREAD_MEMBERS_UPDATE.bits(),
    );

    /// All [`EventTypeFlags`] in [`Intents::GUILD_MESSAGES`].
    ///
    /// [`Intents::GUILD_MESSAGES`]: crate::Intents::GUILD_MESSAGES
    pub const GUILD_MESSAGES: EventTypeFlags = EventTypeFlags::from_bits_truncate(
        EventTypeFlags::MESSAGE_CREATE.bits()
            | EventTypeFlags::MESSAGE_DELETE.bits()
            | EventTypeFlags::MESSAGE_DELETE.bits()
            | EventTypeFlags::MESSAGE_DELETE_BULK.bits(),
    );

    /// All [`EventTypeFlags`] in [`Intents::GUILD_MESSAGE_REACTIONS`].
    ///
    /// [`Intents::GUILD_MESSAGE_REACTIONS`]: crate::Intents::GUILD_MESSAGE_REACTIONS
    pub const GUILD_MESSAGE_REACTIONS: EventTypeFlags = EventTypeFlags::from_bits_truncate(
        EventTypeFlags::REACTION_ADD.bits()
            | EventTypeFlags::REACTION_REMOVE.bits()
            | EventTypeFlags::REACTION_REMOVE_ALL.bits()
            | EventTypeFlags::REACTION_REMOVE_EMOJI.bits(),
    );

    /// All [`EventTypeFlags`] in [`Intents::GUILD_MESSAGE_TYPING`].
    ///
    /// [`Intents::GUILD_MESSAGE_TYPING`]: crate::Intents::GUILD_MESSAGE_TYPING
    pub const GUILD_MESSAGE_TYPING: EventTypeFlags =
        EventTypeFlags::from_bits_truncate(EventTypeFlags::TYPING_START.bits());

    /// All [`EventTypeFlags`] in [`Intents::GUILD_PRESENCES`].
    ///
    /// [`Intents::GUILD_PRESENCES`]: crate::Intents::GUILD_PRESENCES
    pub const GUILD_PRESENCES: EventTypeFlags =
        EventTypeFlags::from_bits_truncate(EventTypeFlags::PRESENCE_UPDATE.bits());

    /// All [`EventTypeFlags`] in [`Intents::GUILD_SCHEDULED_EVENTS`].
    ///
    /// [`Intents::GUILD_SCHEDULED_EVENTS`]: crate::Intents::GUILD_SCHEDULED_EVENTS
    pub const GUILD_SCHEDULED_EVENTS: EventTypeFlags = EventTypeFlags::from_bits_truncate(
        EventTypeFlags::GUILD_SCHEDULED_EVENT_CREATE.bits()
            | EventTypeFlags::GUILD_SCHEDULED_EVENT_DELETE.bits()
            | EventTypeFlags::GUILD_SCHEDULED_EVENT_UPDATE.bits()
            | EventTypeFlags::GUILD_SCHEDULED_EVENT_USER_ADD.bits()
            | EventTypeFlags::GUILD_SCHEDULED_EVENT_USER_REMOVE.bits(),
    );

    /// All [`EventTypeFlags`] in [`Intents::GUILD_VOICE_STATES`].
    ///
    /// [`Intents::GUILD_VOICE_STATES`]: crate::Intents::GUILD_VOICE_STATES
    pub const GUILD_VOICE_STATES: EventTypeFlags =
        EventTypeFlags::from_bits_truncate(EventTypeFlags::VOICE_STATE_UPDATE.bits());

    /// All [`EventTypeFlags`] in [`Intents::GUILD_WEBHOOKS`].
    ///
    /// [`Intents::GUILD_WEBHOOKS`]: crate::Intents::GUILD_WEBHOOKS
    pub const GUILD_WEBHOOKS: EventTypeFlags =
        EventTypeFlags::from_bits_truncate(EventTypeFlags::WEBHOOKS_UPDATE.bits());
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
            EventType::CommandPermissionsUpdate => EventTypeFlags::COMMAND_PERMISSIONS_UPDATE,
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
            EventType::GuildScheduledEventCreate => EventTypeFlags::GUILD_SCHEDULED_EVENT_CREATE,
            EventType::GuildScheduledEventDelete => EventTypeFlags::GUILD_SCHEDULED_EVENT_DELETE,
            EventType::GuildScheduledEventUpdate => EventTypeFlags::GUILD_SCHEDULED_EVENT_UPDATE,
            EventType::GuildScheduledEventUserAdd => EventTypeFlags::GUILD_SCHEDULED_EVENT_USER_ADD,
            EventType::GuildScheduledEventUserRemove => {
                EventTypeFlags::GUILD_SCHEDULED_EVENT_USER_REMOVE
            }
            EventType::GuildStickersUpdate => EventTypeFlags::GUILD_STICKERS_UPDATE,
            EventType::GuildUpdate => EventTypeFlags::GUILD_UPDATE,
            EventType::IntegrationCreate => EventTypeFlags::INTEGRATION_CREATE,
            EventType::IntegrationDelete => EventTypeFlags::INTEGRATION_DELETE,
            EventType::IntegrationUpdate => EventTypeFlags::INTEGRATION_UPDATE,
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
            EventType::StageInstanceCreate => EventTypeFlags::STAGE_INSTANCE_CREATE,
            EventType::StageInstanceDelete => EventTypeFlags::STAGE_INSTANCE_DELETE,
            EventType::StageInstanceUpdate => EventTypeFlags::STAGE_INSTANCE_UPDATE,
            EventType::ThreadCreate => EventTypeFlags::THREAD_CREATE,
            EventType::ThreadDelete => EventTypeFlags::THREAD_DELETE,
            EventType::ThreadListSync => EventTypeFlags::THREAD_LIST_SYNC,
            EventType::ThreadMembersUpdate => EventTypeFlags::THREAD_MEMBERS_UPDATE,
            EventType::ThreadMemberUpdate => EventTypeFlags::THREAD_MEMBER_UPDATE,
            EventType::ThreadUpdate => EventTypeFlags::THREAD_UPDATE,
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
        Self::all()
    }
}

#[cfg(test)]
mod tests {
    use super::{EventType, EventTypeFlags};
    use static_assertions::assert_impl_all;
    use std::{fmt::Debug, hash::Hash};

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
