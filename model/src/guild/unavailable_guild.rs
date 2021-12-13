use crate::id::{marker, Id};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct UnavailableGuild {
    pub id: Id<marker::Guild>,
    pub unavailable: bool,
}

#[cfg(test)]
mod tests {
    use super::UnavailableGuild;
    use crate::id::Id;
    use serde_test::Token;

    #[test]
    fn test_unavailable_guild() {
        let value = UnavailableGuild {
            id: Id::new(1).expect("non zero"),
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
                Token::NewtypeStruct { name: "Id" },
                Token::Str("1"),
                Token::Str("unavailable"),
                Token::Bool(true),
                Token::StructEnd,
            ],
        );
    }
}
