use crate::{
    id::{marker::EmojiMarker, Id},
    util::HexColor,
};
use serde::{Deserialize, Serialize};

/// Reaction below a message.
#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct Reaction {
    /// HEX colors used for super reaction.
    pub burst_colors: Vec<HexColor>,
    /// Amount of reactions this emoji has.
    pub count: u64,
    /// Reaction count details for each type of reaction.
    pub count_details: ReactionCountDetails,
    /// Emoji of this reaction.
    pub emoji: ReactionType,
    /// Whether the current user has reacted with this emoji.
    pub me: bool,
    /// Whether the current user super-reacted using this emoji
    pub me_burst: bool,
}

/// Type of [`Reaction`].
#[derive(Clone, Debug, Eq, Hash, PartialEq, Deserialize, Serialize)]
#[serde(untagged)]
pub enum ReactionType {
    /// Custom [`Emoji`].
    ///
    /// [`Emoji`]: crate::guild::Emoji
    Custom {
        /// Whether the emoji is animated.
        #[serde(default)]
        animated: bool,
        /// Emoji identifier.
        // Even though it says that the id can be nil in the docs,
        // it is a bit misleading as that should only happen when
        // the reaction is a unicode emoji and then it is caught by
        // the other variant.
        id: Id<EmojiMarker>,
        /// Emoji name.
        // Name is nil if the emoji data is no longer available, for
        // example if the emoji have been deleted off the guild.
        name: Option<String>,
    },
    /// Standard [Unicode] emoji value.
    ///
    /// Unicode reactions must be specified by their unicode value, and *not*
    /// their Discord display name. Instead of using `:arrow_right:`, use "➡️".
    ///
    /// [Unicode]: https://unicode.org/emoji/
    Unicode {
        /// Unicode name identifier.
        name: String,
    },
}

/// Breakdown of normal and super reaction counts for the associated emoji.
#[derive(Clone, Debug, Eq, Hash, PartialEq, Deserialize, Serialize)]
pub struct ReactionCountDetails {
    /// Count of super reactions.
    pub burst: u64,
    /// Count of normal reactions.
    pub normal: u64,
}

#[cfg(test)]
mod tests {
    use super::{Reaction, ReactionCountDetails, ReactionType};
    use crate::{id::Id, util::HexColor};
    use serde_test::Token;

    #[test]
    fn message_reaction_unicode() {
        let value = Reaction {
            burst_colors: Vec::from([HexColor(255, 255, 255)]),
            count: 7,
            count_details: ReactionCountDetails {
                burst: 0,
                normal: 7,
            },
            emoji: ReactionType::Unicode {
                name: "a".to_owned(),
            },
            me: true,
            me_burst: false,
        };

        serde_test::assert_tokens(
            &value,
            &[
                Token::Struct {
                    name: "Reaction",
                    len: 6,
                },
                Token::Str("burst_colors"),
                Token::Seq { len: Some(1) },
                Token::Str("#FFFFFF"),
                Token::SeqEnd,
                Token::Str("count"),
                Token::U64(7),
                Token::Str("count_details"),
                Token::Struct {
                    name: "ReactionCountDetails",
                    len: 2,
                },
                Token::Str("burst"),
                Token::U64(0),
                Token::Str("normal"),
                Token::U64(7),
                Token::StructEnd,
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
                Token::Str("me_burst"),
                Token::Bool(false),
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
