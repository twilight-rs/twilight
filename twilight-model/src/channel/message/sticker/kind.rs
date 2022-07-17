use serde::{Deserialize, Serialize};

/// Type of a [`Sticker`].
///
/// [`Sticker`]: super::Sticker
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
#[serde(from = "u8", into = "u8")]
pub enum StickerType {
    /// Official sticker in a pack.
    ///
    /// Part of nitro or in a removed purchasable pack.
    Standard,
    /// Sticker uploaded to a boosted guild for the guild's members.
    Guild,
    /// Variant value is unknown to the library.
    Unknown(u8),
}

impl From<u8> for StickerType {
    fn from(value: u8) -> Self {
        match value {
            1 => StickerType::Standard,
            2 => StickerType::Guild,
            unknown => StickerType::Unknown(unknown),
        }
    }
}

impl From<StickerType> for u8 {
    fn from(value: StickerType) -> Self {
        match value {
            StickerType::Standard => 1,
            StickerType::Guild => 2,
            StickerType::Unknown(unknown) => unknown,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::StickerType;
    use serde_test::Token;

    #[test]
    fn variants() {
        serde_test::assert_tokens(&StickerType::Standard, &[Token::U8(1)]);
        serde_test::assert_tokens(&StickerType::Guild, &[Token::U8(2)]);
        serde_test::assert_tokens(&StickerType::Unknown(99), &[Token::U8(99)]);
    }

    #[test]
    fn conversions() {
        assert_eq!(StickerType::from(1), StickerType::Standard);
        assert_eq!(StickerType::from(2), StickerType::Guild);
        assert_eq!(StickerType::from(99), StickerType::Unknown(99));
    }
}
