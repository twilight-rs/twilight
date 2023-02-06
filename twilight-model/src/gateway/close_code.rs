use serde::{Deserialize, Serialize};

/// Gateway close event codes.
///
/// See [Discord Docs/Gateway Close Event Codes] for more information.
///
/// [Discord Docs/Gateway Close Event Codes]: https://discord.com/developers/docs/topics/opcodes-and-status-codes#gateway-gateway-close-event-codes
#[derive(Clone, Copy, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct CloseCode(u16);

impl CloseCode {
    /// An unknown error occurred.
    pub const UNKNOWN_ERROR: Self = Self::new(4000);

    /// An invalid opcode or payload for an opcode was sent.
    pub const UNKNOWN_OPCODE: Self = Self::new(4001);

    /// An invalid payload was sent.
    pub const DECODE_ERROR: Self = Self::new(4002);

    /// A payload was sent prior to identifying.
    pub const NOT_AUTHENTICATED: Self = Self::new(4003);

    /// An invalid token was sent when identifying.
    pub const AUTHENTICATION_FAILED: Self = Self::new(4004);

    /// Multiple identify payloads were sent.
    pub const ALREADY_AUTHENTICATED: Self = Self::new(4005);

    /// An invalid sequence was sent for resuming.
    pub const INVALID_SEQUENCE: Self = Self::new(4007);

    /// Too many payloads were sent in a certain amount of time.
    pub const RATE_LIMITED: Self = Self::new(4008);

    /// The session timed out.
    pub const SESSION_TIMED_OUT: Self = Self::new(4009);

    /// An invalid shard was sent when identifying.
    pub const INVALID_SHARD: Self = Self::new(4010);

    /// Sharding is required because there are too many guilds.
    pub const SHARDING_REQUIRED: Self = Self::new(4011);

    /// An invalid version for the gateway was sent.
    pub const INVALID_API_VERSION: Self = Self::new(4012);

    /// An invalid intent was sent.
    pub const INVALID_INTENTS: Self = Self::new(4013);

    /// A disallowed intent was sent, may need allowlisting.
    pub const DISALLOWED_INTENTS: Self = Self::new(4014);

    /// Name of the associated constant.
    ///
    /// Returns `None` if the value doesn't have a defined constant.
    pub const fn name(self) -> Option<&'static str> {
        Some(match self {
            Self::UNKNOWN_ERROR => "UNKNOWN_ERROR",
            Self::UNKNOWN_OPCODE => "UNKNOWN_OPCODE",
            Self::DECODE_ERROR => "DECODE_ERROR",
            Self::NOT_AUTHENTICATED => "NOT_AUTHENTICATED",
            Self::AUTHENTICATION_FAILED => "AUTHENTICATION_FAILED",
            Self::ALREADY_AUTHENTICATED => "ALREADY_AUTHENTICATED",
            Self::INVALID_SEQUENCE => "INVALID_SEQUENCE",
            Self::RATE_LIMITED => "RATE_LIMITED",
            Self::SESSION_TIMED_OUT => "SESSION_TIMED_OUT",
            Self::INVALID_SHARD => "INVALID_SHARD",
            Self::SHARDING_REQUIRED => "SHARDING_REQUIRED",
            Self::INVALID_API_VERSION => "INVALID_API_VERSION",
            Self::INVALID_INTENTS => "INVALID_INTENTS",
            Self::DISALLOWED_INTENTS => "DISALLOWED_INTENTS",
            _ => return None,
        })
    }

    /// Whether the close code is one that allows reconnection of a shard.
    ///
    /// Refer to the type-level documentation for Discord's table on close codes
    /// that can be reconnected.
    pub const fn can_reconnect(self) -> bool {
        !matches!(
            self,
            Self::AUTHENTICATION_FAILED
                | Self::INVALID_SHARD
                | Self::SHARDING_REQUIRED
                | Self::INVALID_API_VERSION
                | Self::INVALID_INTENTS
                | Self::DISALLOWED_INTENTS
        )
    }
}

impl_typed!(CloseCode, u16);

#[cfg(test)]
mod tests {
    use super::CloseCode;
    use serde_test::Token;

    pub const MAP: &[(CloseCode, u16)] = &[
        (CloseCode::UNKNOWN_ERROR, 4000),
        (CloseCode::UNKNOWN_OPCODE, 4001),
        (CloseCode::DECODE_ERROR, 4002),
        (CloseCode::NOT_AUTHENTICATED, 4003),
        (CloseCode::AUTHENTICATION_FAILED, 4004),
        (CloseCode::ALREADY_AUTHENTICATED, 4005),
        (CloseCode::INVALID_SEQUENCE, 4007),
        (CloseCode::RATE_LIMITED, 4008),
        (CloseCode::SESSION_TIMED_OUT, 4009),
        (CloseCode::INVALID_SHARD, 4010),
        (CloseCode::SHARDING_REQUIRED, 4011),
        (CloseCode::INVALID_API_VERSION, 4012),
        (CloseCode::INVALID_INTENTS, 4013),
        (CloseCode::DISALLOWED_INTENTS, 4014),
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
