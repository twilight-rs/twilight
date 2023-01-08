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
pub struct GuildFeature(KnownString<64>);

impl GuildFeature {
    /// Has access to set an animated guild banner image.
    pub const ANIMATED_BANNER: Self = Self::from_bytes(b"ANIMATED_BANNER");

    /// Has access to set an animated guild icon.
    pub const ANIMATED_ICON: Self = Self::from_bytes(b"ANIMATED_ICON");

    /// Has set up auto moderation rules.
    pub const AUTO_MODERATION: Self = Self::from_bytes(b"AUTO_MODERATION");

    /// Has access to set a guild banner image.
    pub const BANNER: Self = Self::from_bytes(b"BANNER");

    /// Has access to use commerce features (create store channels).
    #[deprecated]
    pub const COMMERCE: Self = Self::from_bytes(b"COMMERCE");

    /// Can enable welcome screen, membership screening, stage channels,
    /// discovery, and receives community updates.
    pub const COMMUNITY: Self = Self::from_bytes(b"COMMUNITY");

    /// Guild has been set as a support server on the App Directory.
    pub const DEVELOPER_SUPPORT_SERVER: Self = Self::from_bytes(b"DEVELOPER_SUPPORT_SERVER");

    /// Is able to be discovered in the directory.
    pub const DISCOVERABLE: Self = Self::from_bytes(b"DISCOVERABLE");

    /// Is able to be featured in the directory.
    pub const FEATURABLE: Self = Self::from_bytes(b"FEATURABLE");

    /// Invites have been paused, this prevents new users from joining.
    pub const INVITES_DISABLED: Self = Self::from_bytes(b"INVITES_DISABLED");

    /// Has access to set an invite splash background.
    pub const INVITE_SPLASH: Self = Self::from_bytes(b"INVITE_SPLASH");

    /// Has enabled membership screening.
    pub const MEMBER_VERIFICATION_GATE_ENABLED: Self =
        Self::from_bytes(b"MEMBER_VERIFICATION_GATE_ENABLED");

    /// Has enabled monetization.
    pub const MONETIZATION_ENABLED: Self = Self::from_bytes(b"MONETIZATION_ENABLED");

    /// Has increased custom sticker slots.
    pub const MORE_STICKERS: Self = Self::from_bytes(b"MORE_STICKERS");

    /// Has access to create news channels.
    pub const NEWS: Self = Self::from_bytes(b"NEWS");

    /// Is partnered.
    pub const PARTNERED: Self = Self::from_bytes(b"PARTNERED");

    /// Can be previewed before joining via membership screening or the
    /// directory.
    pub const PREVIEW_ENABLED: Self = Self::from_bytes(b"PREVIEW_ENABLED");

    /// Has access to create private threads.
    pub const PRIVATE_THREADS: Self = Self::from_bytes(b"PRIVATE_THREADS");

    /// Is able to set role icons.
    pub const ROLE_ICONS: Self = Self::from_bytes(b"ROLE_ICONS");

    /// Has enabled ticketed events.
    pub const TICKETED_EVENTS_ENABLED: Self = Self::from_bytes(b"TICKETED_EVENTS_ENABLED");

    /// Has access to set a vanity URL.
    pub const VANITY_URL: Self = Self::from_bytes(b"VANITY_URL");

    /// Is verified.
    pub const VERIFIED: Self = Self::from_bytes(b"VERIFIED");

    /// Has access to set 384kps bitrate in voice (previously VIP voice
    /// servers).
    pub const VIP_REGIONS: Self = Self::from_bytes(b"VIP_REGIONS");

    /// Has enabled the welcome screen.
    pub const WELCOME_SCREEN_ENABLED: Self = Self::from_bytes(b"WELCOME_SCREEN_ENABLED");

    /// Create a guild feature from a dynamic value.
    ///
    /// The provided guild feature must be 64 bytes or smaller.
    pub fn new(guild_feature: &str) -> Option<Self> {
        KnownString::from_str(guild_feature).map(Self)
    }

    /// Get the value of the guild feature.
    ///
    /// # Panics
    ///
    /// Panics if the guild feature isn't valid UTF-8.
    pub fn get(&self) -> &str {
        self.0.get()
    }

    /// Create a guild feature from a set of bytes.
    const fn from_bytes(input: &[u8]) -> Self {
        Self(KnownString::from_bytes(input))
    }
}

impl AsRef<str> for GuildFeature {
    fn as_ref(&self) -> &str {
        self.get()
    }
}

impl Debug for GuildFeature {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        f.write_str(self.get())
    }
}

impl Deref for GuildFeature {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        self.get()
    }
}

impl FromStr for GuildFeature {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::try_from(s)
    }
}

impl ToString for GuildFeature {
    fn to_string(&self) -> String {
        KnownString::to_string(&self.0)
    }
}

impl TryFrom<&str> for GuildFeature {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::new(value).ok_or(())
    }
}

#[cfg(test)]
mod tests {
    use super::GuildFeature;
    use serde::{Deserialize, Serialize};
    use serde_test::Token;
    use static_assertions::assert_impl_all;
    use std::{fmt::Debug, hash::Hash, str::FromStr, string::ToString};

    assert_impl_all!(
        GuildFeature: AsRef<str>,
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

    const MAP: &[(GuildFeature, &str)] = &[
        (GuildFeature::ANIMATED_BANNER, "ANIMATED_BANNER"),
        (GuildFeature::ANIMATED_ICON, "ANIMATED_ICON"),
        (GuildFeature::AUTO_MODERATION, "AUTO_MODERATION"),
        (GuildFeature::BANNER, "BANNER"),
        #[allow(deprecated)]
        (GuildFeature::COMMERCE, "COMMERCE"),
        (GuildFeature::COMMUNITY, "COMMUNITY"),
        (
            GuildFeature::DEVELOPER_SUPPORT_SERVER,
            "DEVELOPER_SUPPORT_SERVER",
        ),
        (GuildFeature::DISCOVERABLE, "DISCOVERABLE"),
        (GuildFeature::FEATURABLE, "FEATURABLE"),
        (GuildFeature::INVITES_DISABLED, "INVITES_DISABLED"),
        (GuildFeature::INVITE_SPLASH, "INVITE_SPLASH"),
        (
            GuildFeature::MEMBER_VERIFICATION_GATE_ENABLED,
            "MEMBER_VERIFICATION_GATE_ENABLED",
        ),
        (GuildFeature::MONETIZATION_ENABLED, "MONETIZATION_ENABLED"),
        (GuildFeature::MORE_STICKERS, "MORE_STICKERS"),
        (GuildFeature::NEWS, "NEWS"),
        (GuildFeature::PARTNERED, "PARTNERED"),
        (GuildFeature::PREVIEW_ENABLED, "PREVIEW_ENABLED"),
        (GuildFeature::PRIVATE_THREADS, "PRIVATE_THREADS"),
        (GuildFeature::ROLE_ICONS, "ROLE_ICONS"),
        (
            GuildFeature::TICKETED_EVENTS_ENABLED,
            "TICKETED_EVENTS_ENABLED",
        ),
        (GuildFeature::VANITY_URL, "VANITY_URL"),
        (GuildFeature::VERIFIED, "VERIFIED"),
        (GuildFeature::VIP_REGIONS, "VIP_REGIONS"),
        (
            GuildFeature::WELCOME_SCREEN_ENABLED,
            "WELCOME_SCREEN_ENABLED",
        ),
    ];

    #[test]
    fn variants() {
        for (kind, name) in MAP {
            serde_test::assert_tokens(
                kind,
                &[
                    Token::NewtypeStruct {
                        name: "GuildFeature",
                    },
                    Token::Str(name),
                ],
            );
            assert_eq!(Some(*kind), GuildFeature::new(name));
            assert_eq!(*name, kind.as_ref());
            assert_eq!(Ok(*kind), GuildFeature::from_str(name));
            assert_eq!(Ok(*kind), GuildFeature::try_from(*name));
            assert_eq!(name, &kind.to_string());
            assert_eq!(*name, kind.get());
        }
    }
}
