use crate::{
    application::interaction::InteractionType,
    guild::PartialMember,
    id::{marker::InteractionMarker, Id},
    user::User,
};
use serde::{Deserialize, Serialize};

/// Associated interaction metadata.
#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct MessageInteraction {
    /// ID of the interaction.
    pub id: Id<InteractionMarker>,
    /// Type of the interaction.
    #[serde(rename = "type")]
    pub kind: InteractionType,
    /// If in a guild, the member who invoked the interaction.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member: Option<PartialMember>,
    /// Name of the [`Command`].
    ///
    /// [`Command`]: crate::application::command::Command
    pub name: String,
    /// User who invoked the interaction.
    pub user: User,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        test::image_hash,
        user::{PremiumType, UserFlags},
        util::Timestamp,
    };
    use serde_test::Token;
    use std::{error::Error, str::FromStr};

    #[allow(clippy::too_many_lines)]
    #[test]
    fn message_interaction() -> Result<(), Box<dyn Error>> {
        let joined_at = Timestamp::from_str("2015-04-26T06:26:56.936000+00:00")?;

        let value = MessageInteraction {
            id: Id::new(1),
            kind: InteractionType::ApplicationCommand,
            member: Some(PartialMember {
                avatar: None,
                communication_disabled_until: None,
                deaf: false,
                joined_at,
                mute: true,
                nick: Some("a nickname".to_owned()),
                permissions: None,
                premium_since: None,
                roles: Vec::from([Id::new(2)]),
                user: None,
            }),
            name: "search".into(),
            user: User {
                accent_color: None,
                avatar: Some(image_hash::AVATAR),
                banner: Some(image_hash::BANNER),
                bot: false,
                discriminator: 1,
                email: Some("address@example.com".to_owned()),
                flags: Some(UserFlags::PREMIUM_EARLY_SUPPORTER | UserFlags::VERIFIED_DEVELOPER),
                id: Id::new(3),
                locale: Some("en-us".to_owned()),
                mfa_enabled: Some(true),
                name: "test".to_owned(),
                premium_type: Some(PremiumType::Nitro),
                public_flags: Some(
                    UserFlags::PREMIUM_EARLY_SUPPORTER | UserFlags::VERIFIED_DEVELOPER,
                ),
                system: None,
                verified: Some(true),
            },
        };

        serde_test::assert_tokens(
            &value,
            &[
                Token::Struct {
                    name: "MessageInteraction",
                    len: 5,
                },
                Token::Str("id"),
                Token::NewtypeStruct { name: "Id" },
                Token::Str("1"),
                Token::Str("type"),
                Token::U8(InteractionType::ApplicationCommand as u8),
                Token::Str("member"),
                Token::Some,
                Token::Struct {
                    name: "PartialMember",
                    len: 8,
                },
                Token::Str("communication_disabled_until"),
                Token::None,
                Token::Str("deaf"),
                Token::Bool(false),
                Token::Str("joined_at"),
                Token::Str("2015-04-26T06:26:56.936000+00:00"),
                Token::Str("mute"),
                Token::Bool(true),
                Token::Str("nick"),
                Token::Some,
                Token::Str("a nickname"),
                Token::Str("permissions"),
                Token::None,
                Token::Str("roles"),
                Token::Seq { len: Some(1) },
                Token::NewtypeStruct { name: "Id" },
                Token::Str("2"),
                Token::SeqEnd,
                Token::Str("user"),
                Token::None,
                Token::StructEnd,
                Token::Str("name"),
                Token::Str("search"),
                Token::Str("user"),
                Token::Struct {
                    name: "User",
                    len: 14,
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
                Token::NewtypeStruct { name: "Id" },
                Token::Str("3"),
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
                Token::StructEnd,
            ],
        );

        Ok(())
    }
}
