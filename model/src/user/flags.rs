use bitflags::bitflags;
use serde::{
    de::{Deserialize, Deserializer},
    ser::{Serialize, Serializer},
};

bitflags! {
    pub struct UserFlags: u64 {
        const DISCORD_EMPLOYEE = 1;
        const DISCORD_PARTNER = 1 << 1;
        const HYPESQUAD_EVENTS = 1 << 2;
        const BUG_HUNTER = 1 << 3;
        const HOUSE_BRAVERY = 1 << 6;
        const HOUSE_BRILLIANCE = 1 << 7;
        const HOUSE_BALANCE = 1 << 8;
        const EARLY_SUPPORTER = 1 << 9;
        const TEAM_USER = 1 << 10;
        const BUG_HUNTER_LEVEL_2 = 1 << 14;
        const VERIFIED_BOT = 1 << 16;
        const VERIFIED_BOT_DEVELOPER = 1 << 17;
    }
}

impl<'de> Deserialize<'de> for UserFlags {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        Ok(Self::from_bits_truncate(u64::deserialize(deserializer)?))
    }
}

impl Serialize for UserFlags {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_u64(self.bits())
    }
}

#[cfg(test)]
mod tests {
    use super::UserFlags;
    use serde_test::Token;

    #[test]
    fn test_variants() {
        serde_test::assert_tokens(&UserFlags::DISCORD_EMPLOYEE, &[Token::U64(1)]);
        serde_test::assert_tokens(&UserFlags::DISCORD_PARTNER, &[Token::U64(1 << 1)]);
        serde_test::assert_tokens(&UserFlags::HYPESQUAD_EVENTS, &[Token::U64(1 << 2)]);
        serde_test::assert_tokens(&UserFlags::BUG_HUNTER, &[Token::U64(1 << 3)]);
        serde_test::assert_tokens(&UserFlags::HOUSE_BRAVERY, &[Token::U64(1 << 6)]);
        serde_test::assert_tokens(&UserFlags::HOUSE_BRILLIANCE, &[Token::U64(1 << 7)]);
        serde_test::assert_tokens(&UserFlags::HOUSE_BALANCE, &[Token::U64(1 << 8)]);
        serde_test::assert_tokens(&UserFlags::EARLY_SUPPORTER, &[Token::U64(1 << 9)]);
        serde_test::assert_tokens(&UserFlags::TEAM_USER, &[Token::U64(1 << 10)]);
        serde_test::assert_tokens(&UserFlags::BUG_HUNTER_LEVEL_2, &[Token::U64(1 << 14)]);
        serde_test::assert_tokens(&UserFlags::VERIFIED_BOT, &[Token::U64(1 << 16)]);
        serde_test::assert_tokens(&UserFlags::VERIFIED_BOT_DEVELOPER, &[Token::U64(1 << 17)]);
    }
}
