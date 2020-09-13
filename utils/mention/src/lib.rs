//! # twilight-mention
//!
//! [![discord badge][]][discord link] [![github badge][]][github link] [![license badge][]][license link] ![rust badge]
//!
//! `twilight-mention` is a utility crate for the Discord [`twilight-rs`]
//! ecosystem to mention its model types.
//!
//! With this library, you can create mentions for various types, such as users,
//! emojis, roles, members, or channels.
//!
//! ## Examples
//!
//! Create a mention formatter for a user ID, and then format it in a message:
//!
//! ```rust
//! use twilight_mention::Mention;
//! use twilight_model::id::UserId;
//!
//! let user_id = UserId(123);
//! let message = format!("Hey there, {}!", user_id.mention());
//! ```
//!
//! [`twilight-rs`]: https://github.com/twilight-rs/twilight
//! [discord badge]: https://img.shields.io/discord/745809834183753828?color=%237289DA&label=discord%20server&logo=discord&style=for-the-badge
//! [discord link]: https://discord.gg/7jj8n7D
//! [github badge]: https://img.shields.io/badge/github-twilight-6f42c1.svg?style=for-the-badge&logo=github
//! [github link]: https://github.com/twilight-rs/twilight
//! [license badge]: https://img.shields.io/badge/license-ISC-blue.svg?style=for-the-badge&logo=pastebin
//! [license link]: https://github.com/twilight-rs/twilight/blob/trunk/LICENSE.md
//! [rust badge]: https://img.shields.io/badge/rust-stable-93450a.svg?style=for-the-badge&logo=rust

#![deny(
    clippy::all,
    clippy::pedantic,
    future_incompatible,
    missing_docs,
    nonstandard_style,
    rust_2018_idioms,
    unsafe_code,
    unused,
    warnings
)]

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
/// assert_eq!("<@123>", UserId(123).mention().to_string());
/// ```
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MentionFormat<T>(T);

/// Mention a channel. This will format as `<#ID>`.
impl Display for MentionFormat<ChannelId> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        f.write_fmt(format_args!("<#{}>", self.0))
    }
}

/// Mention an emoji. This will format as `<:emoji:ID>`.
impl Display for MentionFormat<EmojiId> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        f.write_fmt(format_args!("<:emoji:{}>", self.0))
    }
}

/// Mention a role. This will format as `<@&ID>`.
impl Display for MentionFormat<RoleId> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        f.write_fmt(format_args!("<@&{}>", self.0))
    }
}

/// Mention a user. This will format as `<@ID>`.
impl Display for MentionFormat<UserId> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        f.write_fmt(format_args!("<@{}>", self.0))
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
/// assert_eq!("<#123>", ChannelId(123).mention().to_string());
/// ```
pub trait Mention<T> {
    /// Mention a resource by using its ID.
    fn mention(&self) -> MentionFormat<T>;
}

/// Mention a channel ID. This will format as `<#ID>`.
impl Mention<ChannelId> for ChannelId {
    fn mention(&self) -> MentionFormat<ChannelId> {
        MentionFormat(*self)
    }
}

/// Mention a channel ID. This will format as `<#ID>`.
impl Mention<ChannelId> for &'_ ChannelId {
    fn mention(&self) -> MentionFormat<ChannelId> {
        (*self).mention()
    }
}

/// Mention a guild category channel. This will format as `<#ID>`.
impl Mention<ChannelId> for CategoryChannel {
    fn mention(&self) -> MentionFormat<ChannelId> {
        MentionFormat(self.id)
    }
}

/// Mention a guild category channel. This will format as `<#ID>`.
impl Mention<ChannelId> for &'_ CategoryChannel {
    fn mention(&self) -> MentionFormat<ChannelId> {
        (*self).mention()
    }
}

/// Mention a channel. This will format as `<#ID>`.
impl Mention<ChannelId> for Channel {
    fn mention(&self) -> MentionFormat<ChannelId> {
        MentionFormat(self.id())
    }
}

/// Mention a channel. This will format as `<#ID>`.
impl Mention<ChannelId> for &'_ Channel {
    fn mention(&self) -> MentionFormat<ChannelId> {
        (*self).mention()
    }
}

/// Mention the current user. This will format as `<@ID>`.
impl Mention<UserId> for CurrentUser {
    fn mention(&self) -> MentionFormat<UserId> {
        MentionFormat(self.id)
    }
}

/// Mention the current user. This will format as `<@ID>`.
impl Mention<UserId> for &'_ CurrentUser {
    fn mention(&self) -> MentionFormat<UserId> {
        (*self).mention()
    }
}

/// Mention an emoji. This will format as `<:emoji:ID>`.
impl Mention<EmojiId> for EmojiId {
    fn mention(&self) -> MentionFormat<EmojiId> {
        MentionFormat(*self)
    }
}

/// Mention an emoji. This will format as `<:emoji:ID>`.
impl Mention<EmojiId> for &'_ EmojiId {
    fn mention(&self) -> MentionFormat<EmojiId> {
        (*self).mention()
    }
}

/// Mention an emoji. This will format as `<:emoji:ID>`.
impl Mention<EmojiId> for Emoji {
    fn mention(&self) -> MentionFormat<EmojiId> {
        MentionFormat(self.id)
    }
}

/// Mention an emoji. This will format as `<:emoji:ID>`.
impl Mention<EmojiId> for &'_ Emoji {
    fn mention(&self) -> MentionFormat<EmojiId> {
        (*self).mention()
    }
}

/// Mention a group. This will format as `<#ID>`.
impl Mention<ChannelId> for Group {
    fn mention(&self) -> MentionFormat<ChannelId> {
        MentionFormat(self.id)
    }
}

/// Mention a group. This will format as `<#ID>`.
impl Mention<ChannelId> for &'_ Group {
    fn mention(&self) -> MentionFormat<ChannelId> {
        (*self).mention()
    }
}

/// Mention a guild channel. This will format as `<#ID>`.
impl Mention<ChannelId> for GuildChannel {
    fn mention(&self) -> MentionFormat<ChannelId> {
        MentionFormat(self.id())
    }
}

/// Mention a guild channel. This will format as `<#ID>`.
impl Mention<ChannelId> for &'_ GuildChannel {
    fn mention(&self) -> MentionFormat<ChannelId> {
        (*self).mention()
    }
}

/// Mention a member's user. This will format as `<@ID>`.
impl Mention<UserId> for Member {
    fn mention(&self) -> MentionFormat<UserId> {
        MentionFormat(self.user.id)
    }
}

/// Mention a member's user. This will format as `<@ID>`.
impl Mention<UserId> for &'_ Member {
    fn mention(&self) -> MentionFormat<UserId> {
        (*self).mention()
    }
}

/// Mention a private channel. This will format as `<#ID>`.
impl Mention<ChannelId> for PrivateChannel {
    fn mention(&self) -> MentionFormat<ChannelId> {
        MentionFormat(self.id)
    }
}

/// Mention a private channel. This will format as `<#ID>`.
impl Mention<ChannelId> for &'_ PrivateChannel {
    fn mention(&self) -> MentionFormat<ChannelId> {
        (*self).mention()
    }
}

/// Mention a role ID. This will format as `<@&ID>`.
impl Mention<RoleId> for RoleId {
    fn mention(&self) -> MentionFormat<RoleId> {
        MentionFormat(*self)
    }
}

/// Mention a role ID. This will format as `<@&ID>`.
impl Mention<RoleId> for &'_ RoleId {
    fn mention(&self) -> MentionFormat<RoleId> {
        (*self).mention()
    }
}

/// Mention a role ID. This will format as `<@&ID>`.
impl Mention<RoleId> for Role {
    fn mention(&self) -> MentionFormat<RoleId> {
        MentionFormat(self.id)
    }
}

/// Mention a role ID. This will format as `<@&ID>`.
impl Mention<RoleId> for &'_ Role {
    fn mention(&self) -> MentionFormat<RoleId> {
        (*self).mention()
    }
}

/// Mention a guild text channel. This will format as `<#ID>`.
impl Mention<ChannelId> for TextChannel {
    fn mention(&self) -> MentionFormat<ChannelId> {
        MentionFormat(self.id)
    }
}

/// Mention a guild text channel. This will format as `<#ID>`.
impl Mention<ChannelId> for &'_ TextChannel {
    fn mention(&self) -> MentionFormat<ChannelId> {
        (*self).mention()
    }
}

/// Mention a user ID. This will format as `<&ID>`.
impl Mention<UserId> for UserId {
    fn mention(&self) -> MentionFormat<UserId> {
        MentionFormat(*self)
    }
}

/// Mention a user ID. This will format as `<&ID>`.
impl Mention<UserId> for &'_ UserId {
    fn mention(&self) -> MentionFormat<UserId> {
        (*self).mention()
    }
}

/// Mention a user. This will format as `<&ID>`.
impl Mention<UserId> for User {
    fn mention(&self) -> MentionFormat<UserId> {
        MentionFormat(self.id)
    }
}

/// Mention a user. This will format as `<&ID>`.
impl Mention<UserId> for &'_ User {
    fn mention(&self) -> MentionFormat<UserId> {
        (*self).mention()
    }
}

/// Mention a guild voice channel. This will format as `<#ID>`.
impl Mention<ChannelId> for VoiceChannel {
    fn mention(&self) -> MentionFormat<ChannelId> {
        MentionFormat(self.id)
    }
}

/// Mention a guild voice channel. This will format as `<#ID>`.
impl Mention<ChannelId> for &'_ VoiceChannel {
    fn mention(&self) -> MentionFormat<ChannelId> {
        (*self).mention()
    }
}

#[cfg(test)]
mod tests {
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
        assert_eq!("<#123>", ChannelId(123).mention().to_string());
    }

    #[test]
    fn test_mention_format_emoji_id() {
        assert_eq!("<:emoji:123>", EmojiId(123).mention().to_string());
    }

    #[test]
    fn test_mention_format_role_id() {
        assert_eq!("<@&123>", RoleId(123).mention().to_string());
    }

    #[test]
    fn test_mention_format_user_id() {
        assert_eq!("<@123>", UserId(123).mention().to_string());
    }
}
