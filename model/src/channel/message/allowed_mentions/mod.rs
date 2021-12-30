use crate::{
    id::{
        marker::{RoleMarker, UserMarker},
        Id,
    },
    util::is_false,
};
use serde::{Deserialize, Serialize};

mod builder;

pub use self::builder::AllowedMentionsBuilder;

/// Parse types.
#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
#[non_exhaustive]
#[serde(rename_all = "lowercase")]
pub enum ParseTypes {
    Everyone,
    Roles,
    Users,
}

/// Allowed mentions structure.
#[derive(Clone, Debug, Default, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct AllowedMentions {
    #[serde(default)]
    pub parse: Vec<ParseTypes>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub users: Vec<Id<UserMarker>>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub roles: Vec<Id<RoleMarker>>,
    #[serde(default, skip_serializing_if = "is_false")]
    pub replied_user: bool,
}

impl AllowedMentions {
    pub const fn builder() -> AllowedMentionsBuilder {
        AllowedMentionsBuilder::new()
    }
}

#[cfg(test)]
mod tests {
    use super::{AllowedMentions, ParseTypes};
    use crate::id::Id;
    use serde_test::Token;

    #[test]
    fn test_minimal() {
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
    fn test_full() {
        let value = AllowedMentions {
            parse: vec![ParseTypes::Everyone],
            users: vec![Id::new(100)],
            roles: vec![Id::new(200)],
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
