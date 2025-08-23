use super::Sticker;
use crate::id::{
    Id,
    marker::{StickerBannerAssetMarker, StickerMarker, StickerPackMarker, StickerPackSkuMarker},
};
use serde::{Deserialize, Serialize};

/// Pack of [`Standard`] stickers.
///
/// [`Standard`]: super::StickerType::Standard
#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct StickerPack {
    /// ID of the sticker pack's banner image.
    pub banner_asset_id: Option<Id<StickerBannerAssetMarker>>,
    /// ID of the sticker that is shown as the pack's icon.
    pub cover_sticker_id: Option<Id<StickerMarker>>,
    /// Description of the sticker pack.
    pub description: String,
    /// ID of the sticker pack.
    pub id: Id<StickerPackMarker>,
    /// Name of the sticker pack.
    pub name: String,
    /// ID of the pack's SKU.
    pub sku_id: Id<StickerPackSkuMarker>,
    /// List of stickers in the pack.
    pub stickers: Vec<Sticker>,
}

#[cfg(test)]
mod tests {
    use super::{
        super::{StickerFormatType, StickerType},
        Sticker, StickerPack,
    };
    use crate::id::Id;
    use serde::{Deserialize, Serialize};
    use serde_test::Token;
    use static_assertions::{assert_fields, assert_impl_all};
    use std::{fmt::Debug, hash::Hash};

    assert_fields!(
        StickerPack: banner_asset_id,
        cover_sticker_id,
        description,
        id,
        name,
        sku_id,
        stickers
    );

    assert_impl_all!(
        StickerPack: Clone,
        Debug,
        Deserialize<'static>,
        Eq,
        Hash,
        PartialEq,
        Send,
        Serialize,
        Sync,
    );

    #[test]
    fn full() {
        let value = StickerPack {
            banner_asset_id: Some(Id::new(761_773_777_976_819_732)),
            cover_sticker_id: Some(Id::new(749_053_689_419_006_003)),
            description: "Say hello to Wumpus!".into(),
            id: Id::new(847_199_849_233_514_549),
            name: "Wumpus Beyond".into(),
            sku_id: Id::new(847_199_849_233_514_547),
            stickers: Vec::from([Sticker {
                available: true,
                description: Some("Wumpus waves hello".into()),
                format_type: StickerFormatType::Lottie,
                guild_id: None,
                id: Id::new(749_054_660_769_218_631),
                kind: StickerType::Standard,
                name: "Wave".into(),
                pack_id: Some(Id::new(847_199_849_233_514_549)),
                sort_value: Some(12),
                tags: "wumpus, hello, sup, hi, oi, heyo, heya, yo, wave".into(),
                user: None,
            }]),
        };

        serde_test::assert_tokens(
            &value,
            &[
                Token::Struct {
                    name: "StickerPack",
                    len: 7,
                },
                Token::Str("banner_asset_id"),
                Token::Some,
                Token::NewtypeStruct { name: "Id" },
                Token::Str("761773777976819732"),
                Token::Str("cover_sticker_id"),
                Token::Some,
                Token::NewtypeStruct { name: "Id" },
                Token::Str("749053689419006003"),
                Token::Str("description"),
                Token::Str("Say hello to Wumpus!"),
                Token::Str("id"),
                Token::NewtypeStruct { name: "Id" },
                Token::Str("847199849233514549"),
                Token::Str("name"),
                Token::Str("Wumpus Beyond"),
                Token::Str("sku_id"),
                Token::NewtypeStruct { name: "Id" },
                Token::Str("847199849233514547"),
                Token::Str("stickers"),
                Token::Seq { len: Some(1) },
                Token::Struct {
                    name: "Sticker",
                    len: 9,
                },
                Token::Str("available"),
                Token::Bool(true),
                Token::Str("description"),
                Token::Some,
                Token::Str("Wumpus waves hello"),
                Token::Str("format_type"),
                Token::U8(3),
                Token::Str("id"),
                Token::NewtypeStruct { name: "Id" },
                Token::Str("749054660769218631"),
                Token::Str("type"),
                Token::U8(1),
                Token::Str("name"),
                Token::Str("Wave"),
                Token::Str("pack_id"),
                Token::Some,
                Token::NewtypeStruct { name: "Id" },
                Token::Str("847199849233514549"),
                Token::Str("sort_value"),
                Token::Some,
                Token::U64(12),
                Token::Str("tags"),
                Token::Str("wumpus, hello, sup, hi, oi, heyo, heya, yo, wave"),
                Token::StructEnd,
                Token::SeqEnd,
                Token::StructEnd,
            ],
        );
    }
}
