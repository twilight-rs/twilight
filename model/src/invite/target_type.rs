use serde_repr::{Deserialize_repr, Serialize_repr};

#[derive(Clone, Copy, Debug, Deserialize_repr, Eq, Hash, PartialEq, Serialize_repr)]
#[repr(u8)]
pub enum TargetType {
    Stream = 1,
    EmbeddedApplication = 2,
}

#[cfg(test)]
mod tests {
    use super::TargetType;
    use serde_test::Token;

    #[test]
    fn test_variants() {
        serde_test::assert_tokens(&TargetType::Stream, &[Token::U8(1)]);
        serde_test::assert_tokens(&TargetType::EmbeddedApplication, &[Token::U8(2)]);
    }
}
