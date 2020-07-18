use serde::{Deserialize, Serialize};

#[allow(clippy::struct_excessive_bools)]
#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct VoiceRegion {
    pub custom: bool,
    pub deprecated: bool,
    pub id: String,
    pub name: String,
    pub optimal: bool,
    pub vip: bool,
}

#[cfg(test)]
mod tests {
    use super::VoiceRegion;
    use serde_test::Token;

    #[test]
    fn test_voice_region() {
        let region = VoiceRegion {
            custom: false,
            deprecated: false,
            id: "region".to_owned(),
            name: "Region".to_owned(),
            optimal: false,
            vip: false,
        };

        serde_test::assert_tokens(
            &region,
            &[
                Token::Struct {
                    name: "VoiceRegion",
                    len: 6,
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
                Token::Str("vip"),
                Token::Bool(false),
                Token::StructEnd,
            ],
        );
    }
}
