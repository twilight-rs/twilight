#![allow(deprecated)]
use std::borrow::Cow;

use serde::{Deserialize, Serialize};

/// Special and optional guild features.
///
/// See [Discord Docs/Guild Features].
///
/// [Discord Docs/Guild Features]: https://discord.com/developers/docs/resources/guild#guild-object-guild-features
#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
#[non_exhaustive]
#[serde(from = "String", into = "Cow<'static, str>")]
pub enum IntegrationType {
    /// Integration is a Discord application.
    Discord,
    /// Integration is a Twitch connection.
    Twitch,
    /// Integration is a YouTube connection.
    YouTube,
    /// Variant value is unknown to the library.
    Unknown(String),
}

impl From<IntegrationType> for Cow<'static, str> {
    fn from(value: IntegrationType) -> Self {
        match value {
            IntegrationType::Discord => "discord".into(),
            IntegrationType::Twitch => "twitch".into(),
            IntegrationType::YouTube => "youtube".into(),
            IntegrationType::Unknown(unknown) => unknown.into(),
        }
    }
}

impl From<String> for IntegrationType {
    fn from(value: String) -> Self {
        match value.as_str() {
            "discord" => Self::Discord,
            "twitch" => Self::Twitch,
            "youtube" => Self::YouTube,
            _ => Self::Unknown(value),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::IntegrationType;
    use serde_test::Token;

    #[test]
    fn variants() {
        const MAP: &[(IntegrationType, &str)] = &[
            (IntegrationType::Discord, "discord"),
            (IntegrationType::Twitch, "twitch"),
            (IntegrationType::YouTube, "youtube"),
        ];

        for (integration_type, value) in MAP {
            serde_test::assert_tokens(integration_type, &[Token::Str(value)]);
        }
    }
}
