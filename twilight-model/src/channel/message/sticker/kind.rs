use serde::{Deserialize, Serialize};

/// Type of a [`Sticker`].
///
/// [`Sticker`]: super::Sticker
#[derive(Clone, Copy, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct StickerType(u8);

impl StickerType {
    /// Official sticker in a pack.
    ///
    /// Part of nitro or in a removed purchasable pack.
    pub const STANDARD: Self = Self::new(1);

    /// Sticker uploaded to a boosted guild for the guild's members.
    pub const GUILD: Self = Self::new(2);

    /// Name of the associated constant.
    ///
    /// Returns `None` if the value doesn't have a defined constant.
    pub const fn name(self) -> Option<&'static str> {
        Some(match self {
            Self::STANDARD => "STANDARD",
            Self::GUILD => "GUILD",
            _ => return None,
        })
    }
}

impl_typed!(StickerType, u8);

#[cfg(test)]
mod tests {
    use super::StickerType;
    use serde_test::Token;

    #[test]
    fn variants() {
        serde_test::assert_tokens(
            &StickerType::STANDARD,
            &[
                Token::NewtypeStruct {
                    name: "StickerType",
                },
                Token::U8(1),
            ],
        );
        serde_test::assert_tokens(
            &StickerType::GUILD,
            &[
                Token::NewtypeStruct {
                    name: "StickerType",
                },
                Token::U8(2),
            ],
        );
        serde_test::assert_tokens(
            &StickerType::new(99),
            &[
                Token::NewtypeStruct {
                    name: "StickerType",
                },
                Token::U8(99),
            ],
        );
    }

    #[test]
    fn conversions() {
        assert_eq!(StickerType::from(1), StickerType::STANDARD);
        assert_eq!(StickerType::from(2), StickerType::GUILD);
        assert_eq!(StickerType::from(99), StickerType::new(99));
    }
}
