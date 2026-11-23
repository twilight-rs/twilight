//! Data types for performing a message search.

use serde::{Deserialize, Serialize};
use std::fmt::{Display, Error as FmtError, Formatter};

mod sealed {
    use super::{AuthorType, SearchHasType};

    pub trait Sealed {}

    impl Sealed for AuthorType {}
    impl Sealed for SearchHasType {}
}

/// Denotes whether a field value can be negated, excluding results with the
/// field instead of including results.
// # Development implementation note
//
// When implementing this trait for a new type be sure to make a note on the
// relevant HTTP request builder method that the value can be negated.
pub trait Negate: Display + sealed::Sealed {}

impl Negate for AuthorType {}

impl Negate for SearchHasType {}

/// Mark whether a field is being marked for inclusion or exclusion.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum SearchFilter<T: Negate> {
    /// Results must include this value.
    Include(T),
    /// Results must exclude this value.
    Exclude(T),
}

impl<T: Negate> Display for SearchFilter<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), FmtError> {
        match self {
            SearchFilter::Include(value) => Display::fmt(value, f),
            SearchFilter::Exclude(value) => {
                f.write_str("-")?;

                Display::fmt(value, f)
            }
        }
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
pub enum SearchHasType {
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

impl Display for SearchHasType {
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
pub enum SearchEmbedType {
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

impl Display for SearchEmbedType {
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
pub enum SearchSortMode {
    /// Sort by the relevance of the message to the search query.
    Relevance,
    /// Sort by the message creation time.
    ///
    /// This is the default sort mode Discord uses.
    Timestamp,
}

impl Display for SearchSortMode {
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
        AuthorType, Negate, SearchEmbedType, SearchFilter, SearchHasType, SearchSortMode,
        SearchSortOrder,
    };
    use serde::{Deserialize, Serialize};
    use static_assertions::assert_impl_all;
    use std::{fmt::Debug, hash::Hash};

    assert_impl_all!(AuthorType: Clone, Copy, Debug, Deserialize<'static>, Eq, Hash, Negate, PartialEq, Send, Serialize, Sync);
    assert_impl_all!(SearchEmbedType: Clone, Copy, Debug, Deserialize<'static>, Eq, Hash, PartialEq, Send, Serialize, Sync);
    assert_impl_all!(SearchHasType: Clone, Copy, Debug, Deserialize<'static>, Eq, Hash, Negate, PartialEq, Send, Serialize, Sync);
    assert_impl_all!(SearchSortMode: Clone, Copy, Debug, Deserialize<'static>, Eq, Hash, PartialEq, Send, Serialize, Sync);
    assert_impl_all!(SearchSortOrder: Clone, Copy, Debug, Deserialize<'static>, Eq, Hash, PartialEq, Send, Serialize, Sync);

    #[test]
    fn filter_display() {
        assert_eq!(
            "file",
            SearchFilter::Include(SearchHasType::File).to_string()
        );
        assert_eq!(
            "-file",
            SearchFilter::Exclude(SearchHasType::File).to_string()
        );
    }

    #[test]
    fn author_type_display() {
        assert_eq!("bot", AuthorType::Bot.to_string());
        assert_eq!("user", AuthorType::User.to_string());
        assert_eq!("webhook", AuthorType::Webhook.to_string());
    }

    #[test]
    fn search_has_types_display() {
        assert_eq!("embed", SearchHasType::Embed.to_string());
        assert_eq!("file", SearchHasType::File.to_string());
        assert_eq!("image", SearchHasType::Image.to_string());
        assert_eq!("link", SearchHasType::Link.to_string());
        assert_eq!("poll", SearchHasType::Poll.to_string());
        assert_eq!("snapshot", SearchHasType::Snapshot.to_string());
        assert_eq!("sound", SearchHasType::Sound.to_string());
        assert_eq!("sticker", SearchHasType::Sticker.to_string());
        assert_eq!("video", SearchHasType::Video.to_string());
    }

    #[test]
    fn search_embed_types_display() {
        assert_eq!("article", SearchEmbedType::Article.to_string());
        assert_eq!("gif", SearchEmbedType::Gif.to_string());
        assert_eq!("image", SearchEmbedType::Image.to_string());
        assert_eq!("sound", SearchEmbedType::Sound.to_string());
        assert_eq!("video", SearchEmbedType::Video.to_string());
    }

    #[test]
    fn search_sort_modes_display() {
        assert_eq!("relevance", SearchSortMode::Relevance.to_string());
        assert_eq!("timestamp", SearchSortMode::Timestamp.to_string());
    }

    #[test]
    fn search_sort_order_display() {
        assert_eq!("asc", SearchSortOrder::Ascending.to_string());
        assert_eq!("desc", SearchSortOrder::Descending.to_string());
    }
}
