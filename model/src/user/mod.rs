mod connection;
mod connection_visibility;
mod current_user;
mod current_user_guild;
mod flags;
mod premium_type;
mod profile;

pub use self::{
    connection::Connection, connection_visibility::ConnectionVisibility, current_user::CurrentUser,
    current_user_guild::CurrentUserGuild, flags::UserFlags, premium_type::PremiumType,
    profile::UserProfile,
};

use crate::id::UserId;
use serde::{Deserialize, Serialize};
use serde_mappable_seq::Key;

pub(crate) mod discriminator {
    use serde::{
        de::{Deserializer, Error as DeError, Visitor},
        ser::Serializer,
    };
    use std::fmt::{Formatter, Result as FmtResult};

    struct DiscriminatorVisitor;

    impl<'de> Visitor<'de> for DiscriminatorVisitor {
        type Value = String;

        fn expecting(&self, f: &mut Formatter<'_>) -> FmtResult {
            f.write_str("string or integer discriminator")
        }

        fn visit_u64<E: DeError>(self, value: u64) -> Result<Self::Value, E> {
            Ok(format!("{:04}", value))
        }

        fn visit_str<E: DeError>(self, value: &str) -> Result<Self::Value, E> {
            Ok(value.to_owned())
        }
    }

    pub fn deserialize<'de, D: Deserializer<'de>>(deserializer: D) -> Result<String, D::Error> {
        deserializer.deserialize_any(DiscriminatorVisitor)
    }

    pub fn serialize<S: Serializer>(value: &str, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.collect_str(value)
    }
}

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct User {
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
    #[serde(with = "discriminator")]
    pub discriminator: String,
    pub email: Option<String>,
    pub flags: Option<UserFlags>,
    pub id: UserId,
    pub locale: Option<String>,
    pub mfa_enabled: Option<bool>,
    #[serde(rename = "username")]
    pub name: String,
    pub premium_type: Option<PremiumType>,
    pub public_flags: Option<UserFlags>,
    pub system: Option<bool>,
    pub verified: Option<bool>,
}

impl Key<'_, UserId> for User {
    fn key(&self) -> UserId {
        self.id
    }
}

#[cfg(test)]
mod tests {
    use super::{PremiumType, User, UserFlags, UserId};
    use serde_test::Token;

    fn user_tokens(discriminator_token: Token) -> Vec<Token> {
        vec![
            Token::Struct {
                name: "User",
                len: 13,
            },
            Token::Str("avatar"),
            Token::Some,
            Token::Str("aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"),
            Token::Str("bot"),
            Token::Bool(false),
            Token::Str("discriminator"),
            discriminator_token,
            Token::Str("email"),
            Token::Some,
            Token::Str("address@example.com"),
            Token::Str("flags"),
            Token::Some,
            Token::U64(131_584),
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
            Token::Str("test"),
            Token::Str("premium_type"),
            Token::Some,
            Token::U8(2),
            Token::Str("public_flags"),
            Token::Some,
            Token::U64(131_584),
            Token::Str("system"),
            Token::None,
            Token::Str("verified"),
            Token::Some,
            Token::Bool(true),
            Token::StructEnd,
        ]
    }

    #[test]
    fn test_user() {
        let value = User {
            avatar: Some("aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa".to_owned()),
            bot: false,
            discriminator: "0001".to_owned(),
            email: Some("address@example.com".to_owned()),
            flags: Some(UserFlags::EARLY_SUPPORTER | UserFlags::VERIFIED_BOT_DEVELOPER),
            id: UserId(1),
            locale: Some("en-us".to_owned()),
            mfa_enabled: Some(true),
            name: "test".to_owned(),
            premium_type: Some(PremiumType::Nitro),
            public_flags: Some(UserFlags::EARLY_SUPPORTER | UserFlags::VERIFIED_BOT_DEVELOPER),
            system: None,
            verified: Some(true),
        };

        // Deserializing a user with a string discriminator (which Discord
        // provides)
        serde_test::assert_tokens(&value, &user_tokens(Token::Str("0001")));

        // Deserializing a user with an integer discriminator. Userland code
        // may have this due to being a more compact memory representation of a
        // discriminator.
        serde_test::assert_de_tokens(&value, &user_tokens(Token::U64(1)));
    }
}
