use crate::{id::ApplicationId, user::User};

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct IntegrationApplication {
    pub bot: Option<User>,
    pub description: String,
    pub icon: Option<String>,
    pub id: ApplicationId,
    pub name: String,
    pub summary: String,
}

#[cfg(test)]
mod tests {
    use super::IntegrationApplication;
    use crate::id::ApplicationId;
    use serde_test::Token;

    #[test]
    fn test_integration_account() {
        let value = IntegrationApplication {
            bot: None,
            description: "Friendship is Magic".to_string(),
            icon: None,
            id: ApplicationId(123),
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
                Token::None,
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
