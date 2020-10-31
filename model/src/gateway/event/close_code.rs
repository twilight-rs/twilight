use serde_repr::{Deserialize_repr, Serialize_repr};

/// Gateway close event codes.
#[derive(
    Clone, Copy, Debug, Deserialize_repr, Eq, Hash, PartialEq, PartialOrd, Ord, Serialize_repr,
)]
#[repr(u16)]
pub enum CloseCode {
    UnknownError = 4000,
    UnknownOpcode = 4001,
    DecodeError = 4002,
    NotAuthenticated = 4003,
    AuthenticationFailed = 4004,
    AlreadyAuthenticated = 4005,
    InvalidSequence = 4007,
    RateLimited = 4008,
    SessionTimedOut = 4009,
    InvalidShard = 4010,
    ShardingRequired = 4011,
    InvalidApiVersion = 4012,
    InvalidIntents = 4013,
    DisallowedIntents = 4014,
}

#[cfg(test)]
mod tests {
    use super::CloseCode;
    use serde_test::Token;

    #[test]
    fn test_variants() {
        serde_test::assert_tokens(&CloseCode::UnknownError, &[Token::U16(4000)]);
        serde_test::assert_tokens(&CloseCode::UnknownOpcode, &[Token::U16(4001)]);
        serde_test::assert_tokens(&CloseCode::DecodeError, &[Token::U16(4002)]);
        serde_test::assert_tokens(&CloseCode::NotAuthenticated, &[Token::U16(4003)]);
        serde_test::assert_tokens(&CloseCode::AuthenticationFailed, &[Token::U16(4004)]);
        serde_test::assert_tokens(&CloseCode::AlreadyAuthenticated, &[Token::U16(4005)]);
        serde_test::assert_tokens(&CloseCode::InvalidSequence, &[Token::U16(4007)]);
        serde_test::assert_tokens(&CloseCode::RateLimited, &[Token::U16(4008)]);
        serde_test::assert_tokens(&CloseCode::SessionTimedOut, &[Token::U16(4009)]);
        serde_test::assert_tokens(&CloseCode::InvalidShard, &[Token::U16(4010)]);
        serde_test::assert_tokens(&CloseCode::ShardingRequired, &[Token::U16(4011)]);
        serde_test::assert_tokens(&CloseCode::InvalidApiVersion, &[Token::U16(4012)]);
        serde_test::assert_tokens(&CloseCode::InvalidIntents, &[Token::U16(4013)]);
        serde_test::assert_tokens(&CloseCode::DisallowedIntents, &[Token::U16(4014)]);
    }
}
