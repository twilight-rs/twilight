pub mod answer;
pub mod answer_count;
pub mod layout_type;
pub mod media;
pub mod results;

use crate::util::Timestamp;
use serde::{Deserialize, Serialize};

pub use self::{
    answer::PollAnswer,
    answer_count::AnswerCount,
    layout_type::PollLayoutType,
    media::{PartialPollMediaEmoji, PollMedia},
    results::PollResults,
};

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct Poll {
    /// Each of the answers available in the poll.
    pub answers: Vec<PollAnswer>,
    /// Whether a user can select multiple answers.
    pub allow_multiselect: bool,
    /// The time when the poll ends.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiry: Option<Timestamp>,
    /// The layout type of the poll.
    pub layout_type: PollLayoutType,
    /// The question of the poll. Only text is supported.
    pub question: PollMedia,
    /// The results of the poll.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub results: Option<PollResults>,
}

#[cfg(test)]
mod tests {
    use super::{AnswerCount, Poll, PollAnswer, PollLayoutType, PollMedia, PollResults};
    use crate::{id::Id, poll::media::PartialPollMediaEmoji};
    use serde_test::Token;

    #[test]
    #[allow(clippy::too_many_lines)]
    fn poll() {
        let value = Poll {
            answers: vec![
                PollAnswer {
                    answer_id: 1,
                    poll_media: PollMedia {
                        emoji: Some(PartialPollMediaEmoji {
                            animated: true,
                            id: Some(Id::new(1)),
                            name: Some("a".to_owned()),
                        }),
                        text: Some("b".to_owned()),
                    },
                },
                PollAnswer {
                    answer_id: 2,
                    poll_media: PollMedia {
                        emoji: Some(PartialPollMediaEmoji {
                            animated: false,
                            id: Some(Id::new(3)),
                            name: Some("c".to_owned()),
                        }),
                        text: Some("d".to_owned()),
                    },
                },
            ],
            allow_multiselect: true,
            expiry: None,
            layout_type: PollLayoutType::Default,
            question: PollMedia {
                emoji: None,
                text: Some("e".to_owned()),
            },
            results: Some(PollResults {
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
            }),
        };

        serde_test::assert_tokens(
            &value,
            &[
                Token::Struct {
                    name: "Poll",
                    len: 5,
                },
                Token::Str("answers"),
                Token::Seq { len: Some(2) },
                Token::Struct {
                    name: "PollAnswer",
                    len: 2,
                },
                Token::Str("answer_id"),
                Token::U8(1),
                Token::Str("poll_media"),
                Token::Struct {
                    name: "PollMedia",
                    len: 2,
                },
                Token::Str("emoji"),
                Token::Some,
                Token::Struct {
                    name: "PartialPollMediaEmoji",
                    len: 3,
                },
                Token::Str("animated"),
                Token::Bool(true),
                Token::Str("id"),
                Token::Some,
                Token::NewtypeStruct { name: "Id" },
                Token::Str("1"),
                Token::Str("name"),
                Token::Some,
                Token::Str("a"),
                Token::StructEnd,
                Token::Str("text"),
                Token::Some,
                Token::Str("b"),
                Token::StructEnd,
                Token::StructEnd,
                Token::Struct {
                    name: "PollAnswer",
                    len: 2,
                },
                Token::Str("answer_id"),
                Token::U8(2),
                Token::Str("poll_media"),
                Token::Struct {
                    name: "PollMedia",
                    len: 2,
                },
                Token::Str("emoji"),
                Token::Some,
                Token::Struct {
                    name: "PartialPollMediaEmoji",
                    len: 3,
                },
                Token::Str("animated"),
                Token::Bool(false),
                Token::Str("id"),
                Token::Some,
                Token::NewtypeStruct { name: "Id" },
                Token::Str("3"),
                Token::Str("name"),
                Token::Some,
                Token::Str("c"),
                Token::StructEnd,
                Token::Str("text"),
                Token::Some,
                Token::Str("d"),
                Token::StructEnd,
                Token::StructEnd,
                Token::SeqEnd,
                Token::Str("allow_multiselect"),
                Token::Bool(true),
                Token::Str("layout_type"),
                Token::U8(1),
                Token::Str("question"),
                Token::Struct {
                    name: "PollMedia",
                    len: 2,
                },
                Token::Str("emoji"),
                Token::None,
                Token::Str("text"),
                Token::Some,
                Token::Str("e"),
                Token::StructEnd,
                Token::Str("results"),
                Token::Some,
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
                Token::StructEnd,
            ],
        );
    }
}
