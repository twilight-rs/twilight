//! Configure granular control over mentions and avoid phantom pings.

mod builder;

pub use self::builder::AllowedMentionsBuilder;

use crate::{
    id::{
        marker::{RoleMarker, UserMarker},
        Id,
    },
    util::is_false,
};
use serde::{Deserialize, Serialize};

/// Allows for more granular control over mentions.
///
/// Validates against the message content to avoid phantom pings, but you must
/// still have e.g. `@everyone` in the message content to ping everyone.
#[derive(Clone, Debug, Default, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct AllowedMentions {
    /// List of allowed [`ParseTypes`].
    #[serde(default)]
    pub parse: Vec<ParseTypes>,
    /// List of [`Id<UserMarker>`] to mention.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub users: Vec<Id<UserMarker>>,
    /// List of [`Id<RoleMarker>`] to mention.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub roles: Vec<Id<RoleMarker>>,
    /// For replies, whether to mention the message author being replied to.
    ///
    /// Defaults to false.
    #[serde(default, skip_serializing_if = "is_false")]
    pub replied_user: bool,
}

impl AllowedMentions {
    /// Create a new [`AllowedMentionsBuilder`].
    pub const fn builder() -> AllowedMentionsBuilder {
        AllowedMentionsBuilder::new()
    }
}

/// Allowed mention type in message content.
#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
#[non_exhaustive]
#[serde(rename_all = "lowercase")]
pub enum ParseTypes {
    /// `@everyone` and `here` mentions.
    Everyone,
    /// Role mentions.
    Roles,
    /// User mentions.
    Users,
}

#[cfg(test)]
mod tests {
    use super::{AllowedMentions, ParseTypes};
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
            parse: Vec::from([ParseTypes::Everyone]),
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
                Token::UnitVariant {
                    name: "ParseTypes",
                    variant: "everyone",
                },
                Token::SeqEnd,
                Token::Str("users"),
                Token::Seq { len: Some(1) },
                Token::NewtypeStruct { name: "Id" },
                Token::Str("100"),
                Token::SeqEnd,
                Token::Str("roles"),
                Token::Seq { len: Some(1) },
                Token::NewtypeStruct { name: "Id" },
                Token::Str("200"),
                Token::SeqEnd,
                Token::Str("replied_user"),
                Token::Bool(true),
                Token::StructEnd,
            ],
        );
    }
}
