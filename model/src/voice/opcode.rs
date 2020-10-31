use serde_repr::{Deserialize_repr, Serialize_repr};

// Voice gateway opcodes.
#[derive(
    Clone, Copy, Debug, Deserialize_repr, Eq, Hash, PartialEq, PartialOrd, Ord, Serialize_repr,
)]
#[repr(u8)]
pub enum OpCode {
    Identify = 0,
    SelectProtocol = 1,
    Ready = 2,
    Heartbeat = 3,
    SessionDescription = 4,
    Speaking = 5,
    HeartbeatAck = 6,
    Resume = 7,
    Hello = 8,
    Resumed = 9,
    ClientDisconnect = 13,
}

#[cfg(test)]
mod tests {
    use super::OpCode;
    use serde_test::Token;

    #[test]
    fn test_variants() {
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
