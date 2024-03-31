use crate::{
    guild::{Guild, UnavailableGuild},
    id::{marker::GuildMarker, Id},
};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(untagged)]
pub enum GuildCreate {
    Available(Guild),
    Unavailable(UnavailableGuild),
}

impl GuildCreate {
    /// Extract guild id.
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

        // Note: This looks a bit strange because it does not use
        //       Token::TupleVariant, this is because it will
        //       serialize back into a struct, and thus make it
        //       fails. This also tests that the enum is transparent
        //       for serde.
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
