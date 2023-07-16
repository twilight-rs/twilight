use bitflags::bitflags;
use serde::{
    de::{Deserialize, Deserializer},
    ser::{Serialize, Serializer},
};

bitflags! {
    /// The flags for a role represented as a bitfield.
    pub struct RoleFlags: u64 {
        /// A role that is available to select as an option within the guild onboarding flow.
        const IN_PROMPT = 1 << 0;
    }
}

impl<'de> Deserialize<'de> for RoleFlags {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        Ok(Self::from_bits_truncate(u64::deserialize(deserializer)?))
    }
}

impl Serialize for RoleFlags {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_u64(self.bits())
    }
}

#[cfg(test)]
mod tests {
    use super::RoleFlags;
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
        RoleFlags: Binary,
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
        Extend<RoleFlags>,
        FromIterator<RoleFlags>,
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
        UpperHex,
    );

    const_assert_eq!(1 << 0, RoleFlags::IN_PROMPT.bits());

    #[test]
    fn deserialize() {
        serde_test::assert_tokens(
            &RoleFlags::IN_PROMPT,
            &[Token::U64(RoleFlags::IN_PROMPT.bits())],
        );
    }

    #[test]
    fn serialize() {
        serde_test::assert_tokens(
            &RoleFlags::IN_PROMPT,
            &[Token::U64(RoleFlags::IN_PROMPT.bits())],
        );
    }
}
