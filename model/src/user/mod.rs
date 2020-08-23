mod connection;
mod connection_visibility;
mod current_user;
mod flags;
mod premium_type;
mod profile;

pub use self::{
    connection::Connection, connection_visibility::ConnectionVisibility, current_user::CurrentUser,
    flags::UserFlags, premium_type::PremiumType, profile::UserProfile,
};

use crate::id::UserId;
use serde::{Deserialize, Serialize};
use serde_mappable_seq::Key;

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct User {
    pub avatar: Option<String>,
    #[serde(default)]
    pub bot: bool,
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

        serde_test::assert_tokens(
            &value,
            &[
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
                Token::Str("0001"),
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
            ],
        );
    }
}
