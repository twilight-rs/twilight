use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct VideoQualityMode(u8);

impl VideoQualityMode {
    /// Discord chooses the quality for optimal performance.
    pub const AUTO: Self = Self::new(1);

    /// 720p.
    pub const FULL: Self = Self::new(2);

    pub const fn name(self) -> &'static str {
        match self {
            Self::AUTO => "Auto",
            Self::FULL => "Full",
            _ => "UNKNOWN",
        }
    }

    /// Create a new video quality mode from a dynamic value.
    ///
    /// The provided value isn't validated. Known valid values are associated
    /// constants such as [`AUTO`][`Self::AUTO`].
    pub const fn new(video_quality_mode: u8) -> Self {
        Self(video_quality_mode)
    }

    /// Retrieve the value of the video quality mode.
    ///
    /// # Examples
    ///
    /// ```
    /// use twilight_model::channel::VideoQualityMode;
    ///
    /// assert_eq!(2, VideoQualityMode::FULL.get());
    /// ```
    pub const fn get(&self) -> u8 {
        self.0
    }
}

impl From<u8> for VideoQualityMode {
    fn from(value: u8) -> Self {
        Self(value)
    }
}

impl From<VideoQualityMode> for u8 {
    fn from(value: VideoQualityMode) -> Self {
        value.get()
    }
}

#[cfg(test)]
mod tests {
    use super::VideoQualityMode;
    use serde_test::Token;

    const MAP: &[(VideoQualityMode, u8)] =
        &[(VideoQualityMode::AUTO, 1), (VideoQualityMode::FULL, 2)];

    #[test]
    fn variants() {
        for (kind, num) in MAP {
            serde_test::assert_tokens(
                kind,
                &[
                    Token::NewtypeStruct {
                        name: "VideoQualityMode",
                    },
                    Token::U8(*num),
                ],
            );
            assert_eq!(*kind, VideoQualityMode::from(*num));
            assert_eq!(*num, kind.get());
        }
    }
}
