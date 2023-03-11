use serde_repr::{Deserialize_repr, Serialize_repr};
use std::{
    error::Error,
    fmt::{Display, Formatter, Result as FmtResult},
};

/// Gateway close event codes.
///
/// See [Discord Docs/Gateway Close Event Codes] for more information.
///
/// [Discord Docs/Gateway Close Event Codes]: https://discord.com/developers/docs/topics/opcodes-and-status-codes#gateway-gateway-close-event-codes
#[derive(Clone, Copy, Debug, Deserialize_repr, Eq, Hash, PartialEq, Serialize_repr)]
#[non_exhaustive]
#[repr(u16)]
pub enum CloseCode {
    /// An unknown error occurred.
    UnknownError = 4000,
    /// An invalid opcode or payload for an opcode was sent.
    UnknownOpcode = 4001,
    /// An invalid payload was sent.
    DecodeError = 4002,
    /// A payload was sent prior to identifying.
    NotAuthenticated = 4003,
    /// An invalid token was sent when identifying.
    AuthenticationFailed = 4004,
    /// Multiple identify payloads were sent.
    AlreadyAuthenticated = 4005,
    /// An invalid sequence was sent for resuming.
    InvalidSequence = 4007,
    /// Too many payloads were sent in a certain amount of time.
    RateLimited = 4008,
    /// The session timed out.
    SessionTimedOut = 4009,
    /// An invalid shard was sent when identifying.
    InvalidShard = 4010,
    /// Sharding is required because there are too many guilds.
    ShardingRequired = 4011,
    /// An invalid version for the gateway was sent.
    InvalidApiVersion = 4012,
    /// An invalid intent was sent.
    InvalidIntents = 4013,
    /// A disallowed intent was sent, may need allowlisting.
    DisallowedIntents = 4014,
}

impl CloseCode {
    /// Whether the close code is one that allows reconnection of a shard.
    ///
    /// Some close codes are considered *fatal*, meaning that using the same
    /// gateway shard configuration would error. For example, the
    /// [`AuthenticationFailed`] close code occurs when the provided Discord bot
    /// token is invalid, and so attempting to reconnect with the same token
    /// would fail. On the other hand, a close code such as [`RateLimited`]
    /// occurs when too many gateway commands are sent in a short time, and so
    /// creating a new connection would succeed.
    ///
    /// Refer to [Discord Docs/Gateway Close Event Codes][1] for more
    /// information.
    ///
    /// # Reconnectable close codes
    ///
    /// - [`UnknownError`]
    /// - [`DecodeError`]
    /// - [`NotAuthenticated`]
    /// - [`AlreadyAuthenticated`]
    /// - [`InvalidSequence`]
    /// - [`RateLimited`]
    /// - [`SessionTimedOut`]
    ///
    /// # Fatal close codes
    ///
    /// - [`AuthenticationFailed`]
    /// - [`InvalidShard`]
    /// - [`ShardingRequired`]
    /// - [`InvalidApiVersion`]
    /// - [`InvalidIntents`]
    /// - [`DisallowedIntents`]
    ///
    /// [`AlreadyAuthenticated`]: Self::AlreadyAuthenticated
    /// [`AuthenticationFailed`]: Self::AuthenticationFailed
    /// [`DecodeError`]: Self::DecodeError
    /// [`DisallowedIntents`]: Self::DisallowedIntents
    /// [`InvalidApiVersion`]: Self::InvalidApiVersion
    /// [`InvalidIntents`]: Self::InvalidIntents
    /// [`InvalidSequence`]: Self::InvalidSequence
    /// [`InvalidShard`]: Self::InvalidShard
    /// [`NotAuthenticated`]: Self::NotAuthenticated
    /// [`RateLimited`]: Self::RateLimited
    /// [`SessionTimedOut`]: Self::SessionTimedOut
    /// [`ShardingRequired`]: Self::ShardingRequired
    /// [`UnknownError`]: Self::UnknownError
    /// [1]: https://discord.com/developers/docs/topics/opcodes-and-status-codes#gateway-gateway-close-event-codes
    pub const fn can_reconnect(self) -> bool {
        matches!(
            self,
            Self::UnknownError
                | Self::UnknownOpcode
                | Self::DecodeError
                | Self::NotAuthenticated
                | Self::AlreadyAuthenticated
                | Self::InvalidSequence
                | Self::RateLimited
                | Self::SessionTimedOut
        )
    }
}

impl Display for CloseCode {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        f.write_str(match self {
            CloseCode::UnknownError => "Unknown Error",
            CloseCode::UnknownOpcode => "Unknown Opcode",
            CloseCode::DecodeError => "Decode Error",
            CloseCode::NotAuthenticated => "Not Authenticated",
            CloseCode::AuthenticationFailed => "Authentication Failed",
            CloseCode::AlreadyAuthenticated => "Already Authenticated",
            CloseCode::InvalidSequence => "Invalid Sequence",
            CloseCode::RateLimited => "Rate Limited",
            CloseCode::SessionTimedOut => "Session Timed Out",
            CloseCode::InvalidShard => "Invalid Shard",
            CloseCode::ShardingRequired => "Sharding Required",
            CloseCode::InvalidApiVersion => "Invalid Api Version",
            CloseCode::InvalidIntents => "Invalid Intents",
            CloseCode::DisallowedIntents => "Disallowed Intents",
        })
    }
}

#[derive(Debug, Eq, PartialEq)]
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
            4000 => CloseCode::UnknownError,
            4001 => CloseCode::UnknownOpcode,
            4002 => CloseCode::DecodeError,
            4003 => CloseCode::NotAuthenticated,
            4004 => CloseCode::AuthenticationFailed,
            4005 => CloseCode::AlreadyAuthenticated,
            4007 => CloseCode::InvalidSequence,
            4008 => CloseCode::RateLimited,
            4009 => CloseCode::SessionTimedOut,
            4010 => CloseCode::InvalidShard,
            4011 => CloseCode::ShardingRequired,
            4012 => CloseCode::InvalidApiVersion,
            4013 => CloseCode::InvalidIntents,
            4014 => CloseCode::DisallowedIntents,
            _ => return Err(CloseCodeConversionError::new(value)),
        };

        Ok(close_code)
    }
}

#[cfg(test)]
mod tests {
    use super::{CloseCode, CloseCodeConversionError};
    use serde::{Deserialize, Serialize};
    use serde_test::Token;
    use static_assertions::assert_impl_all;
    use std::{fmt::Debug, hash::Hash};

    assert_impl_all!(
        CloseCode: Clone,
        Copy,
        Debug,
        Deserialize<'static>,
        Eq,
        Hash,
        PartialEq,
        Send,
        Serialize,
        Sync
    );
    assert_impl_all!(CloseCodeConversionError: Debug, Eq, PartialEq, Send, Sync);

    const MAP: &[(CloseCode, u16, bool)] = &[
        (CloseCode::UnknownError, 4000, true),
        (CloseCode::UnknownOpcode, 4001, true),
        (CloseCode::DecodeError, 4002, true),
        (CloseCode::NotAuthenticated, 4003, true),
        (CloseCode::AuthenticationFailed, 4004, false),
        (CloseCode::AlreadyAuthenticated, 4005, true),
        (CloseCode::InvalidSequence, 4007, true),
        (CloseCode::RateLimited, 4008, true),
        (CloseCode::SessionTimedOut, 4009, true),
        (CloseCode::InvalidShard, 4010, false),
        (CloseCode::ShardingRequired, 4011, false),
        (CloseCode::InvalidApiVersion, 4012, false),
        (CloseCode::InvalidIntents, 4013, false),
        (CloseCode::DisallowedIntents, 4014, false),
    ];

    #[test]
    fn variants() {
        for (kind, num, can_reconnect) in MAP {
            serde_test::assert_tokens(kind, &[Token::U16(*num)]);
            assert_eq!(*kind, CloseCode::try_from(*num).unwrap());
            assert_eq!(*num, *kind as u16);
            assert!(kind.can_reconnect() == *can_reconnect)
        }
    }

    #[test]
    fn try_from() {
        assert!(
            matches!(CloseCode::try_from(5000), Err(CloseCodeConversionError { code }) if code == 5000)
        );
    }
}
