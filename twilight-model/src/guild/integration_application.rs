use crate::{
    id::{marker::ApplicationMarker, Id},
    user::User,
    util::image_hash::ImageHash,
};

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct IntegrationApplication {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bot: Option<User>,
    pub description: String,
    pub icon: Option<ImageHash>,
    pub id: Id<ApplicationMarker>,
    pub name: String,
}

#[cfg(test)]
mod tests {
    use super::{IntegrationApplication, User};
    use crate::id::Id;
    use serde_test::Token;

    #[test]
    fn integration_account() {
        let value = IntegrationApplication {
            bot: None,
            description: "Friendship is Magic".to_string(),
            icon: None,
            id: Id::new(123),
            name: "Twilight".to_string(),
        };

        serde_test::assert_tokens(
            &value,
            &[
                Token::Struct {
                    name: "IntegrationApplication",
                    len: 4,
                },
                Token::Str("description"),
                Token::Str("Friendship is Magic"),
                Token::Str("icon"),
                Token::None,
                Token::Str("id"),
                Token::NewtypeStruct { name: "Id" },
                Token::Str("123"),
                Token::Str("name"),
                Token::Str("Twilight"),
                Token::StructEnd,
            ],
        );
    }

    #[test]
    fn integration_account_complete() {
        let value = IntegrationApplication {
            bot: Some(User {
                accent_color: None,
                avatar: None,
                banner: None,
                bot: false,
                discriminator: 1,
                email: None,
                flags: None,
                id: Id::new(2),
                locale: None,
                mfa_enabled: None,
                name: "test".to_owned(),
                premium_type: None,
                public_flags: None,
                system: None,
                verified: None,
            }),
            description: "Friendship is Magic".to_string(),
            icon: None,
            id: Id::new(123),
            name: "Twilight".to_string(),
        };

        serde_test::assert_tokens(
            &value,
            &[
                Token::Struct {
                    name: "IntegrationApplication",
                    len: 5,
                },
                Token::Str("bot"),
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
                Token::Str("2"),
                Token::Str("username"),
                Token::Str("test"),
                Token::StructEnd,
                Token::Str("description"),
                Token::Str("Friendship is Magic"),
                Token::Str("icon"),
                Token::None,
                Token::Str("id"),
                Token::NewtypeStruct { name: "Id" },
                Token::Str("123"),
                Token::Str("name"),
                Token::Str("Twilight"),
                Token::StructEnd,
            ],
        );
    }
}
