#![allow(clippy::wildcard_imports)]
//! Events that the shard emits to event streams.
//!
//! Included is the larger [`Event`] exposed to event streams. It contains
//! variants with all of the possible events that can come in: new channels,
//! heartbeat acknowledgements, "meta" events of when the shard disconnects or
//! connects, etc.
//!
//! Also included is the [`EventType`] bitflags, which can be used to identify
//! the type of an event and to filter events from event streams via
//! [`Shard::some_events`].
//!
//! [`Event`]: enum.Event.html
//! [`Shard::some_events`]: ../struct.Shard.html#method.some_events

use crate::event::{DispatchEvent, GatewayEvent};
use bitflags::bitflags;
use futures::{
    channel::mpsc::UnboundedReceiver,
    stream::{Stream, StreamExt},
};
use std::{
    pin::Pin,
    task::{Context, Poll},
};
use twilight_model::gateway::payload::*;

bitflags! {
    /// Bitflags representing all of the possible types of events.
    pub struct EventType: u64 {
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

impl Default for EventType {
    fn default() -> Self {
        let mut flags = Self::all();
        flags.remove(Self::SHARD_PAYLOAD);

        flags
    }
}

/// Indicator that a shard is now fully connected.
#[derive(Clone, Debug)]
pub struct Connected {
    /// The interval that heartbeats are being sent to the gateway.
    pub heartbeat_interval: u64,
    /// The ID of the shard that's now connected.
    pub shard_id: u64,
}

/// Indicator that a shard is now connecting.
#[derive(Clone, Debug)]
pub struct Connecting {
    /// The URL used to connect to the gateway.
    pub gateway: String,
    /// The ID of the shard that's now connecting.
    pub shard_id: u64,
}

/// Indicator that a shard is now disconnected and may soon be reconnecting if
/// not explicitly shutdown.
#[derive(Clone, Debug)]
pub struct Disconnected {
    /// The code for the disconnect if not initiated by the host, if any.
    pub code: Option<u16>,
    /// The reason for the disconnect if not initiated by the host, if any.
    pub reason: Option<String>,
    /// The ID of the shard that's now disconnected.
    pub shard_id: u64,
}

/// Indicator that a shard is now identifying with the gateway to create a new
/// session.
#[derive(Clone, Debug)]
pub struct Identifying {
    /// The ID of the shard that identified with the gateway.
    pub shard_id: u64,
    /// The total shards used by the bot.
    pub shard_total: u64,
}

/// A payload of bytes came in through the gateway.
#[derive(Clone, Debug)]
pub struct Payload {
    /// The bytes that came in.
    pub bytes: Vec<u8>,
}

/// Indicator that a shard is now reconnecting.
#[derive(Clone, Debug)]
pub struct Reconnecting {
    /// The ID of the shard that began reconnecting.
    pub shard_id: u64,
}

/// Indicator that a shard is now resuming a session after a disconnect.
#[derive(Clone, Debug)]
pub struct Resuming {
    /// The event sequence sent when resuming was initiated.
    pub seq: u64,
    /// The ID of the shard that began resuming.
    pub shard_id: u64,
}

/// "Meta" events about a shard's status, not from the gateway.
#[derive(Clone, Debug)]
pub enum ShardEvent {
    /// A shard is now in [`Stage::Connected`] phase after being fully connected
    /// to the gateway.
    ///
    /// [`Stage::Connected`]: ../stage/enum.Stage.html#variant.Connected
    Connected(Connected),
    /// A shard is now in [`Stage::Connecting`] phase after starting to connect
    /// to the gateway.
    ///
    /// [`Stage::Connecting`]: ../stage/enum.Stage.html#variant.Connecting
    Connecting(Connecting),
    /// A shard is now in [`Stage::Disconnected`] phase after the connection was
    /// closed.
    ///
    /// [`Stage::Disconnected`]: ../stage/enum.Stage.html#variant.Disconnected
    Disconnected(Disconnected),
    /// A shard is now in [`Stage::Identifying`] phase after starting a new
    /// session.
    ///
    /// [`Stage::Identifying`]: ../stage/enum.Stage.html#variant.Identifying
    Identifying(Identifying),
    /// A payload of bytes came in through the shard's connection.
    Payload(Payload),
    /// A shard is now in [`Stage::Reconnecting`] phase after a disconnect
    /// or session was ended.
    ///
    /// [`Stage::Reconnecting`]: ../stage/enum.Stage.html#variant.Reconnecting
    Reconnecting(Reconnecting),
    /// A shard is now in [`Stage::Resuming`] phase after a disconnect.
    ///
    /// [`Stage::Resuming`]: ../stage/enum.Stage.html#variant.Resuming
    Resuming(Resuming),
}

/// Any type of event that a [`Shard`] emits.
///
/// This brings together all of the types of [`DispatchEvent`]s,
/// [`GatewayEvent`]s, and [`ShardEvent`]s.
///
/// [`DispatchEvent`]: ../../event/struct.DispatchEvent.html
/// [`GatewayEvent`]: ../../event/struct.GatewayEvent.html
/// [`ShardEvent`]: struct.ShardEvent.html
/// [`Shard`]: ../struct.Shard.html
#[derive(Clone, Debug)]
pub enum Event {
    /// A user was banned from a guild.
    BanAdd(BanAdd),
    /// A user's ban from a guild was removed.
    BanRemove(BanRemove),
    /// A channel was created.
    ChannelCreate(ChannelCreate),
    /// A channel was deleted.
    ChannelDelete(ChannelDelete),
    /// A channel's pins were updated.
    ChannelPinsUpdate(ChannelPinsUpdate),
    /// A channel was updated.
    ChannelUpdate(ChannelUpdate),
    /// A heartbeat was sent to or received from the gateway.
    GatewayHeartbeat(u64),
    /// A heartbeat acknowledgement was received from the gateway.
    GatewayHeartbeatAck,
    /// A "hello" packet was received from the gateway.
    GatewayHello(u64),
    /// A shard's session was invalidated.
    ///
    /// `true` if resumeable. If not, then the shard must do a full reconnect.
    GatewayInvalidateSession(bool),
    /// The gateway is indicating to perform a reconnect.
    GatewayReconnect,
    /// A guild was created.
    GuildCreate(Box<GuildCreate>),
    /// A guild was deleted or the current user was removed from a guild.
    GuildDelete(Box<GuildDelete>),
    /// A guild's emojis were updated.
    GuildEmojisUpdate(GuildEmojisUpdate),
    /// A guild's integrations were updated.
    GuildIntegrationsUpdate(GuildIntegrationsUpdate),
    /// A guild was updated.
    GuildUpdate(Box<GuildUpdate>),
    /// A invite was made.
    InviteCreate(Box<InviteCreate>),
    /// A invite was deleted.
    InviteDelete(InviteDelete),
    /// A user was added to a guild.
    MemberAdd(Box<MemberAdd>),
    /// A user was removed from a guild.
    MemberRemove(MemberRemove),
    /// A user's member object in a guild was updated.
    MemberUpdate(Box<MemberUpdate>),
    /// A chunk of members were received from the gateway.
    MemberChunk(MemberChunk),
    /// A message was created in a channel.
    MessageCreate(Box<MessageCreate>),
    /// A message was deleted in a channel.
    MessageDelete(MessageDelete),
    /// Multiple messages were deleted in a channel.
    MessageDeleteBulk(MessageDeleteBulk),
    /// A message was updated in a channel.
    MessageUpdate(Box<MessageUpdate>),
    /// A user's active presence (such as game or online status) was updated.
    PresenceUpdate(Box<PresenceUpdate>),
    /// Multiple presences outside of a guild were updated.
    ///
    /// For bots this is always empty and useless.
    PresencesReplace,
    /// A reaction was added to a message.
    ReactionAdd(Box<ReactionAdd>),
    /// A reaction was removed from a message.
    ReactionRemove(Box<ReactionRemove>),
    /// All reactions were removed from a message.
    ReactionRemoveAll(ReactionRemoveAll),
    /// All instances of a given emoji from the reactions of a message were removed
    ReactionRemoveEmoji(ReactionRemoveEmoji),
    /// A shard is now "ready" and fully connected.
    Ready(Box<Ready>),
    /// A shard has successfully resumed.
    Resumed,
    /// A role was created in a guild.
    RoleCreate(RoleCreate),
    /// A role was deleted in a guild.
    RoleDelete(RoleDelete),
    /// A role was updated in a guild.
    RoleUpdate(RoleUpdate),
    /// A shard is now in [`Stage::Connected`] phase after being fully connected
    /// to the gateway.
    ///
    /// [`Stage::Connected`]: ../stage/enum.Stage.html#variant.Connected
    ShardConnected(Connected),
    /// A shard is now in [`Stage::Connecting`] phase after starting to connect
    /// to the gateway.
    ///
    /// [`Stage::Connecting`]: ../stage/enum.Stage.html#variant.Connecting
    ShardConnecting(Connecting),
    /// A shard is now in [`Stage::Disconnected`] phase after the connection was
    /// closed.
    ///
    /// [`Stage::Disconnected`]: ../stage/enum.Stage.html#variant.Disconnected
    ShardDisconnected(Disconnected),
    /// A shard is now in [`Stage::Identifying`] phase after starting a new
    /// session.
    ///
    /// [`Stage::Identifying`]: ../stage/enum.Stage.html#variant.Identifying
    ShardIdentifying(Identifying),
    /// A shard is now in [`Stage::Reconnecting`] phase after a disconnect
    /// or session was ended.
    ///
    /// [`Stage::Reconnecting`]: ../stage/enum.Stage.html#variant.Reconnecting
    ShardReconnecting(Reconnecting),
    /// A payload of bytes came in through the shard's connection.
    ShardPayload(Payload),
    /// A shard is now in [`Stage::Resuming`] phase after a disconnect.
    ///
    /// [`Stage::Resuming`]: ../stage/enum.Stage.html#variant.Resuming
    ShardResuming(Resuming),
    /// A user started typing in a channel.
    TypingStart(Box<TypingStart>),
    /// A guild is now unavailable.
    UnavailableGuild(UnavailableGuild),
    /// The current user was updated.
    UserUpdate(UserUpdate),
    /// A voice server update was sent.
    VoiceServerUpdate(VoiceServerUpdate),
    /// A voice state in a voice channel was updated.
    VoiceStateUpdate(Box<VoiceStateUpdate>),
    /// A webhook was updated.
    WebhooksUpdate(WebhooksUpdate),
}

impl Event {
    /// Returns the type of event that this event is, switching the equivalent
    /// bit in the returned bitflags to true.
    ///
    /// # Examples
    ///
    /// ```
    /// use twilight_gateway::shard::{Event, EventType};
    ///
    /// assert_eq!(EventType::RESUMED, Event::Resumed.event_type());
    /// ```
    pub fn event_type(&self) -> EventType {
        match self {
            Self::BanAdd(_) => EventType::BAN_ADD,
            Self::BanRemove(_) => EventType::BAN_REMOVE,
            Self::ChannelCreate(_) => EventType::CHANNEL_CREATE,
            Self::ChannelDelete(_) => EventType::CHANNEL_DELETE,
            Self::ChannelPinsUpdate(_) => EventType::CHANNEL_PINS_UPDATE,
            Self::ChannelUpdate(_) => EventType::CHANNEL_UPDATE,
            Self::GatewayHeartbeat(_) => EventType::GATEWAY_HEARTBEAT,
            Self::GatewayHeartbeatAck => EventType::GATEWAY_HEARTBEAT_ACK,
            Self::GatewayHello(_) => EventType::GATEWAY_HELLO,
            Self::GatewayInvalidateSession(_) => EventType::GATEWAY_INVALIDATE_SESSION,
            Self::GatewayReconnect => EventType::GATEWAY_RECONNECT,
            Self::GuildCreate(_) => EventType::GUILD_CREATE,
            Self::GuildDelete(_) => EventType::GUILD_DELETE,
            Self::GuildEmojisUpdate(_) => EventType::GUILD_EMOJIS_UPDATE,
            Self::GuildIntegrationsUpdate(_) => EventType::GUILD_INTEGRATIONS_UPDATE,
            Self::GuildUpdate(_) => EventType::GUILD_UPDATE,
            Self::InviteCreate(_) => EventType::INVITE_CREATE,
            Self::InviteDelete(_) => EventType::INVITE_DELETE,
            Self::MemberAdd(_) => EventType::MEMBER_ADD,
            Self::MemberRemove(_) => EventType::MEMBER_REMOVE,
            Self::MemberUpdate(_) => EventType::MEMBER_UPDATE,
            Self::MemberChunk(_) => EventType::MEMBER_CHUNK,
            Self::MessageCreate(_) => EventType::MESSAGE_CREATE,
            Self::MessageDelete(_) => EventType::MESSAGE_DELETE,
            Self::MessageDeleteBulk(_) => EventType::MESSAGE_DELETE_BULK,
            Self::MessageUpdate(_) => EventType::MESSAGE_UPDATE,
            Self::PresenceUpdate(_) => EventType::PRESENCE_UPDATE,
            Self::PresencesReplace => EventType::PRESENCES_REPLACE,
            Self::ReactionAdd(_) => EventType::REACTION_ADD,
            Self::ReactionRemove(_) => EventType::REACTION_REMOVE,
            Self::ReactionRemoveAll(_) => EventType::REACTION_REMOVE_ALL,
            Self::ReactionRemoveEmoji(_) => EventType::REACTION_REMOVE_EMOJI,
            Self::Ready(_) => EventType::READY,
            Self::Resumed => EventType::RESUMED,
            Self::RoleCreate(_) => EventType::ROLE_CREATE,
            Self::RoleDelete(_) => EventType::ROLE_DELETE,
            Self::RoleUpdate(_) => EventType::ROLE_UPDATE,
            Self::ShardConnected(_) => EventType::SHARD_CONNECTED,
            Self::ShardConnecting(_) => EventType::SHARD_CONNECTING,
            Self::ShardDisconnected(_) => EventType::SHARD_DISCONNECTED,
            Self::ShardIdentifying(_) => EventType::SHARD_IDENTIFYING,
            Self::ShardPayload(_) => EventType::SHARD_PAYLOAD,
            Self::ShardReconnecting(_) => EventType::SHARD_RECONNECTING,
            Self::ShardResuming(_) => EventType::SHARD_RESUMING,
            Self::TypingStart(_) => EventType::TYPING_START,
            Self::UnavailableGuild(_) => EventType::UNAVAILABLE_GUILD,
            Self::UserUpdate(_) => EventType::USER_UPDATE,
            Self::VoiceServerUpdate(_) => EventType::VOICE_SERVER_UPDATE,
            Self::VoiceStateUpdate(_) => EventType::VOICE_STATE_UPDATE,
            Self::WebhooksUpdate(_) => EventType::WEBHOOKS_UPDATE,
        }
    }
}

impl From<Box<DispatchEvent>> for Event {
    fn from(event: Box<DispatchEvent>) -> Self {
        match *event {
            DispatchEvent::BanAdd(v) => Self::BanAdd(v),
            DispatchEvent::BanRemove(v) => Self::BanRemove(v),
            DispatchEvent::ChannelCreate(v) => Self::ChannelCreate(v),
            DispatchEvent::ChannelDelete(v) => Self::ChannelDelete(v),
            DispatchEvent::ChannelPinsUpdate(v) => Self::ChannelPinsUpdate(v),
            DispatchEvent::ChannelUpdate(v) => Self::ChannelUpdate(v),
            DispatchEvent::GuildCreate(v) => Self::GuildCreate(v),
            DispatchEvent::GuildDelete(v) => Self::GuildDelete(v),
            DispatchEvent::GuildEmojisUpdate(v) => Self::GuildEmojisUpdate(v),
            DispatchEvent::GuildIntegrationsUpdate(v) => Self::GuildIntegrationsUpdate(v),
            DispatchEvent::InviteCreate(v) => Self::InviteCreate(v),
            DispatchEvent::InviteDelete(v) => Self::InviteDelete(v),
            DispatchEvent::MemberAdd(v) => Self::MemberAdd(v),
            DispatchEvent::MemberRemove(v) => Self::MemberRemove(v),
            DispatchEvent::MemberUpdate(v) => Self::MemberUpdate(v),
            DispatchEvent::MemberChunk(v) => Self::MemberChunk(v),
            DispatchEvent::RoleCreate(v) => Self::RoleCreate(v),
            DispatchEvent::RoleDelete(v) => Self::RoleDelete(v),
            DispatchEvent::RoleUpdate(v) => Self::RoleUpdate(v),
            DispatchEvent::GuildUpdate(v) => Self::GuildUpdate(v),
            DispatchEvent::MessageCreate(v) => Self::MessageCreate(v),
            DispatchEvent::MessageDelete(v) => Self::MessageDelete(v),
            DispatchEvent::MessageDeleteBulk(v) => Self::MessageDeleteBulk(v),
            DispatchEvent::MessageUpdate(v) => Self::MessageUpdate(v),
            DispatchEvent::PresenceUpdate(v) => Self::PresenceUpdate(v),
            DispatchEvent::PresencesReplace => Self::PresencesReplace,
            DispatchEvent::ReactionAdd(v) => Self::ReactionAdd(v),
            DispatchEvent::ReactionRemove(v) => Self::ReactionRemove(v),
            DispatchEvent::ReactionRemoveAll(v) => Self::ReactionRemoveAll(v),
            DispatchEvent::ReactionRemoveEmoji(v) => Self::ReactionRemoveEmoji(v),
            DispatchEvent::Ready(v) => Self::Ready(v),
            DispatchEvent::Resumed => Self::Resumed,
            DispatchEvent::TypingStart(v) => Self::TypingStart(v),
            DispatchEvent::UnavailableGuild(v) => Self::UnavailableGuild(v),
            DispatchEvent::UserUpdate(v) => Self::UserUpdate(v),
            DispatchEvent::VoiceServerUpdate(v) => Self::VoiceServerUpdate(v),
            DispatchEvent::VoiceStateUpdate(v) => Self::VoiceStateUpdate(v),
            DispatchEvent::WebhooksUpdate(v) => Self::WebhooksUpdate(v),
        }
    }
}

impl From<GatewayEvent> for Event {
    fn from(event: GatewayEvent) -> Self {
        match event {
            GatewayEvent::Dispatch(_, e) => Self::from(e),
            GatewayEvent::Heartbeat(interval) => Self::GatewayHeartbeat(interval),
            GatewayEvent::HeartbeatAck => Self::GatewayHeartbeatAck,
            GatewayEvent::Hello(interval) => Self::GatewayHello(interval),
            GatewayEvent::InvalidateSession(r) => Self::GatewayInvalidateSession(r),
            GatewayEvent::Reconnect => Self::GatewayReconnect,
        }
    }
}

impl From<ShardEvent> for Event {
    fn from(event: ShardEvent) -> Self {
        match event {
            ShardEvent::Connected(v) => Self::ShardConnected(v),
            ShardEvent::Connecting(v) => Self::ShardConnecting(v),
            ShardEvent::Disconnected(v) => Self::ShardDisconnected(v),
            ShardEvent::Identifying(v) => Self::ShardIdentifying(v),
            ShardEvent::Payload(v) => Self::ShardPayload(v),
            ShardEvent::Reconnecting(v) => Self::ShardReconnecting(v),
            ShardEvent::Resuming(v) => Self::ShardResuming(v),
        }
    }
}

/// A stream of events from a [`Shard`].
///
/// The events of this stream may or may not be filtered. You can check the
/// event types returned by [`Events::event_types`] to see what events can come
/// in through this stream.
///
/// [`Events::event_types`]: #method.event_types
/// [`Shard`]: ../struct.Shard.html
pub struct Events {
    event_types: EventType,
    rx: UnboundedReceiver<Event>,
}

impl Events {
    pub(super) fn new(event_types: EventType, rx: UnboundedReceiver<Event>) -> Self {
        Self { event_types, rx }
    }

    /// Returns the event types that can be passed to this stream.
    pub fn event_types(&self) -> EventType {
        self.event_types
    }
}

impl Stream for Events {
    type Item = Event;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        self.rx.poll_next_unpin(cx)
    }
}
