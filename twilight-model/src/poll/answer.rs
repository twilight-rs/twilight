use super::media::PollMedia;
use serde::{Deserialize, Serialize};

/// A poll answer.
#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct PollAnswer {
    /// The ID of the answer.
    ///
    /// This is unique within the poll. And increases
    /// sequentially with each answer. Currently, only
    /// 1-10 answers are allowed.
    pub answer_id: u8,
    /// The data of the answer.
    pub poll_media: PollMedia,
}

#[cfg(test)]
mod tests {
    use super::{PollAnswer, PollMedia};
    use crate::{id::Id, poll::media::PartialPollMediaEmoji};
    use serde_test::Token;

    #[test]
    fn poll_answer() {
        let value = PollAnswer {
            answer_id: 1,
            poll_media: PollMedia {
                emoji: Some(PartialPollMediaEmoji {
                    animated: true,
                    id: Some(Id::new(1)),
                    name: Some("a".to_owned()),
                }),
                text: Some("b".to_owned()),
            },
        };

        serde_test::assert_tokens(
            &value,
            &[
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
            ],
        );
    }
}
