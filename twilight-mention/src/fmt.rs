//! Formatters for creating mentions.

use super::timestamp::Timestamp;
use std::fmt::{Display, Formatter, Result as FmtResult, Write};
use twilight_model::{
    channel::Channel,
    guild::{Emoji, Member, Role},
    id::{
        marker::{ChannelMarker, CommandMarker, EmojiMarker, RoleMarker, UserMarker},
        Id,
    },
    user::{CurrentUser, User},
};

/// Formatter to mention a resource that implements `std::fmt::Display`.
///
/// # Examples
///
/// Mention a `Id<UserMarker>`:
///
/// ```
/// use twilight_mention::Mention;
/// use twilight_model::id::{marker::UserMarker, Id};
///
/// assert_eq!("<@123>", Id::<UserMarker>::new(123).mention().to_string());
/// ```
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MentionFormat<T>(T);

/// Mention a channel. This will format as `<#ID>`.
impl Display for MentionFormat<Id<ChannelMarker>> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        f.write_str("<#")?;
        Display::fmt(&self.0, f)?;

        f.write_str(">")
    }
}

/// Mention a command. This will format as:
/// - `</NAME:COMMAND_ID>` for commands
/// - `</NAME SUBCOMMAND:ID>` for subcommands
/// - `</NAME SUBCOMMAND_GROUP SUBCOMMAND:ID>` for subcommand groups
impl Display for MentionFormat<CommandMention> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        f.write_str("</")?;

        match &self.0 {
            CommandMention::Command { name, id } => {
                // </NAME:COMMAND_ID>
                f.write_str(name)?;
                f.write_char(':')?;
                Display::fmt(id, f)?;
            }
            CommandMention::SubCommand {
                name,
                sub_command,
                id,
            } => {
                // </NAME SUBCOMMAND:ID>
                f.write_str(name)?;
                f.write_char(' ')?;
                f.write_str(sub_command)?;
                f.write_char(':')?;
                Display::fmt(id, f)?;
            }
            CommandMention::SubCommandGroup {
                name,
                sub_command_group,
                sub_command,
                id,
            } => {
                // </NAME SUBCOMMAND_GROUP SUBCOMMAND:ID>
                f.write_str(name)?;
                f.write_char(' ')?;
                f.write_str(sub_command_group)?;
                f.write_char(' ')?;
                f.write_str(sub_command)?;
                f.write_char(':')?;
                Display::fmt(id, f)?;
            }
        }

        f.write_char('>')
    }
}

/// Mention an emoji. This will format as `<:emoji:ID>`.
impl Display for MentionFormat<Id<EmojiMarker>> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        f.write_str("<:emoji:")?;
        Display::fmt(&self.0, f)?;

        f.write_str(">")
    }
}

/// Mention a role. This will format as `<@&ID>`.
impl Display for MentionFormat<Id<RoleMarker>> {
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
impl Display for MentionFormat<Id<UserMarker>> {
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
/// Mention a channel ID:
///
/// ```
/// use twilight_mention::Mention;
/// use twilight_model::id::{marker::ChannelMarker, Id};
///
/// let id = Id::<ChannelMarker>::new(123);
/// assert_eq!("<#123>", id.mention().to_string());
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
impl Mention<Id<ChannelMarker>> for Id<ChannelMarker> {
    fn mention(&self) -> MentionFormat<Id<ChannelMarker>> {
        MentionFormat(*self)
    }
}

/// Mention a channel. This will format as `<#ID>`.
impl Mention<Id<ChannelMarker>> for Channel {
    fn mention(&self) -> MentionFormat<Id<ChannelMarker>> {
        MentionFormat(self.id)
    }
}

/// Mention the current user. This will format as `<@ID>`.
impl Mention<Id<UserMarker>> for CurrentUser {
    fn mention(&self) -> MentionFormat<Id<UserMarker>> {
        MentionFormat(self.id)
    }
}

/// Mention an emoji. This will format as `<:emoji:ID>`.
impl Mention<Id<EmojiMarker>> for Id<EmojiMarker> {
    fn mention(&self) -> MentionFormat<Id<EmojiMarker>> {
        MentionFormat(*self)
    }
}

/// Mention an emoji. This will format as `<:emoji:ID>`.
impl Mention<Id<EmojiMarker>> for Emoji {
    fn mention(&self) -> MentionFormat<Id<EmojiMarker>> {
        MentionFormat(self.id)
    }
}

/// Mention a member's user. This will format as `<@ID>`.
impl Mention<Id<UserMarker>> for Member {
    fn mention(&self) -> MentionFormat<Id<UserMarker>> {
        MentionFormat(self.user.id)
    }
}

/// Mention a role ID. This will format as `<@&ID>`.
impl Mention<Id<RoleMarker>> for Id<RoleMarker> {
    fn mention(&self) -> MentionFormat<Id<RoleMarker>> {
        MentionFormat(*self)
    }
}

/// Mention a role ID. This will format as `<@&ID>`.
impl Mention<Id<RoleMarker>> for Role {
    fn mention(&self) -> MentionFormat<Id<RoleMarker>> {
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
impl Mention<Id<UserMarker>> for Id<UserMarker> {
    fn mention(&self) -> MentionFormat<Id<UserMarker>> {
        MentionFormat(*self)
    }
}

/// Mention a user. This will format as `<&ID>`.
impl Mention<Id<UserMarker>> for User {
    fn mention(&self) -> MentionFormat<Id<UserMarker>> {
        MentionFormat(self.id)
    }
}

/// Components to construct a slash command mention.
///
/// Format slash commands, subcommands and subcommand groups.
/// See [Discord Docs/Message Formatting].
/// See [Discord Docs Changelog/Slash Command Mentions].
///
/// [Discord Docs/Message Formatting]: https://discord.com/developers/docs/reference#message-formatting
/// [Discord Docs Changelog/Slash Command Mentions]: https://discord.com/developers/docs/change-log#slash-command-mentions
#[allow(missing_docs)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum CommandMention {
    Command {
        id: Id<CommandMarker>,
        name: String,
    },

    SubCommand {
        id: Id<CommandMarker>,
        name: String,
        sub_command: String,
    },

    SubCommandGroup {
        id: Id<CommandMarker>,
        name: String,
        sub_command: String,
        sub_command_group: String,
    },
}

#[cfg(test)]
mod tests {
    use crate::timestamp::{Timestamp, TimestampStyle};

    use super::{CommandMention, Mention, MentionFormat};
    use static_assertions::assert_impl_all;
    use std::fmt::{Debug, Display};
    use twilight_model::id::marker::CommandMarker;
    use twilight_model::{
        channel::Channel,
        guild::{Emoji, Member, Role},
        id::{
            marker::{ChannelMarker, EmojiMarker, RoleMarker, UserMarker},
            Id,
        },
        user::{CurrentUser, User},
    };

    assert_impl_all!(MentionFormat<()>: Clone, Copy, Debug, Eq, PartialEq, Send, Sync);
    assert_impl_all!(MentionFormat<Id<ChannelMarker>>: Clone, Copy, Debug, Display, Eq, PartialEq, Send, Sync);
    assert_impl_all!(MentionFormat<CommandMention>: Clone, Debug, Display, Eq, PartialEq, Send, Sync);
    assert_impl_all!(MentionFormat<Id<EmojiMarker>>: Clone, Copy, Debug, Display, Eq, PartialEq, Send, Sync);
    assert_impl_all!(MentionFormat<Id<RoleMarker>>: Clone, Copy, Debug, Display, Eq, PartialEq, Send, Sync);
    assert_impl_all!(MentionFormat<Id<UserMarker>>: Clone, Copy, Debug, Display, Eq, PartialEq, Send, Sync);
    assert_impl_all!(Id<ChannelMarker>: Mention<Id<ChannelMarker>>);
    assert_impl_all!(&'static Id<ChannelMarker>: Mention<Id<ChannelMarker>>);
    assert_impl_all!(Channel: Mention<Id<ChannelMarker>>);
    assert_impl_all!(&'static Channel: Mention<Id<ChannelMarker>>);
    assert_impl_all!(CurrentUser: Mention<Id<UserMarker>>);
    assert_impl_all!(&'static CurrentUser: Mention<Id<UserMarker>>);
    assert_impl_all!(Id<EmojiMarker>: Mention<Id<EmojiMarker>>);
    assert_impl_all!(&'static Id<EmojiMarker>: Mention<Id<EmojiMarker>>);
    assert_impl_all!(Emoji: Mention<Id<EmojiMarker>>);
    assert_impl_all!(&'static Emoji: Mention<Id<EmojiMarker>>);
    assert_impl_all!(Member: Mention<Id<UserMarker>>);
    assert_impl_all!(&'static Member: Mention<Id<UserMarker>>);
    assert_impl_all!(Id<RoleMarker>: Mention<Id<RoleMarker>>);
    assert_impl_all!(&'static Id<RoleMarker>: Mention<Id<RoleMarker>>);
    assert_impl_all!(Role: Mention<Id<RoleMarker>>);
    assert_impl_all!(&'static Role: Mention<Id<RoleMarker>>);
    assert_impl_all!(Id<UserMarker>: Mention<Id<UserMarker>>);
    assert_impl_all!(&'static Id<UserMarker>: Mention<Id<UserMarker>>);
    assert_impl_all!(User: Mention<Id<UserMarker>>);
    assert_impl_all!(&'static User: Mention<Id<UserMarker>>);

    #[test]
    fn mention_format_channel_id() {
        assert_eq!(
            "<#123>",
            Id::<ChannelMarker>::new(123).mention().to_string()
        );
    }

    #[test]
    fn mention_format_command() {
        assert_eq!(
            "</name:123>",
            MentionFormat(CommandMention::Command {
                id: Id::<CommandMarker>::new(123),
                name: "name".to_string()
            })
            .to_string()
        );
    }

    #[test]
    fn mention_format_sub_command() {
        assert_eq!(
            "</name subcommand:123>",
            MentionFormat(CommandMention::SubCommand {
                id: Id::<CommandMarker>::new(123),
                name: "name".to_string(),
                sub_command: "subcommand".to_string()
            })
            .to_string()
        );
    }

    #[test]
    fn mention_format_sub_command_group() {
        assert_eq!(
            "</name subcommand_group subcommand:123>",
            MentionFormat(CommandMention::SubCommandGroup {
                id: Id::<CommandMarker>::new(123),
                name: "name".to_string(),
                sub_command: "subcommand".to_string(),
                sub_command_group: "subcommand_group".to_string()
            })
            .to_string()
        );
    }

    #[test]
    fn mention_format_emoji_id() {
        assert_eq!(
            "<:emoji:123>",
            Id::<EmojiMarker>::new(123).mention().to_string()
        );
    }

    #[test]
    fn mention_format_role_id() {
        assert_eq!("<@&123>", Id::<RoleMarker>::new(123).mention().to_string());
    }

    /// Test that a timestamp with a style displays correctly.
    #[test]
    fn mention_format_timestamp_styled() {
        let timestamp = Timestamp::new(1_624_047_064, Some(TimestampStyle::RelativeTime));

        assert_eq!("<t:1624047064:R>", timestamp.mention().to_string());
    }

    /// Test that a timestamp without a style displays correctly.
    #[test]
    fn mention_format_timestamp_unstyled() {
        let timestamp = Timestamp::new(1_624_047_064, None);

        assert_eq!("<t:1624047064>", timestamp.mention().to_string());
    }

    #[test]
    fn mention_format_user_id() {
        assert_eq!("<@123>", Id::<UserMarker>::new(123).mention().to_string());
    }
}
