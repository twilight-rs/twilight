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
