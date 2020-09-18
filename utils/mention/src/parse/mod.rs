//! Parse mentions out of strings.
//!
//! Included is a trait over select IDs that can be mentioned and an iterator
//! to lazily parse mentions.
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
//! assert_eq!(Some(UserId(123)), iter.next());
//! assert_eq!(Some(UserId(789)), iter.next());
//! assert!(iter.next().is_none());
//! ```

mod error;
mod r#impl;
mod iter;

pub use self::{error::ParseMentionError, iter::MentionIter, r#impl::ParseMention};
