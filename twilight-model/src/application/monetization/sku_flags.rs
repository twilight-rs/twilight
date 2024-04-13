use bitflags::bitflags;
use serde::{
    de::{Deserialize, Deserializer},
    ser::{Serialize, Serializer},
};

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct SKUFlags: u64 {
      /// SKU is available for purchase.
      const AVAILABLE = 1 << 2;
      /// A subscription purchased by a user and applied to a single server.
      /// Everyone in that server gets your premium benefits.
      const GUILD_SUBSCRIPTION = 1 << 7;
      /// A subscription purchased by a user for themselves. They get access
      /// to your premium benefits in every server.
      const USER_SUBSCRIPTION = 1 << 8;
    }
}

impl<'de> Deserialize<'de> for SKUFlags {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        Ok(Self::from_bits_truncate(u64::deserialize(deserializer)?))
    }
}

impl Serialize for SKUFlags {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_u64(self.bits())
    }
}

#[cfg(test)]
mod tests {
    use super::SKUFlags;
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
        SKUFlags: Binary,
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
        Extend<SKUFlags>,
        FromIterator<SKUFlags>,
        Hash,
        LowerHex,
        Not,
        Octal,
        Ord,
        PartialEq,
        PartialOrd,
        Send,
        Serialize,
        Sub,
        SubAssign,
        Sync,
        UpperHex
    );

    const_assert_eq!(SKUFlags::AVAILABLE.bits(), 1 << 2);
    const_assert_eq!(SKUFlags::GUILD_SUBSCRIPTION.bits(), 1 << 7);
    const_assert_eq!(SKUFlags::USER_SUBSCRIPTION.bits(), 1 << 8);

    #[test]
    fn serde() {
        serde_test::assert_tokens(
            &SKUFlags::AVAILABLE,
            &[Token::U64(SKUFlags::AVAILABLE.bits())],
        );

        serde_test::assert_tokens(
            &SKUFlags::GUILD_SUBSCRIPTION,
            &[Token::U64(SKUFlags::GUILD_SUBSCRIPTION.bits())],
        );

        serde_test::assert_tokens(
            &SKUFlags::USER_SUBSCRIPTION,
            &[Token::U64(SKUFlags::USER_SUBSCRIPTION.bits())],
        );

        serde_test::assert_de_tokens(&SKUFlags::empty(), &[Token::U64(1 << 63)]);
    }
}
