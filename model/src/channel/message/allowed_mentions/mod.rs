use crate::id::{RoleId, UserId};
use serde::{Deserialize, Serialize};

mod builder;

pub use self::builder::AllowedMentionsBuilder;

#[allow(clippy::trivially_copy_pass_by_ref)]
fn is_false(value: &bool) -> bool {
    !value
}

/// Parse types.
#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
#[non_exhaustive]
#[serde(rename_all = "lowercase")]
pub enum ParseTypes {
    Users,
    Roles,
    Everyone,
}

/// Allowed mentions structure.
#[derive(Clone, Debug, Default, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct AllowedMentions {
    #[serde(default)]
    parse: Vec<ParseTypes>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    users: Vec<UserId>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    roles: Vec<RoleId>,
    #[serde(default, skip_serializing_if = "is_false")]
    replied_user: bool,
}

#[cfg(test)]
mod tests {
    use super::{AllowedMentions, ParseTypes};
    use crate::id::{RoleId, UserId};
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
            users: vec![UserId(100)],
            roles: vec![RoleId(200)],
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
                Token::NewtypeStruct { name: "UserId" },
                Token::Str("100"),
                Token::SeqEnd,
                Token::Str("roles"),
                Token::Seq { len: Some(1) },
                Token::NewtypeStruct { name: "RoleId" },
                Token::Str("200"),
                Token::SeqEnd,
                Token::Str("replied_user"),
                Token::Bool(true),
                Token::StructEnd,
            ],
        );
    }
}
