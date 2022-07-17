use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, PartialEq, PartialOrd, Ord, Serialize)]
#[serde(from = "u8", into = "u8")]
pub enum MfaLevel {
    None,
    Elevated,
    /// Variant value is unknown to the library.
    Unknown(u8),
}

impl From<u8> for MfaLevel {
    fn from(value: u8) -> Self {
        match value {
            0 => MfaLevel::None,
            1 => MfaLevel::Elevated,
            unknown => MfaLevel::Unknown(unknown),
        }
    }
}

impl From<MfaLevel> for u8 {
    fn from(value: MfaLevel) -> Self {
        match value {
            MfaLevel::None => 0,
            MfaLevel::Elevated => 1,
            MfaLevel::Unknown(unknown) => unknown,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::MfaLevel;
    use serde_test::Token;

    #[test]
    fn variants() {
        serde_test::assert_tokens(&MfaLevel::None, &[Token::U8(0)]);
        serde_test::assert_tokens(&MfaLevel::Elevated, &[Token::U8(1)]);
        serde_test::assert_tokens(&MfaLevel::Unknown(99), &[Token::U8(99)]);
    }
}
