use super::answer_count::AnswerCount;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
/// This contains the number of votes for each answer.
pub struct PollResults {
    /// The counts for each answer.
    pub answer_counts: Vec<AnswerCount>,
    /// Whether the votes have been precisely counted.
    pub is_finalized: bool,
}

#[cfg(test)]
mod tests {
    use super::{AnswerCount, PollResults};
    use serde_test::Token;

    #[test]
    fn poll_results() {
        let value = PollResults {
            answer_counts: vec![
                AnswerCount {
                    answer_id: 1,
                    count: 2,
                    me_voted: true,
                },
                AnswerCount {
                    answer_id: 3,
                    count: 4,
                    me_voted: false,
                },
            ],
            is_finalized: true,
        };

        serde_test::assert_tokens(
            &value,
            &[
                Token::Struct {
                    name: "PollResults",
                    len: 2,
                },
                Token::Str("answer_counts"),
                Token::Seq { len: Some(2) },
                Token::Struct {
                    name: "AnswerCount",
                    len: 3,
                },
                Token::Str("answer_id"),
                Token::U8(1),
                Token::Str("count"),
                Token::U8(2),
                Token::Str("me_voted"),
                Token::Bool(true),
                Token::StructEnd,
                Token::Struct {
                    name: "AnswerCount",
                    len: 3,
                },
                Token::Str("answer_id"),
                Token::U8(3),
                Token::Str("count"),
                Token::U8(4),
                Token::Str("me_voted"),
                Token::Bool(false),
                Token::StructEnd,
                Token::SeqEnd,
                Token::Str("is_finalized"),
                Token::Bool(true),
                Token::StructEnd,
            ],
        );
    }
}
