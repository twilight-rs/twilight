use serde_repr::{Deserialize_repr, Serialize_repr};

#[derive(Clone, Copy, Debug, Deserialize_repr, Eq, Hash, PartialEq, Serialize_repr)]
#[repr(u8)]
pub enum MessageActivityType {
    Join = 1,
    Spectate = 2,
    Listen = 3,
    JoinRequest = 5,
}

#[cfg(test)]
mod tests {
    use super::MessageActivityType;
    use serde_test::Token;

    #[test]
    fn variants() {
        serde_test::assert_tokens(&MessageActivityType::Join, &[Token::U8(1)]);
        serde_test::assert_tokens(&MessageActivityType::Spectate, &[Token::U8(2)]);
        serde_test::assert_tokens(&MessageActivityType::Listen, &[Token::U8(3)]);
        serde_test::assert_tokens(&MessageActivityType::JoinRequest, &[Token::U8(5)]);
    }
}
