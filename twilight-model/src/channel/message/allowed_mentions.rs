use crate::{
    id::{
        marker::{RoleMarker, UserMarker},
        Id,
    },
    util::is_false,
};
use serde::{Deserialize, Serialize};

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
#[cfg_attr(
    feature = "rkyv",
    derive(rkyv::Archive, rkyv::Deserialize, rkyv::Serialize)
)]
pub struct AllowedMentions {
    /// List of allowed mention types.
    ///
    /// [`MentionType::Roles`] and [`MentionType::Users`] allows all roles and users to be
    /// mentioned; they are mutually exclusive with the [`roles`] and [`users`] fields.
    ///
    /// [`roles`]: Self::roles
    /// [`users`]: Self::users
    #[serde(default)]
    #[cfg_attr(feature = "rkyv", with(rkyv::with::CopyOptimize))]
    pub parse: Vec<MentionType>,
    /// For replies, whether to mention the message author.
    ///
    /// Defaults to false.
    #[serde(default, skip_serializing_if = "is_false")]
    pub replied_user: bool,
    /// List of roles to mention.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[cfg_attr(feature = "rkyv", with(rkyv::with::CopyOptimize))]
    pub roles: Vec<Id<RoleMarker>>,
    /// List of users to mention.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[cfg_attr(feature = "rkyv", with(rkyv::with::CopyOptimize))]
    pub users: Vec<Id<UserMarker>>,
}

/// Allowed mention type.
#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
#[non_exhaustive]
#[serde(rename_all = "lowercase")]
#[cfg_attr(
    feature = "rkyv",
    derive(rkyv::Archive, rkyv::Deserialize, rkyv::Serialize),
    archive(as = "Self")
)]
pub enum MentionType {
    /// `@everyone` and `@here` mentions.
    Everyone,
    /// Role mentions.
    Roles,
    /// User mentions.
    Users,
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
            parse: Vec::from([MentionType::Everyone]),
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
                    name: "MentionType",
                    variant: "everyone",
                },
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
