use bitflags::bitflags;
use serde::{
    de::{Deserialize, Deserializer},
    ser::{Serialize, Serializer},
};

bitflags! {
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

impl<'de> Deserialize<'de> for MessageFlags {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        Ok(Self::from_bits_truncate(u64::deserialize(deserializer)?))
    }
}

impl Serialize for MessageFlags {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_u64(self.bits())
    }
}
