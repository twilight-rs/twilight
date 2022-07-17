use serde_repr::{Deserialize_repr, Serialize_repr};

/// Gateway opcodes.
#[derive(Clone, Copy, Debug, Deserialize_repr, Eq, Hash, PartialEq, Serialize_repr)]
#[non_exhaustive]
#[repr(u8)]
pub enum OpCode {
    /// An event was received.
    Event = 0,
    /// Fired periodically to keep connection alive.
    Heartbeat = 1,
    /// Start a new session.
    Identify = 2,
    /// Update the client's presence information.
    PresenceUpdate = 3,
    /// Join, leave or move between voice channels.
    VoiceStateUpdate = 4,
    /// Voice ping checking. This opcode is deprecated.
    VoiceServerPing = 5,
    /// Resume a previously disconnected session.
    Resume = 6,
    /// Received to indicate a reconnect is required.
    Reconnect = 7,
    /// Request a list of members for a guild.
    RequestGuildMembers = 8,
    /// Received when the session is invalidated.
    InvalidSession = 9,
    /// Received after connecting, contains heartbeat interval.
    Hello = 10,
    /// Received in response to a heartbeat.
    HeartbeatAck = 11,
}

impl TryFrom<u8> for OpCode {
    type Error = u8;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Ok(match value {
            0 => Self::Event,
            1 => Self::Heartbeat,
            2 => Self::Identify,
            3 => Self::PresenceUpdate,
            4 => Self::VoiceStateUpdate,
            5 => Self::VoiceServerPing,
            6 => Self::Resume,
            7 => Self::Reconnect,
            8 => Self::RequestGuildMembers,
            9 => Self::InvalidSession,
            10 => Self::Hello,
            11 => Self::HeartbeatAck,
            other => return Err(other),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::OpCode;
    use serde::{Deserialize, Serialize};
    use serde_test::Token;
    use static_assertions::assert_impl_all;
    use std::fmt::Debug;

    assert_impl_all!(
        OpCode: Clone,
        Copy,
        Debug,
        Deserialize<'static>,
        Eq,
        PartialEq,
        Send,
        Serialize,
        Sync,
        TryFrom<u8>
    );

    #[test]
    fn variants() {
        serde_test::assert_tokens(&OpCode::Event, &[Token::U8(0)]);
        serde_test::assert_tokens(&OpCode::Heartbeat, &[Token::U8(1)]);
        serde_test::assert_tokens(&OpCode::Identify, &[Token::U8(2)]);
        serde_test::assert_tokens(&OpCode::PresenceUpdate, &[Token::U8(3)]);
        serde_test::assert_tokens(&OpCode::VoiceStateUpdate, &[Token::U8(4)]);
        serde_test::assert_tokens(&OpCode::VoiceServerPing, &[Token::U8(5)]);
        serde_test::assert_tokens(&OpCode::Resume, &[Token::U8(6)]);
        serde_test::assert_tokens(&OpCode::Reconnect, &[Token::U8(7)]);
        serde_test::assert_tokens(&OpCode::RequestGuildMembers, &[Token::U8(8)]);
        serde_test::assert_tokens(&OpCode::InvalidSession, &[Token::U8(9)]);
        serde_test::assert_tokens(&OpCode::Hello, &[Token::U8(10)]);
        serde_test::assert_tokens(&OpCode::HeartbeatAck, &[Token::U8(11)]);
    }
}
