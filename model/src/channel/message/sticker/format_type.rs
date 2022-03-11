use serde_repr::{Deserialize_repr, Serialize_repr};
use std::{
    error::Error,
    fmt::{Display, Formatter, Result as FmtResult},
};

/// Format type of a [`Sticker`].
///
/// [`Sticker`]: super::Sticker
#[derive(Clone, Copy, Debug, Deserialize_repr, Eq, Hash, PartialEq, Serialize_repr)]
#[repr(u8)]
pub enum StickerFormatType {
    /// Sticker format is a PNG.
    Png = 1,
    /// Sticker format is an APNG.
    Apng = 2,
    /// Sticker format is a LOTTIE.
    Lottie = 3,
}

impl TryFrom<u8> for StickerFormatType {
    type Error = StickerFormatTypeConversionError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Ok(match value {
            1 => StickerFormatType::Png,
            2 => StickerFormatType::Apng,
            3 => StickerFormatType::Lottie,
            _ => return Err(StickerFormatTypeConversionError { value }),
        })
    }
}

/// Converting into a [`StickerFormatType`] failed.
///
/// This occurs only when the input value doesn't map to a sticker type variant.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct StickerFormatTypeConversionError {
    value: u8,
}

impl<'a> StickerFormatTypeConversionError {
    /// Retrieve a copy of the input value that couldn't be parsed.
    pub const fn value(&self) -> u8 {
        self.value
    }
}

impl Display for StickerFormatTypeConversionError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        f.write_str("Value (")?;
        Display::fmt(&self.value, f)?;

        f.write_str(") doesn't match a sticker type")
    }
}

impl Error for StickerFormatTypeConversionError {}

#[cfg(test)]
mod tests {
    use super::StickerFormatType;
    use serde_test::Token;

    #[test]
    fn test_variants() {
        serde_test::assert_tokens(&StickerFormatType::Png, &[Token::U8(1)]);
        serde_test::assert_tokens(&StickerFormatType::Apng, &[Token::U8(2)]);
        serde_test::assert_tokens(&StickerFormatType::Lottie, &[Token::U8(3)]);
    }

    #[test]
    fn test_conversions() {
        assert_eq!(
            StickerFormatType::try_from(1).unwrap(),
            StickerFormatType::Png
        );
        assert_eq!(
            StickerFormatType::try_from(2).unwrap(),
            StickerFormatType::Apng
        );
        assert_eq!(
            StickerFormatType::try_from(3).unwrap(),
            StickerFormatType::Lottie
        );
    }
}
