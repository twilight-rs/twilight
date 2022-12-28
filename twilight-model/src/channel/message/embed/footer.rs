use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct EmbedFooter {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proxy_icon_url: Option<String>,
    pub text: String,
}

#[cfg(test)]
mod tests {
    use super::EmbedFooter;
    use serde_test::Token;

    #[test]
    fn embed_footer_with_icon() {
        let value = EmbedFooter {
            icon_url: Some("https://example.com/1.png".to_owned()),
            proxy_icon_url: Some("https://cdn.example.com/1-hash.png".to_owned()),
            text: "a footer".to_owned(),
        };

        serde_test::assert_tokens(
            &value,
            &[
                Token::Struct {
                    name: "EmbedFooter",
                    len: 3,
                },
                Token::Str("icon_url"),
                Token::Some,
                Token::Str("https://example.com/1.png"),
                Token::Str("proxy_icon_url"),
                Token::Some,
                Token::Str("https://cdn.example.com/1-hash.png"),
                Token::Str("text"),
                Token::Str("a footer"),
                Token::StructEnd,
            ],
        );
    }

    #[test]
    fn embed_footer_without_icon() {
        let value = EmbedFooter {
            icon_url: None,
            proxy_icon_url: None,
            text: "a footer".to_owned(),
        };

        serde_test::assert_tokens(
            &value,
            &[
                Token::Struct {
                    name: "EmbedFooter",
                    len: 1,
                },
                Token::Str("text"),
                Token::Str("a footer"),
                Token::StructEnd,
            ],
        );
    }
}
