use super::{DiscriminatorDisplay, PremiumType, UserFlags};
use crate::id::UserId;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct UserProfile {
    /// Accent color of the user's banner.
    ///
    /// This is an integer representation of a hexadecimal color code.
    pub accent_color: Option<u64>,
    pub avatar: Option<String>,
    /// Hash of the user's banner image.
    pub banner: Option<String>,
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flags: Option<UserFlags>,
    pub id: UserId,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locale: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mfa_enabled: Option<bool>,
    #[serde(rename = "username")]
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub premium_type: Option<PremiumType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verified: Option<bool>,
}

impl UserProfile {
    /// Create a [`Display`] formatter for a user discriminator.
    ///
    /// [`Display`]: core::fmt::Display
    pub const fn discriminator(&self) -> DiscriminatorDisplay {
        DiscriminatorDisplay::new(self.discriminator)
    }
}

#[cfg(test)]
mod tests {
    use super::{PremiumType, UserFlags, UserId, UserProfile};
    use serde_test::Token;

    fn user_tokens(discriminator_token: Token) -> Vec<Token> {
        vec![
            Token::Struct {
                name: "UserProfile",
                len: 13,
            },
            Token::Str("accent_color"),
            Token::Some,
            Token::U64(16_579_836),
            Token::Str("avatar"),
            Token::Some,
            Token::Str("hash"),
            Token::Str("banner"),
            Token::None,
            Token::Str("bot"),
            Token::Bool(false),
            Token::Str("discriminator"),
            discriminator_token,
            Token::Str("email"),
            Token::Some,
            Token::Str("email@example.com"),
            Token::Str("flags"),
            Token::Some,
            Token::U64(131_072),
            Token::Str("id"),
            Token::NewtypeStruct { name: "UserId" },
            Token::Str("1"),
            Token::Str("locale"),
            Token::Some,
            Token::Str("en-us"),
            Token::Str("mfa_enabled"),
            Token::Some,
            Token::Bool(true),
            Token::Str("username"),
            Token::Str("user name"),
            Token::Str("premium_type"),
            Token::Some,
            Token::U8(2),
            Token::Str("verified"),
            Token::Some,
            Token::Bool(true),
            Token::StructEnd,
        ]
    }

    #[test]
    fn test_user_profile() {
        let value = UserProfile {
            accent_color: Some(16_579_836),
            avatar: Some("hash".to_owned()),
            banner: None,
            bot: false,
            discriminator: 4,
            email: Some("email@example.com".to_owned()),
            flags: Some(UserFlags::VERIFIED_BOT_DEVELOPER),
            id: UserId::new(1).expect("non zero"),
            locale: Some("en-us".to_owned()),
            mfa_enabled: Some(true),
            name: "user name".to_owned(),
            premium_type: Some(PremiumType::Nitro),
            verified: Some(true),
        };

        // Deserializing a user profile with a string discriminator (which
        // Discord provides)
        serde_test::assert_tokens(&value, &user_tokens(Token::Str("0004")));

        // Deserializing a user profile with an integer discriminator. Userland
        // code may have this due to being a more compact memory representation
        // of a discriminator.
        serde_test::assert_de_tokens(&value, &user_tokens(Token::U64(4)));
    }
}
