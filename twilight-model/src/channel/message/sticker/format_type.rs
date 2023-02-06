use serde::{Deserialize, Serialize};

/// Format type of a [`Sticker`].
///
/// [`Sticker`]: super::Sticker
#[derive(Clone, Copy, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct StickerFormatType(u8);

impl StickerFormatType {
    /// Sticker format is a PNG.
    pub const PNG: Self = Self::new(1);

    /// Sticker format is an APNG.
    pub const APNG: Self = Self::new(2);

    /// Sticker format is a LOTTIE.
    pub const LOTTIE: Self = Self::new(3);

    /// Sticker format is a GIF.
    pub const GIF: Self = Self::new(4);

    /// Name of the associated constant.
    ///
    /// Returns `None` if the value doesn't have a defined constant.
    pub const fn name(self) -> Option<&'static str> {
        Some(match self {
            Self::APNG => "APNG",
            Self::LOTTIE => "LOTTIE",
            Self::PNG => "PNG",
            _ => return None,
        })
    }
}

impl_typed!(StickerFormatType, u8);

#[cfg(test)]
mod tests {
    use super::StickerFormatType;
    use serde_test::Token;

    const MAP: &[(StickerFormatType, u8)] = &[
        (StickerFormatType::PNG, 1),
        (StickerFormatType::APNG, 2),
        (StickerFormatType::LOTTIE, 3),
        (StickerFormatType::GIF, 4),
    ];

    #[test]
    fn variants() {
        for (kind, num) in MAP {
            serde_test::assert_tokens(
                kind,
                &[
                    Token::NewtypeStruct {
                        name: "StickerFormatType",
                    },
                    Token::U8(*num),
                ],
            );
            assert_eq!(*kind, StickerFormatType::from(*num));
            assert_eq!(*num, kind.get());
        }
    }
}
