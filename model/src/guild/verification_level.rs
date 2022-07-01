use serde_repr::{Deserialize_repr, Serialize_repr};

#[derive(
    Clone, Copy, Debug, Deserialize_repr, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize_repr,
)]
#[non_exhaustive]
#[repr(u8)]
pub enum VerificationLevel {
    None = 0,
    Low = 1,
    Medium = 2,
    High = 3,
    VeryHigh = 4,
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
    }
}
