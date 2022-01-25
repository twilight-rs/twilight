//! Message stickers.
//!
//! See the [Discord Docs/Sticker Object] for more information.
//!
//! [Discord Docs/Sticker Object]: https://discord.com/developers/docs/resources/sticker#sticker-object-sticker-structure

mod format_type;
mod kind;
mod message;
mod pack;

pub use self::{
    format_type::{StickerFormatType, StickerFormatTypeConversionError},
    kind::{StickerType, StickerTypeConversionError},
    message::MessageSticker,
    pack::StickerPack,
};

use crate::{
    id::{
        marker::{GuildMarker, StickerMarker, StickerPackMarker},
        Id,
    },
    user::User,
    util::is_false,
};
use serde::{Deserialize, Serialize};

/// Message sticker.
#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct Sticker {
    /// Whether the sticker is available.
    #[serde(default, skip_serializing_if = "is_false")]
    pub available: bool,
    /// Description of the sticker.
    pub description: Option<String>,
    /// Format type.
    pub format_type: StickerFormatType,
    /// ID of the guild that owns the sticker.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub guild_id: Option<Id<GuildMarker>>,
    /// Unique ID of the sticker.
    pub id: Id<StickerMarker>,
    /// Kind of sticker.
    #[serde(rename = "type")]
    pub kind: StickerType,
    /// Name of the sticker.
    pub name: String,
    /// Unique ID of the pack the sticker is in.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pack_id: Option<Id<StickerPackMarker>>,
    /// Sticker's sort order within a pack.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_value: Option<u64>,
    /// CSV list of tags the sticker is assigned to, if any.
    pub tags: String,
    /// ID of the user that uploaded the sticker.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<User>,
}

#[cfg(test)]
mod tests {
    use super::{Sticker, StickerFormatType, StickerType, User};
    use crate::{
        id::Id,
        test::image_hash,
        user::{PremiumType, UserFlags},
    };
    use serde::{Deserialize, Serialize};
    use serde_test::Token;
    use static_assertions::{assert_fields, assert_impl_all};
    use std::{fmt::Debug, hash::Hash};

    assert_fields!(
        Sticker: available,
        description,
        format_type,
        guild_id,
        id,
        kind,
        name,
        pack_id,
        sort_value,
        tags,
        user
    );

    assert_impl_all!(
        Sticker: Clone,
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
    fn test_minimal() {
        let value = Sticker {
            available: false,
            description: Some("foo2".to_owned()),
            format_type: StickerFormatType::Png,
            guild_id: None,
            id: Id::new(1),
            kind: StickerType::Standard,
            name: "sticker name".to_owned(),
            pack_id: None,
            sort_value: None,
            tags: "foo,bar,baz".to_owned(),
            user: None,
        };

        serde_test::assert_tokens(
            &value,
            &[
                Token::Struct {
                    name: "Sticker",
                    len: 6,
                },
                Token::Str("description"),
                Token::Some,
                Token::Str("foo2"),
                Token::Str("format_type"),
                Token::U8(StickerFormatType::Png as u8),
                Token::Str("id"),
                Token::NewtypeStruct { name: "Id" },
                Token::Str("1"),
                Token::Str("type"),
                Token::U8(1),
                Token::Str("name"),
                Token::Str("sticker name"),
                Token::Str("tags"),
                Token::Str("foo,bar,baz"),
                Token::StructEnd,
            ],
        );
    }

    #[allow(clippy::too_many_lines)]
    #[test]
    fn test_full() {
        let value = Sticker {
            available: true,
            description: Some("sticker".into()),
            format_type: StickerFormatType::Png,
            guild_id: Some(Id::new(1)),
            id: Id::new(2),
            kind: StickerType::Guild,
            name: "stick".into(),
            pack_id: Some(Id::new(3)),
            sort_value: Some(1),
            tags: "foo,bar,baz".into(),
            user: Some(User {
                accent_color: None,
                avatar: Some(image_hash::AVATAR),
                banner: None,
                bot: false,
                discriminator: 1,
                email: Some("address@example.com".to_owned()),
                flags: Some(UserFlags::PREMIUM_EARLY_SUPPORTER | UserFlags::VERIFIED_DEVELOPER),
                id: Id::new(1),
                locale: Some("en-us".to_owned()),
                mfa_enabled: Some(true),
                name: "test".to_owned(),
                premium_type: Some(PremiumType::Nitro),
                public_flags: Some(
                    UserFlags::PREMIUM_EARLY_SUPPORTER | UserFlags::VERIFIED_DEVELOPER,
                ),
                system: Some(true),
                verified: Some(true),
            }),
        };

        serde_test::assert_tokens(
            &value,
            &[
                Token::Struct {
                    name: "Sticker",
                    len: 11,
                },
                Token::Str("available"),
                Token::Bool(true),
                Token::Str("description"),
                Token::Some,
                Token::Str("sticker"),
                Token::Str("format_type"),
                Token::U8(StickerFormatType::Png as u8),
                Token::Str("guild_id"),
                Token::Some,
                Token::NewtypeStruct { name: "Id" },
                Token::Str("1"),
                Token::Str("id"),
                Token::NewtypeStruct { name: "Id" },
                Token::Str("2"),
                Token::Str("type"),
                Token::U8(2),
                Token::Str("name"),
                Token::Str("stick"),
                Token::Str("pack_id"),
                Token::Some,
                Token::NewtypeStruct { name: "Id" },
                Token::Str("3"),
                Token::Str("sort_value"),
                Token::Some,
                Token::U64(1),
                Token::Str("tags"),
                Token::Str("foo,bar,baz"),
                Token::Str("user"),
                Token::Some,
                Token::Struct {
                    name: "User",
                    len: 15,
                },
                Token::Str("accent_color"),
                Token::None,
                Token::Str("avatar"),
                Token::Some,
                Token::Str(image_hash::AVATAR_INPUT),
                Token::Str("banner"),
                Token::None,
                Token::Str("bot"),
                Token::Bool(false),
                Token::Str("discriminator"),
                Token::Str("0001"),
                Token::Str("email"),
                Token::Some,
                Token::Str("address@example.com"),
                Token::Str("flags"),
                Token::Some,
                Token::U64(131_584),
                Token::Str("id"),
                Token::NewtypeStruct { name: "Id" },
                Token::Str("1"),
                Token::Str("locale"),
                Token::Some,
                Token::Str("en-us"),
                Token::Str("mfa_enabled"),
                Token::Some,
                Token::Bool(true),
                Token::Str("username"),
                Token::Str("test"),
                Token::Str("premium_type"),
                Token::Some,
                Token::U8(2),
                Token::Str("public_flags"),
                Token::Some,
                Token::U64(131_584),
                Token::Str("system"),
                Token::Some,
                Token::Bool(true),
                Token::Str("verified"),
                Token::Some,
                Token::Bool(true),
                Token::StructEnd,
                Token::StructEnd,
            ],
        );
    }
}
