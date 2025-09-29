use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
#[non_exhaustive]
#[serde(from = "u8", into = "u8")]
#[derive(Default)]
pub enum PremiumTier {
    #[default]
    None,
    Tier1,
    Tier2,
    Tier3,
    Other(u8),
}

impl From<u8> for PremiumTier {
    fn from(value: u8) -> Self {
        match value {
            0 => PremiumTier::None,
            1 => PremiumTier::Tier1,
            2 => PremiumTier::Tier2,
            3 => PremiumTier::Tier3,
            other => PremiumTier::Other(other),
        }
    }
}

impl From<PremiumTier> for u8 {
    fn from(value: PremiumTier) -> Self {
        match value {
            PremiumTier::None => 0,
            PremiumTier::Tier1 => 1,
            PremiumTier::Tier2 => 2,
            PremiumTier::Tier3 => 3,
            PremiumTier::Other(other) => other,
        }
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
        serde_test::assert_tokens(&PremiumTier::Other(99), &[Token::U8(99)]);
    }
}
