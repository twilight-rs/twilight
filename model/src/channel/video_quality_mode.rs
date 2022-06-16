use serde_repr::{Deserialize_repr, Serialize_repr};

#[derive(Clone, Copy, Debug, Deserialize_repr, Eq, Hash, PartialEq, Serialize_repr)]
#[non_exhaustive]
#[repr(u8)]
pub enum VideoQualityMode {
    /// Discord chooses the quality for optimal performance.
    Auto = 1,
    /// 720p.
    Full = 2,
}

impl VideoQualityMode {
    pub const fn name(self) -> &'static str {
        match self {
            Self::Auto => "Auto",
            Self::Full => "Full",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::VideoQualityMode;
    use serde_test::Token;

    #[test]
    fn variants() {
        serde_test::assert_tokens(&VideoQualityMode::Auto, &[Token::U8(1)]);
        serde_test::assert_tokens(&VideoQualityMode::Full, &[Token::U8(2)]);
    }

    #[test]
    fn names() {
        assert_eq!("Auto", VideoQualityMode::Auto.name());
        assert_eq!("Full", VideoQualityMode::Full.name());
    }
}
