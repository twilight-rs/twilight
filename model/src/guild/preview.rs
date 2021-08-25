use crate::{guild::Emoji, id::GuildId};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct GuildPreview {
    pub approximate_member_count: u64,
    pub approximate_presence_count: u64,
    pub description: Option<String>,
    pub discovery_splash: Option<String>,
    pub emojis: Vec<Emoji>,
    pub features: Vec<String>,
    pub id: GuildId,
    pub name: String,
    pub icon: Option<String>,
    pub splash: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::{Emoji, GuildId, GuildPreview};
    use crate::id::EmojiId;
    use serde_test::Token;

    #[test]
    fn test_guild_preview() {
        let value = GuildPreview {
            approximate_member_count: 1_000,
            approximate_presence_count: 500,
            description: Some("guild description".to_owned()),
            discovery_splash: Some("discovery splash hash".to_owned()),
            emojis: vec![Emoji {
                animated: false,
                available: true,
                id: EmojiId::new(2).expect("non zero"),
                managed: false,
                name: "test".to_owned(),
                require_colons: true,
                roles: Vec::new(),
                user: None,
            }],
            features: vec!["a feature".to_owned()],
            id: GuildId::new(1).expect("non zero"),
            name: "guild name".to_owned(),
            icon: Some("icon hash".to_owned()),
            splash: Some("splash hash".to_owned()),
        };

        serde_test::assert_tokens(
            &value,
            &[
                Token::Struct {
                    name: "GuildPreview",
                    len: 10,
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
                Token::Str("discovery splash hash"),
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
                Token::NewtypeStruct { name: "EmojiId" },
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
                Token::NewtypeStruct { name: "GuildId" },
                Token::Str("1"),
                Token::Str("name"),
                Token::Str("guild name"),
                Token::Str("icon"),
                Token::Some,
                Token::Str("icon hash"),
                Token::Str("splash"),
                Token::Some,
                Token::Str("splash hash"),
                Token::StructEnd,
            ],
        );
    }
}
