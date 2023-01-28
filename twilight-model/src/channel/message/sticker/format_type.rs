use serde::{Deserialize, Serialize};

/// Format type of a [`Sticker`].
///
/// [`Sticker`]: super::Sticker
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
#[non_exhaustive]
#[serde(from = "u8", into = "u8")]
pub enum StickerFormatType {
    /// Sticker format is a PNG.
    Png,
    /// Sticker format is an APNG.
    Apng,
    /// Sticker format is a LOTTIE.
    Lottie,
    /// Sticker format is a GIF.
    Gif,
    /// Variant value is unknown to the library.
    Unknown(u8),
}

impl From<u8> for StickerFormatType {
    fn from(value: u8) -> Self {
        match value {
            1 => StickerFormatType::Png,
            2 => StickerFormatType::Apng,
            3 => StickerFormatType::Lottie,
            4 => StickerFormatType::Gif,
            unknown => StickerFormatType::Unknown(unknown),
        }
    }
}

impl From<StickerFormatType> for u8 {
    fn from(value: StickerFormatType) -> Self {
        match value {
            StickerFormatType::Png => 1,
            StickerFormatType::Apng => 2,
            StickerFormatType::Lottie => 3,
            StickerFormatType::Gif => 4,
            StickerFormatType::Unknown(unknown) => unknown,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::StickerFormatType;
    use serde_test::Token;

    #[test]
    fn variants() {
        serde_test::assert_tokens(&StickerFormatType::Png, &[Token::U8(1)]);
        serde_test::assert_tokens(&StickerFormatType::Apng, &[Token::U8(2)]);
        serde_test::assert_tokens(&StickerFormatType::Lottie, &[Token::U8(3)]);
        serde_test::assert_tokens(&StickerFormatType::Gif, &[Token::U8(4)]);
        serde_test::assert_tokens(&StickerFormatType::Unknown(99), &[Token::U8(99)]);
    }

    #[test]
    fn conversions() {
        assert_eq!(StickerFormatType::from(1), StickerFormatType::Png);
        assert_eq!(StickerFormatType::from(2), StickerFormatType::Apng);
        assert_eq!(StickerFormatType::from(3), StickerFormatType::Lottie);
        assert_eq!(StickerFormatType::from(4), StickerFormatType::Gif);
        assert_eq!(StickerFormatType::from(99), StickerFormatType::Unknown(99));
    }
}
