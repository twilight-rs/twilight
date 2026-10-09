use bitflags::bitflags;
use serde::{Deserialize, Serialize, de::Deserializer, ser::Serializer};

bitflags! {
    /// Flags for a media element of an [`Embed`], such as [`EmbedImage`] or
    /// [`EmbedVideo`].
    ///
    /// [`Embed`]: super::Embed
    /// [`EmbedImage`]: super::EmbedImage
    /// [`EmbedVideo`]: super::EmbedVideo
    #[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
    pub struct EmbedMediaFlags: u64 {
        /// This image is animated.
        const IS_ANIMATED = 1 << 5;
    }
}

impl<'de> Deserialize<'de> for EmbedMediaFlags {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        Ok(Self::from_bits_truncate(u64::deserialize(deserializer)?))
    }
}

impl Serialize for EmbedMediaFlags {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_u64(self.bits())
    }
}

#[cfg(test)]
mod tests {
    use super::EmbedMediaFlags;
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
        EmbedMediaFlags: Binary,
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
        Extend<EmbedMediaFlags>,
        FromIterator<EmbedMediaFlags>,
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
    const_assert_eq!(EmbedMediaFlags::IS_ANIMATED.bits(), 1 << 5);

    #[test]
    fn serde() {
        serde_test::assert_tokens(
            &EmbedMediaFlags::IS_ANIMATED,
            &[Token::U64(EmbedMediaFlags::IS_ANIMATED.bits())],
        );
    }
}
