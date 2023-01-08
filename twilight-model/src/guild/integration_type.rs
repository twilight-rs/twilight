use crate::util::known_string::KnownString;
use serde::{Deserialize, Serialize};
use std::{
    fmt::{Debug, Formatter, Result as FmtResult},
    ops::Deref,
    str::FromStr,
};

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

    /// Create a guild integration type from a dynamic value.
    ///
    /// The provided guild integration type must be 64 bytes or smaller.
    pub fn new(guild_integration_type: &str) -> Option<Self> {
        KnownString::from_str(guild_integration_type).map(Self)
    }

    /// Get the value of the guild integration type.
    ///
    /// # Panics
    ///
    /// Panics if the guild integration type isn't valid UTF-8.
    pub fn get(&self) -> &str {
        self.0.get()
    }

    /// Create a guild integration type from a set of bytes.
    const fn from_bytes(input: &[u8]) -> Self {
        Self(KnownString::from_bytes(input))
    }
}

impl AsRef<str> for GuildIntegrationType {
    fn as_ref(&self) -> &str {
        self.get()
    }
}

impl Debug for GuildIntegrationType {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        f.write_str(self.get())
    }
}

impl Deref for GuildIntegrationType {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        self.get()
    }
}

impl FromStr for GuildIntegrationType {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::try_from(s)
    }
}

impl ToString for GuildIntegrationType {
    fn to_string(&self) -> String {
        KnownString::to_string(&self.0)
    }
}

impl TryFrom<&str> for GuildIntegrationType {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::new(value).ok_or(())
    }
}

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
