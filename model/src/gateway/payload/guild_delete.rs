use crate::id::GuildId;

#[cfg_attr(
    feature = "serde-support",
    derive(serde::Deserialize, serde::Serialize)
)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GuildDelete {
    pub id: GuildId,
    // If `unavailable` is `None` the user was removed from the guild.
    #[cfg_attr(
        feature = "serde-support",
        serde(default, deserialize_with = "if_serde_support::nullable_unavailable")
    )]
    pub unavailable: bool,
}

#[cfg(feature = "serde-support")]
mod if_serde_support {
    use serde::{Deserialize, Deserializer};

    pub fn nullable_unavailable<'de, D: Deserializer<'de>>(
        deserializer: D,
    ) -> Result<bool, D::Error> {
        Ok(Deserialize::deserialize(deserializer).unwrap_or_default())
    }

    #[cfg(test)]
    mod tests {
        use super::super::GuildDelete;
        use crate::id::GuildId;
        use serde_json::json;
        use serde_test::Token;

        #[test]
        fn test_guild_delete_available() {
            let expected = GuildDelete {
                id: GuildId(123),
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
                    Token::NewtypeStruct { name: "GuildId" },
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
                    Token::NewtypeStruct { name: "GuildId" },
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
                id: GuildId(123),
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
                    Token::NewtypeStruct { name: "GuildId" },
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
                    Token::NewtypeStruct { name: "GuildId" },
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
                id: GuildId(123),
                unavailable: false,
            };

            assert_eq!(
                expected,
                serde_json::from_value(json!({
                    "id": "123",
                    "unavailable": null,
                }))
                .unwrap()
            );
        }
    }
}
