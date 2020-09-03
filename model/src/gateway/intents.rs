use bitflags::bitflags;
use serde::{
    de::{Deserialize, Deserializer},
    ser::{Serialize, Serializer},
};

bitflags! {
    pub struct Intents: u64 {
        const GUILDS = 1;
        const GUILD_MEMBERS = 1 << 1;
        const GUILD_BANS = 1 << 2;
        const GUILD_EMOJIS = 1 << 3;
        const GUILD_INTEGRATIONS = 1 << 4;
        const GUILD_WEBHOOKS = 1 << 5;
        const GUILD_INVITES = 1 << 6;
        const GUILD_VOICE_STATES = 1 << 7;
        const GUILD_PRESENCES = 1 << 8;
        const GUILD_MESSAGES = 1 << 9;
        const GUILD_MESSAGE_REACTIONS = 1 << 10;
        const GUILD_MESSAGE_TYPING = 1 << 11;
        const DIRECT_MESSAGES = 1 << 12;
        const DIRECT_MESSAGE_REACTIONS = 1 << 13;
        const DIRECT_MESSAGE_TYPING = 1 << 14;
    }
}

impl<'de> Deserialize<'de> for Intents {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        Ok(Self::from_bits_truncate(u64::deserialize(deserializer)?))
    }
}

impl Serialize for Intents {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_u64(self.bits())
    }
}

#[cfg(test)]
mod tests {
    use super::Intents;
    use serde_test::Token;

    #[test]
    fn test_variants() {
        serde_test::assert_tokens(&Intents::GUILDS, &[Token::U64(1)]);
        serde_test::assert_tokens(&Intents::GUILD_MEMBERS, &[Token::U64(1 << 1)]);
        serde_test::assert_tokens(&Intents::GUILD_BANS, &[Token::U64(1 << 2)]);
        serde_test::assert_tokens(&Intents::GUILD_EMOJIS, &[Token::U64(1 << 3)]);
        serde_test::assert_tokens(&Intents::GUILD_INTEGRATIONS, &[Token::U64(1 << 4)]);
        serde_test::assert_tokens(&Intents::GUILD_WEBHOOKS, &[Token::U64(1 << 5)]);
        serde_test::assert_tokens(&Intents::GUILD_INVITES, &[Token::U64(1 << 6)]);
        serde_test::assert_tokens(&Intents::GUILD_VOICE_STATES, &[Token::U64(1 << 7)]);
        serde_test::assert_tokens(&Intents::GUILD_PRESENCES, &[Token::U64(1 << 8)]);
        serde_test::assert_tokens(&Intents::GUILD_MESSAGES, &[Token::U64(1 << 9)]);
        serde_test::assert_tokens(&Intents::GUILD_MESSAGE_REACTIONS, &[Token::U64(1 << 10)]);
        serde_test::assert_tokens(&Intents::GUILD_MESSAGE_TYPING, &[Token::U64(1 << 11)]);
        serde_test::assert_tokens(&Intents::DIRECT_MESSAGES, &[Token::U64(1 << 12)]);
        serde_test::assert_tokens(&Intents::DIRECT_MESSAGE_REACTIONS, &[Token::U64(1 << 13)]);
        serde_test::assert_tokens(&Intents::DIRECT_MESSAGE_TYPING, &[Token::U64(1 << 14)]);
    }
}
