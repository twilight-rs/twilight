use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
#[non_exhaustive]
#[serde(from = "u8", into = "u8")]
pub enum ConnectionVisibility {
    None,
    Everyone,
    Unknown(u8),
}

impl From<u8> for ConnectionVisibility {
    fn from(value: u8) -> Self {
        match value {
            0 => ConnectionVisibility::None,
            1 => ConnectionVisibility::Everyone,
            unknown => ConnectionVisibility::Unknown(unknown),
        }
    }
}

impl From<ConnectionVisibility> for u8 {
    fn from(value: ConnectionVisibility) -> Self {
        match value {
            ConnectionVisibility::None => 0,
            ConnectionVisibility::Everyone => 1,
            ConnectionVisibility::Unknown(unknown) => unknown,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::ConnectionVisibility;
    use serde_test::Token;

    #[test]
    fn variants() {
        serde_test::assert_tokens(&ConnectionVisibility::None, &[Token::U8(0)]);
        serde_test::assert_tokens(&ConnectionVisibility::Everyone, &[Token::U8(1)]);
        serde_test::assert_tokens(&ConnectionVisibility::Unknown(99), &[Token::U8(99)]);
    }
}
