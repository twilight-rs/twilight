use bitflags::bitflags;
use serde::{
    de::{Deserialize, Deserializer},
    ser::{Serialize, Serializer},
};

bitflags! {
    pub struct ApplicationFlags: u64 {
        const GATEWAY_PRESENCE = 1 << 12;
        const GATEWAY_PRESENCE_LIMITED = 1 << 13;
        const GATEWAY_GUILD_MEMBERS = 1 << 14;
        const GATEWAY_GUILD_MEMBERS_LIMITED = 1 << 15;
        const VERIFICATION_PENDING_GUILD_LIMIT = 1 << 16;
        const EMBEDDED = 1 << 17;
    }
}

impl<'de> Deserialize<'de> for ApplicationFlags {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        Ok(Self::from_bits_truncate(u64::deserialize(deserializer)?))
    }
}

impl Serialize for ApplicationFlags {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_u64(self.bits())
    }
}

#[cfg(test)]
mod tests {
    use super::ApplicationFlags;
    use serde_test::Token;

    #[test]
    fn test_variants() {
        serde_test::assert_tokens(&ApplicationFlags::GATEWAY_PRESENCE, &[Token::U64(1 << 12)]);
        serde_test::assert_tokens(
            &ApplicationFlags::GATEWAY_PRESENCE_LIMITED,
            &[Token::U64(1 << 13)],
        );
        serde_test::assert_tokens(
            &ApplicationFlags::GATEWAY_GUILD_MEMBERS,
            &[Token::U64(1 << 14)],
        );
        serde_test::assert_tokens(
            &ApplicationFlags::GATEWAY_GUILD_MEMBERS_LIMITED,
            &[Token::U64(1 << 15)],
        );
        serde_test::assert_tokens(
            &ApplicationFlags::VERIFICATION_PENDING_GUILD_LIMIT,
            &[Token::U64(1 << 16)],
        );
        serde_test::assert_tokens(&ApplicationFlags::EMBEDDED, &[Token::U64(1 << 17)]);
    }
}
