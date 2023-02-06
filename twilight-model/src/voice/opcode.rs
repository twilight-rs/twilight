use serde::{Deserialize, Serialize};

/// Voice gateway opcodes.
#[derive(Clone, Copy, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct OpCode(u8);

impl OpCode {
    /// Start a voice websocket connection.
    pub const IDENTIFY: Self = Self::new(0);

    /// Select the protocol to use.
    pub const SELECT_PROTOCOL: Self = Self::new(1);

    /// Received to indicate completion of handshake.
    pub const READY: Self = Self::new(2);

    /// Fired periodically to keep connection alive.
    pub const HEARTBEAT: Self = Self::new(3);

    /// Received to indicate session description.
    pub const SESSION_DESCRIPTION: Self = Self::new(4);

    /// Sent and received to indicate speaking status.
    pub const SPEAKING: Self = Self::new(5);

    /// Received in response to a heartbeat.
    pub const HEARTBEAT_ACK: Self = Self::new(6);

    /// Resume a previously disconnected session.
    pub const RESUME: Self = Self::new(7);

    /// Received after connecting, contains heartbeat interval.
    pub const HELLO: Self = Self::new(8);

    /// Received to indicate a successful resume.
    pub const RESUMED: Self = Self::new(9);

    /// Received to indicate someone was disconnected.
    pub const CLIENT_DISCONNECT: Self = Self::new(13);

    /// Name of the associated constant.
    ///
    /// Returns `None` if the value doesn't have a defined constant.
    pub const fn name(self) -> Option<&'static str> {
        Some(match self {
            Self::CLIENT_DISCONNECT => "CLIENT_DISCONNECT",
            Self::HEARTBEAT => "HEARTBEAT",
            Self::HEARTBEAT_ACK => "HEARTBEAT_ACK",
            Self::HELLO => "HELLO",
            Self::IDENTIFY => "IDENTIFY",
            Self::READY => "READY",
            Self::RESUME => "RESUME",
            Self::RESUMED => "RESUMED",
            Self::SELECT_PROTOCOL => "SELECT_PROTOCOL",
            Self::SESSION_DESCRIPTION => "SESSION_DESCRIPTION",
            Self::SPEAKING => "SPEAKING",
            _ => return None,
        })
    }
}

impl_typed!(OpCode, u8);

#[cfg(test)]
mod tests {
    use super::OpCode;
    use serde_test::Token;

    const MAP: &[(OpCode, u8)] = &[
        (OpCode::SELECT_PROTOCOL, 1),
        (OpCode::READY, 2),
        (OpCode::HEARTBEAT, 3),
        (OpCode::SESSION_DESCRIPTION, 4),
        (OpCode::SPEAKING, 5),
        (OpCode::HEARTBEAT_ACK, 6),
        (OpCode::RESUME, 7),
        (OpCode::HELLO, 8),
        (OpCode::RESUMED, 9),
        (OpCode::CLIENT_DISCONNECT, 13),
    ];

    #[test]
    fn variants() {
        for (kind, num) in MAP {
            serde_test::assert_tokens(
                kind,
                &[Token::NewtypeStruct { name: "OpCode" }, Token::U8(*num)],
            );
            assert_eq!(*kind, OpCode::from(*num));
            assert_eq!(*num, kind.get());
        }
    }
}
