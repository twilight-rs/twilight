use crate::{
    id::{EmojiId, RoleId},
    user::User,
};
use serde::{Deserialize, Serialize};

#[allow(clippy::struct_excessive_bools)]
#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct Emoji {
    #[serde(default)]
    pub animated: bool,
    #[serde(default)]
    pub available: bool,
    // This does not need to be optional here as it can only be optional
    // in a unicode emoji. Which can only happen in reactions, and we use
    // another struct for emojis in that case.
    pub id: EmojiId,
    #[serde(default)]
    pub managed: bool,
    pub name: String,
    #[serde(default)]
    pub require_colons: bool,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub roles: Vec<RoleId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<User>,
}

#[cfg(test)]
mod tests {
    use super::{Emoji, EmojiId, RoleId, User};
    use crate::id::UserId;
    use serde_test::Token;

    #[test]
    fn test_emoji() {
        let emoji = Emoji {
            animated: false,
            available: true,
            id: EmojiId::new(100_000_000_000_000_000).expect("non zero"),
            managed: false,
            name: "test".to_owned(),
            require_colons: true,
            roles: Vec::new(),
            user: Some(User {
                accent_color: None,
                avatar: None,
                banner: None,
                bot: false,
                discriminator: 1,
                email: None,
                flags: None,
                id: UserId::new(1).expect("non zero"),
                locale: None,
                mfa_enabled: None,
                name: "test".to_owned(),
                premium_type: None,
                public_flags: None,
                system: None,
                verified: None,
            }),
        };

        serde_test::assert_tokens(
            &emoji,
            &[
                Token::Struct {
                    name: "Emoji",
                    len: 7,
                },
                Token::Str("animated"),
                Token::Bool(false),
                Token::Str("available"),
                Token::Bool(true),
                Token::Str("id"),
                Token::NewtypeStruct { name: "EmojiId" },
                Token::Str("100000000000000000"),
                Token::Str("managed"),
                Token::Bool(false),
                Token::Str("name"),
                Token::Str("test"),
                Token::Str("require_colons"),
                Token::Bool(true),
                Token::Str("user"),
                Token::Some,
                Token::Struct {
                    name: "User",
                    len: 7,
                },
                Token::Str("accent_color"),
                Token::None,
                Token::Str("avatar"),
                Token::None,
                Token::Str("banner"),
                Token::None,
                Token::Str("bot"),
                Token::Bool(false),
                Token::Str("discriminator"),
                Token::Str("0001"),
                Token::Str("id"),
                Token::NewtypeStruct { name: "UserId" },
                Token::Str("1"),
                Token::Str("username"),
                Token::Str("test"),
                Token::StructEnd,
                Token::StructEnd,
            ],
        )
    }

    #[test]
    fn test_emoji_complete() {
        let emoji = Emoji {
            animated: false,
            available: true,
            id: EmojiId::new(100_000_000_000_000_000).expect("non zero"),
            managed: false,
            name: "test".to_owned(),
            require_colons: true,
            roles: vec![RoleId::new(1).expect("non zero")],
            user: Some(User {
                accent_color: None,
                avatar: None,
                banner: None,
                bot: false,
                discriminator: 1,
                email: None,
                flags: None,
                id: UserId::new(1).expect("non zero"),
                locale: None,
                mfa_enabled: None,
                name: "test".to_owned(),
                premium_type: None,
                public_flags: None,
                system: None,
                verified: None,
            }),
        };

        serde_test::assert_tokens(
            &emoji,
            &[
                Token::Struct {
                    name: "Emoji",
                    len: 8,
                },
                Token::Str("animated"),
                Token::Bool(false),
                Token::Str("available"),
                Token::Bool(true),
                Token::Str("id"),
                Token::NewtypeStruct { name: "EmojiId" },
                Token::Str("100000000000000000"),
                Token::Str("managed"),
                Token::Bool(false),
                Token::Str("name"),
                Token::Str("test"),
                Token::Str("require_colons"),
                Token::Bool(true),
                Token::Str("roles"),
                Token::Seq { len: Some(1) },
                Token::NewtypeStruct { name: "RoleId" },
                Token::Str("1"),
                Token::SeqEnd,
                Token::Str("user"),
                Token::Some,
                Token::Struct {
                    name: "User",
                    len: 7,
                },
                Token::Str("accent_color"),
                Token::None,
                Token::Str("avatar"),
                Token::None,
                Token::Str("banner"),
                Token::None,
                Token::Str("bot"),
                Token::Bool(false),
                Token::Str("discriminator"),
                Token::Str("0001"),
                Token::Str("id"),
                Token::NewtypeStruct { name: "UserId" },
                Token::Str("1"),
                Token::Str("username"),
                Token::Str("test"),
                Token::StructEnd,
                Token::StructEnd,
            ],
        )
    }
}
