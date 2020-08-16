use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct EmbedProvider {
    pub name: Option<String>,
    pub url: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::EmbedProvider;
    use serde_test::Token;

    #[test]
    fn test_embed_provider() {
        let value = EmbedProvider {
            name: Some("Example".to_owned()),
            url: Some("https://example.com".to_owned()),
        };

        serde_test::assert_tokens(
            &value,
            &[
                Token::Struct {
                    name: "EmbedProvider",
                    len: 2,
                },
                Token::Str("name"),
                Token::Some,
                Token::Str("Example"),
                Token::Str("url"),
                Token::Some,
                Token::Str("https://example.com"),
                Token::StructEnd,
            ],
        );
    }
}
