use super::Event;
use serde::{Deserialize, Serialize};
use std::convert::TryFrom;

/// Indicator that a shard is now fully connected.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Connected {
    /// The interval that heartbeats are being sent to the gateway.
    pub heartbeat_interval: u64,
    /// The ID of the shard that's now connected.
    pub shard_id: u64,
}

/// Indicator that a shard is now connecting.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Connecting {
    /// The URL used to connect to the gateway.
    pub gateway: String,
    /// The ID of the shard that's now connecting.
    pub shard_id: u64,
}

/// Indicator that a shard is now disconnected and may soon be reconnecting if
/// not explicitly shutdown.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
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
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Identifying {
    /// The ID of the shard that identified with the gateway.
    pub shard_id: u64,
    /// The total shards used by the bot.
    pub shard_total: u64,
}

/// A payload of bytes came in through the gateway.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Payload {
    /// The bytes that came in.
    pub bytes: Vec<u8>,
}

/// Indicator that a shard is now reconnecting.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Reconnecting {
    /// The ID of the shard that began reconnecting.
    pub shard_id: u64,
}

/// Indicator that a shard is now resuming a session after a disconnect.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Resuming {
    /// The event sequence sent when resuming was initiated.
    pub seq: u64,
    /// The ID of the shard that began resuming.
    pub shard_id: u64,
}

/// "Meta" events about a shard's status, not from the gateway.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(untagged)]
pub enum ShardEvent {
    /// A shard is now in a Connected stage after being fully connected to the
    /// gateway.
    Connected(Connected),
    /// A shard is now in a Connecting stage after starting to connect to the
    /// gateway.
    Connecting(Connecting),
    /// A shard is now in a Disconnected stage after the connection was closed.
    Disconnected(Disconnected),
    /// A shard is now in a Identifying stage after starting a new session.
    Identifying(Identifying),
    /// A payload of bytes came in through the shard's connection.
    Payload(Payload),
    /// A shard is now in a Reconnecting stage after a disconnect or session was
    /// ended.
    Reconnecting(Reconnecting),
    /// A shard is now in a Resuming stage after a disconnect.
    Resuming(Resuming),
}

impl TryFrom<Event> for ShardEvent {
    type Error = &'static str;

    fn try_from(event: Event) -> Result<Self, Self::Error> {
        match event {
            Event::ShardConnected(v) => Ok(Self::Connected(v)),
            Event::ShardConnecting(v) => Ok(Self::Connecting(v)),
            Event::ShardDisconnected(v) => Ok(Self::Disconnected(v)),
            Event::ShardIdentifying(v) => Ok(Self::Identifying(v)),
            Event::ShardPayload(v) => Ok(Self::Payload(v)),
            Event::ShardReconnecting(v) => Ok(Self::Reconnecting(v)),
            Event::ShardResuming(v) => Ok(Self::Resuming(v)),

            _ => Err("event was not a ShardEvent"),
        }
    }
}
