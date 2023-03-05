use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
#[non_exhaustive]
#[serde(from = "u8", into = "u8")]
#[cfg_attr(
    feature = "rkyv",
    derive(rkyv::Archive, rkyv::Deserialize, rkyv::Serialize),
    archive(as = "Self")
)]
pub enum NSFWLevel {
    Default,
    Explicit,
    Safe,
    AgeRestricted,
    Unknown(u8),
}

impl From<u8> for NSFWLevel {
    fn from(value: u8) -> Self {
        match value {
            0 => NSFWLevel::Default,
            1 => NSFWLevel::Explicit,
            2 => NSFWLevel::Safe,
            3 => NSFWLevel::AgeRestricted,
            unknown => NSFWLevel::Unknown(unknown),
        }
    }
}

impl From<NSFWLevel> for u8 {
    fn from(value: NSFWLevel) -> Self {
        match value {
            NSFWLevel::Default => 0,
            NSFWLevel::Explicit => 1,
            NSFWLevel::Safe => 2,
            NSFWLevel::AgeRestricted => 3,
            NSFWLevel::Unknown(unknown) => unknown,
        }
    }
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
        serde_test::assert_tokens(&NSFWLevel::Unknown(99), &[Token::U8(99)]);
    }
}
