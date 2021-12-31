use super::{DiscriminatorDisplay, PremiumType, UserFlags};
use crate::id::{marker::UserMarker, Id};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct UserProfile {
    pub(crate) accent_color: Option<u64>,
    pub(crate) avatar: Option<String>,
    pub(crate) banner: Option<String>,
    #[serde(default)]
    pub(crate) bot: bool,
    // TODO: figure out how to fix this
    /// Discriminator used to differentiate people with the same username.
    ///
    /// # serde
    ///
    /// The discriminator field can be deserialized from either a string or an
    /// integer.
    #[serde(with = "super::discriminator")]
    pub discriminator: u16,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) email: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) flags: Option<UserFlags>,
    pub(crate) id: Id<UserMarker>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) locale: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) mfa_enabled: Option<bool>,
    #[serde(rename = "username")]
    pub(crate) name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) premium_type: Option<PremiumType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) verified: Option<bool>,
}

impl UserProfile {
    /// Create a [`Display`] formatter for a user discriminator.
    ///
    /// [`Display`]: core::fmt::Display
    pub const fn discriminator(&self) -> DiscriminatorDisplay {
        DiscriminatorDisplay::new(self.discriminator)
    }

    /// Accent color of the user's banner.
    ///
    /// This is an integer representation of a hexadecimal color code.
    pub const fn accent_color(&self) -> Option<u64> {
        self.accent_color
    }

    pub fn avatar(&self) -> Option<&str> {
        self.avatar.as_deref()
    }

    /// Hash of the user's banner image.
    pub fn banner(&self) -> Option<&str> {
        self.banner.as_deref()
    }

    pub const fn bot(&self) -> bool {
        self.bot
    }

    pub fn email(&self) -> Option<&str> {
        self.email.as_deref()
    }

    pub const fn flags(&self) -> Option<UserFlags> {
        self.flags
    }

    pub const fn id(&self) -> Id<UserMarker> {
        self.id
    }

    pub fn locale(&self) -> Option<&str> {
        self.locale.as_deref()
    }

    pub const fn mfa_enabled(&self) -> Option<bool> {
        self.mfa_enabled
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub const fn premium_type(&self) -> Option<PremiumType> {
        self.premium_type
    }

    pub const fn verified(&self) -> Option<bool> {
        self.verified
    }
}

#[cfg(test)]
mod tests {
    use super::{PremiumType, UserFlags, UserProfile};
    use crate::id::Id;
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
            Token::NewtypeStruct { name: "Id" },
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
            flags: Some(UserFlags::VERIFIED_DEVELOPER),
            id: Id::new(1).expect("non zero"),
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
