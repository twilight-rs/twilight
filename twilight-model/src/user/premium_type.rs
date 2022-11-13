use serde::{Deserialize, Serialize};
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
#[non_exhaustive]
#[serde(from = "u8", into = "u8")]
pub enum PremiumType {
    None,
    NitroClassic,
    Nitro,
    Unknown(u8),
}

impl From<u8> for PremiumType {
    fn from(value: u8) -> Self {
        match value {
            0 => PremiumType::None,
            1 => PremiumType::NitroClassic,
            2 => PremiumType::Nitro,
            unknown => PremiumType::Unknown(unknown),
        }
    }
}

impl From<PremiumType> for u8 {
    fn from(value: PremiumType) -> Self {
        match value {
            PremiumType::None => 0,
            PremiumType::NitroClassic => 1,
            PremiumType::Nitro => 2,
            PremiumType::Unknown(unknown) => unknown,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::PremiumType;
    use serde_test::Token;

    #[test]
    fn variants() {
        serde_test::assert_tokens(&PremiumType::None, &[Token::U8(0)]);
        serde_test::assert_tokens(&PremiumType::NitroClassic, &[Token::U8(1)]);
        serde_test::assert_tokens(&PremiumType::Nitro, &[Token::U8(2)]);
        serde_test::assert_tokens(&PremiumType::Unknown(42), &[Token::U8(42)]);
    }
}
