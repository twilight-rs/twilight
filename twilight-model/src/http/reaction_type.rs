use crate::id::{marker::EmojiMarker, Id};
use percent_encoding::{utf8_percent_encode, NON_ALPHANUMERIC};
use std::fmt::{Display, Formatter, Result as FmtResult};

/// Specifically for HTTP requests, type of [`Reaction`].
///
/// [`Reaction`]: crate::channel::message::Reaction
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum RequestReactionType<'a> {
    /// Custom [`Emoji`].
    ///
    /// [`Emoji`]: crate::guild::Emoji
    Custom {
        /// Emoji identifier.
        id: Id<EmojiMarker>,
    },
    /// Standard [Unicode] emoji value.
    ///
    /// Unicode reactions must be specified by their unicode value, and *not*
    /// their Discord display name. Instead of using ":arrow_right:", use "➡️".
    ///
    /// [Unicode]: https://unicode.org/emoji/
    Unicode {
        /// Unicode name identifier.
        name: &'a str,
    },
}

/// Format a [`RequestReactionType`] into a format acceptable for use in URLs.
///
/// # Examples
///
/// Format a custom reaction for use in a URL:
///
/// ```
/// use twilight_http::request::channel::reaction::RequestReactionType;
/// use twilight_model::id::Id;
///
/// let reaction = RequestReactionType::Custom {
///     id: Id::new(123),
///     name: Some("rarity"),
/// };
///
/// assert_eq!("rarity:123", reaction.to_string());
/// ```
///
/// Format the transgeneder flag for use in a URL:
///
/// ```
/// use twilight_http::request::channel::reaction::RequestReactionType;
///
/// let reaction = RequestReactionType::Unicode {
///     name: "🏳️‍⚧️"
/// };
///
/// assert_eq!(
///     "%F0%9F%8F%B3%EF%B8%8F%E2%80%8D%E2%9A%A7%EF%B8%8F",
///     reaction.to_string(),
/// );
/// ```
impl Display for RequestReactionType<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            RequestReactionType::Custom { id } => {
                // Discord requires a name, but it need not be correct. Let's use `e`.
                f.write_str("e:")?;

                Display::fmt(id, f)
            }
            RequestReactionType::Unicode { name } => {
                Display::fmt(&utf8_percent_encode(name, NON_ALPHANUMERIC), f)
            }
        }
    }
}
#[cfg(test)]
mod tests {
    use super::RequestReactionType;
    use crate::id::Id;
    use static_assertions::{assert_fields, assert_impl_all};
    use std::{
        fmt::{Debug, Display},
        hash::Hash,
    };

    assert_fields!(RequestReactionType::Custom: id);
    assert_fields!(RequestReactionType::Unicode: name);
    assert_impl_all!(RequestReactionType<'_>: Clone, Copy, Debug, Display, Eq, Hash, PartialEq, Send, Sync);

    #[test]
    fn display_custom() {
        let reaction = RequestReactionType::Custom { id: Id::new(123) };

        assert_eq!("e:123", reaction.to_string());
    }

    /// Test that unicode reactions format with percent encoding.
    #[test]
    fn display_unicode() {
        let reaction = RequestReactionType::Unicode {
            // Rainbow flag
            name: "\u{1F3F3}\u{FE0F}\u{200D}\u{1F308}",
        };

        assert_eq!(
            "%F0%9F%8F%B3%EF%B8%8F%E2%80%8D%F0%9F%8C%88",
            reaction.to_string()
        );
    }
}
