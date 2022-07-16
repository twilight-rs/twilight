use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
#[serde(from = "u8", into = "u8")]
pub enum ActivityType {
    Playing,
    Streaming,
    Listening,
    Watching,
    Custom,
    Competing,
    Unknown(u8),
}

impl From<u8> for ActivityType {
    fn from(value: u8) -> Self {
        match value {
            0 => ActivityType::Playing,
            1 => ActivityType::Streaming,
            2 => ActivityType::Listening,
            3 => ActivityType::Watching,
            4 => ActivityType::Custom,
            5 => ActivityType::Competing,
            unknown => ActivityType::Unknown(unknown),
        }
    }
}

impl From<ActivityType> for u8 {
    fn from(value: ActivityType) -> Self {
        match value {
            ActivityType::Playing => 0,
            ActivityType::Streaming => 1,
            ActivityType::Listening => 2,
            ActivityType::Watching => 3,
            ActivityType::Custom => 4,
            ActivityType::Competing => 5,
            ActivityType::Unknown(unknown) => unknown,
        }
    }
}

impl Default for ActivityType {
    fn default() -> Self {
        Self::Playing
    }
}

#[cfg(test)]
mod tests {
    use super::ActivityType;
    use serde_test::Token;

    #[test]
    fn default() {
        assert_eq!(ActivityType::Playing, ActivityType::default());
    }

    #[test]
    fn variants() {
        serde_test::assert_tokens(&ActivityType::Playing, &[Token::U8(0)]);
        serde_test::assert_tokens(&ActivityType::Streaming, &[Token::U8(1)]);
        serde_test::assert_tokens(&ActivityType::Listening, &[Token::U8(2)]);
        serde_test::assert_tokens(&ActivityType::Watching, &[Token::U8(3)]);
        serde_test::assert_tokens(&ActivityType::Custom, &[Token::U8(4)]);
        serde_test::assert_tokens(&ActivityType::Competing, &[Token::U8(5)]);
        serde_test::assert_tokens(&ActivityType::Unknown(99), &[Token::U8(99)]);
    }
}
