use serde_repr::{Deserialize_repr, Serialize_repr};

#[derive(
    Clone, Copy, Debug, Deserialize_repr, Eq, Hash, PartialEq, PartialOrd, Ord, Serialize_repr,
)]
#[repr(u8)]
pub enum PremiumTier {
    None = 0,
    Tier1 = 1,
    Tier2 = 2,
    Tier3 = 3,
}

impl Default for PremiumTier {
    fn default() -> Self {
        Self::None
    }
}

#[cfg(test)]
mod tests {
    use super::PremiumTier;
    use serde_test::Token;

    #[test]
    fn variants() {
        serde_test::assert_tokens(&PremiumTier::None, &[Token::U8(0)]);
        serde_test::assert_tokens(&PremiumTier::Tier1, &[Token::U8(1)]);
        serde_test::assert_tokens(&PremiumTier::Tier2, &[Token::U8(2)]);
        serde_test::assert_tokens(&PremiumTier::Tier3, &[Token::U8(3)]);
    }
}
