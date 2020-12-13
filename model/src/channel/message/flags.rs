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
        const EPHEMERAL = 1 << 6;
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

#[cfg(test)]
mod tests {
    use super::MessageFlags;
    use serde_test::Token;

    #[test]
    fn test_variants() {
        serde_test::assert_tokens(&MessageFlags::CROSSPOSTED, &[Token::U64(1)]);
        serde_test::assert_tokens(&MessageFlags::IS_CROSSPOST, &[Token::U64(1 << 1)]);
        serde_test::assert_tokens(&MessageFlags::SUPPRESS_EMBEDS, &[Token::U64(1 << 2)]);
        serde_test::assert_tokens(&MessageFlags::SOURCE_MESSAGE_DELETED, &[Token::U64(1 << 3)]);
        serde_test::assert_tokens(&MessageFlags::URGENT, &[Token::U64(1 << 4)]);
        serde_test::assert_tokens(&MessageFlags::EPHEMERAL, &[Token::U64(1 << 6)]);
    }
}
