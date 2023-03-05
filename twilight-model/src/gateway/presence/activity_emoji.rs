use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
#[cfg_attr(
    feature = "rkyv",
    derive(rkyv::Archive, rkyv::Deserialize, rkyv::Serialize)
)]
pub struct ActivityEmoji {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub animated: Option<bool>,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::ActivityEmoji;
    use serde_test::Token;

    #[test]
    fn activity_emoji() {
        let value = ActivityEmoji {
            animated: Some(false),
            name: "a".to_owned(),
            id: None,
        };

        serde_test::assert_tokens(
            &value,
            &[
                Token::Struct {
                    name: "ActivityEmoji",
                    len: 2,
                },
                Token::Str("animated"),
                Token::Some,
                Token::Bool(false),
                Token::Str("name"),
                Token::Str("a"),
                Token::StructEnd,
            ],
        );
    }

    #[test]
    fn activity_emoji_complete() {
        let value = ActivityEmoji {
            animated: Some(false),
            name: "a".to_owned(),
            id: Some("123".to_owned()),
        };

        serde_test::assert_tokens(
            &value,
            &[
                Token::Struct {
                    name: "ActivityEmoji",
                    len: 3,
                },
                Token::Str("animated"),
                Token::Some,
                Token::Bool(false),
                Token::Str("name"),
                Token::Str("a"),
                Token::Str("id"),
                Token::Some,
                Token::Str("123"),
                Token::StructEnd,
            ],
        );
    }
}
