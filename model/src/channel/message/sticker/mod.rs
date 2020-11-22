//! Message stickers.
//!
//! See the [Discord documentation] for more information.
//!
//! [Discord documentation]: https://discord.com/developers/docs/resources/channel#message-object-message-sticker-structure

mod id;
mod kind;

pub use self::{
    id::{StickerId, StickerPackId},
    kind::{StickerFormatType, StickerFormatTypeConversionError},
};

use serde::{Deserialize, Serialize};

/// Message sticker.
#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct Sticker {
    /// Hash of the asset.
    pub asset: String,
    /// Description of the sticker.
    pub description: String,
    /// Format type.
    pub format_type: StickerFormatType,
    /// Unique ID of the sticker.
    pub id: StickerId,
    /// Name of the sticker.
    pub name: String,
    /// Unique ID of the pack the sticker is in.
    pub pack_id: StickerPackId,
    /// Hash of the preview asset, if it has one.
    pub preview_asset: Option<String>,
    /// CSV list of tags the sticker is assigned to, if any.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::{Sticker, StickerFormatType, StickerId, StickerPackId};
    use serde_test::Token;

    #[test]
    fn test_minimal() {
        let value = Sticker {
            asset: "foo1".to_owned(),
            description: "foo2".to_owned(),
            format_type: StickerFormatType::Png,
            id: StickerId(1),
            name: "sticker name".to_owned(),
            pack_id: StickerPackId(2),
            preview_asset: None,
            tags: Some("foo,bar,baz".to_owned()),
        };

        serde_test::assert_tokens(
            &value,
            &[
                Token::Struct {
                    name: "Sticker",
                    len: 8,
                },
                Token::Str("asset"),
                Token::Str("foo1"),
                Token::Str("description"),
                Token::Str("foo2"),
                Token::Str("format_type"),
                Token::U8(1),
                Token::Str("id"),
                Token::NewtypeStruct { name: "StickerId" },
                Token::Str("1"),
                Token::Str("name"),
                Token::Str("sticker name"),
                Token::Str("pack_id"),
                Token::NewtypeStruct {
                    name: "StickerPackId",
                },
                Token::Str("2"),
                Token::Str("preview_asset"),
                Token::None,
                Token::Str("tags"),
                Token::Some,
                Token::Str("foo,bar,baz"),
                Token::StructEnd,
            ],
        );
    }
}
