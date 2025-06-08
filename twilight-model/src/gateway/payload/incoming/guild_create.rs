use crate::{
    guild::{Guild, UnavailableGuild},
    id::{marker::GuildMarker, Id},
};
use serde::{Deserialize, Serialize};

// Developer note: Do not change order as we want unavailable to fail
// first.
#[allow(clippy::large_enum_variant)]
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(untagged)]
pub enum GuildCreate {
    Available(Guild),
    Unavailable(UnavailableGuild),
}

impl GuildCreate {
    /// ID of the guild.
    pub const fn id(&self) -> Id<GuildMarker> {
        match self {
            GuildCreate::Available(g) => g.id,
            GuildCreate::Unavailable(g) => g.id,
        }
    }
}

#[cfg(test)]
mod tests {
    use serde_test::Token;

    use crate::{guild::UnavailableGuild, id::Id};

    use super::GuildCreate;

    #[test]
    fn unavailable_guild() {
        let expected = GuildCreate::Unavailable(UnavailableGuild {
            id: Id::new(1234),
            unavailable: true,
        });

        // Note: serde(untagged) makes the enum transparent which is
        //       the reason we don't use the variant here.
        serde_test::assert_tokens(
            &expected,
            &[
                Token::Struct {
                    name: "UnavailableGuild",
                    len: 2,
                },
                Token::Str("id"),
                Token::NewtypeStruct { name: "Id" },
                Token::Str("1234"),
                Token::Str("unavailable"),
                Token::Bool(true),
                Token::StructEnd,
            ],
        );
    }
}
