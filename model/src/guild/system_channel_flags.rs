use bitflags::bitflags;
use serde::{
    de::{Deserialize, Deserializer},
    ser::{Serialize, Serializer},
};

bitflags! {
    pub struct SystemChannelFlags: u64 {
        const SUPPRESS_JOIN_NOTIFICATIONS = 1;
        const SUPPRESS_PREMIUM_SUBSCRIPTIONS = 1 << 1;
        const SUPPRESS_GUILD_REMINDER_NOTIFICATIONS = 1 << 2;
    }
}

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

#[cfg(test)]
mod tests {
    use super::SystemChannelFlags;
    use serde_test::Token;

    #[test]
    fn test_variants() {
        serde_test::assert_tokens(
            &SystemChannelFlags::SUPPRESS_JOIN_NOTIFICATIONS,
            &[Token::U64(1)],
        );
        serde_test::assert_tokens(
            &SystemChannelFlags::SUPPRESS_PREMIUM_SUBSCRIPTIONS,
            &[Token::U64(1 << 1)],
        );
        serde_test::assert_tokens(
            &SystemChannelFlags::SUPPRESS_GUILD_REMINDER_NOTIFICATIONS,
            &[Token::U64(1 << 2)],
        );
    }
}
