use bitflags::bitflags;
use serde::{Deserialize, Serialize};

bitflags! {
    #[allow(clippy::unsafe_derive_deserialize)]
    #[derive(Deserialize, Serialize)]
    #[serde(transparent)]
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
        // Safety:
        //
        // Deserialization doesn't truncate unknown bits.
        //
        // `bitflags` requires unsafe code to create bitflags with unknown bits
        // due to an unorthodox definition of unsafe:
        //
        // <https://github.com/bitflags/bitflags/issues/262>
        #[allow(unsafe_code)]
        let value = unsafe { ActivityFlags::from_bits_unchecked(1 << 63) };
        serde_test::assert_de_tokens(&value, &[Token::U64(1 << 63)]);
    }
}
