//! Parse mentions out of strings.
//!
//! Included is a trait over select IDs that can be mentioned and an iterator
//! to lazily parse mentions.
//!
//! There is also the [`MentionType`]: it's an enum wrapping all possible types
//! of mentions and works just like the individual IDs.
//!
//! While the syntax of mentions will be validated and the IDs within them
//! parsed, they won't be validated as being proper snowflakes or as real IDs in
//! use.
//!
//! # Examples
//!
//! Parse IDs out of strings that you know is just a mention:
//!
//! ```
//! use twilight_mention::ParseMention;
//! use twilight_model::id::{ChannelId, EmojiId, RoleId};
//!
//! # fn main() -> Result<(), Box<dyn std::error::Error>> {
//! assert_eq!(EmojiId(123), EmojiId::parse("<:name:123>")?);
//! assert_eq!(RoleId(456), RoleId::parse("<@&456>")?);
//! assert!(ChannelId::parse("<#notamention>").is_err());
//! # Ok(()) }
//! ```
//!
//! Iterate over the user mentions in a buffer:
//!
//! ```
//! use twilight_mention::ParseMention;
//! use twilight_model::id::UserId;
//!
//! let mut iter = UserId::iter("these <@123> are <#456> mentions <@789>");
//! assert!(matches!(iter.next(), Some((UserId(123), _, _))));
//! assert!(matches!(iter.next(), Some((UserId(789), _, _))));
//! assert!(iter.next().is_none());
//! ```

mod error;
mod r#impl;
mod iter;

pub use self::{
    error::{ParseMentionError, ParseMentionErrorType},
    iter::MentionIter,
    r#impl::ParseMention,
};

use std::fmt::{Display, Formatter, Result as FmtResult};
use twilight_model::id::{ChannelId, EmojiId, RoleId, UserId};

/// Any type of mention.
///
/// Contains variants for every possible kind of mention. Can be used with
/// [`ParseMention`] and iterated over just like any other mention.
///
/// # Examples
///
/// Parse any type of mention out of a string:
///
/// ```
/// use twilight_mention::parse::{MentionType, ParseMention};
/// use twilight_model::id::{ChannelId, RoleId, UserId};
///
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
/// assert_eq!(MentionType::Channel(ChannelId(123)), MentionType::parse("<#123>")?);
/// assert_eq!(MentionType::Role(RoleId(123)), MentionType::parse("<@&123>")?);
/// assert_eq!(MentionType::User(UserId(123)), MentionType::parse("<@!123>")?);
/// # Ok(()) }
/// ```
///
/// Iterate over all types of mentions in a buffer:
///
/// ```
/// use twilight_mention::parse::{MentionType, ParseMention};
/// use twilight_model::id::{ChannelId, EmojiId, RoleId, UserId};
///
/// let buf = "channel <#12> emoji <:name:34> role <@&56> user <@78>";
///
/// let mut iter = MentionType::iter(buf);
/// assert!(matches!(iter.next(), Some((MentionType::Channel(ChannelId(12)), _, _))));
/// assert!(matches!(iter.next(), Some((MentionType::Emoji(EmojiId(34)), _, _))));
/// assert!(matches!(iter.next(), Some((MentionType::Role(RoleId(56)), _, _))));
/// assert!(matches!(iter.next(), Some((MentionType::User(UserId(78)), _, _))));
/// assert!(iter.next().is_none());
/// ```
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum MentionType {
    /// Channel mention.
    Channel(ChannelId),
    /// Emoji mention.
    Emoji(EmojiId),
    /// Role mention.
    Role(RoleId),
    /// User mention.
    User(UserId),
}

impl Display for MentionType {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            Self::Channel(id) => Display::fmt(id, f),
            Self::Emoji(id) => Display::fmt(id, f),
            Self::Role(id) => Display::fmt(id, f),
            Self::User(id) => Display::fmt(id, f),
        }
    }
}
