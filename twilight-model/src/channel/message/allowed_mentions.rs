use crate::{
    id::{
        marker::{RoleMarker, UserMarker},
        Id,
    },
    util::{is_false, known_string::KnownString},
};
use serde::{Deserialize, Serialize};
use std::{
    fmt::{Debug, Formatter, Result as FmtResult},
    ops::Deref,
    str::FromStr,
};

/// Allowed mentions (pings).
///
/// Filters mentions to only ping one's specified here, regardless of the message's content[^1].
///
/// Mentions can be clicked to reveal additional context, whilst only requiring an ID to create. See
/// [Discord Docs/Message Formatting].
///
/// [`AllowedMentions::default`] disallows all pings.
///
/// [^1]: Messages must still contain mentions, e.g. `@everyone`!
///
/// [Discord Docs/Message Formatting]: https://discord.com/developers/docs/reference#message-formatting
#[derive(Clone, Debug, Default, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct AllowedMentions {
    /// List of allowed mention types.
    ///
    /// [`MentionType::ROLES`] and [`MentionType::USERS`] allows all roles and users to be
    /// mentioned; they are mutually exclusive with the [`roles`] and [`users`] fields.
    ///
    /// [`roles`]: Self::roles
    /// [`users`]: Self::users
    #[serde(default)]
    pub parse: Vec<MentionType>,
    /// For replies, whether to mention the message author.
    ///
    /// Defaults to false.
    #[serde(default, skip_serializing_if = "is_false")]
    pub replied_user: bool,
    /// List of roles to mention.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub roles: Vec<Id<RoleMarker>>,
    /// List of users to mention.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub users: Vec<Id<UserMarker>>,
}

/// Allowed mention type.
#[derive(Clone, Copy, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct MentionType(KnownString<16>);

impl MentionType {
    /// `@everyone` and `@here` mentions.
    pub const EVERYONE: Self = Self::from_bytes(b"everyone");

    /// Role mentions.
    pub const ROLES: Self = Self::from_bytes(b"roles");

    /// User mentions.
    pub const USERS: Self = Self::from_bytes(b"users");

    /// Create a mention type from a dynamic value.
    ///
    /// The provided mention type must be 64 bytes or smaller.
    pub fn new(mention_type: &str) -> Option<Self> {
        KnownString::from_str(mention_type).map(Self)
    }

    /// Get the value of the mention type.
    ///
    /// # Panics
    ///
    /// Panics if the mention type isn't valid UTF-8.
    pub fn get(&self) -> &str {
        self.0.get()
    }

    /// Create a mention type from a set of bytes.
    const fn from_bytes(input: &[u8]) -> Self {
        Self(KnownString::from_bytes(input))
    }
}

impl AsRef<str> for MentionType {
    fn as_ref(&self) -> &str {
        self.get()
    }
}

impl Debug for MentionType {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        f.write_str(self.get())
    }
}

impl Deref for MentionType {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        self.get()
    }
}

impl FromStr for MentionType {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::try_from(s)
    }
}

impl ToString for MentionType {
    fn to_string(&self) -> String {
        KnownString::to_string(&self.0)
    }
}

impl TryFrom<&str> for MentionType {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::new(value).ok_or(())
    }
}

#[cfg(test)]
mod tests {
    use super::{AllowedMentions, MentionType};
    use crate::id::Id;
    use serde_test::Token;

    #[test]
    fn minimal() {
        let value = AllowedMentions {
            parse: Vec::new(),
            users: Vec::new(),
            roles: Vec::new(),
            replied_user: false,
        };

        serde_test::assert_tokens(
            &value,
            &[
                Token::Struct {
                    name: "AllowedMentions",
                    len: 1,
                },
                Token::Str("parse"),
                Token::Seq { len: Some(0) },
                Token::SeqEnd,
                Token::StructEnd,
            ],
        );
    }

    #[test]
    fn full() {
        let value = AllowedMentions {
            parse: Vec::from([MentionType::EVERYONE]),
            users: Vec::from([Id::new(100)]),
            roles: Vec::from([Id::new(200)]),
            replied_user: true,
        };

        serde_test::assert_tokens(
            &value,
            &[
                Token::Struct {
                    name: "AllowedMentions",
                    len: 4,
                },
                Token::Str("parse"),
                Token::Seq { len: Some(1) },
                Token::NewtypeStruct {
                    name: "MentionType",
                },
                Token::Str("everyone"),
                Token::SeqEnd,
                Token::Str("replied_user"),
                Token::Bool(true),
                Token::Str("roles"),
                Token::Seq { len: Some(1) },
                Token::NewtypeStruct { name: "Id" },
                Token::Str("200"),
                Token::SeqEnd,
                Token::Str("users"),
                Token::Seq { len: Some(1) },
                Token::NewtypeStruct { name: "Id" },
                Token::Str("100"),
                Token::SeqEnd,
                Token::StructEnd,
            ],
        );
    }
}
