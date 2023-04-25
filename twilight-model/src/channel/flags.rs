use bitflags::bitflags;
use serde::{
    de::{Deserialize, Deserializer},
    ser::{Serialize, Serializer},
};

bitflags! {
    #[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
    pub struct ChannelFlags: u64 {
        /// Channel is pinned in a forum.
        const PINNED = 1 << 1;
        /// New threads in a forum channel require a tag.
        const REQUIRE_TAG = 1 << 4;
    }
}

impl<'de> Deserialize<'de> for ChannelFlags {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        Ok(Self::from_bits_truncate(u64::deserialize(deserializer)?))
    }
}

impl Serialize for ChannelFlags {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_u64(self.bits())
    }
}

#[cfg(test)]
mod tests {
    use super::ChannelFlags;
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
        ChannelFlags: Binary,
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
        Extend<ChannelFlags>,
        FromIterator<ChannelFlags>,
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
    const_assert_eq!(ChannelFlags::PINNED.bits(), 1 << 1);
    const_assert_eq!(ChannelFlags::REQUIRE_TAG.bits(), 1 << 4);

    #[test]
    fn serde() {
        serde_test::assert_tokens(
            &ChannelFlags::PINNED,
            &[Token::U64(ChannelFlags::PINNED.bits())],
        );
        // Deserialization truncates unknown bits.
        serde_test::assert_de_tokens(&ChannelFlags::empty(), &[Token::U64(1 << 63)]);
    }
}
