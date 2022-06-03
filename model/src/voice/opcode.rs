use serde_repr::{Deserialize_repr, Serialize_repr};

// Voice gateway opcodes.
#[derive(
    Clone, Copy, Debug, Deserialize_repr, Eq, Hash, PartialEq, PartialOrd, Ord, Serialize_repr,
)]
#[non_exhaustive]
#[repr(u8)]
pub enum OpCode {
    /// Start a voice websocket connection.
    Identify = 0,
    /// Select the protocol to use.
    SelectProtocol = 1,
    /// Received to indicate completion of handshake.
    Ready = 2,
    /// Fired periodically to keep connection alive.
    Heartbeat = 3,
    /// Received to indicate session description.
    SessionDescription = 4,
    /// Sent and received to indicate speaking status.
    Speaking = 5,
    /// Received in response to a heartbeat.
    HeartbeatAck = 6,
    /// Resume a previously disconnected session.
    Resume = 7,
    /// Received after connecting, contains heartbeat interval.
    Hello = 8,
    /// Received to indicate a successful resume.
    Resumed = 9,
    /// Received to indicate someone was disconnected.
    ClientDisconnect = 13,
}

#[cfg(test)]
mod tests {
    use super::OpCode;
    use serde_test::Token;

    #[test]
    fn variants() {
        serde_test::assert_tokens(&OpCode::Identify, &[Token::U8(0)]);
        serde_test::assert_tokens(&OpCode::SelectProtocol, &[Token::U8(1)]);
        serde_test::assert_tokens(&OpCode::Ready, &[Token::U8(2)]);
        serde_test::assert_tokens(&OpCode::Heartbeat, &[Token::U8(3)]);
        serde_test::assert_tokens(&OpCode::SessionDescription, &[Token::U8(4)]);
        serde_test::assert_tokens(&OpCode::Speaking, &[Token::U8(5)]);
        serde_test::assert_tokens(&OpCode::HeartbeatAck, &[Token::U8(6)]);
        serde_test::assert_tokens(&OpCode::Resume, &[Token::U8(7)]);
        serde_test::assert_tokens(&OpCode::Hello, &[Token::U8(8)]);
        serde_test::assert_tokens(&OpCode::Resumed, &[Token::U8(9)]);
        serde_test::assert_tokens(&OpCode::ClientDisconnect, &[Token::U8(13)]);
    }
}
