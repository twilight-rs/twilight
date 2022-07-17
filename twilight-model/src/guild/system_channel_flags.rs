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
