use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(from = "u8", into = "u8")]
pub enum VideoQualityMode {
    /// Discord chooses the quality for optimal performance.
    Auto,
    /// 720p.
    Full,
    /// Variant value is unknown to the library.
    Unknown(u8),
}

impl From<u8> for VideoQualityMode {
    fn from(value: u8) -> Self {
        match value {
            1 => VideoQualityMode::Auto,
            2 => VideoQualityMode::Full,
            unknown => VideoQualityMode::Unknown(unknown),
        }
    }
}

impl From<VideoQualityMode> for u8 {
    fn from(value: VideoQualityMode) -> Self {
        match value {
            VideoQualityMode::Auto => 1,
            VideoQualityMode::Full => 2,
            VideoQualityMode::Unknown(unknown) => unknown,
        }
    }
}

impl VideoQualityMode {
    pub const fn name(self) -> &'static str {
        match self {
            Self::Auto => "Auto",
            Self::Full => "Full",
            Self::Unknown(_) => "Unknown",
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
        serde_test::assert_tokens(&VideoQualityMode::Unknown(99), &[Token::U8(99)]);
    }

    #[test]
    fn names() {
        assert_eq!("Auto", VideoQualityMode::Auto.name());
        assert_eq!("Full", VideoQualityMode::Full.name());
        assert_eq!("Unknown", VideoQualityMode::Unknown(99).name());
    }
}
