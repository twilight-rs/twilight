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
#[cfg_attr(
    feature = "rkyv",
    derive(rkyv::Archive, rkyv::Deserialize, rkyv::Serialize)
)]
pub enum GuildIntegrationType {
    /// Integration is a Discord application.
    Discord,
    /// Integration is a Twitch connection.
    Twitch,
    /// Integration is a YouTube connection.
    YouTube,
    /// Variant value is unknown to the library.
    Unknown(String),
}

impl From<GuildIntegrationType> for Cow<'static, str> {
    fn from(value: GuildIntegrationType) -> Self {
        match value {
            GuildIntegrationType::Discord => "discord".into(),
            GuildIntegrationType::Twitch => "twitch".into(),
            GuildIntegrationType::YouTube => "youtube".into(),
            GuildIntegrationType::Unknown(unknown) => unknown.into(),
        }
    }
}

impl From<String> for GuildIntegrationType {
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
    use super::GuildIntegrationType;
    use serde_test::Token;

    #[test]
    fn variants() {
        const MAP: &[(GuildIntegrationType, &str)] = &[
            (GuildIntegrationType::Discord, "discord"),
            (GuildIntegrationType::Twitch, "twitch"),
            (GuildIntegrationType::YouTube, "youtube"),
        ];

        for (integration_type, value) in MAP {
            serde_test::assert_tokens(integration_type, &[Token::Str(value)]);
        }
    }
}
