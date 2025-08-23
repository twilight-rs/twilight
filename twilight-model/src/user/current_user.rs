use super::{DiscriminatorDisplay, PremiumType, UserFlags};
use crate::{
    id::{Id, marker::UserMarker},
    util::image_hash::ImageHash,
};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct CurrentUser {
    /// Accent color of the user's banner.
    ///
    /// This is an integer representation of a hexadecimal color code.
    pub accent_color: Option<u32>,
    /// User's avatar hash.
    ///
    /// To retrieve the url to the avatar, see [Discord Docs/Image Formatting].
    ///
    /// [Discord Docs/Image Formatting]: https://discord.com/developers/docs/reference#image-formatting
    pub avatar: Option<ImageHash>,
    /// Hash of the user's banner image.
    pub banner: Option<ImageHash>,
    /// Whether the user belongs to an OAuth2 application.
    #[serde(default)]
    pub bot: bool,
    /// Discriminator used to differentiate people with the same username.
    ///
    /// # Formatting
    ///
    /// Because discriminators are stored as an integer they're not in the
    /// format of Discord user tags due to a lack of padding with zeros. The
    /// [`discriminator`] method can be used to retrieve a formatter to pad the
    /// discriminator with zeros.
    ///
    /// # serde
    ///
    /// The discriminator field can be deserialized from either a string or an
    /// integer. The field will always serialize into a string due to that being
    /// the type Discord's API uses.
    ///
    /// [`discriminator`]: Self::discriminator
    #[serde(with = "super::discriminator")]
    pub discriminator: u16,
    /// User's email address associated to the account.
    ///
    /// Requires the `email` oauth scope. See [Discord Docs/User Object].
    ///
    /// [Discord Docs/User Object]: https://discord.com/developers/docs/resources/user#user-object-user-structure
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    /// All flags on a user's account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flags: Option<UserFlags>,
    /// User's global display name, if set. For bots, this is the application name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub global_name: Option<String>,
    /// User's id.
    pub id: Id<UserMarker>,
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
    /// Requires the `email` oauth scope. See [Discord Docs/User Object].
    ///
    /// [Discord Docs/User Object]: https://discord.com/developers/docs/resources/user#user-object-user-structure
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verified: Option<bool>,
}

impl CurrentUser {
    /// Create a [`Display`] formatter for a user discriminator that pads the
    /// discriminator with zeros up to 4 digits.
    ///
    /// [`Display`]: core::fmt::Display
    pub const fn discriminator(&self) -> DiscriminatorDisplay {
        DiscriminatorDisplay::new(self.discriminator)
    }
}

#[cfg(test)]
mod tests {
    use super::{CurrentUser, PremiumType, UserFlags};
    use crate::{id::Id, test::image_hash};
    use serde_test::Token;

    fn user_tokens(discriminator_token: Token) -> Vec<Token> {
        vec![
            Token::Struct {
                name: "CurrentUser",
                len: 12,
            },
            Token::Str("accent_color"),
            Token::Some,
            Token::U32(16_711_680),
            Token::Str("avatar"),
            Token::Some,
            Token::Str(image_hash::AVATAR_INPUT),
            Token::Str("banner"),
            Token::None,
            Token::Str("bot"),
            Token::Bool(true),
            Token::Str("discriminator"),
            discriminator_token,
            Token::Str("id"),
            Token::NewtypeStruct { name: "Id" },
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
                len: 15,
            },
            Token::Str("accent_color"),
            Token::None,
            Token::Str("avatar"),
            Token::Some,
            Token::Str(image_hash::AVATAR_INPUT),
            Token::Str("banner"),
            Token::Some,
            Token::Str(image_hash::BANNER_INPUT),
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
            Token::Str("global_name"),
            Token::Some,
            Token::Str("twilight sparkle"),
            Token::Str("id"),
            Token::NewtypeStruct { name: "Id" },
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
    fn current_user() {
        let value = CurrentUser {
            accent_color: Some(16_711_680),
            avatar: Some(image_hash::AVATAR),
            banner: None,
            bot: true,
            discriminator: 9999,
            email: None,
            id: Id::new(1),
            mfa_enabled: true,
            name: "test name".to_owned(),
            verified: Some(true),
            premium_type: Some(PremiumType::NitroClassic),
            public_flags: Some(UserFlags::STAFF),
            flags: None,
            locale: Some("test locale".to_owned()),
            global_name: None,
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
    fn current_user_complete() {
        let value = CurrentUser {
            accent_color: None,
            avatar: Some(image_hash::AVATAR),
            banner: Some(image_hash::BANNER),
            bot: true,
            discriminator: 9999,
            email: Some("test@example.com".to_owned()),
            flags: Some(UserFlags::STAFF),
            global_name: Some("twilight sparkle".to_owned()),
            id: Id::new(1),
            locale: Some("test locale".to_owned()),
            mfa_enabled: true,
            name: "test name".to_owned(),
            premium_type: Some(PremiumType::NitroClassic),
            public_flags: Some(UserFlags::STAFF),
            verified: Some(true),
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
