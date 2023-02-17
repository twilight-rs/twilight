use serde_repr::{Deserialize_repr, Serialize_repr};

/// Gateway event's payload type.
#[derive(Clone, Copy, Debug, Deserialize_repr, Eq, Hash, PartialEq, Serialize_repr)]
#[non_exhaustive]
#[repr(u8)]
pub enum OpCode {
    /// [`DispatchEvent`] and sequence number.
    ///
    /// Will only be received when connected to the gateway with an active
    /// session.
    ///
    /// [`DispatchEvent`]: super::event::DispatchEvent
    Dispatch = 0,
    /// Periodically sent to maintain the gateway connection and may be received
    /// to immediately request one.
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

    /// Whether the opcode is received by the client.
    ///
    /// This includes the following opcodes:
    ///
    /// - [`Dispatch`]
    /// - [`Heartbeat`]
    /// - [`HeartbeatAck`]
    /// - [`Hello`]
    /// - [`InvalidSession`]
    /// - [`Reconnect`]
    ///
    /// [`Dispatch`]: Self::Dispatch
    /// [`Heartbeat`]: Self::Heartbeat
    /// [`HeartbeatAck`]: Self::HeartbeatAck
    /// [`Hello`]: Self::Hello
    /// [`InvalidSession`]: Self::InvalidSession
    /// [`Reconnect`]: Self::Reconnect
    pub const fn is_received(self) -> bool {
        matches!(
            self,
            Self::Dispatch
                | Self::Heartbeat
                | Self::HeartbeatAck
                | Self::Hello
                | Self::InvalidSession
                | Self::Reconnect
        )
    }

    /// Whether the opcode is sent by the client.
    ///
    /// This includes the following opcodes:
    ///
    /// - [`Heartbeat`]
    /// - [`Identify`]
    /// - [`PresenceUpdate`]
    /// - [`Resume`]
    /// - [`RequestGuildMembers`]
    /// - [`VoiceStateUpdate`]
    ///
    /// [`Heartbeat`]: Self::Heartbeat
    /// [`Identify`]: Self::Identify
    /// [`PresenceUpdate`]: Self::PresenceUpdate
    /// [`Resume`]: Self::Resume
    /// [`RequestGuildMembers`]: Self::RequestGuildMembers
    /// [`VoiceStateUpdate`]: Self::VoiceStateUpdate
    pub const fn is_sent(self) -> bool {
        matches!(
            self,
            Self::Heartbeat
                | Self::Identify
                | Self::PresenceUpdate
                | Self::Resume
                | Self::RequestGuildMembers
                | Self::VoiceStateUpdate
        )
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

    const MAP: &[(OpCode, u8, bool, bool)] = &[
        (OpCode::Dispatch, 0, true, false),
        (OpCode::Heartbeat, 1, true, true),
        (OpCode::Identify, 2, false, true),
        (OpCode::PresenceUpdate, 3, false, true),
        (OpCode::VoiceStateUpdate, 4, false, true),
        (OpCode::Resume, 6, false, true),
        (OpCode::Reconnect, 7, true, false),
        (OpCode::RequestGuildMembers, 8, false, true),
        (OpCode::InvalidSession, 9, true, false),
        (OpCode::Hello, 10, true, false),
        (OpCode::HeartbeatAck, 11, true, false),
    ];

    #[test]
    fn variants() {
        for (value, integer, received, sent) in MAP {
            serde_test::assert_tokens(value, &[Token::U8(*integer)]);
            assert_eq!(value.is_received(), *received);
            assert_eq!(value.is_sent(), *sent);
        }
    }
}
