use serde::{Deserialize, Serialize};

#[allow(clippy::struct_excessive_bools)]
#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct VoiceRegion {
    pub(crate) custom: bool,
    pub(crate) deprecated: bool,
    pub(crate) id: String,
    pub(crate) name: String,
    pub(crate) optimal: bool,
}

impl VoiceRegion {
    pub const fn custom(&self) -> bool {
        self.custom
    }

    pub const fn deprecated(&self) -> bool {
        self.deprecated
    }

    pub fn id(&self) -> &str {
        &self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub const fn optimal(&self) -> bool {
        self.optimal
    }
}

#[cfg(test)]
mod tests {
    use super::VoiceRegion;
    use serde_test::Token;

    #[test]
    fn test_voice_region() {
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
