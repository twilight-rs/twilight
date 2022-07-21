use crate::channel::ReactionType;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct MessageReaction {
    pub count: u64,
    pub emoji: ReactionType,
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
