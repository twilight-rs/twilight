use bitflags::bitflags;
use serde::{
    de::{Deserialize, Deserializer},
    ser::{Serialize, Serializer},
};

bitflags! {
    pub struct MessageFlags: u64 {
        const CROSSPOSTED = 1;
        const IS_CROSSPOST = 1 << 1;
        const SUPPRESS_EMBEDS = 1 << 2;
        const SOURCE_MESSAGE_DELETED = 1 << 3;
        const URGENT = 1 << 4;
        /// When used, only shows a message to the invoking user.
        ///
        /// Used when responding to an [`Interaction`].
        ///
        /// [`Interaction`]: crate::application::interaction::Interaction
        const EPHEMERAL = 1 << 6;
        /// A thread has been started from this message.
        ///
        /// All threads must be started from a message, but can be
        /// orphaned if the message is later deleted.
        const HAS_THREAD = 1 << 5;
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
