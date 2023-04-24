use bitflags::bitflags;
use serde::{
    de::{Deserialize, Deserializer},
    ser::{Serialize, Serializer},
};

bitflags! {
    #[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, PartialOrd, Ord)]
    pub struct ActivityFlags: u64 {
        const INSTANCE = 1;
        const JOIN = 1 << 1;
        const SPECTATE = 1 << 2;
        const JOIN_REQUEST = 1 << 3;
        const SYNC = 1 << 4;
        const PLAY = 1 << 5;
        const PARTY_PRIVACY_FRIENDS = 1 << 6;
        const PARTY_PRIVACY_VOICE_CHANNEL = 1 << 7;
        const EMBEDDED = 1 << 8;
    }
}

impl<'de> Deserialize<'de> for ActivityFlags {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        Ok(Self::from_bits_truncate(u64::deserialize(deserializer)?))
    }
}

impl Serialize for ActivityFlags {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_u64(self.bits())
    }
}

#[cfg(test)]
mod tests {
    use super::ActivityFlags;
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
        ActivityFlags: Binary,
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
        Extend<ActivityFlags>,
        FromIterator<ActivityFlags>,
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
    const_assert_eq!(ActivityFlags::INSTANCE.bits(), 1);
    const_assert_eq!(ActivityFlags::JOIN.bits(), 1 << 1);
    const_assert_eq!(ActivityFlags::SPECTATE.bits(), 1 << 2);
    const_assert_eq!(ActivityFlags::JOIN_REQUEST.bits(), 1 << 3);
    const_assert_eq!(ActivityFlags::SYNC.bits(), 1 << 4);
    const_assert_eq!(ActivityFlags::PLAY.bits(), 1 << 5);
    const_assert_eq!(ActivityFlags::PARTY_PRIVACY_FRIENDS.bits(), 1 << 6);
    const_assert_eq!(ActivityFlags::PARTY_PRIVACY_VOICE_CHANNEL.bits(), 1 << 7);
    const_assert_eq!(ActivityFlags::EMBEDDED.bits(), 1 << 8);

    #[test]
    fn serde() {
        serde_test::assert_tokens(
            &ActivityFlags::EMBEDDED,
            &[Token::U64(ActivityFlags::EMBEDDED.bits())],
        );
        // Deserialization truncates unknown bits.
        serde_test::assert_de_tokens(&ActivityFlags::empty(), &[Token::U64(1 << 63)]);
    }
}
