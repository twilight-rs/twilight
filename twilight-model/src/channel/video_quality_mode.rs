use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct VideoQualityMode(u8);

impl VideoQualityMode {
    /// Discord chooses the quality for optimal performance.
    pub const AUTO: Self = Self::new(1);

    /// 720p.
    pub const FULL: Self = Self::new(2);

    /// Name of the associated constant.
    ///
    /// Returns `None` if the value doesn't have a defined constant.
    pub const fn name(self) -> Option<&'static str> {
        Some(match self {
            Self::AUTO => "AUTO",
            Self::FULL => "FULL",
            _ => return None,
        })
    }
}

impl_typed!(VideoQualityMode, u8);

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
