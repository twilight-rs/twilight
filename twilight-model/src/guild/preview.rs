use crate::{
    channel::message::Sticker,
    guild::Emoji,
    id::{marker::GuildMarker, Id},
    util::image_hash::ImageHash,
};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct GuildPreview {
    pub approximate_member_count: u64,
    pub approximate_presence_count: u64,
    pub description: Option<String>,
    pub discovery_splash: Option<ImageHash>,
    pub emojis: Vec<Emoji>,
    pub features: Vec<String>,
    pub id: Id<GuildMarker>,
    pub name: String,
    pub icon: Option<ImageHash>,
    pub splash: Option<ImageHash>,
    /// Guild's custom stickers.
    pub stickers: Vec<Sticker>,
}

#[cfg(test)]
mod tests {
    use super::{Emoji, GuildPreview};
    use crate::{id::Id, test::image_hash};
    use serde_test::Token;

    #[test]
    fn guild_preview() {
        let value = GuildPreview {
            approximate_member_count: 1_000,
            approximate_presence_count: 500,
            description: Some("guild description".to_owned()),
            discovery_splash: Some(image_hash::SPLASH),
            emojis: vec![Emoji {
                animated: Some(false),
                available: Some(true),
                id: Some(Id::new(2)),
                managed: Some(false),
                name: Some("test".to_owned()),
                require_colons: Some(true),
                roles: Some(Vec::new()),
                user: None,
            }],
            features: vec!["a feature".to_owned()],
            id: Id::new(1),
            name: "guild name".to_owned(),
            icon: Some(image_hash::ICON),
            splash: Some(image_hash::SPLASH),
            stickers: Vec::new(),
        };

        serde_test::assert_tokens(
            &value,
            &[
                Token::Struct {
                    name: "GuildPreview",
                    len: 11,
                },
                Token::Str("approximate_member_count"),
                Token::U64(1_000),
                Token::Str("approximate_presence_count"),
                Token::U64(500),
                Token::Str("description"),
                Token::Some,
                Token::Str("guild description"),
                Token::Str("discovery_splash"),
                Token::Some,
                Token::Str(image_hash::SPLASH_INPUT),
                Token::Str("emojis"),
                Token::Seq { len: Some(1) },
                Token::Struct {
                    name: "Emoji",
                    len: 6,
                },
                Token::Str("animated"),
                Token::Bool(false),
                Token::Str("available"),
                Token::Bool(true),
                Token::Str("id"),
                Token::NewtypeStruct { name: "Id" },
                Token::Str("2"),
                Token::Str("managed"),
                Token::Bool(false),
                Token::Str("name"),
                Token::Str("test"),
                Token::Str("require_colons"),
                Token::Bool(true),
                Token::StructEnd,
                Token::SeqEnd,
                Token::Str("features"),
                Token::Seq { len: Some(1) },
                Token::Str("a feature"),
                Token::SeqEnd,
                Token::Str("id"),
                Token::NewtypeStruct { name: "Id" },
                Token::Str("1"),
                Token::Str("name"),
                Token::Str("guild name"),
                Token::Str("icon"),
                Token::Some,
                Token::Str(image_hash::ICON_INPUT),
                Token::Str("splash"),
                Token::Some,
                Token::Str(image_hash::SPLASH_INPUT),
                Token::Str("stickers"),
                Token::Seq { len: Some(0) },
                Token::SeqEnd,
                Token::StructEnd,
            ],
        );
    }
}
