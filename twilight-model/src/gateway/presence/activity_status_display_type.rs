use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
#[non_exhaustive]
#[serde(from = "u8", into = "u8")]
pub enum ActivityStatusDisplayType {
    Name,
    State,
    Details,
    Unknown(u8),
}

impl From<u8> for ActivityStatusDisplayType {
    fn from(value: u8) -> Self {
        match value {
            0 => ActivityStatusDisplayType::Name,
            1 => ActivityStatusDisplayType::State,
            2 => ActivityStatusDisplayType::Details,
            unknown => ActivityStatusDisplayType::Unknown(unknown),
        }
    }
}

impl From<ActivityStatusDisplayType> for u8 {
    fn from(value: ActivityStatusDisplayType) -> Self {
        match value {
            ActivityStatusDisplayType::Name => 0,
            ActivityStatusDisplayType::State => 1,
            ActivityStatusDisplayType::Details => 2,
            ActivityStatusDisplayType::Unknown(unknown) => unknown,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::ActivityStatusDisplayType;
    use serde_test::Token;

    #[test]
    fn variants() {
        serde_test::assert_tokens(&ActivityStatusDisplayType::Name, &[Token::U8(0)]);
        serde_test::assert_tokens(&ActivityStatusDisplayType::State, &[Token::U8(1)]);
        serde_test::assert_tokens(&ActivityStatusDisplayType::Details, &[Token::U8(2)]);
        serde_test::assert_tokens(&ActivityStatusDisplayType::Unknown(99), &[Token::U8(99)]);
    }
}
