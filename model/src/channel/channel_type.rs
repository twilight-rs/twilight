use serde_repr::{Deserialize_repr, Serialize_repr};

#[derive(Clone, Copy, Debug, Deserialize_repr, Eq, Hash, PartialEq, Serialize_repr)]
#[non_exhaustive]
#[repr(u8)]
pub enum ChannelType {
    GuildText = 0,
    Private = 1,
    GuildVoice = 2,
    Group = 3,
    GuildCategory = 4,
    GuildNews = 5,
    GuildNewsThread = 10,
    GuildPublicThread = 11,
    GuildPrivateThread = 12,
    GuildStageVoice = 13,
    /// Channel in a [hub] containing the listed servers.
    ///
    /// [hub]: https://support.discord.com/hc/en-us/articles/4406046651927-Discord-Student-Hubs-FAQ
    GuildDirectory = 14,
    /// Channel that can only contain threads.
    GuildForum = 15,
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
    fn variants() {
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
    }

    #[test]
    fn names() {
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
    }
}
