use serde_repr::{Deserialize_repr, Serialize_repr};

#[derive(
    Clone, Copy, Debug, Deserialize_repr, Eq, Hash, PartialEq, PartialOrd, Ord, Serialize_repr,
)]
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

#[cfg(test)]
mod tests {
    use super::OpCode;
    use serde_test::Token;

    #[test]
    fn test_variants() {
        serde_test::assert_tokens(&OpCode::Event, &[Token::U8(0)]);
        serde_test::assert_tokens(&OpCode::Heartbeat, &[Token::U8(1)]);
        serde_test::assert_tokens(&OpCode::Identify, &[Token::U8(2)]);
        serde_test::assert_tokens(&OpCode::StatusUpdate, &[Token::U8(3)]);
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
