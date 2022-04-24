use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
#[serde(from = "u8", into = "u8")]
pub enum ChannelType {
    GuildText,
    Private,
    GuildVoice,
    Group,
    GuildCategory,
    GuildNews,
    GuildNewsThread,
    GuildPublicThread,
    GuildPrivateThread,
    GuildStageVoice,
    GuildDirectory,
    GuildForum,
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
            5 => ChannelType::GuildNews,
            10 => ChannelType::GuildNewsThread,
            11 => ChannelType::GuildPublicThread,
            12 => ChannelType::GuildPrivateThread,
            13 => ChannelType::GuildStageVoice,
            14 => ChannelType::GuildDirectory,
            15 => ChannelType::GuildForum,
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
            ChannelType::GuildNews => 5,
            ChannelType::GuildNewsThread => 10,
            ChannelType::GuildPublicThread => 11,
            ChannelType::GuildPrivateThread => 12,
            ChannelType::GuildStageVoice => 13,
            ChannelType::GuildDirectory => 14,
            ChannelType::GuildForum => 15,
            ChannelType::Unknown(unknown) => unknown,
        }
    }
}

impl ChannelType {
    /// Whether the channel type is that of a guild.
    ///
    /// The following channel types are considered guild channel types:
    ///
    /// - [`GuildCategory`][`Self::GuildCategory`]
    /// - [`GuildDirectory`][`Self::GuildDirectory`]
    /// - [`GuildNews`][`Self::GuildNews`]
    /// - [`GuildNewsThread`][`Self::GuildNewsThread`]
    /// - [`GuildPublicThread`][`Self::GuildPublicThread`]
    /// - [`GuildPrivateThread`][`Self::GuildPrivateThread`]
    /// - [`GuildStageVoice`][`Self::GuildStageVoice`]
    /// - [`GuildText`][`Self::GuildText`]
    /// - [`GuildVoice`][`Self::GuildVoice`]
    pub const fn is_guild(self) -> bool {
        matches!(
            self,
            Self::GuildCategory
                | Self::GuildDirectory
                | Self::GuildNews
                | Self::GuildNewsThread
                | Self::GuildPublicThread
                | Self::GuildPrivateThread
                | Self::GuildStageVoice
                | Self::GuildText
                | Self::GuildVoice
        )
    }

    /// Whether the channel type is a thread.
    ///
    /// The following channel types are considered guild channel types:
    ///
    /// - [`GuildNewsThread`][`Self::GuildNewsThread`]
    /// - [`GuildPublicThread`][`Self::GuildPublicThread`]
    /// - [`GuildPrivateThread`][`Self::GuildPrivateThread`]
    pub const fn is_thread(self) -> bool {
        matches!(
            self,
            Self::GuildNewsThread | Self::GuildPublicThread | Self::GuildPrivateThread
        )
    }

    /// Name of the variant as a string slice.
    pub const fn name(self) -> &'static str {
        match self {
            Self::Group => "Group",
            Self::GuildCategory => "GuildCategory",
            Self::GuildDirectory => "GuildDirectory",
            Self::GuildForum => "GuildForum",
            Self::GuildNews => "GuildNews",
            Self::GuildNewsThread => "GuildNewsThread",
            Self::GuildPrivateThread => "GuildPrivateThread",
            Self::GuildPublicThread => "GuildPublicThread",
            Self::GuildStageVoice => "GuildStageVoice",
            Self::GuildText => "GuildText",
            Self::GuildVoice => "GuildVoice",
            Self::Private => "Private",
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
    const_assert!(ChannelType::GuildNews.is_guild());
    const_assert!(ChannelType::GuildNewsThread.is_guild());
    const_assert!(ChannelType::GuildPublicThread.is_guild());
    const_assert!(ChannelType::GuildPrivateThread.is_guild());
    const_assert!(ChannelType::GuildStageVoice.is_guild());
    const_assert!(ChannelType::GuildText.is_guild());
    const_assert!(ChannelType::GuildVoice.is_guild());

    const_assert!(ChannelType::GuildNewsThread.is_thread());
    const_assert!(ChannelType::GuildPublicThread.is_thread());
    const_assert!(ChannelType::GuildPrivateThread.is_thread());

    #[test]
    fn test_variants() {
        serde_test::assert_tokens(&ChannelType::GuildText, &[Token::U8(0)]);
        serde_test::assert_tokens(&ChannelType::Private, &[Token::U8(1)]);
        serde_test::assert_tokens(&ChannelType::GuildVoice, &[Token::U8(2)]);
        serde_test::assert_tokens(&ChannelType::Group, &[Token::U8(3)]);
        serde_test::assert_tokens(&ChannelType::GuildCategory, &[Token::U8(4)]);
        serde_test::assert_tokens(&ChannelType::GuildNews, &[Token::U8(5)]);
        serde_test::assert_tokens(&ChannelType::GuildNewsThread, &[Token::U8(10)]);
        serde_test::assert_tokens(&ChannelType::GuildPublicThread, &[Token::U8(11)]);
        serde_test::assert_tokens(&ChannelType::GuildPrivateThread, &[Token::U8(12)]);
        serde_test::assert_tokens(&ChannelType::GuildStageVoice, &[Token::U8(13)]);
        serde_test::assert_tokens(&ChannelType::GuildDirectory, &[Token::U8(14)]);
        serde_test::assert_tokens(&ChannelType::Unknown(99), &[Token::U8(99)]);
    }

    #[test]
    fn test_names() {
        assert_eq!("Group", ChannelType::Group.name());
        assert_eq!("GuildCategory", ChannelType::GuildCategory.name());
        assert_eq!("GuildDirectory", ChannelType::GuildDirectory.name());
        assert_eq!("GuildNews", ChannelType::GuildNews.name());
        assert_eq!("GuildNewsThread", ChannelType::GuildNewsThread.name());
        assert_eq!("GuildPrivateThread", ChannelType::GuildPrivateThread.name());
        assert_eq!("GuildPublicThread", ChannelType::GuildPublicThread.name());
        assert_eq!("GuildStageVoice", ChannelType::GuildStageVoice.name());
        assert_eq!("GuildText", ChannelType::GuildText.name());
        assert_eq!("GuildVoice", ChannelType::GuildVoice.name());
        assert_eq!("Private", ChannelType::Private.name());
        assert_eq!("Unknown", ChannelType::Unknown(99).name());
    }
}
