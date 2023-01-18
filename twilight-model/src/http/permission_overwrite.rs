//! Models for sending permission overwrites to Discord.

use crate::{
    guild::Permissions,
    id::{marker::GenericMarker, Id},
};
use serde::{Deserialize, Serialize};
use std::fmt::{Debug, Formatter, Result as FmtResult};

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
#[derive(Clone, Copy, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct PermissionOverwriteType(u8);

impl PermissionOverwriteType {
    /// Permission overwrite targets an individual member.
    pub const MEMBER: Self = Self::new(1);

    /// Permission overwrite targets an individual role.
    pub const ROLE: Self = Self::new(0);

    /// Create a new permission overwrite from a dynamic value.
    ///
    /// The provided value isn't validated. Known valid values are associated
    /// constants such as [`MEMBER`][`Self::MEMBER`].
    pub const fn new(permission_overwrite: u8) -> Self {
        Self(permission_overwrite)
    }

    /// Retrieve the value of the permission overwrite.
    ///
    /// # Examples
    ///
    /// ```
    /// use twilight_model::http::permission_overwrite::PermissionOverwriteType;
    ///
    /// assert_eq!(0, PermissionOverwriteType::ROLE.get());
    /// ```
    pub const fn get(&self) -> u8 {
        self.0
    }

    /// Name of the associated constant.
    ///
    /// Returns `None` if the value doesn't have a defined constant.
    pub const fn name(self) -> Option<&'static str> {
        Some(match self {
            Self::MEMBER => "MEMBER",
            Self::ROLE => "ROLE",
            _ => return None,
        })
    }
}

impl Debug for PermissionOverwriteType {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        if let Some(name) = self.name() {
            f.debug_struct("PermissionOverwriteType")
                .field("name", &name)
                .field("value", &self.0)
                .finish()
        } else {
            f.debug_tuple("PermissionOverwriteType")
                .field(&self.0)
                .finish()
        }
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

    #[test]
    fn overwrite() {
        let value = PermissionOverwrite {
            allow: Some(Permissions::CREATE_INVITE),
            deny: Some(Permissions::KICK_MEMBERS),
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
                Token::Some,
                Token::Str("1"),
                Token::Str("deny"),
                Token::Some,
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
            allow: Some(Permissions::CREATE_INVITE),
            deny: Some(Permissions::KICK_MEMBERS),
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
                Token::Some,
                Token::Str("1"),
                Token::Str("deny"),
                Token::Some,
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
    fn overwrite_type() {
        const MAP: &[(PermissionOverwriteType, u8)] = &[
            (PermissionOverwriteType::MEMBER, 1),
            (PermissionOverwriteType::ROLE, 0),
        ];

        for (kind, num) in MAP {
            serde_test::assert_tokens(
                kind,
                &[
                    Token::NewtypeStruct {
                        name: "PermissionOverwriteType",
                    },
                    Token::U8(*num),
                ],
            );
            assert_eq!(*kind, PermissionOverwriteType::from(*num));
            assert_eq!(*num, kind.get());
        }
    }
}
