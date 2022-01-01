use serde_repr::{Deserialize_repr, Serialize_repr};
use std::{
    error::Error,
    fmt::{Display, Formatter, Result as FmtResult},
};

/// Voice gateway close event codes.
#[derive(
    Clone, Copy, Debug, Deserialize_repr, Eq, Hash, PartialEq, PartialOrd, Ord, Serialize_repr,
)]
#[non_exhaustive]
#[repr(u16)]
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

#[derive(Debug, PartialEq)]
pub struct CloseCodeConversionError {
    code: u16,
}

impl CloseCodeConversionError {
    const fn new(code: u16) -> Self {
        Self { code }
    }

    pub const fn code(&self) -> u16 {
        self.code
    }
}

impl Display for CloseCodeConversionError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        Display::fmt(&self.code, f)?;

        f.write_str(" isn't a valid close code")
    }
}

impl Error for CloseCodeConversionError {}

impl TryFrom<u16> for CloseCode {
    type Error = CloseCodeConversionError;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        let close_code = match value {
            4001 => CloseCode::UnknownOpcode,
            4002 => CloseCode::DecodeError,
            4003 => CloseCode::NotAuthenticated,
            4004 => CloseCode::AuthenticationFailed,
            4005 => CloseCode::AlreadyAuthenticated,
            4006 => CloseCode::SessionNoLongerValid,
            4009 => CloseCode::SessionTimedOut,
            4011 => CloseCode::ServerNotFound,
            4012 => CloseCode::UnknownProtocol,
            4014 => CloseCode::Disconnected,
            4015 => CloseCode::VoiceServerCrashed,
            4016 => CloseCode::UnknownEncryptionMode,
            _ => return Err(CloseCodeConversionError::new(value)),
        };

        Ok(close_code)
    }
}

#[cfg(test)]
mod tests {
    use super::CloseCode;
    use serde_test::Token;

    #[test]
    fn test_variants() {
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

    #[test]
    fn test_conversion() {
        assert_eq!(CloseCode::try_from(4001).unwrap(), CloseCode::UnknownOpcode);
        assert_eq!(CloseCode::try_from(4002).unwrap(), CloseCode::DecodeError);
        assert_eq!(
            CloseCode::try_from(4003).unwrap(),
            CloseCode::NotAuthenticated
        );
        assert_eq!(
            CloseCode::try_from(4004).unwrap(),
            CloseCode::AuthenticationFailed
        );
        assert_eq!(
            CloseCode::try_from(4005).unwrap(),
            CloseCode::AlreadyAuthenticated
        );
        assert_eq!(
            CloseCode::try_from(4009).unwrap(),
            CloseCode::SessionTimedOut
        );
        assert_eq!(
            CloseCode::try_from(4011).unwrap(),
            CloseCode::ServerNotFound
        );
        assert_eq!(
            CloseCode::try_from(4012).unwrap(),
            CloseCode::UnknownProtocol
        );
        assert_eq!(CloseCode::try_from(4014).unwrap(), CloseCode::Disconnected);
        assert_eq!(
            CloseCode::try_from(4015).unwrap(),
            CloseCode::VoiceServerCrashed
        );
        assert_eq!(
            CloseCode::try_from(4016).unwrap(),
            CloseCode::UnknownEncryptionMode
        );
        assert!(CloseCode::try_from(5000).is_err());
    }
}
