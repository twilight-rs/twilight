use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct EmbedField {
    #[serde(default)]
    pub inline: bool,
    pub name: String,
    pub value: String,
}

#[cfg(test)]
mod tests {
    use super::EmbedField;
    use serde_test::Token;

    #[test]
    fn embed_field() {
        let value = EmbedField {
            inline: true,
            name: "name".to_owned(),
            value: "value".to_owned(),
        };

        serde_test::assert_tokens(
            &value,
            &[
                Token::Struct {
                    name: "EmbedField",
                    len: 3,
                },
                Token::Str("inline"),
                Token::Bool(true),
                Token::Str("name"),
                Token::Str("name"),
                Token::Str("value"),
                Token::Str("value"),
                Token::StructEnd,
            ],
        );
    }
}
