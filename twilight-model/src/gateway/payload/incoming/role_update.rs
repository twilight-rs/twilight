use crate::{
    guild::Role,
    id::{marker::GuildMarker, Id},
};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct RoleUpdate {
    pub guild_id: Id<GuildMarker>,
    pub role: Role,
}

#[cfg(test)]
mod tests {
    use super::{Role, RoleUpdate};
    use crate::{
        guild::{Permissions, RoleFlags},
        id::Id,
    };
    use serde_test::Token;

    #[test]
    fn role_update() {
        let value = RoleUpdate {
            guild_id: Id::new(1),
            role: Role {
                color: 0,
                hoist: true,
                icon: None,
                id: Id::new(1),
                managed: false,
                mentionable: false,
                name: "a role".to_owned(),
                permissions: Permissions::SEND_MESSAGES,
                position: 12,
                flags: RoleFlags::empty(),
                tags: None,
                unicode_emoji: None,
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
                Token::NewtypeStruct { name: "Id" },
                Token::Str("1"),
                Token::Str("role"),
                Token::Struct {
                    name: "Role",
                    len: 9,
                },
                Token::Str("color"),
                Token::U32(0),
                Token::Str("hoist"),
                Token::Bool(true),
                Token::Str("id"),
                Token::NewtypeStruct { name: "Id" },
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
                Token::Str("flags"),
                Token::U64(0),
                Token::StructEnd,
                Token::StructEnd,
            ],
        );
    }
}
