use crate::id::{marker::GuildMarker, Id};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct GuildDelete {
    pub id: Id<GuildMarker>,
    /// If `None` the user was removed from the guild.
    pub unavailable: Option<bool>,
}

#[cfg(test)]
mod tests {
    use super::super::GuildDelete;
    use crate::id::Id;
    use serde_test::Token;

    #[test]
    fn guild_delete_available() {
        let expected = GuildDelete {
            id: Id::new(123),
            unavailable: Some(true),
        };

        serde_test::assert_de_tokens(
            &expected,
            &[
                Token::Struct {
                    name: "GuildDelete",
                    len: 2,
                },
                Token::Str("id"),
                Token::NewtypeStruct { name: "Id" },
                Token::Str("123"),
                Token::Str("unavailable"),
                Token::Some,
                Token::Bool(true),
                Token::StructEnd,
            ],
        );
        serde_test::assert_ser_tokens(
            &expected,
            &[
                Token::Struct {
                    name: "GuildDelete",
                    len: 2,
                },
                Token::Str("id"),
                Token::NewtypeStruct { name: "Id" },
                Token::Str("123"),
                Token::Str("unavailable"),
                Token::Some,
                Token::Bool(true),
                Token::StructEnd,
            ],
        );
    }

    #[test]
    fn guild_delete_unavailable() {
        let expected = GuildDelete {
            id: Id::new(123),
            unavailable: Some(false),
        };

        serde_test::assert_de_tokens(
            &expected,
            &[
                Token::Struct {
                    name: "GuildDelete",
                    len: 2,
                },
                Token::Str("id"),
                Token::NewtypeStruct { name: "Id" },
                Token::Str("123"),
                Token::Str("unavailable"),
                Token::Some,
                Token::Bool(false),
                Token::StructEnd,
            ],
        );
        serde_test::assert_ser_tokens(
            &expected,
            &[
                Token::Struct {
                    name: "GuildDelete",
                    len: 2,
                },
                Token::Str("id"),
                Token::NewtypeStruct { name: "Id" },
                Token::Str("123"),
                Token::Str("unavailable"),
                Token::Some,
                Token::Bool(false),
                Token::StructEnd,
            ],
        );
    }

    #[test]
    fn guild_delete_unavailable_null_default() {
        let expected = GuildDelete {
            id: Id::new(123),
            unavailable: None,
        };

        serde_test::assert_de_tokens(
            &expected,
            &[
                Token::Struct {
                    name: "GuildDelete",
                    len: 1,
                },
                Token::Str("id"),
                Token::NewtypeStruct { name: "Id" },
                Token::Str("123"),
                Token::StructEnd,
            ],
        );
    }
}
