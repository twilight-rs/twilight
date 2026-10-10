//! Data types for performing a message search.

use serde::{Deserialize, Serialize};
use std::fmt::{Display, Error as FmtError, Formatter};

/// Denotes whether a field can be negated, excluding results with the field
/// instead of including results.
// # Development implementation note
//
// When implementing this trait for a new type be sure to make a note on the
// relevant HTTP request builder method that the value can be negated.
pub trait Negate: Display {}

impl Negate for AuthorType {}

impl Negate for SearchHasTypes {}

/// Mark whether a field is being marked for inclusion or exclusion.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Filter<T: Negate> {
    /// Results must include this value.
    Include(T),
    /// Results must exclude this value.
    Exclude(T),
}

impl<T: Negate> Display for Filter<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), FmtError> {
        match self {
            Filter::Include(value) => Display::fmt(value, f),
            Filter::Exclude(value) => {
                f.write_str("-")?;

                Display::fmt(value, f)
            }
        }
    }
}

impl<T: Negate> From<T> for Filter<T> {
    fn from(value: T) -> Self {
        Self::Include(value)
    }
}

/// Filter messages by the type of author.
///
/// All types can be negated, which means results will not include messages that
/// match the type.
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum AuthorType {
    /// Return messages sent by bot accounts.
    Bot,
    /// Return messages sent by user accounts.
    User,
    /// Return messages sent by webhooks.
    Webhook,
}

impl Display for AuthorType {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), FmtError> {
        match self {
            Self::Bot => f.write_str("bot"),
            Self::User => f.write_str("user"),
            Self::Webhook => f.write_str("webhook"),
        }
    }
}

/// Filter messages by whether they have a particular type of data.
///
/// All types can be negated, which means results will not include messages that
/// match the type.
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum SearchHasTypes {
    /// Return messages that have an embed.
    Embed,
    /// Return messages that have an attachment.
    File,
    /// Return messages that have an image.
    Image,
    /// Return messages that have a link.
    Link,
    /// Return messages that have a poll.
    Poll,
    /// Return messages that have a forwarded message.
    Snapshot,
    /// Return messages that have a sound attachment.
    Sound,
    /// Return messages that have a sent sticker.
    Sticker,
    /// Return messages that have a video.
    Video,
}

impl Display for SearchHasTypes {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), FmtError> {
        match self {
            Self::Embed => f.write_str("embed"),
            Self::File => f.write_str("file"),
            Self::Image => f.write_str("image"),
            Self::Link => f.write_str("link"),
            Self::Poll => f.write_str("poll"),
            Self::Snapshot => f.write_str("snapshot"),
            Self::Sound => f.write_str("sound"),
            Self::Sticker => f.write_str("sticker"),
            Self::Video => f.write_str("video"),
        }
    }
}

/// Embed types to filter results by.
///
/// These do not correspond 1:1 to actual embed types and encompass a wider
/// range of actual types.
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum SearchEmbedTypes {
    /// Return messages that have an article embed.
    Article,
    /// Return messages that have a gifv embed.
    ///
    /// Messages sent before February 24, 2026 may not be properly indexed under
    /// the gif embed type.
    Gif,
    /// Return messages that have an image embed.
    Image,
    /// Return messages that have a sound embed.
    Sound,
    /// Return messages that have a video embed.
    Video,
}

impl Display for SearchEmbedTypes {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), FmtError> {
        match self {
            Self::Article => f.write_str("article"),
            Self::Gif => f.write_str("gif"),
            Self::Image => f.write_str("image"),
            Self::Sound => f.write_str("sound"),
            Self::Video => f.write_str("video"),
        }
    }
}

/// Sorting algorithm to use.
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum SearchSortModes {
    /// Sort by the relevance of the message to the search query.
    Relevance,
    /// Sort by the message creation time.
    ///
    /// This is the default sort mode Discord uses.
    Timestamp,
}

impl Display for SearchSortModes {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), FmtError> {
        match self {
            Self::Relevance => f.write_str("relevance"),
            Self::Timestamp => f.write_str("timestamp"),
        }
    }
}

/// Sorting order to use.
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub enum SearchSortOrder {
    /// Results will be in ascending order.
    #[serde(rename = "asc")]
    Ascending,
    /// Results will be in descending order.
    #[serde(rename = "desc")]
    Descending,
}

impl Display for SearchSortOrder {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), FmtError> {
        match self {
            Self::Ascending => f.write_str("asc"),
            Self::Descending => f.write_str("desc"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{
        AuthorType, Filter, Negate, SearchEmbedTypes, SearchHasTypes, SearchSortModes,
        SearchSortOrder,
    };
    use serde::{Deserialize, Serialize};
    use static_assertions::assert_impl_all;
    use std::{fmt::Debug, hash::Hash};

    assert_impl_all!(AuthorType: Clone, Copy, Debug, Deserialize<'static>, Eq, Hash, Negate, PartialEq, Send, Serialize, Sync);
    assert_impl_all!(SearchEmbedTypes: Clone, Copy, Debug, Deserialize<'static>, Eq, Hash, PartialEq, Send, Serialize, Sync);
    assert_impl_all!(SearchHasTypes: Clone, Copy, Debug, Deserialize<'static>, Eq, Hash, Negate, PartialEq, Send, Serialize, Sync);
    assert_impl_all!(SearchSortModes: Clone, Copy, Debug, Deserialize<'static>, Eq, Hash, PartialEq, Send, Serialize, Sync);
    assert_impl_all!(SearchSortOrder: Clone, Copy, Debug, Deserialize<'static>, Eq, Hash, PartialEq, Send, Serialize, Sync);

    #[test]
    fn filter_display() {
        assert_eq!("file", Filter::Include(SearchHasTypes::File).to_string());
        assert_eq!("-file", Filter::Exclude(SearchHasTypes::File).to_string());
    }

    #[test]
    fn author_type_display() {
        assert_eq!("bot", AuthorType::Bot.to_string());
        assert_eq!("user", AuthorType::User.to_string());
        assert_eq!("webhook", AuthorType::Webhook.to_string());
    }

    #[test]
    fn search_has_types_display() {
        assert_eq!("embed", SearchHasTypes::Embed.to_string());
        assert_eq!("file", SearchHasTypes::File.to_string());
        assert_eq!("image", SearchHasTypes::Image.to_string());
        assert_eq!("link", SearchHasTypes::Link.to_string());
        assert_eq!("poll", SearchHasTypes::Poll.to_string());
        assert_eq!("snapshot", SearchHasTypes::Snapshot.to_string());
        assert_eq!("sound", SearchHasTypes::Sound.to_string());
        assert_eq!("sticker", SearchHasTypes::Sticker.to_string());
        assert_eq!("video", SearchHasTypes::Video.to_string());
    }

    #[test]
    fn search_embed_types_display() {
        assert_eq!("article", SearchEmbedTypes::Article.to_string());
        assert_eq!("gif", SearchEmbedTypes::Gif.to_string());
        assert_eq!("image", SearchEmbedTypes::Image.to_string());
        assert_eq!("sound", SearchEmbedTypes::Sound.to_string());
        assert_eq!("video", SearchEmbedTypes::Video.to_string());
    }

    #[test]
    fn search_sort_modes_display() {
        assert_eq!("relevance", SearchSortModes::Relevance.to_string());
        assert_eq!("timestamp", SearchSortModes::Timestamp.to_string());
    }

    #[test]
    fn search_sort_order_display() {
        assert_eq!("asc", SearchSortOrder::Ascending.to_string());
        assert_eq!("desc", SearchSortOrder::Descending.to_string());
    }
}
