use super::RoleTags;
use crate::{guild::Permissions, id::RoleId};
use serde::{Deserialize, Serialize};

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

#[cfg(test)]
mod tests {
    use super::{Permissions, Role, RoleId};
    use serde_test::Token;

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
}
