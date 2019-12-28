use bitflags::bitflags;

bitflags! {
    pub struct SystemChannelFlags: u64 {
        const SUPPRESS_JOIN_NOTIFICATIONS = 1 << 0;
        const SUPPRESS_PREMIUM_SUBSCRIPTIONS = 1 << 1;
    }
}

#[cfg(feature = "serde-support")]
mod serde_support {
    use super::SystemChannelFlags;
    use serde::{
        de::{Deserialize, Deserializer},
        ser::{Serialize, Serializer},
    };

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
}
