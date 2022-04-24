use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
#[serde(from = "u8", into = "u8")]

pub enum MessageActivityType {
    Join,
    Spectate,
    Listen,
    JoinRequest,
    Unknown(u8),
}

impl From<u8> for MessageActivityType {
    fn from(value: u8) -> Self {
        match value {
            1 => MessageActivityType::Join,
            2 => MessageActivityType::Spectate,
            3 => MessageActivityType::Listen,
            5 => MessageActivityType::JoinRequest,
            unknown => MessageActivityType::Unknown(unknown),
        }
    }
}

impl From<MessageActivityType> for u8 {
    fn from(value: MessageActivityType) -> Self {
        match value {
            MessageActivityType::Join => 1,
            MessageActivityType::Spectate => 2,
            MessageActivityType::Listen => 3,
            MessageActivityType::JoinRequest => 5,
            MessageActivityType::Unknown(unknown) => unknown,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::MessageActivityType;
    use serde_test::Token;

    #[test]
    fn test_variants() {
        serde_test::assert_tokens(&MessageActivityType::Join, &[Token::U8(1)]);
        serde_test::assert_tokens(&MessageActivityType::Spectate, &[Token::U8(2)]);
        serde_test::assert_tokens(&MessageActivityType::Listen, &[Token::U8(3)]);
        serde_test::assert_tokens(&MessageActivityType::JoinRequest, &[Token::U8(5)]);
        serde_test::assert_tokens(&MessageActivityType::Unknown(99), &[Token::U8(99)]);
    }
}
