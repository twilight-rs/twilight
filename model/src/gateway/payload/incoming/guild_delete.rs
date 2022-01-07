use crate::id::{marker::GuildMarker, Id};
use serde::{Deserialize, Deserializer, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct GuildDelete {
    pub id: Id<GuildMarker>,
    // If `unavailable` is `None` the user was removed from the guild.
    #[serde(default, deserialize_with = "nullable_unavailable")]
    pub unavailable: bool,
}

#[allow(clippy::unnecessary_wraps)]
fn nullable_unavailable<'de, D: Deserializer<'de>>(deserializer: D) -> Result<bool, D::Error> {
    Ok(Deserialize::deserialize(deserializer).unwrap_or_default())
}

#[cfg(test)]
mod tests {
    use super::super::GuildDelete;
    use crate::id::Id;
    use serde_test::Token;

    #[test]
    fn test_guild_delete_available() {
        let expected = GuildDelete {
            id: Id::new(123),
            unavailable: true,
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
                Token::Bool(true),
                Token::StructEnd,
            ],
        );
    }

    #[test]
    fn test_guild_delete_unavailable() {
        let expected = GuildDelete {
            id: Id::new(123),
            unavailable: false,
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
                Token::Bool(false),
                Token::StructEnd,
            ],
        );
    }

    #[test]
    fn test_guild_delete_unavailable_null_default() {
        let expected = GuildDelete {
            id: Id::new(123),
            unavailable: false,
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
                Token::None,
                Token::StructEnd,
            ],
        );
    }
}
