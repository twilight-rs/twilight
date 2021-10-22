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
use std::fmt::{Display, Formatter, Result as FmtResult};

pub(crate) mod discriminator {
    use super::DiscriminatorDisplay;
    use serde::{
        de::{Deserializer, Error as DeError, Visitor},
        ser::Serializer,
    };
    use std::{
        convert::TryInto,
        fmt::{Formatter, Result as FmtResult},
    };

    struct DiscriminatorVisitor;

    impl<'de> Visitor<'de> for DiscriminatorVisitor {
        type Value = u16;

        fn expecting(&self, f: &mut Formatter<'_>) -> FmtResult {
            f.write_str("string or integer discriminator")
        }

        fn visit_u64<E: DeError>(self, value: u64) -> Result<Self::Value, E> {
            value.try_into().map_err(DeError::custom)
        }

        fn visit_str<E: DeError>(self, value: &str) -> Result<Self::Value, E> {
            value.parse().map_err(DeError::custom)
        }
    }

    pub fn deserialize<'de, D: Deserializer<'de>>(deserializer: D) -> Result<u16, D::Error> {
        deserializer.deserialize_any(DiscriminatorVisitor)
    }

    // Allow this lint because taking a reference is required by serde.
    #[allow(clippy::trivially_copy_pass_by_ref)]
    pub fn serialize<S: Serializer>(value: &u16, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.collect_str(&DiscriminatorDisplay(*value))
    }
}

/// Display formatter for a user discriminator.
///
/// When formatted this will pad a discriminator with zeroes.
///
/// This may be preferable to use instead of using `format!` to avoid a String
/// allocation, and may also be preferable to use rather than defining your own
/// implementations via `format_args!("{:04}", discriminator)`.
///
/// # Examples
///
/// Display the discriminator value `16` as a string:
///
/// ```
/// use twilight_model::user::DiscriminatorDisplay;
///
/// let display = DiscriminatorDisplay::new(16);
/// assert_eq!("0016", display.to_string());
/// ```
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
#[must_use = "display implementations should be formatted"]
pub struct DiscriminatorDisplay(u16);

impl DiscriminatorDisplay {
    /// Create a new display formatter for a discriminator.
    ///
    /// # Examples
    ///
    /// Display the discriminator value `5` as a string:
    ///
    /// ```
    /// use twilight_model::user::DiscriminatorDisplay;
    ///
    /// let display = DiscriminatorDisplay::new(5);
    /// assert_eq!("0005", display.to_string());
    /// ```
    pub const fn new(discriminator: u16) -> Self {
        Self(discriminator)
    }

    /// Retrieve the inner discriminator value.
    pub const fn get(self) -> u16 {
        self.0
    }
}

impl Display for DiscriminatorDisplay {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        // Pad the formatted value with zeroes depending on the number of
        // digits.
        //
        // If the value is [1000, u16::MAX] then we don't need to pad.
        match self.0 {
            0..=9 => f.write_str("000")?,
            10..=99 => f.write_str("00")?,
            100..=999 => f.write_str("0")?,
            _ => {}
        }

        Display::fmt(&self.0, f)
    }
}

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct User {
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
    #[serde(with = "discriminator")]
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
    pub public_flags: Option<UserFlags>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub system: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verified: Option<bool>,
}

impl User {
    /// Create a [`Display`] formatter for a user discriminator.
    ///
    /// [`Display`]: core::fmt::Display
    pub const fn discriminator(&self) -> DiscriminatorDisplay {
        DiscriminatorDisplay::new(self.discriminator)
    }
}

#[cfg(test)]
mod tests {
    use super::{DiscriminatorDisplay, PremiumType, User, UserFlags, UserId};
    use serde_test::Token;
    use static_assertions::assert_impl_all;
    use std::{fmt::Debug, hash::Hash};

    assert_impl_all!(
        DiscriminatorDisplay: Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        PartialEq,
        Send,
        Sync
    );

    fn user_tokens(discriminator_token: Token) -> Vec<Token> {
        vec![
            Token::Struct {
                name: "User",
                len: 14,
            },
            Token::Str("accent_color"),
            Token::None,
            Token::Str("avatar"),
            Token::Some,
            Token::Str("aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"),
            Token::Str("banner"),
            Token::Some,
            Token::Str("06c16474723fe537c283b8efa61a30c8"),
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
            Token::Str("verified"),
            Token::Some,
            Token::Bool(true),
            Token::StructEnd,
        ]
    }

    fn user_tokens_complete(discriminator_token: Token) -> Vec<Token> {
        vec![
            Token::Struct {
                name: "User",
                len: 15,
            },
            Token::Str("accent_color"),
            Token::None,
            Token::Str("avatar"),
            Token::Some,
            Token::Str("aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"),
            Token::Str("banner"),
            Token::Some,
            Token::Str("06c16474723fe537c283b8efa61a30c8"),
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
            Token::Some,
            Token::Bool(true),
            Token::Str("verified"),
            Token::Some,
            Token::Bool(true),
            Token::StructEnd,
        ]
    }

    #[test]
    fn test_discriminator_display() {
        assert_eq!(3030, DiscriminatorDisplay::new(3030).get());
        assert_eq!("0003", DiscriminatorDisplay::new(3).to_string());
        assert_eq!("0033", DiscriminatorDisplay::new(33).to_string());
        assert_eq!("0333", DiscriminatorDisplay::new(333).to_string());
        assert_eq!("3333", DiscriminatorDisplay::new(3333).to_string());
    }

    #[test]
    fn test_user() {
        let value = User {
            accent_color: None,
            avatar: Some("aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa".to_owned()),
            banner: Some("06c16474723fe537c283b8efa61a30c8".to_owned()),
            bot: false,
            discriminator: 1,
            email: Some("address@example.com".to_owned()),
            flags: Some(UserFlags::EARLY_SUPPORTER | UserFlags::VERIFIED_BOT_DEVELOPER),
            id: UserId::new(1).expect("non zero"),
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

    #[test]
    fn test_user_complete() {
        let value = User {
            accent_color: None,
            avatar: Some("aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa".to_owned()),
            banner: Some("06c16474723fe537c283b8efa61a30c8".to_owned()),
            bot: false,
            discriminator: 1,
            email: Some("address@example.com".to_owned()),
            flags: Some(UserFlags::EARLY_SUPPORTER | UserFlags::VERIFIED_BOT_DEVELOPER),
            id: UserId::new(1).expect("non zero"),
            locale: Some("en-us".to_owned()),
            mfa_enabled: Some(true),
            name: "test".to_owned(),
            premium_type: Some(PremiumType::Nitro),
            public_flags: Some(UserFlags::EARLY_SUPPORTER | UserFlags::VERIFIED_BOT_DEVELOPER),
            system: Some(true),
            verified: Some(true),
        };

        // Deserializing a user with a string discriminator (which Discord
        // provides)
        serde_test::assert_tokens(&value, &user_tokens_complete(Token::Str("0001")));

        // Deserializing a user with an integer discriminator. Userland code
        // may have this due to being a more compact memory representation of a
        // discriminator.
        serde_test::assert_de_tokens(&value, &user_tokens_complete(Token::U64(1)));
    }
}
