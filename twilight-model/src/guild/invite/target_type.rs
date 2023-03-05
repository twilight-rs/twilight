use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
#[non_exhaustive]
#[serde(from = "u8", into = "u8")]
#[cfg_attr(
    feature = "rkyv",
    derive(rkyv::Archive, rkyv::Deserialize, rkyv::Serialize),
    archive(as = "Self")
)]
pub enum TargetType {
    Stream,
    EmbeddedApplication,
    Unknown(u8),
}

impl From<u8> for TargetType {
    fn from(value: u8) -> Self {
        match value {
            1 => TargetType::Stream,
            2 => TargetType::EmbeddedApplication,
            unknown => TargetType::Unknown(unknown),
        }
    }
}

impl From<TargetType> for u8 {
    fn from(value: TargetType) -> Self {
        match value {
            TargetType::Stream => 1,
            TargetType::EmbeddedApplication => 2,
            TargetType::Unknown(unknown) => unknown,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::TargetType;
    use serde_test::Token;

    #[test]
    fn variants() {
        serde_test::assert_tokens(&TargetType::Stream, &[Token::U8(1)]);
        serde_test::assert_tokens(&TargetType::EmbeddedApplication, &[Token::U8(2)]);
        serde_test::assert_tokens(&TargetType::Unknown(99), &[Token::U8(99)]);
    }
}
