use serde_repr::{Deserialize_repr, Serialize_repr};

#[derive(
    Clone, Copy, Debug, Deserialize_repr, Eq, Hash, PartialEq, PartialOrd, Ord, Serialize_repr,
)]
#[repr(u8)]
pub enum NSFWLevel {
    Default = 0,
    Explicit = 1,
    Safe = 2,
    AgeRestricted = 3,
}

#[cfg(test)]
mod tests {
    use super::NSFWLevel;
    use serde_test::Token;

    #[test]
    fn variants() {
        serde_test::assert_tokens(&NSFWLevel::Default, &[Token::U8(0)]);
        serde_test::assert_tokens(&NSFWLevel::Explicit, &[Token::U8(1)]);
        serde_test::assert_tokens(&NSFWLevel::Safe, &[Token::U8(2)]);
        serde_test::assert_tokens(&NSFWLevel::AgeRestricted, &[Token::U8(3)]);
    }
}
