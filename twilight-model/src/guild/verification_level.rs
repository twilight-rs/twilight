use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
#[non_exhaustive]
#[serde(from = "u8", into = "u8")]
pub enum VerificationLevel {
    None,
    Low,
    Medium,
    High,
    VeryHigh,
    /// Variant value is unknown to the library.
    Unknown(u8),
}

impl From<u8> for VerificationLevel {
    fn from(value: u8) -> Self {
        match value {
            0 => VerificationLevel::None,
            1 => VerificationLevel::Low,
            2 => VerificationLevel::Medium,
            3 => VerificationLevel::High,
            4 => VerificationLevel::VeryHigh,
            unknown => VerificationLevel::Unknown(unknown),
        }
    }
}

impl From<VerificationLevel> for u8 {
    fn from(value: VerificationLevel) -> Self {
        match value {
            VerificationLevel::None => 0,
            VerificationLevel::Low => 1,
            VerificationLevel::Medium => 2,
            VerificationLevel::High => 3,
            VerificationLevel::VeryHigh => 4,
            VerificationLevel::Unknown(unknown) => unknown,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::VerificationLevel;
    use serde_test::Token;

    #[test]
    fn variants() {
        serde_test::assert_tokens(&VerificationLevel::None, &[Token::U8(0)]);
        serde_test::assert_tokens(&VerificationLevel::Low, &[Token::U8(1)]);
        serde_test::assert_tokens(&VerificationLevel::Medium, &[Token::U8(2)]);
        serde_test::assert_tokens(&VerificationLevel::High, &[Token::U8(3)]);
        serde_test::assert_tokens(&VerificationLevel::VeryHigh, &[Token::U8(4)]);
        serde_test::assert_tokens(&VerificationLevel::Unknown(99), &[Token::U8(99)]);
    }
}
