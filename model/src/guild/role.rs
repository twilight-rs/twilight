use super::RoleTags;
use crate::{guild::Permissions, id::RoleId};
use serde::{Deserialize, Serialize};
use std::cmp::{Ord, Ordering, PartialOrd};

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct Role {
    pub color: u32,
    pub hoist: bool,
    pub id: RoleId,
    pub managed: bool,
    pub mentionable: bool,
    pub name: String,
    pub permissions: Permissions,
    pub position: i64,
    /// Tags about the role.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<RoleTags>,
}

impl Ord for Role {
    /// Roles are ordered primarily by position. Discord does not provide guarentees that role
    /// positions are unique, positive, or contiguous. In the case of equal positions, order is
    /// based on the roles' IDs instead.
    fn cmp(&self, other: &Self) -> Ordering {
        match self.position.cmp(&other.position) {
            Ordering::Equal => self.id.0.cmp(&other.id.0),
            ordering => ordering,
        }
    }
}

impl PartialOrd for Role {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[cfg(test)]
mod tests {
    use super::{Permissions, Role, RoleId};
    use serde_test::Token;
    use std::cmp::Ordering;

    #[test]
    fn test_role() {
        let role = Role {
            color: 0,
            hoist: true,
            id: RoleId(123),
            managed: false,
            mentionable: true,
            name: "test".to_owned(),
            permissions: Permissions::ADMINISTRATOR,
            position: 12,
            tags: None,
        };

        serde_test::assert_tokens(
            &role,
            &[
                Token::Struct {
                    name: "Role",
                    len: 8,
                },
                Token::Str("color"),
                Token::U32(0),
                Token::Str("hoist"),
                Token::Bool(true),
                Token::Str("id"),
                Token::NewtypeStruct { name: "RoleId" },
                Token::Str("123"),
                Token::Str("managed"),
                Token::Bool(false),
                Token::Str("mentionable"),
                Token::Bool(true),
                Token::Str("name"),
                Token::Str("test"),
                Token::Str("permissions"),
                Token::Str("8"),
                Token::Str("position"),
                Token::I64(12),
                Token::StructEnd,
            ],
        );
    }

    #[test]
    fn test_role_ordering() {
        let role_a = Role {
            color: 0,
            hoist: true,
            id: RoleId(123),
            managed: false,
            mentionable: true,
            name: "test".to_owned(),
            permissions: Permissions::ADMINISTRATOR,
            position: 12,
            tags: None,
        };

        let role_b = Role {
            color: 0,
            hoist: true,
            id: RoleId(456),
            managed: false,
            mentionable: true,
            name: "test".to_owned(),
            permissions: Permissions::ADMINISTRATOR,
            position: 120,
            tags: None,
        };

        assert_eq!(Ordering::Less, role_a.cmp(&role_b));
        assert_eq!(Ordering::Greater, role_b.cmp(&role_a));
        assert_eq!(Ordering::Equal, role_a.cmp(&role_a));
        assert_eq!(Ordering::Equal, role_b.cmp(&role_b));
        assert_eq!(Some(Ordering::Less), role_a.partial_cmp(&role_b));
        assert_eq!(Some(Ordering::Greater), role_b.partial_cmp(&role_a));
        assert_eq!(Some(Ordering::Equal), role_a.partial_cmp(&role_a));
        assert_eq!(Some(Ordering::Equal), role_b.partial_cmp(&role_b));
    }

    #[test]
    fn test_role_ordering_equal_position() {
        let role_a = Role {
            color: 0,
            hoist: true,
            id: RoleId(123),
            managed: false,
            mentionable: true,
            name: "test".to_owned(),
            permissions: Permissions::ADMINISTRATOR,
            position: 12,
            tags: None,
        };

        let role_b = Role {
            color: 0,
            hoist: true,
            id: RoleId(456),
            managed: false,
            mentionable: true,
            name: "test".to_owned(),
            permissions: Permissions::ADMINISTRATOR,
            position: 12,
            tags: None,
        };

        assert_eq!(Ordering::Less, role_a.cmp(&role_b));
        assert_eq!(Ordering::Greater, role_b.cmp(&role_a));
        assert_eq!(Ordering::Equal, role_a.cmp(&role_a));
        assert_eq!(Ordering::Equal, role_b.cmp(&role_b));
        assert_eq!(Some(Ordering::Less), role_a.partial_cmp(&role_b));
        assert_eq!(Some(Ordering::Greater), role_b.partial_cmp(&role_a));
        assert_eq!(Some(Ordering::Equal), role_a.partial_cmp(&role_a));
        assert_eq!(Some(Ordering::Equal), role_b.partial_cmp(&role_b));
    }
}
