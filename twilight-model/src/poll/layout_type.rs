use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
#[non_exhaustive]
#[serde(from = "u8", into = "u8")]
/// Layout of a poll.
pub enum PollLayoutType {
    /// Default layout.
    Default,
    /// Unknown layout.
    Unknown(u8),
}

impl From<u8> for PollLayoutType {
    fn from(value: u8) -> Self {
        match value {
            1 => PollLayoutType::Default,
            unknown => PollLayoutType::Unknown(unknown),
        }
    }
}

impl From<PollLayoutType> for u8 {
    fn from(value: PollLayoutType) -> Self {
        match value {
            PollLayoutType::Default => 1,
            PollLayoutType::Unknown(unknown) => unknown,
        }
    }
}

impl PollLayoutType {
    pub const fn name(&self) -> &str {
        match self {
            PollLayoutType::Default => "Default",
            PollLayoutType::Unknown(_) => "Unknown",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::PollLayoutType;
    use serde_test::Token;

    #[test]
    fn variants() {
        serde_test::assert_tokens(&PollLayoutType::Default, &[Token::U8(1)]);
        serde_test::assert_tokens(&PollLayoutType::Unknown(2), &[Token::U8(2)]);
    }

    #[test]
    fn names() {
        assert_eq!(PollLayoutType::Default.name(), "Default");
        assert_eq!(PollLayoutType::Unknown(2).name(), "Unknown");
    }
}
