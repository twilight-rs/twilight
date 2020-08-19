use crate::{
    guild::{Guild, UnavailableGuild},
    id::GuildId,
};
use serde::{Deserialize, Serialize};
use serde_mappable_seq::Key;

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(untagged)]
pub enum GuildStatus {
    Online(Guild),
    Offline(UnavailableGuild),
}

impl Key<'_, GuildId> for GuildStatus {
    fn key(&self) -> GuildId {
        match self {
            Self::Online(g) => g.id,
            Self::Offline(u) => u.id,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{GuildId, GuildStatus, UnavailableGuild};
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
