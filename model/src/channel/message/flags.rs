use bitflags::bitflags;

bitflags! {
    pub struct MessageFlags: u64 {
        const CROSSPOSTED = 1;
        const IS_CROSSPOST = 1 << 1;
        const SUPPRESS_EMBEDS = 1 << 2;
    }
}

#[cfg(feature = "serde-support")]
mod serde_support {
    use super::MessageFlags;
    use serde::{
        de::{Deserialize, Deserializer},
        ser::{Serialize, Serializer},
    };

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
}
