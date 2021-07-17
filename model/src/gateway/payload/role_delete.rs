use crate::id::{GuildId, RoleId};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct RoleDelete {
    pub guild_id: GuildId,
    pub role_id: RoleId,
}

#[cfg(test)]
mod tests {
    use super::{GuildId, RoleDelete, RoleId};
    use serde_test::Token;

    #[test]
    fn test_webhooks_update() {
        let value = RoleDelete {
            guild_id: GuildId::new(1).expect("non zero"),
            role_id: RoleId::new(2).expect("non zero"),
        };

        serde_test::assert_tokens(
            &value,
            &[
                Token::Struct {
                    name: "RoleDelete",
                    len: 2,
                },
                Token::Str("guild_id"),
                Token::NewtypeStruct { name: "GuildId" },
                Token::Str("1"),
                Token::Str("role_id"),
                Token::NewtypeStruct { name: "RoleId" },
                Token::Str("2"),
                Token::StructEnd,
            ],
        );
    }
}
