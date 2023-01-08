use crate::{
    guild::Permissions,
    id::{marker::GenericMarker, Id},
};
use serde::{Deserialize, Serialize};

/// Permission overwrite data for a role or member.
#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct PermissionOverwrite {
    pub allow: Permissions,
    pub deny: Permissions,
    pub id: Id<GenericMarker>,
    #[serde(rename = "type")]
    pub kind: PermissionOverwriteType,
}

/// Type of a permission overwrite target.
// Keep in sync with `twilight_util::permission_calculator::PermissionCalculator`!
#[derive(Clone, Copy, Debug, Serialize, Eq, Hash, PartialEq, Deserialize)]
pub struct PermissionOverwriteType(u8);

impl PermissionOverwriteType {
    /// Permission overwrite targets an individual member.
    pub const MEMBER: Self = Self::new(1);

    /// Permission overwrite targets an individual role.
    pub const ROLE: Self = Self::new(0);

    /// Create a new permission overwrite type from a dynamic value.
    ///
    /// The provided value isn't validated. Known valid values are associated
    /// constants such as [`MEMBER`][`Self::MEMBER`].
    pub const fn new(permission_overwrite_type: u8) -> Self {
        Self(permission_overwrite_type)
    }

    /// Retrieve the value of the permission overwrite type.
    ///
    /// # Examples
    ///
    /// ```
    /// use twilight_model::channel::permission_overwrite::PermissionOverwriteType;
    ///
    /// assert_eq!(0, PermissionOverwriteType::ROLE.get());
    /// ```
    pub const fn get(&self) -> u8 {
        self.0
    }
}

impl From<u8> for PermissionOverwriteType {
    fn from(value: u8) -> Self {
        Self(value)
    }
}

impl From<PermissionOverwriteType> for u8 {
    fn from(value: PermissionOverwriteType) -> Self {
        value.get()
    }
}

#[cfg(test)]
mod tests {
    use super::{PermissionOverwrite, PermissionOverwriteType, Permissions};
    use crate::id::Id;
    use serde::{Deserialize, Serialize};
    use serde_test::Token;
    use static_assertions::{assert_fields, assert_impl_all};
    use std::{fmt::Debug, hash::Hash};

    assert_fields!(PermissionOverwrite: allow, deny, id, kind);
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

    #[test]
    fn overwrite() {
        let value = PermissionOverwrite {
            allow: Permissions::CREATE_INVITE,
            deny: Permissions::KICK_MEMBERS,
            id: Id::new(12_345_678),
            kind: PermissionOverwriteType::MEMBER,
        };

        serde_test::assert_tokens(
            &value,
            &[
                Token::Struct {
                    name: "PermissionOverwrite",
                    len: 4,
                },
                Token::Str("allow"),
                Token::Str("1"),
                Token::Str("deny"),
                Token::Str("2"),
                Token::Str("id"),
                Token::NewtypeStruct { name: "Id" },
                Token::Str("12345678"),
                Token::Str("type"),
                Token::NewtypeStruct {
                    name: "PermissionOverwriteType",
                },
                Token::U8(PermissionOverwriteType::MEMBER.get()),
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
            allow: Permissions::CREATE_INVITE,
            deny: Permissions::KICK_MEMBERS,
            id: Id::new(1),
            kind: PermissionOverwriteType::MEMBER,
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
                Token::Str("1"),
                Token::Str("deny"),
                Token::Str("2"),
                Token::Str("id"),
                Token::NewtypeStruct { name: "Id" },
                Token::Str("1"),
                Token::Str("type"),
                Token::NewtypeStruct {
                    name: "PermissionOverwriteType",
                },
                Token::U8(1),
                Token::StructEnd,
            ],
        );
    }

    #[test]
    fn overwrite_type_name() {
        serde_test::assert_tokens(
            &PermissionOverwriteType::MEMBER,
            &[
                Token::NewtypeStruct {
                    name: "PermissionOverwriteType",
                },
                Token::U8(1),
            ],
        );
        serde_test::assert_tokens(
            &PermissionOverwriteType::ROLE,
            &[
                Token::NewtypeStruct {
                    name: "PermissionOverwriteType",
                },
                Token::U8(0),
            ],
        );
    }
}
