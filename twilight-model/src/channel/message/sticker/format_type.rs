use serde::{Deserialize, Serialize};
use std::fmt::{Debug, Formatter, Result as FmtResult};

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

    /// Create a new sticker format type from a dynamic value.
    ///
    /// The provided value isn't validated. Known valid values are associated
    /// constants such as [`LOTTIE`][`Self::LOTTIE`].
    pub const fn new(command_type: u8) -> Self {
        Self(command_type)
    }

    /// Retrieve the value of the sticker format type.
    ///
    /// # Examples
    ///
    /// ```
    /// use twilight_model::channel::message::sticker::StickerFormatType;
    ///
    /// assert_eq!(1, StickerFormatType::PNG.get());
    /// ```
    pub const fn get(&self) -> u8 {
        self.0
    }

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

impl Debug for StickerFormatType {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        if let Some(name) = self.name() {
            f.debug_struct("StickerFormatType")
                .field("name", &name)
                .field("value", &self.0)
                .finish()
        } else {
            f.debug_tuple("StickerFormatType").field(&self.0).finish()
        }
    }
}

impl From<u8> for StickerFormatType {
    fn from(value: u8) -> Self {
        Self(value)
    }
}

impl From<StickerFormatType> for u8 {
    fn from(value: StickerFormatType) -> Self {
        value.get()
    }
}

#[cfg(test)]
mod tests {
    use super::StickerFormatType;
    use serde_test::Token;

    const MAP: &[(StickerFormatType, u8)] = &[
        (StickerFormatType::PNG, 1),
        (StickerFormatType::APNG, 2),
        (StickerFormatType::LOTTIE, 3),
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
