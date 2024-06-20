use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
#[non_exhaustive]
#[serde(from = "u8", into = "u8")]
/// The kind of reaction.
pub enum ReactionType {
    /// A non-burst/super reaction.
    Normal,
    /// A super reaction.
    Burst,
    /// An unknown reaction type.
    Unknown(u8),
}

impl From<u8> for ReactionType {
    fn from(value: u8) -> Self {
        match value {
            0 => Self::Normal,
            1 => Self::Burst,
            unknown => Self::Unknown(unknown),
        }
    }
}

impl From<ReactionType> for u8 {
    fn from(value: ReactionType) -> Self {
        match value {
            ReactionType::Normal => 0,
            ReactionType::Burst => 1,
            ReactionType::Unknown(unknown) => unknown,
        }
    }
}

impl ReactionType {
    /// The name of the reaction type.
    pub const fn name(self) -> &'static str {
        match self {
            Self::Normal => "Normal",
            Self::Burst => "Burst",
            Self::Unknown(_) => "Unknown",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::ReactionType;
    use serde_test::Token;

    #[test]
    fn variants() {
        serde_test::assert_tokens(&ReactionType::Normal, &[Token::U8(0)]);
        serde_test::assert_tokens(&ReactionType::Burst, &[Token::U8(1)]);
        serde_test::assert_tokens(&ReactionType::Unknown(255), &[Token::U8(255)]);
    }

    #[test]
    fn names() {
        assert_eq!(ReactionType::Normal.name(), "Normal");
        assert_eq!(ReactionType::Burst.name(), "Burst");
        assert_eq!(ReactionType::Unknown(255).name(), "Unknown");
    }
}
