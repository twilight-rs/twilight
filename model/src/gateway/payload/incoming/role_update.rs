use crate::{guild::Role, id::GuildId};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct RoleUpdate {
    pub guild_id: GuildId,
    pub role: Role,
}

#[cfg(test)]
mod tests {
    use super::{GuildId, Role, RoleUpdate};
    use crate::{guild::Permissions, id::RoleId};
    use serde_test::Token;

    #[test]
    fn test_role_update() {
        let value = RoleUpdate {
            guild_id: GuildId::new(1).expect("non zero"),
            role: Role {
                color: 0,
                hoist: true,
                id: RoleId::new(1).expect("non zero"),
                managed: false,
                mentionable: false,
                name: "a role".to_owned(),
                permissions: Permissions::SEND_MESSAGES,
                position: 12,
                tags: None,
            },
        };

        serde_test::assert_tokens(
            &value,
            &[
                Token::Struct {
                    name: "RoleUpdate",
                    len: 2,
                },
                Token::Str("guild_id"),
                Token::NewtypeStruct { name: "GuildId" },
                Token::Str("1"),
                Token::Str("role"),
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
                Token::Str("1"),
                Token::Str("managed"),
                Token::Bool(false),
                Token::Str("mentionable"),
                Token::Bool(false),
                Token::Str("name"),
                Token::Str("a role"),
                Token::Str("permissions"),
                Token::Str("2048"),
                Token::Str("position"),
                Token::I64(12),
                Token::StructEnd,
                Token::StructEnd,
            ],
        );
    }
}
