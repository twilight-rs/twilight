use crate::id::EmojiId;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Eq, Hash, PartialEq, Deserialize, Serialize)]
#[serde(untagged)]
pub enum ReactionType {
    Custom {
        #[serde(default)]
        animated: bool,
        // Even though it says that the id can be nil in the docs,
        // it is a bit misleading as that should only happen when
        // the reaction is a unicode emoji and then it is caught by
        // the other variant.
        id: EmojiId,
        // Name is nil if the emoji data is no longer available, for
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
        let value = ReactionType::Custom {
            animated: false,
            id: EmojiId::new(1337).expect("non zero"),
            name: Some("foo".to_owned()),
        };

        serde_test::assert_tokens(
            &value,
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
            &value,
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
        let value = ReactionType::Unicode {
            name: "\u{1f643}".to_owned(),
        };

        serde_test::assert_tokens(
            &value,
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
