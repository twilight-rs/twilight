use super::{PremiumType, UserFlags};
use crate::id::UserId;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct CurrentUser {
    pub avatar: Option<String>,
    #[serde(default)]
    pub bot: bool,
    /// Discriminator used to differentiate people with the same username.
    ///
    /// # serde
    ///
    /// The discriminator field can be deserialized from either a string or an
    /// integer. The field will always serialize into a string due to that being
    /// the type Discord's API uses.
    #[serde(with = "super::discriminator")]
    pub discriminator: String,
    pub email: Option<String>,
    pub id: UserId,
    pub mfa_enabled: bool,
    #[serde(rename = "username")]
    pub name: String,
    /// Type of Nitro subscription on a user's account
    pub premium_type: Option<PremiumType>,
    /// Public flags on a user's account
    pub public_flags: Option<UserFlags>,
    pub verified: bool,
}

#[cfg(test)]
mod tests {
    use super::{CurrentUser, PremiumType, UserFlags, UserId};
    use serde_test::Token;

    fn user_tokens(discriminator_token: Token) -> Vec<Token> {
        vec![
            Token::Struct {
                name: "CurrentUser",
                len: 10,
            },
            Token::Str("avatar"),
            Token::Some,
            Token::Str("avatar hash"),
            Token::Str("bot"),
            Token::Bool(true),
            Token::Str("discriminator"),
            discriminator_token,
            Token::Str("email"),
            Token::None,
            Token::Str("id"),
            Token::NewtypeStruct { name: "UserId" },
            Token::Str("1"),
            Token::Str("mfa_enabled"),
            Token::Bool(true),
            Token::Str("username"),
            Token::Str("test name"),
            Token::Str("premium_type"),
            Token::Some,
            Token::U8(1),
            Token::Str("public_flags"),
            Token::Some,
            Token::U64(1),
            Token::Str("verified"),
            Token::Bool(true),
            Token::StructEnd,
        ]
    }

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
            premium_type: Some(PremiumType::NitroClassic),
            public_flags: Some(UserFlags::DISCORD_EMPLOYEE),
        };

        // Deserializing a current user with a string discriminator (which
        // Discord provides)
        serde_test::assert_tokens(&value, &user_tokens(Token::Str("9999")));

        // Deserializing a current user with an integer discriminator. Userland
        // code may have this due to being a more compact memory representation
        // of a discriminator.
        serde_test::assert_de_tokens(&value, &user_tokens(Token::U64(9999)));
    }
}
