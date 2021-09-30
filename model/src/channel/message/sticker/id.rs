use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter, Result as FmtResult};

/// Unique ID denoting a sticker.
///
/// # serde
///
/// Like all of the IDs in the primary [`crate::id`] crate, these
/// IDs support deserializing from both integers and strings and serialize into
/// strings.
#[derive(
    Clone, Copy, Debug, Default, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize,
)]
pub struct StickerId(#[serde(with = "crate::id::string")] pub u64);

impl Display for StickerId {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        Display::fmt(&self.0, f)
    }
}

/// Unique ID denoting a sticker pack.
///
/// # serde
///
/// Like all of the IDs in the primary [`crate::id`] crate, these
/// IDs support deserializing from both integers and strings and serialize into
/// strings.
#[derive(
    Clone, Copy, Debug, Default, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize,
)]
pub struct StickerPackId(#[serde(with = "crate::id::string")] pub u64);

impl Display for StickerPackId {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        Display::fmt(&self.0, f)
    }
}

/// Unique ID denoting a sticker pack SKU.
///
/// # serde
///
/// Like all of the IDs in the primary [`crate::id`] crate, these
/// IDs support deserializing from both integers and strings and serialize into
/// strings.
#[derive(
    Clone, Copy, Debug, Default, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize,
)]
pub struct StickerPackSkuId(#[serde(with = "crate::id::string")] pub u64);

impl Display for StickerPackSkuId {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        Display::fmt(&self.0, f)
    }
}

/// Unique ID denoting a sticker pack banner image.
///
/// # serde
///
/// Like all of the IDs in the primary [`crate::id`] crate, these
/// IDs support deserializing from both integers and strings and serialize into
/// strings.
#[derive(
    Clone, Copy, Debug, Default, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize,
)]
pub struct StickerBannerAssetId(#[serde(with = "crate::id::string")] pub u64);

impl Display for StickerBannerAssetId {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        Display::fmt(&self.0, f)
    }
}

#[cfg(test)]
mod tests {
    use super::{StickerId, StickerPackId};
    use serde_test::Token;

    #[test]
    fn test_id_deser() {
        serde_test::assert_tokens(
            &StickerId(114_941_315_417_899_012),
            &[
                Token::NewtypeStruct { name: "StickerId" },
                Token::Str("114941315417899012"),
            ],
        );
        serde_test::assert_de_tokens(
            &StickerId(114_941_315_417_899_012),
            &[
                Token::NewtypeStruct { name: "StickerId" },
                Token::U64(114_941_315_417_899_012),
            ],
        );
        serde_test::assert_tokens(
            &StickerPackId(114_941_315_417_899_012),
            &[
                Token::NewtypeStruct {
                    name: "StickerPackId",
                },
                Token::Str("114941315417899012"),
            ],
        );
        serde_test::assert_de_tokens(
            &StickerPackId(114_941_315_417_899_012),
            &[
                Token::NewtypeStruct {
                    name: "StickerPackId",
                },
                Token::U64(114_941_315_417_899_012),
            ],
        );
    }
}
