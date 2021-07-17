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
    /// Compare two roles to each other using their position and ID.
    ///
    /// Roles are primarily ordered by their position in descending order. For example,
    /// a role with a position of 17 is considered a higher role than one with a
    /// position of 12.
    ///
    /// Discord does not guarantee that role positions are positive, unique, or contiguous. When
    /// two or more roles have the same position then the order is based on the roles' IDs in
    /// ascending order. For example, given two roles with positions of 10 then a role
    /// with an ID of 1 would be considered a higher role than one with an ID of 20.
    ///
    /// ### Examples
    ///
    /// Compare the position of two roles:
    ///
    /// ```rust
    /// # use twilight_model::{guild::{Permissions, Role}, id::RoleId};
    /// # use std::cmp::Ordering;
    /// let role_a = Role {
    ///     id: RoleId::new(123).expect("non zero"),
    ///     position: 12,
    ///#    color: 0,
    ///#    hoist: true,
    ///#    managed: false,
    ///#    mentionable: true,
    ///#    name: "test".to_owned(),
    ///#    permissions: Permissions::ADMINISTRATOR,
    ///#    tags: None,
    ///     // ...
    /// };
    /// let role_b = Role {
    ///     id: RoleId::new(456).expect("non zero"),
    ///     position: 13,
    ///#    color: 0,
    ///#    hoist: true,
    ///#    managed: false,
    ///#    mentionable: true,
    ///#    name: "test".to_owned(),
    ///#    permissions: Permissions::ADMINISTRATOR,
    ///#    tags: None,
    ///     // ...
    /// };
    /// assert_eq!(Ordering::Less, role_a.cmp(&role_b));
    /// assert_eq!(Ordering::Greater, role_b.cmp(&role_a));
    /// assert_eq!(Ordering::Equal, role_a.cmp(&role_a));
    /// assert_eq!(Ordering::Equal, role_b.cmp(&role_b));
    /// ```
    ///
    /// Compare the position of two roles with the same position:
    ///
    /// ```rust
    /// # use twilight_model::{guild::{Permissions, Role}, id::RoleId};
    /// # use std::cmp::Ordering;
    /// let role_a = Role {
    ///     id: RoleId::new(123).expect("non zero"),
    ///     position: 12,
    ///#    color: 0,
    ///#    hoist: true,
    ///#    managed: false,
    ///#    mentionable: true,
    ///#    name: "test".to_owned(),
    ///#    permissions: Permissions::ADMINISTRATOR,
    ///#    tags: None,
    /// };
    /// let role_b = Role {
    ///     id: RoleId::new(456).expect("non zero"),
    ///     position: 12,
    ///#    color: 0,
    ///#    hoist: true,
    ///#    managed: false,
    ///#    mentionable: true,
    ///#    name: "test".to_owned(),
    ///#    permissions: Permissions::ADMINISTRATOR,
    ///#    tags: None,
    /// };
    /// assert_eq!(Ordering::Less, role_a.cmp(&role_b));
    /// assert_eq!(Ordering::Greater, role_b.cmp(&role_a));
    /// assert_eq!(Ordering::Equal, role_a.cmp(&role_a));
    /// assert_eq!(Ordering::Equal, role_b.cmp(&role_b));
    /// ```
    fn cmp(&self, other: &Self) -> Ordering {
        self.position
            .cmp(&other.position)
            .then(self.id.0.cmp(&other.id.0))
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

    #[test]
    fn test_role() {
        let role = Role {
            color: 0,
            hoist: true,
            id: RoleId::new(123).expect("non zero"),
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
}
