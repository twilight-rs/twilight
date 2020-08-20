use bitflags::bitflags;
use serde::{
    de::{Deserialize, Deserializer},
    ser::{Serialize, Serializer},
};

bitflags! {
    pub struct ActivityFlags: u64 {
        const INSTANCE = 1;
        const JOIN = 1 << 1;
        const SPECTATE = 1 << 2;
        const JOIN_REQUEST = 1 << 3;
        const SYNC = 1 << 4;
        const PLAY = 1 << 5;
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

#[cfg(test)]
mod tests {
    use super::ActivityFlags;
    use serde_test::Token;

    #[test]
    fn test_variants() {
        serde_test::assert_tokens(&ActivityFlags::INSTANCE, &[Token::U64(1)]);
        serde_test::assert_tokens(&ActivityFlags::JOIN, &[Token::U64(1 << 1)]);
        serde_test::assert_tokens(&ActivityFlags::SPECTATE, &[Token::U64(1 << 2)]);
        serde_test::assert_tokens(&ActivityFlags::JOIN_REQUEST, &[Token::U64(1 << 3)]);
        serde_test::assert_tokens(&ActivityFlags::SYNC, &[Token::U64(1 << 4)]);
        serde_test::assert_tokens(&ActivityFlags::PLAY, &[Token::U64(1 << 5)]);
    }
}
