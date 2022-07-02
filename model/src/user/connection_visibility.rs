use serde_repr::{Deserialize_repr, Serialize_repr};

#[derive(Clone, Copy, Debug, Deserialize_repr, Eq, Hash, PartialEq, Serialize_repr)]
#[non_exhaustive]
#[repr(u8)]
pub enum ConnectionVisibility {
    None = 0,
    Everyone = 1,
}

#[cfg(test)]
mod tests {
    use super::ConnectionVisibility;
    use serde_test::Token;

    #[test]
    fn variants() {
        serde_test::assert_tokens(&ConnectionVisibility::None, &[Token::U8(0)]);
        serde_test::assert_tokens(&ConnectionVisibility::Everyone, &[Token::U8(1)]);
    }
}
