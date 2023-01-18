use serde::{Deserialize, Serialize};
use std::fmt::{Debug, Formatter, Result as FmtResult};

#[derive(Clone, Copy, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct ChannelType(u8);

impl ChannelType {
    pub const GUILD_TEXT: Self = Self::new(0);

    pub const PRIVATE: Self = Self::new(1);

    pub const GUILD_VOICE: Self = Self::new(2);

    pub const GROUP: Self = Self::new(3);

    pub const GUILD_CATEGORY: Self = Self::new(4);

    pub const GUILD_ANNOUNCEMENT: Self = Self::new(5);

    pub const ANNOUNCEMENT_THREAD: Self = Self::new(10);

    pub const PUBLIC_THREAD: Self = Self::new(11);

    pub const PRIVATE_THREAD: Self = Self::new(12);

    pub const GUILD_STAGE_VOICE: Self = Self::new(13);

    /// Channel in a [hub] containing the listed servers.
    ///
    /// [hub]: https://support.discord.com/hc/en-us/articles/4406046651927-Discord-Student-Hubs-FAQ
    pub const GUILD_DIRECTORY: Self = Self::new(14);

    /// Channel that can only contain threads.
    pub const GUILD_FORUM: Self = Self::new(15);

    /// Create a new channel type from a dynamic value.
    ///
    /// The provided value isn't validated. Known valid values are associated
    /// constants such as [`GUILD_TEXT`][`Self::GUILD_TEXT`].
    pub const fn new(channel_type: u8) -> Self {
        Self(channel_type)
    }

    /// Retrieve the value of the channel type.
    ///
    /// # Examples
    ///
    /// ```
    /// use twilight_model::channel::ChannelType;
    ///
    /// assert_eq!(15, ChannelType::GUILD_FORUM.get());
    /// ```
    pub const fn get(&self) -> u8 {
        self.0
    }

    /// Name of the associated constant.
    ///
    /// Returns `None` if the value doesn't have a defined constant.
    pub const fn name(self) -> Option<&'static str> {
        Some(match self {
            Self::GUILD_TEXT => "GUILD_TEXT",
            Self::PRIVATE => "PRIVATE",
            Self::GUILD_VOICE => "GUILD_VOICE",
            Self::GROUP => "GROUP",
            Self::GUILD_CATEGORY => "GUILD_CATEGORY",
            Self::GUILD_ANNOUNCEMENT => "GUILD_ANNOUNCEMENT",
            Self::ANNOUNCEMENT_THREAD => "ANNOUNCEMENT_THREAD",
            Self::PUBLIC_THREAD => "PUBLIC_THREAD",
            Self::PRIVATE_THREAD => "PRIVATE_THREAD",
            Self::GUILD_STAGE_VOICE => "GUILD_STAGE_VOICE",
            Self::GUILD_DIRECTORY => "GUILD_DIRECTORY",
            Self::GUILD_FORUM => "GUILD_FORUM",
            _ => return None,
        })
    }

    /// Whether the channel type is that of a guild.
    ///
    /// The following channel types are considered guild channel types:
    ///
    /// - [`ANNOUNCEMENT_THREAD`][`Self::ANNOUNCEMENT_THREAD`]
    /// - [`GUILD_ANNOUNCEMENT`][`Self::GUILD_ANNOUNCEMENT`]
    /// - [`GUILD_CATEGORY`][`Self::GUILD_CATEGORY`]
    /// - [`GUILD_DIRECTORY`][`Self::GUILD_DIRECTORY`]
    /// - [`GUILD_STAGE_VOICE`][`Self::GUILD_STAGE_VOICE`]
    /// - [`GUILD_TEXT`][`Self::GUILD_TEXT`]
    /// - [`GUILD_VOICE`][`Self::GUILD_VOICE`]
    /// - [`PUBLIC_THREAD`][`Self::PUBLIC_THREAD`]
    /// - [`PRIVATE_THREAD`][`Self::PRIVATE_THREAD`]
    pub const fn is_guild(self) -> bool {
        matches!(
            self,
            Self::GUILD_CATEGORY
                | Self::GUILD_DIRECTORY
                | Self::GUILD_ANNOUNCEMENT
                | Self::ANNOUNCEMENT_THREAD
                | Self::PUBLIC_THREAD
                | Self::PRIVATE_THREAD
                | Self::GUILD_STAGE_VOICE
                | Self::GUILD_TEXT
                | Self::GUILD_VOICE
        )
    }

    /// Whether the channel type is a thread.
    ///
    /// The following channel types are considered guild channel types:
    ///
    /// - [`ANNOUNCEMENT_THREAD`][`Self::ANNOUNCEMENT_THREAD`]
    /// - [`PRIVATE_THREAD`][`Self::PRIVATE_THREAD`]
    /// - [`PUBLIC_THREAD`][`Self::PUBLIC_THREAD`]
    pub const fn is_thread(self) -> bool {
        matches!(
            self,
            Self::ANNOUNCEMENT_THREAD | Self::PUBLIC_THREAD | Self::PRIVATE_THREAD
        )
    }
}

impl Debug for ChannelType {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        if let Some(name) = self.name() {
            f.debug_struct("ChannelType")
                .field("name", &name)
                .field("value", &self.0)
                .finish()
        } else {
            f.debug_tuple("ChannelType").field(&self.0).finish()
        }
    }
}

impl From<u8> for ChannelType {
    fn from(value: u8) -> Self {
        Self(value)
    }
}

impl From<ChannelType> for u8 {
    fn from(value: ChannelType) -> Self {
        value.get()
    }
}

#[cfg(test)]
mod tests {
    use super::ChannelType;
    use serde_test::Token;
    use static_assertions::const_assert;

    const_assert!(ChannelType::GUILD_CATEGORY.is_guild());
    const_assert!(ChannelType::GUILD_DIRECTORY.is_guild());
    const_assert!(ChannelType::GUILD_ANNOUNCEMENT.is_guild());
    const_assert!(ChannelType::ANNOUNCEMENT_THREAD.is_guild());
    const_assert!(ChannelType::PUBLIC_THREAD.is_guild());
    const_assert!(ChannelType::PRIVATE_THREAD.is_guild());
    const_assert!(ChannelType::GUILD_STAGE_VOICE.is_guild());
    const_assert!(ChannelType::GUILD_TEXT.is_guild());
    const_assert!(ChannelType::GUILD_VOICE.is_guild());

    const_assert!(ChannelType::ANNOUNCEMENT_THREAD.is_thread());
    const_assert!(ChannelType::PUBLIC_THREAD.is_thread());
    const_assert!(ChannelType::PRIVATE_THREAD.is_thread());

    const MAP: &[(ChannelType, u8, &str)] = &[
        (ChannelType::GUILD_TEXT, 0, "GUILD_TEXT"),
        (ChannelType::PRIVATE, 1, "PRIVATE"),
        (ChannelType::GUILD_VOICE, 2, "GUILD_VOICE"),
        (ChannelType::GROUP, 3, "GROUP"),
        (ChannelType::GUILD_CATEGORY, 4, "GUILD_CATEGORY"),
        (ChannelType::GUILD_ANNOUNCEMENT, 5, "GUILD_ANNOUNCEMENT"),
        (ChannelType::ANNOUNCEMENT_THREAD, 10, "ANNOUNCEMENT_THREAD"),
        (ChannelType::PUBLIC_THREAD, 11, "PUBLIC_THREAD"),
        (ChannelType::PRIVATE_THREAD, 12, "PRIVATE_THREAD"),
        (ChannelType::GUILD_STAGE_VOICE, 13, "GUILD_STAGE_VOICE"),
        (ChannelType::GUILD_DIRECTORY, 14, "GUILD_DIRECTORY"),
        (ChannelType::GUILD_FORUM, 15, "GUILD_FORUM"),
    ];

    #[test]
    fn variants() {
        for (kind, num, name) in MAP {
            serde_test::assert_tokens(
                kind,
                &[
                    Token::NewtypeStruct {
                        name: "ChannelType",
                    },
                    Token::U8(*num),
                ],
            );
            assert_eq!(*kind, ChannelType::from(*num));
            assert_eq!(*num, kind.get());
            assert_eq!(kind.name(), Some(*name));
        }
    }
}
