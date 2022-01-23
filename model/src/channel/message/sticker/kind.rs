use serde_repr::{Deserialize_repr, Serialize_repr};
use std::{
    error::Error,
    fmt::{Display, Formatter, Result as FmtResult},
};

/// Type of a [`Sticker`].
///
/// [`Sticker`]: super::Sticker
#[derive(
    Clone, Copy, Debug, Deserialize_repr, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize_repr,
)]
#[repr(u8)]
pub enum StickerType {
    /// Official sticker in a pack.
    ///
    /// Part of nitro or in a removed purchasable pack.
    Standard = 1,
    /// Sticker uploaded to a boosted guild for the guild's members.
    Guild = 2,
}

impl TryFrom<u8> for StickerType {
    type Error = StickerTypeConversionError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Ok(match value {
            1 => StickerType::Standard,
            2 => StickerType::Guild,
            _ => return Err(StickerTypeConversionError { value }),
        })
    }
}

/// Converting into a [`StickerType`] failed.
///
/// This occurs only when the input value doesn't map to a sticker type variant.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct StickerTypeConversionError {
    value: u8,
}

impl<'a> StickerTypeConversionError {
    /// Retrieve a copy of the input value that couldn't be parsed.
    pub const fn value(&self) -> u8 {
        self.value
    }
}

impl Display for StickerTypeConversionError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        f.write_str("Value (")?;
        Display::fmt(&self.value, f)?;

        f.write_str(") doesn't match a sticker type")
    }
}

impl Error for StickerTypeConversionError {}

#[cfg(test)]
mod tests {
    use super::StickerType;
    use serde_test::Token;

    #[test]
    fn test_variants() {
        serde_test::assert_tokens(&StickerType::Standard, &[Token::U8(1)]);
        serde_test::assert_tokens(&StickerType::Guild, &[Token::U8(2)]);
    }

    #[test]
    fn test_conversions() {
        assert_eq!(StickerType::try_from(1).unwrap(), StickerType::Standard);
        assert_eq!(StickerType::try_from(2).unwrap(), StickerType::Guild);
    }
}
