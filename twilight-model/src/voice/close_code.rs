use serde_repr::{Deserialize_repr, Serialize_repr};

/// Voice gateway close event codes.
#[derive(Clone, Copy, Debug, Deserialize_repr, Eq, Hash, PartialEq, Serialize_repr)]
#[non_exhaustive]
#[repr(u16)]
#[cfg_attr(
    feature = "rkyv",
    derive(rkyv::Archive, rkyv::Deserialize, rkyv::Serialize),
    archive(as = "Self")
)]
pub enum CloseCode {
    /// An invalid opcode was sent.
    UnknownOpcode = 4001,
    /// An invalid payload was sent.
    DecodeError = 4002,
    /// A payload was sent prior to identifying.
    NotAuthenticated = 4003,
    /// An invalid token was sent when identifying.
    AuthenticationFailed = 4004,
    /// Multiple identify payloads were sent.
    AlreadyAuthenticated = 4005,
    /// The session was invalidated.
    SessionNoLongerValid = 4006,
    /// The session timed out.
    SessionTimedOut = 4009,
    /// The specified voice server was not found.
    ServerNotFound = 4011,
    /// An unknown protocol was sent.
    UnknownProtocol = 4012,
    /// Disconnected from the voice channel.
    Disconnected = 4014,
    /// The voice server crashed.
    VoiceServerCrashed = 4015,
    /// The encryption could not be recognized.
    UnknownEncryptionMode = 4016,
}

#[cfg(test)]
mod tests {
    use super::CloseCode;
    use serde_test::Token;

    #[test]
    fn variants() {
        serde_test::assert_tokens(&CloseCode::UnknownOpcode, &[Token::U16(4001)]);
        serde_test::assert_tokens(&CloseCode::DecodeError, &[Token::U16(4002)]);
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
