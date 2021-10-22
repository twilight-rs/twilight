use super::{DiscriminatorDisplay, PremiumType, UserFlags};
use crate::id::UserId;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct CurrentUser {
    /// Accent color of the user's banner.
    ///
    /// This is an integer representation of a hexadecimal color code.
    pub accent_color: Option<u64>,
    /// User's avatar hash.
    ///
    /// To retrieve the url to the avatar, you can follow [Discord's documentation] on
    /// Image formatting.
    ///
    /// [Discord's documentation]: https://discord.com/developers/docs/reference#image-formatting
    pub avatar: Option<String>,
    /// Hash of the user's banner image.
    pub banner: Option<String>,
    /// Whether the user belongs to an OAuth2 application.
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
    pub discriminator: u16,
    /// User's email address associated to the account.
    ///
    /// Requires the `email` oauth scope. See [Discord's documentation] for
    /// more information.
    ///
    /// [Discord's documentation]: https://discord.com/developers/docs/resources/user#user-object-user-structure
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    /// All flags on a user's account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flags: Option<UserFlags>,
    /// User's id.
    pub id: UserId,
    /// User's chosen language option.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locale: Option<String>,
    /// Whether the user has two factor enabled on their account.
    pub mfa_enabled: bool,
    /// User's username, not unique across the platform.
    #[serde(rename = "username")]
    pub name: String,
    /// Type of Nitro subscription on a user's account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub premium_type: Option<PremiumType>,
    /// Public flags on a user's account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub public_flags: Option<UserFlags>,
    /// Whether the email on this account has been verified.
    ///
    /// Requires the `email` oauth scope. See [Discord's documentation] for
    /// more information.
    ///
    /// [Discord's documentation]: https://discord.com/developers/docs/resources/user#user-object-user-structure
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verified: Option<bool>,
}

impl CurrentUser {
    /// Create a [`Display`] formatter for a user discriminator.
    ///
    /// [`Display`]: core::fmt::Display
    pub const fn discriminator(&self) -> DiscriminatorDisplay {
        DiscriminatorDisplay::new(self.discriminator)
    }
}

#[cfg(test)]
mod tests {
    use super::{CurrentUser, PremiumType, UserFlags, UserId};
    use serde_test::Token;

    fn user_tokens(discriminator_token: Token) -> Vec<Token> {
        vec![
            Token::Struct {
                name: "CurrentUser",
                len: 12,
            },
            Token::Str("accent_color"),
            Token::Some,
            Token::U64(16_711_680),
            Token::Str("avatar"),
            Token::Some,
            Token::Str("avatar hash"),
            Token::Str("banner"),
            Token::None,
            Token::Str("bot"),
            Token::Bool(true),
            Token::Str("discriminator"),
            discriminator_token,
            Token::Str("id"),
            Token::NewtypeStruct { name: "UserId" },
            Token::Str("1"),
            Token::Str("locale"),
            Token::Some,
            Token::Str("test locale"),
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
            Token::Some,
            Token::Bool(true),
            Token::StructEnd,
        ]
    }

    fn user_tokens_complete(discriminator_token: Token) -> Vec<Token> {
        vec![
            Token::Struct {
                name: "CurrentUser",
                len: 14,
            },
            Token::Str("accent_color"),
            Token::None,
            Token::Str("avatar"),
            Token::Some,
            Token::Str("avatar hash"),
            Token::Str("banner"),
            Token::Some,
            Token::Str("06c16474723fe537c283b8efa61a30c8"),
            Token::Str("bot"),
            Token::Bool(true),
            Token::Str("discriminator"),
            discriminator_token,
            Token::Str("email"),
            Token::Some,
            Token::Str("test@example.com"),
            Token::Str("flags"),
            Token::Some,
            Token::U64(1),
            Token::Str("id"),
            Token::NewtypeStruct { name: "UserId" },
            Token::Str("1"),
            Token::Str("locale"),
            Token::Some,
            Token::Str("test locale"),
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
            Token::Some,
            Token::Bool(true),
            Token::StructEnd,
        ]
    }

    #[test]
    fn test_current_user() {
        let value = CurrentUser {
            accent_color: Some(16_711_680),
            avatar: Some("avatar hash".to_owned()),
            banner: None,
            bot: true,
            discriminator: 9999,
            email: None,
            id: UserId::new(1).expect("non zero"),
            mfa_enabled: true,
            name: "test name".to_owned(),
            verified: Some(true),
            premium_type: Some(PremiumType::NitroClassic),
            public_flags: Some(UserFlags::DISCORD_EMPLOYEE),
            flags: None,
            locale: Some("test locale".to_owned()),
        };

        // Deserializing a current user with a string discriminator (which
        // Discord provides)
        serde_test::assert_tokens(&value, &user_tokens(Token::Str("9999")));

        // Deserializing a current user with an integer discriminator. Userland
        // code may have this due to being a more compact memory representation
        // of a discriminator.
        serde_test::assert_de_tokens(&value, &user_tokens(Token::U64(9999)));
    }

    #[test]
    fn test_current_user_complete() {
        let value = CurrentUser {
            accent_color: None,
            avatar: Some("avatar hash".to_owned()),
            banner: Some("06c16474723fe537c283b8efa61a30c8".to_owned()),
            bot: true,
            discriminator: 9999,
            email: Some("test@example.com".to_owned()),
            id: UserId::new(1).expect("non zero"),
            mfa_enabled: true,
            name: "test name".to_owned(),
            verified: Some(true),
            premium_type: Some(PremiumType::NitroClassic),
            public_flags: Some(UserFlags::DISCORD_EMPLOYEE),
            flags: Some(UserFlags::DISCORD_EMPLOYEE),
            locale: Some("test locale".to_owned()),
        };

        // Deserializing a current user with a string discriminator (which
        // Discord provides)
        serde_test::assert_tokens(&value, &user_tokens_complete(Token::Str("9999")));

        // Deserializing a current user with an integer discriminator. Userland
        // code may have this due to being a more compact memory representation
        // of a discriminator.
        serde_test::assert_de_tokens(&value, &user_tokens_complete(Token::U64(9999)));
    }
}
