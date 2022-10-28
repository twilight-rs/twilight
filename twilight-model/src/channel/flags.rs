use bitflags::bitflags;
use serde::{
    de::{Deserialize, Deserializer},
    ser::{Serialize, Serializer},
};

bitflags! {
    pub struct ChannelFlags: u64 {
        /// Channel is pinned in a forum.
        const PINNED = 1 << 1;
        /// New threads in a forum channel require a tag.
        const REQUIRE_TAG = 1 << 4;
    }
}

impl<'de> Deserialize<'de> for ChannelFlags {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        Ok(Self::from_bits_truncate(u64::deserialize(deserializer)?))
    }
}

impl Serialize for ChannelFlags {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_u64(self.bits())
    }
}
