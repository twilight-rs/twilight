use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct ActivityButton {
    /// Text shown on the button.
    pub label: String,
    /// URL opened when clicking the button.
    pub url: String,
}

#[cfg(test)]
mod tests {
    use super::ActivityButton;
    use serde_test::Token;

    #[test]
    fn test_activity_secrets() {
        let value = ActivityButton {
            label: "a".to_owned(),
            url: "b".to_owned(),
        };

        serde_test::assert_tokens(
            &value,
            &[
                Token::Struct {
                    name: "ActivityButton",
                    len: 2,
                },
                Token::Str("label"),
                Token::Str("a"),
                Token::Str("url"),
                Token::Str("b"),
                Token::StructEnd,
            ],
        );
    }
}
