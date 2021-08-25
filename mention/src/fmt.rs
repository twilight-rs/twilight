//! Formatters for creating mentions.

use super::timestamp::Timestamp;
use std::fmt::{Display, Formatter, Result as FmtResult};
use twilight_model::{
    channel::{
        CategoryChannel, Channel, Group, GuildChannel, PrivateChannel, TextChannel, VoiceChannel,
    },
    guild::{Emoji, Member, Role},
    id::{ChannelId, EmojiId, RoleId, UserId},
    user::{CurrentUser, User},
};

/// Formatter to mention a resource that implements `std::fmt::Display`.
///
/// # Examples
///
/// Mention a `UserId`:
///
/// ```rust
/// use twilight_mention::Mention;
/// use twilight_model::id::UserId;
///
/// assert_eq!("<@123>", UserId::new(123).expect("non zero").mention().to_string());
/// ```
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MentionFormat<T>(T);

/// Mention a channel. This will format as `<#ID>`.
impl Display for MentionFormat<ChannelId> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        f.write_str("<#")?;
        Display::fmt(&self.0, f)?;

        f.write_str(">")
    }
}

/// Mention an emoji. This will format as `<:emoji:ID>`.
impl Display for MentionFormat<EmojiId> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        f.write_str("<:emoji:")?;
        Display::fmt(&self.0, f)?;

        f.write_str(">")
    }
}

/// Mention a role. This will format as `<@&ID>`.
impl Display for MentionFormat<RoleId> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        f.write_str("<@&")?;
        Display::fmt(&self.0, f)?;

        f.write_str(">")
    }
}

/// Mention a user. This will format as `<t:UNIX>` if a style is not specified or
/// `<t:UNIX:STYLE>` if a style is specified.
impl Display for MentionFormat<Timestamp> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        f.write_str("<t:")?;
        Display::fmt(&self.0.unix(), f)?;

        if let Some(style) = self.0.style() {
            f.write_str(":")?;
            Display::fmt(&style, f)?;
        }

        f.write_str(">")
    }
}

/// Mention a user. This will format as `<@ID>`.
impl Display for MentionFormat<UserId> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        f.write_str("<@")?;
        Display::fmt(&self.0, f)?;

        f.write_str(">")
    }
}

/// Mention a resource, such as an emoji or user.
///
/// This will create a mention that will link to a user if it exists.
///
/// Look at the implementations list to see what you can mention.
///
/// # Examples
///
/// Mention a `ChannelId`:
///
/// ```rust
/// use twilight_mention::Mention;
/// use twilight_model::id::ChannelId;
///
/// assert_eq!("<#123>", ChannelId::new(123).expect("non zero").mention().to_string());
/// ```
pub trait Mention<T> {
    /// Mention a resource by using its ID.
    fn mention(&self) -> MentionFormat<T>;
}

impl<T, M: Mention<T>> Mention<T> for &'_ M {
    fn mention(&self) -> MentionFormat<T> {
        (*self).mention()
    }
}

/// Mention a channel ID. This will format as `<#ID>`.
impl Mention<ChannelId> for ChannelId {
    fn mention(&self) -> MentionFormat<ChannelId> {
        MentionFormat(*self)
    }
}

/// Mention a guild category channel. This will format as `<#ID>`.
impl Mention<ChannelId> for CategoryChannel {
    fn mention(&self) -> MentionFormat<ChannelId> {
        MentionFormat(self.id)
    }
}

/// Mention a channel. This will format as `<#ID>`.
impl Mention<ChannelId> for Channel {
    fn mention(&self) -> MentionFormat<ChannelId> {
        MentionFormat(self.id())
    }
}

/// Mention the current user. This will format as `<@ID>`.
impl Mention<UserId> for CurrentUser {
    fn mention(&self) -> MentionFormat<UserId> {
        MentionFormat(self.id)
    }
}

/// Mention an emoji. This will format as `<:emoji:ID>`.
impl Mention<EmojiId> for EmojiId {
    fn mention(&self) -> MentionFormat<EmojiId> {
        MentionFormat(*self)
    }
}

/// Mention an emoji. This will format as `<:emoji:ID>`.
impl Mention<EmojiId> for Emoji {
    fn mention(&self) -> MentionFormat<EmojiId> {
        MentionFormat(self.id)
    }
}

/// Mention a group. This will format as `<#ID>`.
impl Mention<ChannelId> for Group {
    fn mention(&self) -> MentionFormat<ChannelId> {
        MentionFormat(self.id)
    }
}

/// Mention a guild channel. This will format as `<#ID>`.
impl Mention<ChannelId> for GuildChannel {
    fn mention(&self) -> MentionFormat<ChannelId> {
        MentionFormat(self.id())
    }
}

/// Mention a member's user. This will format as `<@ID>`.
impl Mention<UserId> for Member {
    fn mention(&self) -> MentionFormat<UserId> {
        MentionFormat(self.user.id)
    }
}

/// Mention a private channel. This will format as `<#ID>`.
impl Mention<ChannelId> for PrivateChannel {
    fn mention(&self) -> MentionFormat<ChannelId> {
        MentionFormat(self.id)
    }
}

/// Mention a role ID. This will format as `<@&ID>`.
impl Mention<RoleId> for RoleId {
    fn mention(&self) -> MentionFormat<RoleId> {
        MentionFormat(*self)
    }
}

/// Mention a role ID. This will format as `<@&ID>`.
impl Mention<RoleId> for Role {
    fn mention(&self) -> MentionFormat<RoleId> {
        MentionFormat(self.id)
    }
}

/// Mention a guild text channel. This will format as `<#ID>`.
impl Mention<ChannelId> for TextChannel {
    fn mention(&self) -> MentionFormat<ChannelId> {
        MentionFormat(self.id)
    }
}

/// Mention a timestamp. This will format as `<t:UNIX>` if a style is not
/// specified or `<t:UNIX:STYLE>` if a style is specified.
impl Mention<Self> for Timestamp {
    fn mention(&self) -> MentionFormat<Self> {
        MentionFormat(*self)
    }
}

/// Mention a user ID. This will format as `<&ID>`.
impl Mention<UserId> for UserId {
    fn mention(&self) -> MentionFormat<UserId> {
        MentionFormat(*self)
    }
}

/// Mention a user. This will format as `<&ID>`.
impl Mention<UserId> for User {
    fn mention(&self) -> MentionFormat<UserId> {
        MentionFormat(self.id)
    }
}

/// Mention a guild voice channel. This will format as `<#ID>`.
impl Mention<ChannelId> for VoiceChannel {
    fn mention(&self) -> MentionFormat<ChannelId> {
        MentionFormat(self.id)
    }
}

#[cfg(test)]
mod tests {
    use crate::timestamp::{Timestamp, TimestampStyle};

    use super::{Mention, MentionFormat};
    use static_assertions::assert_impl_all;
    use std::fmt::{Debug, Display};
    use twilight_model::{
        channel::{
            CategoryChannel, Channel, Group, GuildChannel, PrivateChannel, TextChannel,
            VoiceChannel,
        },
        guild::{Emoji, Member, Role},
        id::{ChannelId, EmojiId, RoleId, UserId},
        user::{CurrentUser, User},
    };

    assert_impl_all!(MentionFormat<()>: Clone, Copy, Debug, Eq, PartialEq, Send, Sync);
    assert_impl_all!(MentionFormat<ChannelId>: Clone, Copy, Debug, Display, Eq, PartialEq, Send, Sync);
    assert_impl_all!(MentionFormat<EmojiId>: Clone, Copy, Debug, Display, Eq, PartialEq, Send, Sync);
    assert_impl_all!(MentionFormat<RoleId>: Clone, Copy, Debug, Display, Eq, PartialEq, Send, Sync);
    assert_impl_all!(MentionFormat<UserId>: Clone, Copy, Debug, Display, Eq, PartialEq, Send, Sync);
    assert_impl_all!(ChannelId: Mention<ChannelId>);
    assert_impl_all!(&'static ChannelId: Mention<ChannelId>);
    assert_impl_all!(CategoryChannel: Mention<ChannelId>);
    assert_impl_all!(&'static CategoryChannel: Mention<ChannelId>);
    assert_impl_all!(Channel: Mention<ChannelId>);
    assert_impl_all!(&'static Channel: Mention<ChannelId>);
    assert_impl_all!(CurrentUser: Mention<UserId>);
    assert_impl_all!(&'static CurrentUser: Mention<UserId>);
    assert_impl_all!(EmojiId: Mention<EmojiId>);
    assert_impl_all!(&'static EmojiId: Mention<EmojiId>);
    assert_impl_all!(Emoji: Mention<EmojiId>);
    assert_impl_all!(&'static Emoji: Mention<EmojiId>);
    assert_impl_all!(Group: Mention<ChannelId>);
    assert_impl_all!(&'static Group: Mention<ChannelId>);
    assert_impl_all!(GuildChannel: Mention<ChannelId>);
    assert_impl_all!(&'static GuildChannel: Mention<ChannelId>);
    assert_impl_all!(Member: Mention<UserId>);
    assert_impl_all!(&'static Member: Mention<UserId>);
    assert_impl_all!(PrivateChannel: Mention<ChannelId>);
    assert_impl_all!(&'static PrivateChannel: Mention<ChannelId>);
    assert_impl_all!(RoleId: Mention<RoleId>);
    assert_impl_all!(&'static RoleId: Mention<RoleId>);
    assert_impl_all!(Role: Mention<RoleId>);
    assert_impl_all!(&'static Role: Mention<RoleId>);
    assert_impl_all!(TextChannel: Mention<ChannelId>);
    assert_impl_all!(&'static TextChannel: Mention<ChannelId>);
    assert_impl_all!(UserId: Mention<UserId>);
    assert_impl_all!(&'static UserId: Mention<UserId>);
    assert_impl_all!(User: Mention<UserId>);
    assert_impl_all!(&'static User: Mention<UserId>);
    assert_impl_all!(VoiceChannel: Mention<ChannelId>);
    assert_impl_all!(&'static VoiceChannel: Mention<ChannelId>);

    #[test]
    fn test_mention_format_channel_id() {
        assert_eq!(
            "<#123>",
            ChannelId::new(123).expect("non zero").mention().to_string()
        );
    }

    #[test]
    fn test_mention_format_emoji_id() {
        assert_eq!(
            "<:emoji:123>",
            EmojiId::new(123).expect("non zero").mention().to_string()
        );
    }

    #[test]
    fn test_mention_format_role_id() {
        assert_eq!(
            "<@&123>",
            RoleId::new(123).expect("non zero").mention().to_string()
        );
    }

    /// Test that a timestamp with a style displays correctly.
    #[test]
    fn test_mention_format_timestamp_styled() {
        let timestamp = Timestamp::new(1_624_047_064, Some(TimestampStyle::RelativeTime));

        assert_eq!("<t:1624047064:R>", timestamp.mention().to_string());
    }

    /// Test that a timestamp without a style displays correctly.
    #[test]
    fn test_mention_format_timestamp_unstyled() {
        let timestamp = Timestamp::new(1_624_047_064, None);

        assert_eq!("<t:1624047064>", timestamp.mention().to_string());
    }

    #[test]
    fn test_mention_format_user_id() {
        assert_eq!(
            "<@123>",
            UserId::new(123).expect("non zero").mention().to_string()
        );
    }
}
