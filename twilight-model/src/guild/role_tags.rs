use crate::{
    id::{
        marker::{IntegrationMarker, UserMarker},
        Id,
    },
    util::is_false,
};
use serde::{Deserialize, Serialize};

/// Tags that a [`Role`] has.
///
/// [`Role`]: super::Role
#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct RoleTags {
    /// ID of the bot the role belongs to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bot_id: Option<Id<UserMarker>>,
    /// ID of the integration the role belongs to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub integration_id: Option<Id<IntegrationMarker>>,
    /// Whether this is the guild's premium subscriber role.
    #[serde(
        default,
        skip_serializing_if = "is_false",
        with = "crate::visitor::null_boolean"
    )]
    pub premium_subscriber: bool,
}

#[cfg(test)]
mod tests {
    use super::RoleTags;
    use crate::id::Id;
    use serde_test::Token;

    #[test]
    fn role_tags_all() {
        let tags = RoleTags {
            bot_id: Some(Id::new(1)),
            integration_id: Some(Id::new(2)),
            premium_subscriber: true,
        };

        serde_test::assert_tokens(
            &tags,
            &[
                Token::Struct {
                    name: "RoleTags",
                    len: 3,
                },
                Token::Str("bot_id"),
                Token::Some,
                Token::NewtypeStruct { name: "Id" },
                Token::Str("1"),
                Token::Str("integration_id"),
                Token::Some,
                Token::NewtypeStruct { name: "Id" },
                Token::Str("2"),
                Token::Str("premium_subscriber"),
                Token::None,
                Token::StructEnd,
            ],
        );
    }

    /// Test that if all fields are None and `premium_subscriber` is false, then
    /// serialize back into the source payload (where all fields are not
    /// present).
    #[test]
    fn role_tags_none() {
        let tags = RoleTags {
            bot_id: None,
            integration_id: None,
            premium_subscriber: false,
        };

        serde_test::assert_tokens(
            &tags,
            &[
                Token::Struct {
                    name: "RoleTags",
                    len: 0,
                },
                Token::StructEnd,
            ],
        );
    }
}
