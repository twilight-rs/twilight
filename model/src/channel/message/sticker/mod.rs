//! Message stickers.
//!
//! See the [Discord documentation] for more information.
//!
//! [Discord documentation]: https://discord.com/developers/docs/resources/channel#message-object-message-sticker-structure

mod id;
mod kind;
mod message;

pub use self::{
    id::{StickerId, StickerPackId},
    kind::{StickerFormatType, StickerFormatTypeConversionError},
    message::MessageSticker,
};

use crate::{id::GuildId, user::User, util::is_false};
use serde::{Deserialize, Serialize};

/// Message sticker.
#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct Sticker {
    /// Whether the sticker is available.
    #[serde(default, skip_serializing_if = "is_false")]
    pub available: bool,
    /// Description of the sticker.
    pub description: String,
    /// Format type.
    pub format_type: StickerFormatType,
    /// ID of the guild that owns the sticker.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub guild_id: Option<GuildId>,
    /// Unique ID of the sticker.
    pub id: StickerId,
    /// Name of the sticker.
    pub name: String,
    /// Unique ID of the pack the sticker is in.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pack_id: Option<StickerPackId>,
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
    use super::{GuildId, Sticker, StickerFormatType, StickerId, StickerPackId, User};
    use crate::{
        id::UserId,
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
            description: "foo2".to_owned(),
            format_type: StickerFormatType::Png,
            guild_id: None,
            id: StickerId(1),
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
                    len: 5,
                },
                Token::Str("description"),
                Token::Str("foo2"),
                Token::Str("format_type"),
                Token::U8(StickerFormatType::Png as u8),
                Token::Str("id"),
                Token::NewtypeStruct { name: "StickerId" },
                Token::Str("1"),
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
            description: "sticker".into(),
            format_type: StickerFormatType::Png,
            guild_id: Some(GuildId(1)),
            id: StickerId(2),
            name: "stick".into(),
            pack_id: Some(StickerPackId(3)),
            sort_value: Some(1),
            tags: "foo,bar,baz".into(),
            user: Some(User {
                accent_color: None,
                avatar: Some("aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa".to_owned()),
                banner: None,
                bot: false,
                discriminator: 1,
                email: Some("address@example.com".to_owned()),
                flags: Some(UserFlags::EARLY_SUPPORTER | UserFlags::VERIFIED_BOT_DEVELOPER),
                id: UserId(1),
                locale: Some("en-us".to_owned()),
                mfa_enabled: Some(true),
                name: "test".to_owned(),
                premium_type: Some(PremiumType::Nitro),
                public_flags: Some(UserFlags::EARLY_SUPPORTER | UserFlags::VERIFIED_BOT_DEVELOPER),
                system: Some(true),
                verified: Some(true),
            }),
        };

        serde_test::assert_tokens(
            &value,
            &[
                Token::Struct {
                    name: "Sticker",
                    len: 10,
                },
                Token::Str("available"),
                Token::Bool(true),
                Token::Str("description"),
                Token::Str("sticker"),
                Token::Str("format_type"),
                Token::U8(StickerFormatType::Png as u8),
                Token::Str("guild_id"),
                Token::Some,
                Token::NewtypeStruct { name: "GuildId" },
                Token::Str("1"),
                Token::Str("id"),
                Token::NewtypeStruct { name: "StickerId" },
                Token::Str("2"),
                Token::Str("name"),
                Token::Str("stick"),
                Token::Str("pack_id"),
                Token::Some,
                Token::NewtypeStruct {
                    name: "StickerPackId",
                },
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
                Token::Str("aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"),
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
                Token::NewtypeStruct { name: "UserId" },
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
