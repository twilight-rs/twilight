use crate::{guild::Permissions, id::GuildId};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct GuildInfo {
    pub icon: Option<String>,
    pub id: GuildId,
    pub name: String,
    pub owner: bool,
    pub permissions: Permissions,
}

#[cfg(test)]
mod tests {
    use super::{GuildId, GuildInfo, Permissions};
    use serde_test::Token;

    #[test]
    fn test_guild_info() {
        let value = GuildInfo {
            icon: Some("icon hash".to_owned()),
            id: GuildId::new(1).expect("non zero"),
            name: "guild name".to_owned(),
            owner: false,
            permissions: Permissions::MANAGE_CHANNELS,
        };

        serde_test::assert_tokens(
            &value,
            &[
                Token::Struct {
                    name: "GuildInfo",
                    len: 5,
                },
                Token::Str("icon"),
                Token::Some,
                Token::Str("icon hash"),
                Token::Str("id"),
                Token::NewtypeStruct { name: "GuildId" },
                Token::Str("1"),
                Token::Str("name"),
                Token::Str("guild name"),
                Token::Str("owner"),
                Token::Bool(false),
                Token::Str("permissions"),
                Token::Str("16"),
                Token::StructEnd,
            ],
        );
    }
}
