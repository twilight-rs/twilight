use crate::{
    id::{
        marker::{EmojiMarker, RoleMarker},
        Id,
    },
    user::User,
};
use serde::{Deserialize, Serialize};

#[allow(clippy::struct_excessive_bools)]
#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct Emoji {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub animated: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub available: Option<bool>,
    pub id: Option<Id<EmojiMarker>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub managed: Option<bool>,
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub require_colons: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub roles: Option<Vec<Id<RoleMarker>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<User>,
}

#[cfg(test)]
mod tests {
    use super::{Emoji, User};
    use crate::id::Id;
    use serde_test::Token;

    #[test]
    fn emoji() {
        let emoji = Emoji {
            animated: Some(false),
            available: Some(true),
            id: Some(Id::new(100_000_000_000_000_000)),
            managed: Some(false),
            name: Some("test".to_owned()),
            require_colons: Some(true),
            roles: None,
            user: Some(User {
                accent_color: None,
                avatar: None,
                banner: None,
                bot: false,
                discriminator: 1,
                email: None,
                flags: None,
                id: Id::new(1),
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
                Token::Some,
                Token::Bool(false),
                Token::Str("available"),
                Token::Some,
                Token::Bool(true),
                Token::Str("id"),
                Token::Some,
                Token::NewtypeStruct { name: "Id" },
                Token::Str("100000000000000000"),
                Token::Str("managed"),
                Token::Some,
                Token::Bool(false),
                Token::Str("name"),
                Token::Some,
                Token::Str("test"),
                Token::Str("require_colons"),
                Token::Some,
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
                Token::NewtypeStruct { name: "Id" },
                Token::Str("1"),
                Token::Str("username"),
                Token::Str("test"),
                Token::StructEnd,
                Token::StructEnd,
            ],
        )
    }

    #[test]
    fn emoji_complete() {
        let emoji = Emoji {
            animated: Some(false),
            available: Some(true),
            id: Some(Id::new(100_000_000_000_000_000)),
            managed: Some(false),
            name: Some("test".to_owned()),
            require_colons: Some(true),
            roles: Some(vec![Id::new(1)]),
            user: Some(User {
                accent_color: None,
                avatar: None,
                banner: None,
                bot: false,
                discriminator: 1,
                email: None,
                flags: None,
                id: Id::new(1),
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
                Token::Some,
                Token::Bool(false),
                Token::Str("available"),
                Token::Some,
                Token::Bool(true),
                Token::Str("id"),
                Token::Some,
                Token::NewtypeStruct { name: "Id" },
                Token::Str("100000000000000000"),
                Token::Str("managed"),
                Token::Some,
                Token::Bool(false),
                Token::Str("name"),
                Token::Some,
                Token::Str("test"),
                Token::Str("require_colons"),
                Token::Some,
                Token::Bool(true),
                Token::Str("roles"),
                Token::Some,
                Token::Seq { len: Some(1) },
                Token::NewtypeStruct { name: "Id" },
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
                Token::NewtypeStruct { name: "Id" },
                Token::Str("1"),
                Token::Str("username"),
                Token::Str("test"),
                Token::StructEnd,
                Token::StructEnd,
            ],
        )
    }
}
