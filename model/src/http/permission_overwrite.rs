//! Models for sending permission overwrites to Discord.

use crate::{
    guild::Permissions,
    id::{marker::GenericMarker, Id},
};
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

/// Permission overwrite data for a role or member.
#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct PermissionOverwrite {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow: Option<Permissions>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deny: Option<Permissions>,
    pub id: Id<GenericMarker>,
    #[serde(rename = "type")]
    pub kind: PermissionOverwriteType,
}

/// Type of a permission overwrite target.
#[derive(Clone, Copy, Debug, Deserialize_repr, Eq, Hash, PartialEq, Serialize_repr)]
#[repr(u8)]
#[serde(rename_all = "snake_case")]
pub enum PermissionOverwriteType {
    /// Permission overwrite targets an individual member.
    Member = 1,
    /// Permission overwrite targets an individual role.
    Role = 0,
}

#[cfg(test)]
mod tests {
    use super::{PermissionOverwrite, PermissionOverwriteType, Permissions};
    use crate::id::Id;
    use serde::{Deserialize, Serialize};
    use serde_test::Token;
    use static_assertions::{assert_fields, assert_impl_all, const_assert_eq};
    use std::{fmt::Debug, hash::Hash};

    assert_fields!(PermissionOverwrite: allow, deny, kind);
    assert_impl_all!(
        PermissionOverwrite: Clone,
        Debug,
        Deserialize<'static>,
        Eq,
        Hash,
        PartialEq,
        Send,
        Serialize,
        Sync
    );
    assert_impl_all!(
        PermissionOverwriteType: Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        PartialEq,
        Send,
        Sync
    );
    const_assert_eq!(0, PermissionOverwriteType::Role as u8);
    const_assert_eq!(1, PermissionOverwriteType::Member as u8);

    #[test]
    fn overwrite() {
        let value = PermissionOverwrite {
            allow: Some(Permissions::CREATE_INVITE),
            deny: Some(Permissions::KICK_MEMBERS),
            id: Id::new(12_345_678),
            kind: PermissionOverwriteType::Member,
        };

        serde_test::assert_tokens(
            &value,
            &[
                Token::Struct {
                    name: "PermissionOverwrite",
                    len: 4,
                },
                Token::Str("allow"),
                Token::Some,
                Token::Str("1"),
                Token::Str("deny"),
                Token::Some,
                Token::Str("2"),
                Token::Str("id"),
                Token::NewtypeStruct { name: "Id" },
                Token::Str("12345678"),
                Token::Str("type"),
                Token::U8(PermissionOverwriteType::Member as u8),
                Token::StructEnd,
            ],
        );
    }

    #[test]
    fn blank_overwrite() {
        // Test integer deser used in guild templates.
        let raw = r#"{
  "allow": "1",
  "deny": "2",
  "id": 1,
  "type": 1
}"#;

        let value = PermissionOverwrite {
            allow: Some(Permissions::CREATE_INVITE),
            deny: Some(Permissions::KICK_MEMBERS),
            id: Id::new(1),
            kind: PermissionOverwriteType::Member,
        };

        let deserialized = serde_json::from_str::<PermissionOverwrite>(raw).unwrap();

        assert_eq!(deserialized, value);

        serde_test::assert_tokens(
            &value,
            &[
                Token::Struct {
                    name: "PermissionOverwrite",
                    len: 4,
                },
                Token::Str("allow"),
                Token::Some,
                Token::Str("1"),
                Token::Str("deny"),
                Token::Some,
                Token::Str("2"),
                Token::Str("id"),
                Token::NewtypeStruct { name: "Id" },
                Token::Str("1"),
                Token::Str("type"),
                Token::U8(1),
                Token::StructEnd,
            ],
        );
    }

    #[test]
    fn overwrite_type_name() {
        serde_test::assert_tokens(&PermissionOverwriteType::Member, &[Token::U8(1)]);
        serde_test::assert_tokens(&PermissionOverwriteType::Role, &[Token::U8(0)]);
    }
}
