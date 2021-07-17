use crate::{id::ApplicationId, user::User};

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct IntegrationApplication {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bot: Option<User>,
    pub description: String,
    pub icon: Option<String>,
    pub id: ApplicationId,
    pub name: String,
    pub summary: String,
}

#[cfg(test)]
mod tests {
    use super::{IntegrationApplication, User};
    use crate::id::{ApplicationId, UserId};
    use serde_test::Token;
    use std::num::NonZeroU64;

    #[test]
    fn test_integration_account() {
        let value = IntegrationApplication {
            bot: None,
            description: "Friendship is Magic".to_string(),
            icon: None,
            id: ApplicationId(NonZeroU64::new(123).expect("non zero")),
            name: "Twilight".to_string(),
            summary: "A cool pony".to_string(),
        };

        serde_test::assert_tokens(
            &value,
            &[
                Token::Struct {
                    name: "IntegrationApplication",
                    len: 5,
                },
                Token::Str("description"),
                Token::Str("Friendship is Magic"),
                Token::Str("icon"),
                Token::None,
                Token::Str("id"),
                Token::NewtypeStruct {
                    name: "ApplicationId",
                },
                Token::Str("123"),
                Token::Str("name"),
                Token::Str("Twilight"),
                Token::Str("summary"),
                Token::Str("A cool pony"),
                Token::StructEnd,
            ],
        );
    }

    #[test]
    fn test_integration_account_complete() {
        let value = IntegrationApplication {
            bot: Some(User {
                avatar: None,
                bot: false,
                discriminator: "0001".to_owned(),
                email: None,
                flags: None,
                id: UserId(NonZeroU64::new(2).expect("non zero")),
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
            id: ApplicationId(NonZeroU64::new(123).expect("non zero")),
            name: "Twilight".to_string(),
            summary: "A cool pony".to_string(),
        };

        serde_test::assert_tokens(
            &value,
            &[
                Token::Struct {
                    name: "IntegrationApplication",
                    len: 6,
                },
                Token::Str("bot"),
                Token::Some,
                Token::Struct {
                    name: "User",
                    len: 5,
                },
                Token::Str("avatar"),
                Token::None,
                Token::Str("bot"),
                Token::Bool(false),
                Token::Str("discriminator"),
                Token::Str("0001"),
                Token::Str("id"),
                Token::NewtypeStruct { name: "UserId" },
                Token::Str("2"),
                Token::Str("username"),
                Token::Str("test"),
                Token::StructEnd,
                Token::Str("description"),
                Token::Str("Friendship is Magic"),
                Token::Str("icon"),
                Token::None,
                Token::Str("id"),
                Token::NewtypeStruct {
                    name: "ApplicationId",
                },
                Token::Str("123"),
                Token::Str("name"),
                Token::Str("Twilight"),
                Token::Str("summary"),
                Token::Str("A cool pony"),
                Token::StructEnd,
            ],
        );
    }
}
