use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
#[non_exhaustive]
#[serde(from = "u8", into = "u8")]
pub enum ChannelType {
    GuildText,
    Private,
    GuildVoice,
    Group,
    GuildCategory,
    GuildAnnouncement,
    AnnouncementThread,
    PublicThread,
    PrivateThread,
    GuildStageVoice,
    /// Channel in a [hub] containing the listed servers.
    ///
    /// [hub]: https://support.discord.com/hc/en-us/articles/4406046651927-Discord-Student-Hubs-FAQ
    GuildDirectory,
    /// Channel that can only contain threads.
    GuildForum,
    /// Channel the can only contain threads with media content.
    ///
    /// See the [help center article] for more information.
    ///
    /// [help center article]: https://creator-support.discord.com/hc/en-us/articles/14346342766743
    GuildMedia,
    Unknown(u8),
}

impl From<u8> for ChannelType {
    fn from(value: u8) -> Self {
        match value {
            0 => ChannelType::GuildText,
            1 => ChannelType::Private,
            2 => ChannelType::GuildVoice,
            3 => ChannelType::Group,
            4 => ChannelType::GuildCategory,
            5 => ChannelType::GuildAnnouncement,
            10 => ChannelType::AnnouncementThread,
            11 => ChannelType::PublicThread,
            12 => ChannelType::PrivateThread,
            13 => ChannelType::GuildStageVoice,
            14 => ChannelType::GuildDirectory,
            15 => ChannelType::GuildForum,
            16 => ChannelType::GuildMedia,
            unknown => ChannelType::Unknown(unknown),
        }
    }
}

impl From<ChannelType> for u8 {
    fn from(value: ChannelType) -> Self {
        match value {
            ChannelType::GuildText => 0,
            ChannelType::Private => 1,
            ChannelType::GuildVoice => 2,
            ChannelType::Group => 3,
            ChannelType::GuildCategory => 4,
            ChannelType::GuildAnnouncement => 5,
            ChannelType::AnnouncementThread => 10,
            ChannelType::PublicThread => 11,
            ChannelType::PrivateThread => 12,
            ChannelType::GuildStageVoice => 13,
            ChannelType::GuildDirectory => 14,
            ChannelType::GuildForum => 15,
            ChannelType::GuildMedia => 16,
            ChannelType::Unknown(unknown) => unknown,
        }
    }
}

impl ChannelType {
    /// Whether the channel type is that of a guild.
    ///
    /// The following channel types are considered guild channel types:
    ///
    /// - [`AnnouncementThread`][`Self::AnnouncementThread`]
    /// - [`GuildAnnouncement`][`Self::GuildAnnouncement`]
    /// - [`GuildCategory`][`Self::GuildCategory`]
    /// - [`GuildDirectory`][`Self::GuildDirectory`]
    /// - [`GuildStageVoice`][`Self::GuildStageVoice`]
    /// - [`GuildText`][`Self::GuildText`]
    /// - [`GuildVoice`][`Self::GuildVoice`]
    /// - [`PublicThread`][`Self::PublicThread`]
    /// - [`PrivateThread`][`Self::PrivateThread`]
    /// - [`GuildMedia`][`Self::GuildMedia`]
    pub const fn is_guild(self) -> bool {
        matches!(
            self,
            Self::GuildCategory
                | Self::GuildDirectory
                | Self::GuildAnnouncement
                | Self::AnnouncementThread
                | Self::PublicThread
                | Self::PrivateThread
                | Self::GuildStageVoice
                | Self::GuildText
                | Self::GuildVoice
                | Self::GuildMedia
        )
    }

    /// Whether the channel type is a thread.
    ///
    /// The following channel types are considered guild channel types:
    ///
    /// - [`AnnouncementThread`][`Self::AnnouncementThread`]
    /// - [`PrivateThread`][`Self::PrivateThread`]
    /// - [`PublicThread`][`Self::PublicThread`]
    pub const fn is_thread(self) -> bool {
        matches!(
            self,
            Self::AnnouncementThread | Self::PublicThread | Self::PrivateThread
        )
    }

    /// Name of the variant as a string slice.
    pub const fn name(self) -> &'static str {
        match self {
            Self::AnnouncementThread => "AnnouncementThread",
            Self::Group => "Group",
            Self::GuildCategory => "GuildCategory",
            Self::GuildDirectory => "GuildDirectory",
            Self::GuildForum => "GuildForum",
            Self::GuildAnnouncement => "GuildAnnouncement",
            Self::GuildStageVoice => "GuildStageVoice",
            Self::GuildText => "GuildText",
            Self::GuildVoice => "GuildVoice",
            Self::Private => "Private",
            Self::PrivateThread => "PrivateThread",
            Self::PublicThread => "PublicThread",
            Self::GuildMedia => "GuildMedia",
            Self::Unknown(_) => "Unknown",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::ChannelType;
    use serde_test::Token;
    use static_assertions::const_assert;

    const_assert!(ChannelType::GuildCategory.is_guild());
    const_assert!(ChannelType::GuildDirectory.is_guild());
    const_assert!(ChannelType::GuildAnnouncement.is_guild());
    const_assert!(ChannelType::AnnouncementThread.is_guild());
    const_assert!(ChannelType::PublicThread.is_guild());
    const_assert!(ChannelType::PrivateThread.is_guild());
    const_assert!(ChannelType::GuildStageVoice.is_guild());
    const_assert!(ChannelType::GuildText.is_guild());
    const_assert!(ChannelType::GuildVoice.is_guild());
    const_assert!(ChannelType::GuildMedia.is_guild());

    const_assert!(ChannelType::AnnouncementThread.is_thread());
    const_assert!(ChannelType::PublicThread.is_thread());
    const_assert!(ChannelType::PrivateThread.is_thread());

    #[test]
    fn variants() {
        serde_test::assert_tokens(&ChannelType::GuildText, &[Token::U8(0)]);
        serde_test::assert_tokens(&ChannelType::Private, &[Token::U8(1)]);
        serde_test::assert_tokens(&ChannelType::GuildVoice, &[Token::U8(2)]);
        serde_test::assert_tokens(&ChannelType::Group, &[Token::U8(3)]);
        serde_test::assert_tokens(&ChannelType::GuildCategory, &[Token::U8(4)]);
        serde_test::assert_tokens(&ChannelType::GuildAnnouncement, &[Token::U8(5)]);
        serde_test::assert_tokens(&ChannelType::AnnouncementThread, &[Token::U8(10)]);
        serde_test::assert_tokens(&ChannelType::PublicThread, &[Token::U8(11)]);
        serde_test::assert_tokens(&ChannelType::PrivateThread, &[Token::U8(12)]);
        serde_test::assert_tokens(&ChannelType::GuildStageVoice, &[Token::U8(13)]);
        serde_test::assert_tokens(&ChannelType::GuildDirectory, &[Token::U8(14)]);
        serde_test::assert_tokens(&ChannelType::GuildForum, &[Token::U8(15)]);
        serde_test::assert_tokens(&ChannelType::GuildMedia, &[Token::U8(16)]);
        serde_test::assert_tokens(&ChannelType::Unknown(99), &[Token::U8(99)]);
    }

    #[test]
    fn names() {
        assert_eq!("AnnouncementThread", ChannelType::AnnouncementThread.name());
        assert_eq!("Group", ChannelType::Group.name());
        assert_eq!("GuildCategory", ChannelType::GuildCategory.name());
        assert_eq!("GuildDirectory", ChannelType::GuildDirectory.name());
        assert_eq!("GuildAnnouncement", ChannelType::GuildAnnouncement.name());
        assert_eq!("GuildStageVoice", ChannelType::GuildStageVoice.name());
        assert_eq!("GuildText", ChannelType::GuildText.name());
        assert_eq!("GuildVoice", ChannelType::GuildVoice.name());
        assert_eq!("Private", ChannelType::Private.name());
        assert_eq!("PrivateThread", ChannelType::PrivateThread.name());
        assert_eq!("PublicThread", ChannelType::PublicThread.name());
        assert_eq!("GuildMedia", ChannelType::GuildMedia.name());
        assert_eq!("Unknown", ChannelType::Unknown(99).name());
    }
}
