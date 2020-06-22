use serde_repr::{Deserialize_repr, Serialize_repr};

#[derive(
    Clone, Copy, Debug, Deserialize_repr, Eq, Hash, PartialEq, PartialOrd, Ord, Serialize_repr,
)]
#[repr(u8)]
pub enum ActivityType {
    Playing = 0,
    Streaming = 1,
    Listening = 2,
    Watching = 3,
    Custom = 4,
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
    fn test_default() {
        assert_eq!(ActivityType::Playing, ActivityType::default());
    }

    #[test]
    fn test_activity_type_playing() {
        serde_test::assert_tokens(&ActivityType::Playing, &[Token::U8(0)]);
    }

    #[test]
    fn test_activity_type_streaming() {
        serde_test::assert_tokens(&ActivityType::Streaming, &[Token::U8(1)]);
    }

    #[test]
    fn test_activity_type_listening() {
        serde_test::assert_tokens(&ActivityType::Listening, &[Token::U8(2)]);
    }

    #[test]
    fn test_activity_type_watching() {
        serde_test::assert_tokens(&ActivityType::Watching, &[Token::U8(3)]);
    }

    #[test]
    fn test_activity_type_custom() {
        serde_test::assert_tokens(&ActivityType::Custom, &[Token::U8(4)]);
    }
}
