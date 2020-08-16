use crate::id::UserId;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct CurrentUser {
    pub avatar: Option<String>,
    #[serde(default)]
    pub bot: bool,
    pub discriminator: String,
    pub email: Option<String>,
    pub id: UserId,
    pub mfa_enabled: bool,
    #[serde(rename = "username")]
    pub name: String,
    pub verified: bool,
}

#[cfg(test)]
mod tests {
    use super::{CurrentUser, UserId};
    use serde_test::Token;

    #[test]
    fn test_current_user() {
        let value = CurrentUser {
            avatar: Some("avatar hash".to_owned()),
            bot: true,
            discriminator: "9999".to_owned(),
            email: None,
            id: UserId(1),
            mfa_enabled: true,
            name: "test name".to_owned(),
            verified: true,
        };

        serde_test::assert_tokens(
            &value,
            &[
                Token::Struct {
                    name: "CurrentUser",
                    len: 8,
                },
                Token::Str("avatar"),
                Token::Some,
                Token::Str("avatar hash"),
                Token::Str("bot"),
                Token::Bool(true),
                Token::Str("discriminator"),
                Token::Str("9999"),
                Token::Str("email"),
                Token::None,
                Token::Str("id"),
                Token::NewtypeStruct { name: "UserId" },
                Token::Str("1"),
                Token::Str("mfa_enabled"),
                Token::Bool(true),
                Token::Str("username"),
                Token::Str("test name"),
                Token::Str("verified"),
                Token::Bool(true),
                Token::StructEnd,
            ],
        );
    }
}
