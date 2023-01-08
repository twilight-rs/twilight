use serde::{Deserialize, Serialize};

/// Voice gateway close event codes.
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct CloseCode(u16);

impl CloseCode {
    /// An invalid opcode was sent.
    pub const UNKNOWN_OPCODE: Self = Self::new(4001);

    /// An invalid payload was sent.
    pub const DECODE_ERROR: Self = Self::new(4002);

    /// A payload was sent prior to identifying.
    pub const NOT_AUTHENTICATED: Self = Self::new(4003);

    /// An invalid token was sent when identifying.
    pub const AUTHENTICATION_FAILED: Self = Self::new(4004);

    /// Multiple identify payloads were sent.
    pub const ALREADY_AUTHENTICATED: Self = Self::new(4005);

    /// The session was invalidated.
    pub const SESSION_NO_LONGER_VALID: Self = Self::new(4006);

    /// The session timed out.
    pub const SESSION_TIMED_OUT: Self = Self::new(4009);

    /// The specified voice server was not found.
    pub const SERVER_NOT_FOUND: Self = Self::new(4011);

    /// An unknown protocol was sent.
    pub const UNKNOWN_PROTOCOL: Self = Self::new(4012);

    /// Disconnected from the voice channel.
    pub const DISCONNECTED: Self = Self::new(4014);

    /// The voice server crashed.
    pub const VOICE_SERVER_CRASHED: Self = Self::new(4015);

    /// The encryption could not be recognized.
    pub const UNKNOWN_ENCRYPTION_MODE: Self = Self::new(4016);

    /// Create a new close code from a dynamic value.
    ///
    /// The provided value isn't validated. Known valid values are associated
    /// constants such as [`DISCONNECTED`][`Self::DISCONNECTED`].
    pub const fn new(close_code: u16) -> Self {
        Self(close_code)
    }

    /// Retrieve the value of the close code.
    ///
    /// # Examples
    ///
    /// ```
    /// use twilight_model::voice::CloseCode;
    ///
    /// assert_eq!(4002, CloseCode::DECODE_ERROR.get());
    /// ```
    pub const fn get(&self) -> u16 {
        self.0
    }
}

impl From<u16> for CloseCode {
    fn from(value: u16) -> Self {
        Self(value)
    }
}

impl From<CloseCode> for u16 {
    fn from(value: CloseCode) -> Self {
        value.get()
    }
}

#[cfg(test)]
mod tests {
    use super::CloseCode;
    use serde_test::Token;

    const MAP: &[(CloseCode, u16)] = &[
        (CloseCode::UNKNOWN_OPCODE, 4001),
        (CloseCode::DECODE_ERROR, 4002),
        (CloseCode::NOT_AUTHENTICATED, 4003),
        (CloseCode::AUTHENTICATION_FAILED, 4004),
        (CloseCode::ALREADY_AUTHENTICATED, 4005),
        (CloseCode::SESSION_NO_LONGER_VALID, 4006),
        (CloseCode::SESSION_TIMED_OUT, 4009),
        (CloseCode::SERVER_NOT_FOUND, 4011),
        (CloseCode::UNKNOWN_PROTOCOL, 4012),
        (CloseCode::DISCONNECTED, 4014),
        (CloseCode::VOICE_SERVER_CRASHED, 4015),
        (CloseCode::UNKNOWN_ENCRYPTION_MODE, 4016),
    ];

    #[test]
    fn variants() {
        for (kind, num) in MAP {
            serde_test::assert_tokens(
                kind,
                &[Token::NewtypeStruct { name: "CloseCode" }, Token::U16(*num)],
            );
            assert_eq!(*kind, CloseCode::from(*num));
            assert_eq!(*num, kind.get());
        }
    }
}
