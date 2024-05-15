use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct AnswerCount {
    /// The answer ID.
    pub id: u8,
    /// The number of votes for this answer.
    pub count: u64,
    /// Whether the current user voted for this answer.
    pub me_voted: bool,
}

#[cfg(test)]
mod tests {
    use super::AnswerCount;
    use serde_test::Token;

    #[test]
    fn answer_count() {
        let value = AnswerCount {
            id: 1,
            count: 2,
            me_voted: true,
        };

        serde_test::assert_tokens(
            &value,
            &[
                Token::Struct {
                    name: "AnswerCount",
                    len: 3,
                },
                Token::Str("id"),
                Token::U8(1),
                Token::Str("count"),
                Token::U64(2),
                Token::Str("me_voted"),
                Token::Bool(true),
                Token::StructEnd,
            ],
        );
    }
}
