use crate::id::{marker::EmojiMarker, Id};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct PollMedia {
    /// The emoji of the field.
    ///
    /// When creating a poll answer with an emoji, one only
    /// needs to send either the id (custom emoji) or name
    /// (default emoji) as the only field.
    pub emoji: Option<PartialPollMediaEmoji>,
    /// The text of the field.
    pub text: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct PartialPollMediaEmoji {
    #[serde(default)]
    pub animated: bool,
    pub id: Option<Id<EmojiMarker>>,
    pub name: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::{PartialPollMediaEmoji, PollMedia};
    use crate::id::Id;
    use serde_test::Token;

    #[test]
    fn poll_media() {
        let value = PollMedia {
            emoji: Some(PartialPollMediaEmoji {
                animated: true,
                id: Some(Id::new(1)),
                name: Some("a".to_owned()),
            }),
            text: Some("b".to_owned()),
        };

        serde_test::assert_tokens(
            &value,
            &[
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
            ],
        );
    }
}
