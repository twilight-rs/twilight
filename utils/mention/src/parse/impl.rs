use super::{MentionIter, ParseMentionError};
use std::str::Chars;
use twilight_model::id::{ChannelId, EmojiId, RoleId, UserId};

/// Parse mentions out of buffers.
///
/// **Note** that this trait is sealed and is not meant to be manually
/// implemented.
pub trait ParseMention: private::Sealed {
    /// The number of "parts" in the mention.
    ///
    /// A part is defined as sections split by a `:`. For most mentions, like
    /// user or channel mentions, there is only one part: the ID (in `<@12>`,
    /// the only part is `12`). An exception is emojis, which have two parts:
    /// the name and ID (in `<:name:12>`, the first part is `name` and the
    /// second is `12`).
    const PARTS: usize = 1;

    /// Leading sigil(s) of the mention after the leading arrow (`<`).
    ///
    /// In a channel mention, the sigil is `#`. In the case of a user mention,
    /// the sigil may be either `@` or `@!`.
    const SIGILS: &'static [&'static str];

    /// Parse a mention out of a buffer.
    ///
    /// This will not search the buffer for a mention and will instead treat the
    /// entire buffer as a mention.
    ///
    /// # Examples
    ///
    /// ```
    /// use twilight_mention::ParseMention;
    /// use twilight_model::id::{ChannelId, UserId};
    ///
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// assert_eq!(ChannelId(123), ChannelId::parse("<#123>")?);
    /// assert_eq!(UserId(456), UserId::parse("<@456>")?);
    /// assert!(ChannelId::parse("not a mention").is_err());
    /// # Ok(()) }
    /// ```
    ///
    /// # Errors
    ///
    /// Returns [`ParseMentionError::LeadingArrow`] if the leading arrow is not
    /// present.
    ///
    /// Returns [`ParseMentionError::Sigil`] if the mention type's sigil is not
    /// present after the leading arrow.
    ///
    /// Returns [`ParseMentionError::TrailingArrow`] if the trailing arrow is
    /// not present after the ID.
    ///
    /// [`ParseMentionError::LeadingArrow`]: enum.ParseMentionError.html#variant.LeadingArrow
    /// [`ParseMentionError::Sigil`]: enum.ParseMentionError.html#variant.Sigil
    /// [`ParseMentionError::TrailingArrow`]: enum.ParseMentionError.html#variant.TrailingArrow
    fn parse(buf: &str) -> Result<Self, ParseMentionError<'_>>
    where
        Self: Sized;

    /// Search a buffer for mentions and parse out any that are encountered.
    ///
    /// Unlike [`parse`], this will not error if anything that is indicative of
    /// a mention is encountered but did not successfully parse, such as a `<`
    /// but with no trailing mention sigil.
    ///
    /// [`parse`]: #tymethod.parse
    #[must_use = "you must use the iterator to lazily parse mentions"]
    fn iter(buf: &str) -> MentionIter<'_, Self>
    where
        Self: Sized,
    {
        MentionIter::new(buf)
    }
}

impl ParseMention for ChannelId {
    const SIGILS: &'static [&'static str] = &["#"];

    fn parse(buf: &str) -> Result<Self, ParseMentionError<'_>>
    where
        Self: Sized,
    {
        parse_id(buf, Self::SIGILS, Self::PARTS).map(ChannelId)
    }
}

impl ParseMention for EmojiId {
    const PARTS: usize = 2;
    const SIGILS: &'static [&'static str] = &[":"];

    fn parse(buf: &str) -> Result<Self, ParseMentionError<'_>>
    where
        Self: Sized,
    {
        parse_id(buf, Self::SIGILS, Self::PARTS).map(EmojiId)
    }
}

impl ParseMention for RoleId {
    const SIGILS: &'static [&'static str] = &["@&"];

    fn parse(buf: &str) -> Result<Self, ParseMentionError<'_>>
    where
        Self: Sized,
    {
        parse_id(buf, Self::SIGILS, Self::PARTS).map(RoleId)
    }
}

impl ParseMention for UserId {
    /// Sigils for User ID mentions.
    ///
    /// Unlike other IDs, user IDs have two possible sigils: `@!` and `@`.
    const SIGILS: &'static [&'static str] = &["@!", "@"];

    fn parse(buf: &str) -> Result<Self, ParseMentionError<'_>>
    where
        Self: Sized,
    {
        parse_id(buf, Self::SIGILS, Self::PARTS).map(UserId)
    }
}

/// # Errors
///
/// Returns [`ParseMentionError::LeadingArrow`] if the leading arrow is not
/// present.
///
/// Returns [`ParseMentionError::Sigil`] if the mention type's sigil is not
/// present after the leading arrow.
///
/// Returns [`ParseMentionError::TrailingArrow`] if the trailing arrow is not
/// present after the ID.
///
/// [`ParseMentionError::LeadingArrow`]: enum.ParseMentionError.html#variant.LeadingArrow
/// [`ParseMentionError::Sigil`]: enum.ParseMentionError.html#variant.Sigil
/// [`ParseMentionError::TrailingArrow`]: enum.ParseMentionError.html#variant.TrailingArrow
fn parse_id<'a>(
    buf: &'a str,
    sigils: &'a [&'a str],
    parts: usize,
) -> Result<u64, ParseMentionError<'a>> {
    let mut chars = buf.chars();

    let c = chars.next();

    if c.map_or(true, |c| c != '<') {
        return Err(ParseMentionError::LeadingArrow { found: c });
    }

    let sigil_found = sigils.iter().any(|sigil| {
        if chars.as_str().starts_with(sigil) {
            for _ in 0..sigil.chars().count() {
                chars.next();
            }

            return true;
        }

        false
    });

    if !sigil_found {
        return Err(ParseMentionError::Sigil {
            expected: sigils,
            found: chars.next(),
        });
    }

    if parts == 2 && !emoji_sigil_present(&mut chars) {
        return Err(ParseMentionError::PartMissing {
            found: 1,
            expected: 2,
        });
    }

    let remaining = chars
        .as_str()
        .find('>')
        .and_then(|idx| chars.as_str().get(..idx))
        .ok_or_else(|| ParseMentionError::TrailingArrow { found: None })?;

    remaining
        .parse()
        .map_err(|source| ParseMentionError::IdNotU64 {
            found: remaining,
            source,
        })
}

// Don't use `Iterator::skip_while` so we can mutate `chars` in-place;
// `skip_while` is consuming.
fn emoji_sigil_present(chars: &mut Chars<'_>) -> bool {
    for c in chars {
        if c == ':' {
            return true;
        }
    }

    false
}

/// Rust doesn't allow leaking private implementations, but if we make the trait
/// public in a private scope then it gets by the restriction and doesn't allow
/// Sealed to be named.
///
/// Yes, this is the correct way of sealing a trait:
///
/// <https://rust-lang.github.io/api-guidelines/future-proofing.html>
mod private {
    use twilight_model::id::{ChannelId, EmojiId, RoleId, UserId};

    pub trait Sealed {}

    impl Sealed for ChannelId {}
    impl Sealed for EmojiId {}
    impl Sealed for RoleId {}
    impl Sealed for UserId {}
}

#[cfg(test)]
mod tests {
    use super::{super::ParseMentionError, private::Sealed, ParseMention};
    use static_assertions::{assert_impl_all, const_assert_eq};
    use twilight_model::id::{ChannelId, EmojiId, RoleId, UserId};

    assert_impl_all!(ChannelId: ParseMention, Sealed);
    assert_impl_all!(EmojiId: ParseMention, Sealed);
    assert_impl_all!(RoleId: ParseMention, Sealed);
    assert_impl_all!(UserId: ParseMention, Sealed);
    const_assert_eq!(1, ChannelId::PARTS);
    const_assert_eq!(2, EmojiId::PARTS);
    const_assert_eq!(1, RoleId::PARTS);
    const_assert_eq!(1, UserId::PARTS);

    #[test]
    fn test_sigils() {
        assert_eq!(&["#"], ChannelId::SIGILS);
        assert_eq!(&[":"], EmojiId::SIGILS);
        assert_eq!(&["@&"], RoleId::SIGILS);
        assert_eq!(&["@!", "@"], UserId::SIGILS);
    }

    #[test]
    fn test_parse_channel_id() {
        assert_eq!(ChannelId(123), ChannelId::parse("<#123>").unwrap());
        assert_eq!(
            ParseMentionError::Sigil {
                expected: &["#"],
                found: Some('@'),
            },
            ChannelId::parse("<@123>").unwrap_err(),
        );
    }

    #[test]
    fn test_parse_emoji_id() {
        assert_eq!(EmojiId(123), EmojiId::parse("<:name:123>").unwrap());
        assert_eq!(
            ParseMentionError::Sigil {
                expected: &[":"],
                found: Some('@'),
            },
            EmojiId::parse("<@123>").unwrap_err(),
        );
    }

    #[test]
    fn test_parse_role_id() {
        assert_eq!(RoleId(123), RoleId::parse("<@&123>").unwrap());
        assert_eq!(
            ParseMentionError::Sigil {
                expected: &["@&"],
                found: Some('@'),
            },
            RoleId::parse("<@123>").unwrap_err(),
        );
    }

    #[test]
    fn test_parse_user_id() {
        assert_eq!(UserId(123), UserId::parse("<@123>").unwrap());
        assert_eq!(
            ParseMentionError::IdNotU64 {
                found: "&123",
                source: "&123".parse::<u64>().unwrap_err(),
            },
            UserId::parse("<@&123>").unwrap_err(),
        );
    }

    #[test]
    fn test_parse_id_wrong_sigil() {
        assert_eq!(
            ParseMentionError::Sigil {
                expected: &["@"],
                found: Some('#'),
            },
            super::parse_id("<#123>", &["@"], 1).unwrap_err(),
        );
        assert_eq!(
            ParseMentionError::Sigil {
                expected: &["#"],
                found: None,
            },
            super::parse_id("<", &["#"], 1).unwrap_err(),
        );
        assert_eq!(
            ParseMentionError::Sigil {
                expected: &["#"],
                found: None,
            },
            super::parse_id("<", &["#"], 2).unwrap_err(),
        );
    }
}
