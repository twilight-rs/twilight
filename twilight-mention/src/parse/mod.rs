//! Parse mentions out of strings.
//!
//! Included is a trait over select IDs that can be mentioned and an iterator
//! to lazily parse mentions.
//!
//! There is also the [`MentionType`]: it's an enum wrapping all possible types
//! of mentions and works just like the individual IDs and [`Timestamp`].
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
//! use twilight_model::id::{
//!     marker::{ChannelMarker, EmojiMarker, RoleMarker},
//!     Id,
//! };
//!
//! # fn main() -> Result<(), Box<dyn std::error::Error>> {
//! assert_eq!(Id::<EmojiMarker>::new(123), Id::parse("<:name:123>")?);
//! assert_eq!(Id::<RoleMarker>::new(456), Id::parse("<@&456>")?);
//! assert!(Id::<ChannelMarker>::parse("<#notamention>").is_err());
//! # Ok(()) }
//! ```
//!
//! Iterate over the user mentions in a buffer:
//!
//! ```
//! use twilight_mention::ParseMention;
//! use twilight_model::id::{marker::UserMarker, Id};
//!
//! let mut iter = Id::<UserMarker>::iter("these <@123> are <#456> mentions <@789>");
//! assert!(matches!(iter.next(), Some((user, _, _)) if user.get() == 123));
//! assert!(matches!(iter.next(), Some((user, _, _)) if user.get() == 789));
//! assert!(iter.next().is_none());
//! ```
//!
//! Parse a timestamp:
//!
//! ```
//! # fn main() -> Result<(), Box<dyn std::error::Error>> {
//! use twilight_mention::{
//!     parse::ParseMention,
//!     timestamp::{Timestamp, TimestampStyle},
//! };
//!
//! let expected_timestamp = Timestamp::new(1_600_000_000, Some(TimestampStyle::RelativeTime));
//! assert_eq!(expected_timestamp, Timestamp::parse("<t:1600000000:R>")?);
//! # Ok(()) }
//! ```
//!
//! [`Timestamp`]: crate::timestamp::Timestamp

mod error;
mod r#impl;
mod iter;

use crate::{Mention, timestamp::Timestamp};

pub use self::{
    error::{ParseMentionError, ParseMentionErrorType},
    r#impl::ParseMention,
    iter::MentionIter,
};

use std::fmt::{Display, Formatter, Result as FmtResult};
use twilight_model::id::{
    Id,
    marker::{ChannelMarker, EmojiMarker, RoleMarker, UserMarker},
};

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
/// use twilight_mention::{
///     parse::{MentionType, ParseMention},
///     timestamp::Timestamp,
/// };
/// use twilight_model::id::{
///     marker::{ChannelMarker, RoleMarker, UserMarker},
///     Id,
/// };
///
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
/// assert_eq!(
///     MentionType::Channel(Id::<ChannelMarker>::new(123)),
///     MentionType::parse("<#123>")?,
/// );
/// assert_eq!(
///     MentionType::Role(Id::<RoleMarker>::new(123)),
///     MentionType::parse("<@&123>")?,
/// );
///
/// let timestamp = Timestamp::new(123, None);
/// assert_eq!(
///     MentionType::Timestamp(timestamp),
///     MentionType::parse("<t:123>")?
/// );
/// # Ok(()) }
/// ```
///
/// Iterate over all types of mentions in a buffer:
///
/// ```
/// use twilight_mention::{
///     parse::{MentionType, ParseMention},
///     timestamp::Timestamp,
/// };
///
/// let buf = "channel <#12> emoji <:name:34> role <@&56> timestamp <t:1624047978> user <@78>";
///
/// let mut iter = MentionType::iter(buf);
/// assert!(matches!(iter.next(), Some((MentionType::Channel(channel), _, _)) if channel.get() == 12));
/// assert!(matches!(iter.next(), Some((MentionType::Emoji(emoji), _, _)) if emoji.get() == 34));
/// assert!(matches!(iter.next(), Some((MentionType::Role(role), _, _)) if role.get() == 56));
/// assert!(matches!(
///     iter.next(),
///     Some((MentionType::Timestamp(timestamp), _, _))
///     if timestamp.unix() == 1_624_047_978 && timestamp.style().is_none()
/// ));
/// assert!(matches!(iter.next(), Some((MentionType::User(user), _, _)) if user.get() == 78));
/// assert!(iter.next().is_none());
/// ```
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum MentionType {
    /// Channel mention.
    Channel(Id<ChannelMarker>),
    /// Emoji mention.
    Emoji(Id<EmojiMarker>),
    /// Role mention.
    Role(Id<RoleMarker>),
    /// Timestamp mention.
    Timestamp(Timestamp),
    /// User mention.
    User(Id<UserMarker>),
}

impl Display for MentionType {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            Self::Channel(id) => Display::fmt(id, f),
            Self::Emoji(id) => Display::fmt(id, f),
            Self::Role(id) => Display::fmt(id, f),
            Self::Timestamp(timestamp) => Display::fmt(&timestamp.mention(), f),
            Self::User(id) => Display::fmt(id, f),
        }
    }
}
