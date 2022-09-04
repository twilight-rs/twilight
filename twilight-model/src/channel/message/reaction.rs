use crate::id::{marker::EmojiMarker, Id};
use serde::{Deserialize, Serialize};

/// Message reaction struct.
#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct Reaction {
    /// Amount of reactions this emoji has.
    pub count: u64,
    /// Emoji of this reaction.
    pub emoji: ReactionType,
    /// Whether the current user has reacted with this emoji.
    pub me: bool,
}

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
        id: Id<EmojiMarker>,
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
    use super::{Reaction, ReactionType};
    use crate::id::Id;
    use serde_test::Token;

    #[test]
    fn message_reaction_unicode() {
        let value = Reaction {
            count: 7,
            emoji: ReactionType::Unicode {
                name: "a".to_owned(),
            },
            me: true,
        };

        serde_test::assert_tokens(
            &value,
            &[
                Token::Struct {
                    name: "Reaction",
                    len: 3,
                },
                Token::Str("count"),
                Token::U64(7),
                Token::Str("emoji"),
                Token::Struct {
                    name: "ReactionType",
                    len: 1,
                },
                Token::Str("name"),
                Token::Str("a"),
                Token::StructEnd,
                Token::Str("me"),
                Token::Bool(true),
                Token::StructEnd,
            ],
        );
    }

    #[test]
    fn custom() {
        let value = ReactionType::Custom {
            animated: false,
            id: Id::new(1337),
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
                Token::NewtypeStruct { name: "Id" },
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
                Token::NewtypeStruct { name: "Id" },
                Token::Str("1337"),
                Token::Str("name"),
                Token::Some,
                Token::Str("foo"),
                Token::StructEnd,
            ],
        );
    }

    #[test]
    fn unicode() {
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
