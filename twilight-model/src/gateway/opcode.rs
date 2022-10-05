use serde_repr::{Deserialize_repr, Serialize_repr};

/// Gateway event opcodes.
///
/// The documentation is written from a client's perspective.
///
/// [`PresenceUpdate`], [`RequestGuildMembers`], and [`VoiceStateUpdate`] are
/// not requiried for establishing or maintaining a gateway connection.
///
/// [`PresenceUpdate`]: Self::PresenceUpdate
/// [`RequestGuildMembers`]: Self::RequestGuildMembers
/// [`VoiceStateUpdate`]: Self::VoiceStateUpdate
#[derive(Clone, Copy, Debug, Deserialize_repr, Eq, Hash, PartialEq, Serialize_repr)]
#[non_exhaustive]
#[repr(u8)]
pub enum OpCode {
    /// [`DispatchEvent`] and sequence number.
    ///
    /// Will only be received after establishing or resuming a session.
    ///
    /// [`DispatchEvent`]: super::event::DispatchEvent
    Dispatch = 0,
    /// Periodically sent to maintain the connection and may be received to
    /// immediately request one.
    Heartbeat = 1,
    /// Start a new session.
    Identify = 2,
    /// Request to update the client's presence.
    PresenceUpdate = 3,
    /// Request to join, leave or move between voice channels.
    VoiceStateUpdate = 4,
    /// Resume a previously disconnected session, skipping over [`Identify`].
    ///
    /// [`Identify`]: Self::Identify
    Resume = 6,
    /// Indicates that a reconnect is required.
    Reconnect = 7,
    /// Request a list of members for a guild.
    RequestGuildMembers = 8,
    /// Received when the session is invalidated.
    InvalidSession = 9,
    /// Received after connecting, contains the heartbeat interval.
    Hello = 10,
    /// Received in response to sending a [`Heartbeat`].
    ///
    /// [`Heartbeat`]: Self::Heartbeat
    HeartbeatAck = 11,
}

impl OpCode {
    /// Try to match an integer value to an opcode, returning [`None`] if no
    /// match is found.
    pub const fn from(code: u8) -> Option<Self> {
        Some(match code {
            0 => Self::Dispatch,
            1 => Self::Heartbeat,
            2 => Self::Identify,
            3 => Self::PresenceUpdate,
            4 => Self::VoiceStateUpdate,
            6 => Self::Resume,
            7 => Self::Reconnect,
            8 => Self::RequestGuildMembers,
            9 => Self::InvalidSession,
            10 => Self::Hello,
            11 => Self::HeartbeatAck,
            _ => return None,
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
    );

    #[test]
    fn variants() {
        serde_test::assert_tokens(&OpCode::Dispatch, &[Token::U8(0)]);
        serde_test::assert_tokens(&OpCode::Heartbeat, &[Token::U8(1)]);
        serde_test::assert_tokens(&OpCode::Identify, &[Token::U8(2)]);
        serde_test::assert_tokens(&OpCode::PresenceUpdate, &[Token::U8(3)]);
        serde_test::assert_tokens(&OpCode::VoiceStateUpdate, &[Token::U8(4)]);
        serde_test::assert_tokens(&OpCode::Resume, &[Token::U8(6)]);
        serde_test::assert_tokens(&OpCode::Reconnect, &[Token::U8(7)]);
        serde_test::assert_tokens(&OpCode::RequestGuildMembers, &[Token::U8(8)]);
        serde_test::assert_tokens(&OpCode::InvalidSession, &[Token::U8(9)]);
        serde_test::assert_tokens(&OpCode::Hello, &[Token::U8(10)]);
        serde_test::assert_tokens(&OpCode::HeartbeatAck, &[Token::U8(11)]);
    }
}
