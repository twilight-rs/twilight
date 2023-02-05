use bitflags::bitflags;
use serde::{Deserialize, Serialize};

bitflags! {
    #[allow(clippy::unsafe_derive_deserialize)]
    #[derive(Deserialize, Serialize)]
    #[serde(transparent)]
    pub struct ChannelFlags: u64 {
        /// Channel is pinned in a forum.
        const PINNED = 1 << 1;
        /// New threads in a forum channel require a tag.
        const REQUIRE_TAG = 1 << 4;
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
    const_assert_eq!(ChannelFlags::PINNED.bits(), 1 << 1);
    const_assert_eq!(ChannelFlags::REQUIRE_TAG.bits(), 1 << 4);

    #[test]
    fn serde() {
        serde_test::assert_tokens(
            &ChannelFlags::PINNED,
            &[Token::U64(ChannelFlags::PINNED.bits())],
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
        let value = unsafe { ChannelFlags::from_bits_unchecked(1 << 63) };
        serde_test::assert_de_tokens(&value, &[Token::U64(1 << 63)]);
    }
}
