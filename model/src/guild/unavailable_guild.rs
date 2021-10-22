use crate::id::GuildId;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct UnavailableGuild {
    pub id: GuildId,
    pub unavailable: bool,
}

#[cfg(test)]
mod tests {
    use super::{GuildId, UnavailableGuild};
    use serde_test::Token;

    #[test]
    fn test_unavailable_guild() {
        let value = UnavailableGuild {
            id: GuildId::new(1).expect("non zero"),
            unavailable: true,
        };

        serde_test::assert_tokens(
            &value,
            &[
                Token::Struct {
                    name: "UnavailableGuild",
                    len: 2,
                },
                Token::Str("id"),
                Token::NewtypeStruct { name: "GuildId" },
                Token::Str("1"),
                Token::Str("unavailable"),
                Token::Bool(true),
                Token::StructEnd,
            ],
        );
    }
}
