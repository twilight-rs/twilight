use crate::id::{Id, marker::GuildMarker};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct UnavailableGuild {
    pub id: Id<GuildMarker>,
}

#[cfg(test)]
mod tests {
    use super::UnavailableGuild;
    use crate::id::Id;
    use serde_test::Token;

    #[test]
    fn unavailable_guild() {
        let value = UnavailableGuild { id: Id::new(1) };

        serde_test::assert_tokens(
            &value,
            &[
                Token::Struct {
                    name: "UnavailableGuild",
                    len: 1,
                },
                Token::Str("id"),
                Token::NewtypeStruct { name: "Id" },
                Token::Str("1"),
                Token::StructEnd,
            ],
        );
    }
}
