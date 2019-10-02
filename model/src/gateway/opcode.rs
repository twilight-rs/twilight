#[cfg_attr(
    feature = "serde-support",
    derive(serde_repr::Deserialize_repr, serde_repr::Serialize_repr)
)]
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, PartialOrd, Ord)]
#[repr(u8)]
pub enum OpCode {
    Event = 0,
    Heartbeat = 1,
    Identify = 2,
    StatusUpdate = 3,
    VoiceStateUpdate = 4,
    VoiceServerPing = 5,
    Resume = 6,
    Reconnect = 7,
    RequestGuildMembers = 8,
    InvalidSession = 9,
    Hello = 10,
    HeartbeatAck = 11,
}
