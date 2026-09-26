use bitflags::bitflags;
use serde::{
    de::{Deserialize, Deserializer},
    ser::{Serialize, Serializer},
};

bitflags! {
    /// Gateway capabilities.
    ///
    /// Developers may specify capabilities when connecting to the gateway.
    /// Capabilities allow applications to opt-in to gateway behavior. To
    /// specify multiple capabilities, create a union using the `|` operator. See
    /// [Discord Docs/Capabilities].
    ///
    /// [Discord Docs/Capabilities]: https://discord.com/developers/docs/topics/gateway#identify-gateway-capabilities
    #[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
    pub struct Capabilities: u64 {
        /// Opts the client into receiving obfuscated channel metadata over the
        /// Gateway for channels it can't view.
        ///
        /// Discord plans to fully roll out Channel Obfuscation for all users on
        /// November 16th, 2026, at which point specifying this capability would
        /// have no effect.
        ///
        /// See [Discord Docs/Channel Obfuscation].
        ///
        /// [Discord Docs/Channel Obfuscation]: https://docs.discord.com/developers/resources/channel#channel-object-obfuscated-channels
        const CHANNEL_OBFUSCATION = 1 << 15;
    }
}

impl<'de> Deserialize<'de> for Capabilities {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        Ok(Self::from_bits_truncate(u64::deserialize(deserializer)?))
    }
}

impl Serialize for Capabilities {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_u64(self.bits())
    }
}

#[cfg(test)]
mod tests {
    use super::Capabilities;
    use serde::{Deserialize, Serialize};
    use serde_test::Token;
    use static_assertions::{assert_impl_all, const_assert_eq};
    use std::{
        fmt::{Binary, Debug, LowerHex, Octal, UpperHex},
        hash::Hash,
        ops::{
            BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Not, Sub, SubAssign,
        },
    };

    assert_impl_all!(
        Capabilities: Binary,
        BitAnd,
        BitAndAssign,
        BitOr,
        BitOrAssign,
        BitXor,
        BitXorAssign,
        Clone,
        Copy,
        Debug,
        Deserialize<'static>,
        Eq,
        Extend<Capabilities>,
        FromIterator<Capabilities>,
        Hash,
        LowerHex,
        Not,
        Octal,
        PartialEq,
        Send,
        Serialize,
        Sub,
        SubAssign,
        Sync,
        UpperHex
    );
    const_assert_eq!(Capabilities::CHANNEL_OBFUSCATION.bits(), 1 << 15);

    #[test]
    fn serde() {
        serde_test::assert_tokens(
            &Capabilities::CHANNEL_OBFUSCATION,
            &[Token::U64(Capabilities::CHANNEL_OBFUSCATION.bits())],
        );
        // Deserialization truncates unknown bits.
        serde_test::assert_de_tokens(&Capabilities::empty(), &[Token::U64(1 << 63)]);
    }
}
