use crate::{
    guild::PartialMember,
    id::UserId,
    user::{self, UserFlags},
};
use serde::{Deserialize, Serialize};
use serde_mappable_seq::Key;

/// Mention of a user in a message.
#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct Mention {
    /// Hash of the user's avatar, if any.
    pub avatar: Option<String>,
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
    pub discriminator: String,
    /// Unique ID of the user.
    pub id: UserId,
    /// Member object for the user in the guild, if available.
    pub member: Option<PartialMember>,
    #[serde(rename = "username")]
    /// Username of the user.
    pub name: String,
    /// Public flags on the user's account.
    pub public_flags: UserFlags,
}

impl Key<'_, UserId> for Mention {
    fn key(&self) -> UserId {
        self.id
    }
}

#[cfg(test)]
mod tests {
    use super::{Mention, PartialMember, UserFlags, UserId};
    use serde_test::Token;

    #[test]
    fn test_mention_without_member() {
        let value = Mention {
            avatar: None,
            bot: false,
            discriminator: "0001".to_owned(),
            id: UserId(1),
            member: None,
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
                Token::NewtypeStruct { name: "UserId" },
                Token::Str("1"),
                Token::Str("member"),
                Token::None,
                Token::Str("username"),
                Token::Str("foo"),
                Token::Str("public_flags"),
                Token::U64(0),
                Token::StructEnd,
            ],
        );
    }

    #[test]
    fn test_mention_with_member() {
        let value = Mention {
            avatar: None,
            bot: false,
            discriminator: "0001".to_owned(),
            id: UserId(1),
            member: Some(PartialMember {
                deaf: false,
                joined_at: None,
                mute: true,
                nick: Some("bar".to_owned()),
                premium_since: None,
                roles: Vec::new(),
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
                Token::NewtypeStruct { name: "UserId" },
                Token::Str("1"),
                Token::Str("member"),
                Token::Some,
                Token::Struct {
                    name: "PartialMember",
                    len: 6,
                },
                Token::Str("deaf"),
                Token::Bool(false),
                Token::Str("joined_at"),
                Token::None,
                Token::Str("mute"),
                Token::Bool(true),
                Token::Str("nick"),
                Token::Some,
                Token::Str("bar"),
                Token::Str("premium_since"),
                Token::None,
                Token::Str("roles"),
                Token::Seq { len: Some(0) },
                Token::SeqEnd,
                Token::StructEnd,
                Token::Str("username"),
                Token::Str("foo"),
                Token::Str("public_flags"),
                Token::U64(0),
                Token::StructEnd,
            ],
        );
    }
}
