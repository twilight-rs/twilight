use crate::{
    guild::PartialMember,
    id::{Id, marker::UserMarker},
    user::{self, DiscriminatorDisplay, UserFlags},
    util::image_hash::ImageHash,
};
use serde::{Deserialize, Serialize};

/// Mention of a user in a message.
#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct Mention {
    /// Hash of the user's avatar, if any.
    pub avatar: Option<ImageHash>,
    /// Whether the user is a bot.
    #[serde(default)]
    pub bot: bool,
    /// Discriminator used to differentiate people with the same username.
    ///
    /// # serde
    ///
    /// The discriminator field can be deserialized from either a string or an
    /// integer. The field will always serialize into a string due to that being
    /// the type Discord's API uses.
    #[serde(with = "user::discriminator")]
    pub discriminator: u16,
    /// Unique ID of the user.
    pub id: Id<UserMarker>,
    /// Member object for the user in the guild, if available.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member: Option<PartialMember>,
    #[serde(rename = "username")]
    /// Username of the user.
    pub name: String,
    /// Public flags on the user's account.
    pub public_flags: UserFlags,
}

impl Mention {
    /// Create a [`Display`] formatter for a user discriminator.
    ///
    /// [`Display`]: core::fmt::Display
    pub const fn discriminator(&self) -> DiscriminatorDisplay {
        DiscriminatorDisplay::new(self.discriminator)
    }
}

#[cfg(test)]
mod tests {
    use super::{Mention, PartialMember, UserFlags};
    use crate::{
        guild::MemberFlags,
        id::Id,
        util::datetime::{Timestamp, TimestampParseError},
    };
    use serde_test::Token;
    use std::str::FromStr;

    #[test]
    fn mention_without_member() {
        let value = Mention {
            avatar: None,
            bot: false,
            discriminator: 1,
            id: Id::new(1),
            member: None,
            name: "foo".to_owned(),
            public_flags: UserFlags::empty(),
        };

        serde_test::assert_tokens(
            &value,
            &[
                Token::Struct {
                    name: "Mention",
                    len: 6,
                },
                Token::Str("avatar"),
                Token::None,
                Token::Str("bot"),
                Token::Bool(false),
                Token::Str("discriminator"),
                Token::Str("0001"),
                Token::Str("id"),
                Token::NewtypeStruct { name: "Id" },
                Token::Str("1"),
                Token::Str("username"),
                Token::Str("foo"),
                Token::Str("public_flags"),
                Token::U64(0),
                Token::StructEnd,
            ],
        );
    }

    #[test]
    fn mention_with_member() -> Result<(), TimestampParseError> {
        let joined_at = Some(Timestamp::from_str("2015-04-26T06:26:56.936000+00:00")?);
        let flags = MemberFlags::BYPASSES_VERIFICATION | MemberFlags::DID_REJOIN;

        let value = Mention {
            avatar: None,
            bot: false,
            discriminator: 1,
            id: Id::new(1),
            member: Some(PartialMember {
                avatar: None,
                avatar_decoration_data: None,
                banner: None,
                communication_disabled_until: None,
                deaf: false,
                flags,
                joined_at,
                mute: true,
                nick: Some("bar".to_owned()),
                permissions: None,
                premium_since: None,
                roles: Vec::new(),
                user: None,
            }),
            name: "foo".to_owned(),
            public_flags: UserFlags::empty(),
        };

        serde_test::assert_tokens(
            &value,
            &[
                Token::Struct {
                    name: "Mention",
                    len: 7,
                },
                Token::Str("avatar"),
                Token::None,
                Token::Str("bot"),
                Token::Bool(false),
                Token::Str("discriminator"),
                Token::Str("0001"),
                Token::Str("id"),
                Token::NewtypeStruct { name: "Id" },
                Token::Str("1"),
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
                Token::Str("flags"),
                Token::U64(flags.bits()),
                Token::Str("joined_at"),
                Token::Some,
                Token::Str("2015-04-26T06:26:56.936000+00:00"),
                Token::Str("mute"),
                Token::Bool(true),
                Token::Str("nick"),
                Token::Some,
                Token::Str("bar"),
                Token::Str("roles"),
                Token::Seq { len: Some(0) },
                Token::SeqEnd,
                Token::Str("user"),
                Token::None,
                Token::StructEnd,
                Token::Str("username"),
                Token::Str("foo"),
                Token::Str("public_flags"),
                Token::U64(0),
                Token::StructEnd,
            ],
        );
        Ok(())
    }
}
