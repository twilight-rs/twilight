use serde::{Deserialize, Serialize};

/// Geographically based collection of voice servers.
#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct VoiceRegion {
    /// Whether this is a custom voice region, used for e.g. events.
    pub custom: bool,
    /// Whether this is a deprecated voice region (avoid switching to these).
    pub deprecated: bool,
    /// Unique region identifier.
    pub id: String,
    /// Name of the region.
    pub name: String,
    /// Whether this is the closest region to the current user's client.
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
