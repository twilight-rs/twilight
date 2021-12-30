use crate::id::{marker::GuildMarker, Id};
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
    fn test_unavailable_guild() {
        let value = UnavailableGuild {
            id: Id::new_checked(1).expect("non zero"),
        };

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
