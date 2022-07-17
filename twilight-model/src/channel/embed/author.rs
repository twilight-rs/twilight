use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct EmbedAuthor {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon_url: Option<String>,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proxy_icon_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::EmbedAuthor;
    use serde_test::Token;

    #[test]
    fn embed_author() {
        let value = EmbedAuthor {
            icon_url: Some("https://example.com/1.png".to_owned()),
            name: "test".to_owned(),
            proxy_icon_url: None,
            url: Some("https://example.com".to_owned()),
        };

        serde_test::assert_tokens(
            &value,
            &[
                Token::Struct {
                    name: "EmbedAuthor",
                    len: 3,
                },
                Token::Str("icon_url"),
                Token::Some,
                Token::Str("https://example.com/1.png"),
                Token::Str("name"),
                Token::Str("test"),
                Token::Str("url"),
                Token::Some,
                Token::Str("https://example.com"),
                Token::StructEnd,
            ],
        );
    }

    #[test]
    fn embed_author_complete() {
        let value = EmbedAuthor {
            icon_url: Some("https://example.com/1.png".to_owned()),
            name: "test".to_owned(),
            proxy_icon_url: Some("https://example.com".to_owned()),
            url: Some("https://example.com".to_owned()),
        };

        serde_test::assert_tokens(
            &value,
            &[
                Token::Struct {
                    name: "EmbedAuthor",
                    len: 4,
                },
                Token::Str("icon_url"),
                Token::Some,
                Token::Str("https://example.com/1.png"),
                Token::Str("name"),
                Token::Str("test"),
                Token::Str("proxy_icon_url"),
                Token::Some,
                Token::Str("https://example.com"),
                Token::Str("url"),
                Token::Some,
                Token::Str("https://example.com"),
                Token::StructEnd,
            ],
        );
    }
}
