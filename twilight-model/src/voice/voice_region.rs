use serde::{Deserialize, Serialize};

#[allow(clippy::struct_excessive_bools)]
#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct VoiceRegion {
    pub custom: bool,
    pub deprecated: bool,
    pub id: String,
    pub name: String,
    pub optimal: bool,
}

#[cfg(test)]
mod tests {
    use super::VoiceRegion;
    use serde_test::Token;

    #[test]
    fn voice_region() {
        let value = VoiceRegion {
            custom: false,
            deprecated: false,
            id: "region".to_owned(),
            name: "Region".to_owned(),
            optimal: false,
        };

        serde_test::assert_tokens(
            &value,
            &[
                Token::Struct {
                    name: "VoiceRegion",
                    len: 5,
                },
                Token::Str("custom"),
                Token::Bool(false),
                Token::Str("deprecated"),
                Token::Bool(false),
                Token::Str("id"),
                Token::Str("region"),
                Token::Str("name"),
                Token::Str("Region"),
                Token::Str("optimal"),
                Token::Bool(false),
                Token::StructEnd,
            ],
        );
    }
}
