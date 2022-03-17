use serde_repr::{Deserialize_repr, Serialize_repr};
#[derive(Clone, Copy, Debug, Deserialize_repr, Eq, Hash, PartialEq, Serialize_repr)]
#[repr(u8)]
pub enum PremiumType {
    None = 0,
    NitroClassic = 1,
    Nitro = 2,
}

#[cfg(test)]
mod tests {
    use super::PremiumType;
    use serde_test::Token;

    #[test]
    fn test_variants() {
        serde_test::assert_tokens(&PremiumType::None, &[Token::U8(0)]);
        serde_test::assert_tokens(&PremiumType::NitroClassic, &[Token::U8(1)]);
        serde_test::assert_tokens(&PremiumType::Nitro, &[Token::U8(2)]);
    }
}
