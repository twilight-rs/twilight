use bitflags::bitflags;
use serde::{
    de::{Deserialize, Deserializer},
    ser::{Serialize, Serializer},
};

bitflags! {
    pub struct SystemChannelFlags: u64 {
        /// Suppress member join notifications.
        const SUPPRESS_JOIN_NOTIFICATIONS = 1;
        /// Suppress server boost notifications.
        const SUPPRESS_PREMIUM_SUBSCRIPTIONS = 1 << 1;
        /// Suppress server setup tips.
        const SUPPRESS_GUILD_REMINDER_NOTIFICATIONS = 1 << 2;
        /// Hide member join sticker reply buttons.
        const SUPPRESS_JOIN_NOTIFICATION_REPLIES = 1 << 3;
        /// Suppress role subscription purchase and renewal notifications.
        const SUPPRESS_ROLE_SUBSCRIPTION_PURCHASE_NOTIFICATIONS = 1 << 4;
        /// Hide role subscription sticker reply buttons.
        const SUPPRESS_ROLE_SUBSCRIPTION_PURCHASE_NOTIFICATION_REPLIES = 1 << 5;
    }
}

impl<'de> Deserialize<'de> for SystemChannelFlags {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        Ok(Self::from_bits_truncate(u64::deserialize(deserializer)?))
    }
}

impl Serialize for SystemChannelFlags {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_u64(self.bits())
    }
}

#[cfg(test)]
mod tests {
    use super::SystemChannelFlags;
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
        SystemChannelFlags: Binary,
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
        Extend<SystemChannelFlags>,
        FromIterator<SystemChannelFlags>,
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
    const_assert_eq!(SystemChannelFlags::SUPPRESS_JOIN_NOTIFICATIONS.bits(), 1);
    const_assert_eq!(
        SystemChannelFlags::SUPPRESS_PREMIUM_SUBSCRIPTIONS.bits(),
        1 << 1
    );
    const_assert_eq!(
        SystemChannelFlags::SUPPRESS_GUILD_REMINDER_NOTIFICATIONS.bits(),
        1 << 2
    );
    const_assert_eq!(
        SystemChannelFlags::SUPPRESS_JOIN_NOTIFICATION_REPLIES.bits(),
        1 << 3
    );
    const_assert_eq!(
        SystemChannelFlags::SUPPRESS_ROLE_SUBSCRIPTION_PURCHASE_NOTIFICATIONS.bits(),
        1 << 4
    );
    const_assert_eq!(
        SystemChannelFlags::SUPPRESS_ROLE_SUBSCRIPTION_PURCHASE_NOTIFICATION_REPLIES.bits(),
        1 << 5
    );

    #[test]
    fn serde() {
        serde_test::assert_tokens(
            &SystemChannelFlags::SUPPRESS_JOIN_NOTIFICATION_REPLIES,
            &[Token::U64(
                SystemChannelFlags::SUPPRESS_JOIN_NOTIFICATION_REPLIES.bits(),
            )],
        );
        // Deserialization truncates unknown bits.
        serde_test::assert_de_tokens(&SystemChannelFlags::empty(), &[Token::U64(1 << 63)]);
    }
}
