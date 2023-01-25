use bitflags::bitflags;
use serde::{Deserialize, Serialize};

bitflags! {
    /// Flags to signal state and modify the look of a message.
    #[allow(clippy::unsafe_derive_deserialize)]
    #[derive(Deserialize, Serialize)]
    #[serde(transparent)]
    pub struct MessageFlags: u64 {
        /// Has been published to subscribed channels via Channel Following.
        const CROSSPOSTED = 1;
        /// Is a crosspost from another channel via Channel Following.
        const IS_CROSSPOST = 1 << 1;
        /// Do not include any embeds when serializing this message.
        const SUPPRESS_EMBEDS = 1 << 2;
        /// Source message for this message has been deleted via Channel
        /// Following.
        const SOURCE_MESSAGE_DELETED = 1 << 3;
        /// Comes from the urgent message system.
        const URGENT = 1 << 4;
        /// A thread has been started from this message.
        const HAS_THREAD = 1 << 5;
        /// When used, only shows a message to the invoking user.
        ///
        /// Used when responding to an [`Interaction`].
        ///
        /// [`Interaction`]: crate::application::interaction::Interaction
        const EPHEMERAL = 1 << 6;
        /// This message is an Interaction Response, and the bot is "thinking".
        const LOADING = 1 << 7;
        /// This message failed to mention some roles in a thread, which
        /// subsequently failed to add the role's members to the thread.
        const FAILED_TO_MENTION_SOME_ROLES_IN_THREAD  = 1 << 8;
    }
}

#[cfg(test)]
mod tests {
    use super::MessageFlags;
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
        MessageFlags: Binary,
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
        Extend<MessageFlags>,
        FromIterator<MessageFlags>,
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
    const_assert_eq!(MessageFlags::CROSSPOSTED.bits(), 1);
    const_assert_eq!(MessageFlags::IS_CROSSPOST.bits(), 1 << 1);
    const_assert_eq!(MessageFlags::SUPPRESS_EMBEDS.bits(), 1 << 2);
    const_assert_eq!(MessageFlags::SOURCE_MESSAGE_DELETED.bits(), 1 << 3);
    const_assert_eq!(MessageFlags::URGENT.bits(), 1 << 4);
    const_assert_eq!(MessageFlags::HAS_THREAD.bits(), 1 << 5);
    const_assert_eq!(MessageFlags::EPHEMERAL.bits(), 1 << 6);
    const_assert_eq!(MessageFlags::LOADING.bits(), 1 << 7);
    const_assert_eq!(
        MessageFlags::FAILED_TO_MENTION_SOME_ROLES_IN_THREAD.bits(),
        1 << 8
    );

    #[test]
    fn serde() {
        serde_test::assert_tokens(
            &MessageFlags::CROSSPOSTED,
            &[Token::U64(MessageFlags::CROSSPOSTED.bits())],
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
        let value = unsafe { MessageFlags::from_bits_unchecked(1 << 63) };
        serde_test::assert_de_tokens(&value, &[Token::U64(1 << 63)]);
    }
}
