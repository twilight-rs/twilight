use serde_repr::{Deserialize_repr, Serialize_repr};

/// Voice gateway close event codes.
#[derive(
    Clone, Copy, Debug, Deserialize_repr, Eq, Hash, PartialEq, PartialOrd, Ord, Serialize_repr,
)]
#[repr(u16)]
pub enum CloseCode {
    UnknownOpcode = 4001,
    NotAuthenticated = 4003,
    AuthenticationFailed = 4004,
    AlreadyAuthenticated = 4005,
    SessionNoLongerValid = 4006,
    SessionTimedOut = 4009,
    ServerNotFound = 4011,
    UnknownProtocol = 4012,
    Disconnected = 4014,
    VoiceServerCrashed = 4015,
    UnknownEncryptionMode = 4016,
}

#[cfg(test)]
mod tests {
    use super::CloseCode;
    use serde_test::Token;

    #[test]
    fn test_variants() {
        serde_test::assert_tokens(&CloseCode::UnknownOpcode, &[Token::U16(4001)]);
        serde_test::assert_tokens(&CloseCode::NotAuthenticated, &[Token::U16(4003)]);
        serde_test::assert_tokens(&CloseCode::AuthenticationFailed, &[Token::U16(4004)]);
        serde_test::assert_tokens(&CloseCode::AlreadyAuthenticated, &[Token::U16(4005)]);
        serde_test::assert_tokens(&CloseCode::SessionTimedOut, &[Token::U16(4009)]);
        serde_test::assert_tokens(&CloseCode::ServerNotFound, &[Token::U16(4011)]);
        serde_test::assert_tokens(&CloseCode::UnknownProtocol, &[Token::U16(4012)]);
        serde_test::assert_tokens(&CloseCode::Disconnected, &[Token::U16(4014)]);
        serde_test::assert_tokens(&CloseCode::VoiceServerCrashed, &[Token::U16(4015)]);
        serde_test::assert_tokens(&CloseCode::UnknownEncryptionMode, &[Token::U16(4016)]);
    }
}
