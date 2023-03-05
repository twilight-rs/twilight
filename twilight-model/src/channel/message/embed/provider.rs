use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
#[cfg_attr(
    feature = "rkyv",
    derive(rkyv::Archive, rkyv::Deserialize, rkyv::Serialize)
)]
pub struct EmbedProvider {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::EmbedProvider;
    use serde_test::Token;

    #[test]
    fn embed_provider() {
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
