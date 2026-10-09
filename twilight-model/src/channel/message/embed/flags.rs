use bitflags::bitflags;
use serde::{
    de::{Deserialize, Deserializer},
    ser::{Serialize, Serializer},
};

bitflags! {
    /// Flags of an [`Embed`][super::Embed].
    #[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
    pub struct EmbedFlags: u64 {
        /// This embed is a fallback for a reply to an activity card.
        const IS_CONTENT_INVENTORY_ENTRY = 1 << 5;
    }
}

impl<'de> Deserialize<'de> for EmbedFlags {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        Ok(Self::from_bits_retain(u64::deserialize(deserializer)?))
    }
}

impl Serialize for EmbedFlags {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_u64(self.bits())
    }
}

#[cfg(test)]
mod tests {
    use super::EmbedFlags;
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
        EmbedFlags: Binary,
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
        Extend<EmbedFlags>,
        FromIterator<EmbedFlags>,
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
    const_assert_eq!(EmbedFlags::IS_CONTENT_INVENTORY_ENTRY.bits(), 1 << 5);

    #[test]
    fn serde() {
        serde_test::assert_tokens(
            &EmbedFlags::IS_CONTENT_INVENTORY_ENTRY,
            &[Token::U64(EmbedFlags::IS_CONTENT_INVENTORY_ENTRY.bits())],
        );
    }
}
