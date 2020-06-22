use bitflags::bitflags;
use serde::{
    de::{Deserialize, Deserializer},
    ser::{Serialize, Serializer},
};

bitflags! {
    pub struct ActivityFlags: u64 {
        const INSTANCE = 0b001;
        const JOIN = 0b010;
        const SPECTATE = 0b011;
        const JOIN_REQUEST = 0b100;
        const SYNC = 0b101;
        const PLAY = 0b110;
    }
}

impl<'de> Deserialize<'de> for ActivityFlags {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        Ok(Self::from_bits_truncate(u64::deserialize(deserializer)?))
    }
}

impl Serialize for ActivityFlags {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_u64(self.bits())
    }
}
