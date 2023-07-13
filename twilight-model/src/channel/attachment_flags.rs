use bitflags::bitflags;
use serde::{
    de::{Deserialize, Deserializer},
    ser::{Serialize, Serializer},
};

bitflags! {
  pub struct AttachmentFlags: u64 {
    /// This attachment has been edited using the remix feature on mobile
    const IS_REMIX = 1 << 2;
  }
}

impl<'de> Deserialize<'de> for AttachmentFlags {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        Ok(Self::from_bits_truncate(u64::deserialize(deserializer)?))
    }
}

impl Serialize for AttachmentFlags {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_u64(self.bits())
    }
}

#[cfg(test)]
mod tests {
    use super::AttachmentFlags;
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
        AttachmentFlags: Binary,
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
        Extend<AttachmentFlags>,
        FromIterator<AttachmentFlags>,
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

    const_assert_eq!(AttachmentFlags::IS_REMIX.bits(), 4);

    #[test]
    fn serde() {
        serde_test::assert_tokens(
            &AttachmentFlags::IS_REMIX,
            &[Token::U64(AttachmentFlags::IS_REMIX.bits())],
        );

        // Deserialization truncates unknown bits.
        serde_test::assert_de_tokens(&AttachmentFlags::empty(), &[Token::U64(1 << 63)]);
    }
}
