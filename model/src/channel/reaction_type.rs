use crate::id::EmojiId;

#[cfg_attr(
    feature = "serde-support",
    derive(serde::Deserialize, serde::Serialize)
)]
#[cfg_attr(feature = "serde-support", serde(untagged))]
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum ReactionType {
    Custom {
        animated: bool,
        // Although counter-intuitive, these *CAN* be not-present.
        id: Option<EmojiId>,
        name: Option<String>,
    },
    Unicode(String),
}

#[cfg(test)]
mod tests {
    use super::ReactionType;
    use serde_test::Token;

    #[test]
    fn test_custom_null_id() {
        let kind = ReactionType::Custom {
            animated: false,
            id: None,
            name: Some("foo".to_owned()),
        };

        serde_test::assert_de_tokens(
            &kind,
            &[
                Token::Struct {
                    name: "ReactionType",
                    len: 3,
                },
                Token::Str("animated"),
                Token::Bool(false),
                Token::Str("id"),
                Token::None,
                Token::Str("name"),
                Token::Some,
                Token::Str("foo"),
                Token::StructEnd,
            ],
        );
    }
}
