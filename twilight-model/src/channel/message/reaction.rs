use crate::channel::ReactionType;
use serde::{Deserialize, Serialize};

/// Message reaction struct.
#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct MessageReaction {
    /// Amount of reactions this emoji has.
    pub count: u64,
    /// Emoji of this reaction.
    pub emoji: ReactionType,
    /// Whether the current user has reacted with this emoji.
    pub me: bool,
}

#[cfg(test)]
mod tests {
    use super::{MessageReaction, ReactionType};
    use serde_test::Token;

    #[test]
    fn message_reaction_unicode() {
        let value = MessageReaction {
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
                    name: "MessageReaction",
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
}
