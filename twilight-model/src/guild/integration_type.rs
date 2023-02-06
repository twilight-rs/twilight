use crate::util::known_string::KnownString;
use serde::{Deserialize, Serialize};

/// Special and optional guild features.
///
/// See [Discord Docs/Guild Features].
///
/// [Discord Docs/Guild Features]: https://discord.com/developers/docs/resources/guild#guild-object-guild-features
#[derive(Clone, Copy, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct GuildIntegrationType(KnownString<32>);

impl GuildIntegrationType {
    /// Integration is a Discord application.
    pub const DISCORD: Self = Self::from_bytes(b"discord");

    /// Integration is a Twitch connection.
    pub const TWITCH: Self = Self::from_bytes(b"twitch");

    /// Integration is a Youtube connection.
    pub const YOUTUBE: Self = Self::from_bytes(b"youtube");

    /// Name of the associated constant.
    ///
    /// Returns `None` if the value doesn't have a defined constant.
    pub const fn name(self) -> Option<&'static str> {
        Some(match self {
            Self::DISCORD => "DISCORD",
            Self::TWITCH => "TWITCH",
            Self::YOUTUBE => "YOUTUBE",
            _ => return None,
        })
    }
}

impl_typed!(GuildIntegrationType, String);

#[cfg(test)]
mod tests {
    use super::GuildIntegrationType;
    use serde::{Deserialize, Serialize};
    use static_assertions::assert_impl_all;
    use std::{fmt::Debug, hash::Hash, str::FromStr, string::ToString};

    assert_impl_all!(
        GuildIntegrationType: AsRef<str>,
        Clone,
        Copy,
        Debug,
        Deserialize<'static>,
        Eq,
        FromStr,
        Hash,
        PartialEq,
        Send,
        Serialize,
        Sync,
        ToString,
        TryFrom<&'static str>,
    );
}
