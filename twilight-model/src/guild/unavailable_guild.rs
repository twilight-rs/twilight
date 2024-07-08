use crate::{
    id::{marker::GuildMarker, Id},
    util::mustbe::MustBeBool,
};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct UnavailableGuild {
    pub id: Id<GuildMarker>,
    pub unavailable: MustBeBool<true>,
}

#[cfg(test)]
mod tests {
    use super::UnavailableGuild;
    use crate::{id::Id, util::mustbe::MustBeBool};
    use serde_test::Token;

    #[test]
    fn unavailable_guild() {
        let value = UnavailableGuild {
            id: Id::new(1),
            unavailable: MustBeBool,
        };

        serde_test::assert_tokens(
            &value,
            &[
                Token::Struct {
                    name: "UnavailableGuild",
                    len: 2,
                },
                Token::Str("id"),
                Token::NewtypeStruct { name: "Id" },
                Token::Str("1"),
                Token::Str("unavailable"),
                Token::Bool(true),
                Token::StructEnd,
            ],
        );
    }
}
