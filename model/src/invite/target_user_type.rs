use serde_repr::{Deserialize_repr, Serialize_repr};

#[derive(
    Clone, Copy, Debug, Deserialize_repr, Eq, Hash, PartialEq, PartialOrd, Ord, Serialize_repr,
)]
#[repr(u8)]
pub enum TargetUserType {
    Stream = 1,
}

#[cfg(test)]
mod tests {
    use super::TargetUserType;
    use serde_test::Token;

    #[test]
    fn test_variants() {
        serde_test::assert_tokens(&TargetUserType::Stream, &[Token::U8(1)]);
    }
}
