use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct ActivityEmoji {
    pub animated: Option<bool>,
    pub name: String,
    pub id: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::ActivityEmoji;
    use serde_test::Token;

    #[test]
    fn test_activity_emoji() {
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
                    len: 3,
                },
                Token::Str("animated"),
                Token::Some,
                Token::Bool(false),
                Token::Str("name"),
                Token::Str("a"),
                Token::Str("id"),
                Token::None,
                Token::StructEnd,
            ],
        );
    }
}
