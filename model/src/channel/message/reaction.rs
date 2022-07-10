use super::ReactionType;
use serde::{Deserialize, Serialize};

/// User added emoji below a message.
#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct Reaction {
    /// Times this emoji has been used to react.
    pub count: u64,
    /// Emoji information.
    pub emoji: ReactionType,
    /// Whether the current user reacted using this emoji.
    pub me: bool,
}

#[cfg(test)]
mod tests {
    use super::{Reaction, ReactionType};
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
}
