use crate::id::EmojiId;
use serde::{Deserialize, Serialize};

// HACK: Hack needed until this is supported: https://github.com/serde-rs/serde/issues/368
fn false_default() -> bool {
    false
}

#[derive(Clone, Debug, Eq, Hash, PartialEq, Deserialize, Serialize)]
#[serde(untagged)]
pub enum ReactionType {
    Custom {
        #[serde(default = "false_default")]
        animated: bool,
        // Even though it says that the id can be nil in the docs,
        // it is a bit misleading as that should only happen when
        // the reaction is a unicode emoji and then it is caught by
        // the other variant.
        id: EmojiId,
        // Name is nil if the emoji data is no longer avaiable, for
        // example if the emoji have been deleted off the guild.
        name: Option<String>,
    },
    Unicode {
        name: String,
    },
}

#[cfg(test)]
mod tests {
    use super::ReactionType;
    use crate::id::EmojiId;
    use serde_test::Token;

    #[test]
    fn test_custom() {
        let kind = ReactionType::Custom {
            animated: false,
            id: EmojiId(1337),
            name: Some("foo".to_owned()),
        };

        serde_test::assert_tokens(
            &kind,
            &[
                Token::Struct {
                    name: "ReactionType",
                    len: 3,
                },
                Token::Str("animated"),
                Token::Bool(false),
                Token::Str("id"),
                Token::NewtypeStruct { name: "EmojiId" },
                Token::Str("1337"),
                Token::Str("name"),
                Token::Some,
                Token::Str("foo"),
                Token::StructEnd,
            ],
        );

        // When `animated` isn't present in the payload it should default to
        // `false`.
        serde_test::assert_de_tokens(
            &kind,
            &[
                Token::Struct {
                    name: "ReactionType",
                    len: 2,
                },
                Token::Str("id"),
                Token::NewtypeStruct { name: "EmojiId" },
                Token::Str("1337"),
                Token::Str("name"),
                Token::Some,
                Token::Str("foo"),
                Token::StructEnd,
            ],
        );
    }

    #[test]
    fn test_unicode() {
        let kind = ReactionType::Unicode {
            name: "\u{1f643}".to_owned(),
        };

        serde_test::assert_tokens(
            &kind,
            &[
                Token::Struct {
                    name: "ReactionType",
                    len: 1,
                },
                Token::Str("name"),
                Token::Str("\u{1f643}"),
                Token::StructEnd,
            ],
        );
    }
}
