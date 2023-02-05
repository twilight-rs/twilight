use bitflags::bitflags;
use serde::{Deserialize, Serialize};

bitflags! {
    #[allow(clippy::unsafe_derive_deserialize)]
    #[derive(Deserialize, Serialize)]
    #[serde(transparent)]
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
        // Safety:
        //
        // Deserialization doesn't truncate unknown bits.
        //
        // `bitflags` requires unsafe code to create bitflags with unknown bits
        // due to an unorthodox definition of unsafe:
        //
        // <https://github.com/bitflags/bitflags/issues/262>
        #[allow(unsafe_code)]
        let value = unsafe { SystemChannelFlags::from_bits_unchecked(1 << 63) };
        serde_test::assert_de_tokens(&value, &[Token::U64(1 << 63)]);
    }
}
