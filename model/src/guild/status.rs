use crate::guild::{Guild, UnavailableGuild};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(untagged)]
pub enum GuildStatus {
    Online(Guild),
    Offline(UnavailableGuild),
}

#[cfg(test)]
mod tests {
    use super::{GuildStatus, UnavailableGuild};
    use crate::id::GuildId;
    use serde_test::Token;

    // Notably, the important thing to test is that it's untagged.
    #[test]
    fn test_guild_status() {
        let value = GuildStatus::Offline(UnavailableGuild {
            id: GuildId(1),
            unavailable: true,
        });

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
