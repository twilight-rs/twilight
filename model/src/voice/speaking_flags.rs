use bitflags::bitflags;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

bitflags! {
    pub struct SpeakingFlags: u8 {
        /// Normal voice audio transmition.
        const MICROPHONE = 1 << 0;
        /// Context audio for video transmition, no speaking indicator.
        const SOUNDSHARE = 1 << 1;
        /// Priority speaker, lowering audio of other speakers.
        const PRIORITY = 1 << 2;
    }
}

impl<'de> Deserialize<'de> for SpeakingFlags {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        Ok(Self::from_bits_truncate(u8::deserialize(deserializer)?))
    }
}

impl Serialize for SpeakingFlags {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_u8(self.bits())
    }
}
