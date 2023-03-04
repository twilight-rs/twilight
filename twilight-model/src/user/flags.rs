use bitflags::bitflags;
use serde::{
    de::{Deserialize, Deserializer},
    ser::{Serialize, Serializer},
};

bitflags! {
    #[cfg_attr(
        feature = "rkyv",
        derive(rkyv::Archive, rkyv::Deserialize, rkyv::Serialize),
        archive(as = "Self")
    )]
    pub struct UserFlags: u64 {
        /// Discord Employee.
        const STAFF = 1;
        /// Partnered server owner.
        const PARTNER = 1 << 1;
        /// HypeSquad events member.
        const HYPESQUAD = 1 << 2;
        /// Bug hunter level 1.
        const BUG_HUNTER_LEVEL_1 = 1 << 3;
        /// House Bravery member.
        const HYPESQUAD_ONLINE_HOUSE_1 = 1 << 6;
        /// House Brilliance member.
        const HYPESQUAD_ONLINE_HOUSE_2 = 1 << 7;
        /// House Balance member.
        const HYPESQUAD_ONLINE_HOUSE_3 = 1 << 8;
        /// Early Nitro supporter.
        const PREMIUM_EARLY_SUPPORTER = 1 << 9;
        /// User is in a team.
        const TEAM_PSEUDO_USER = 1 << 10;
        /// Bug hunter level 2.
        const BUG_HUNTER_LEVEL_2 = 1 << 14;
        /// Verified bot.
        const VERIFIED_BOT = 1 << 16;
        /// Early verified bot developer.
        const VERIFIED_DEVELOPER = 1 << 17;
        /// Moderator Programs Alumni
        #[deprecated(since = "0.14.0", note = "use `MODERATOR_PROGRAMS_ALUMNI`")]
        const CERTIFIED_MODERATOR = 1 << 18;
        /// Moderator Programs Alumni
        const MODERATOR_PROGRAMS_ALUMNI = 1 << 18;
        /// Bot uses only HTTP interactions and is shown in the online member
        /// list.
        const BOT_HTTP_INTERACTIONS = 1 << 19;
        /// User is an [Active Developer].
        ///
        /// [Active Developer]: https://support-dev.discord.com/hc/articles/10113997751447
        const ACTIVE_DEVELOPER = 1 << 22;
    }
}

impl<'de> Deserialize<'de> for UserFlags {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        Ok(Self::from_bits_truncate(u64::deserialize(deserializer)?))
    }
}

impl Serialize for UserFlags {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_u64(self.bits())
    }
}

#[cfg(test)]
mod tests {
    #![allow(deprecated)]

    use super::UserFlags;
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
        UserFlags: Binary,
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
        Extend<UserFlags>,
        FromIterator<UserFlags>,
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

    const_assert_eq!(UserFlags::STAFF.bits(), 1);
    const_assert_eq!(UserFlags::PARTNER.bits(), 1 << 1);
    const_assert_eq!(UserFlags::HYPESQUAD.bits(), 1 << 2);
    const_assert_eq!(UserFlags::BUG_HUNTER_LEVEL_1.bits(), 1 << 3);
    const_assert_eq!(UserFlags::HYPESQUAD_ONLINE_HOUSE_1.bits(), 1 << 6);
    const_assert_eq!(UserFlags::HYPESQUAD_ONLINE_HOUSE_2.bits(), 1 << 7);
    const_assert_eq!(UserFlags::HYPESQUAD_ONLINE_HOUSE_3.bits(), 1 << 8);
    const_assert_eq!(UserFlags::PREMIUM_EARLY_SUPPORTER.bits(), 1 << 9);
    const_assert_eq!(UserFlags::TEAM_PSEUDO_USER.bits(), 1 << 10);
    const_assert_eq!(UserFlags::BUG_HUNTER_LEVEL_2.bits(), 1 << 14);
    const_assert_eq!(UserFlags::VERIFIED_BOT.bits(), 1 << 16);
    const_assert_eq!(UserFlags::VERIFIED_DEVELOPER.bits(), 1 << 17);
    const_assert_eq!(UserFlags::CERTIFIED_MODERATOR.bits(), 1 << 18);
    const_assert_eq!(UserFlags::MODERATOR_PROGRAMS_ALUMNI.bits(), 1 << 18);
    const_assert_eq!(UserFlags::BOT_HTTP_INTERACTIONS.bits(), 1 << 19);
    const_assert_eq!(UserFlags::ACTIVE_DEVELOPER.bits(), 1 << 22);

    #[test]
    fn serde() {
        serde_test::assert_tokens(
            &UserFlags::PARTNER,
            &[Token::U64(UserFlags::PARTNER.bits())],
        );
        // Deserialization truncates unknown bits.
        serde_test::assert_de_tokens(&UserFlags::empty(), &[Token::U64(1 << 63)]);
    }
}
