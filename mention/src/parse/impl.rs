use super::{MentionIter, MentionType, ParseMentionError, ParseMentionErrorType};
use std::str::Chars;
use twilight_model::id::{ChannelId, EmojiId, RoleId, UserId};

/// Parse mentions out of buffers.
///
/// While the syntax of mentions will be validated and the IDs within them
/// parsed, they won't be validated as being proper snowflakes or as real IDs in
/// use.
///
/// **Note** that this trait is sealed and is not meant to be manually
/// implemented.
pub trait ParseMention: private::Sealed {
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
    /// Returns a [`ParseMentionErrorType::LeadingArrow`] error type if the
    /// leading arrow is not present.
    ///
    /// Returns a [`ParseMentionErrorType::Sigil`] error type if the mention
    /// type's sigil is not present after the leading arrow.
    ///
    /// Returns a [`ParseMentionErrorType::TrailingArrow`] error type if the
    /// trailing arrow is not present after the ID.
    fn parse(buf: &str) -> Result<Self, ParseMentionError<'_>>
    where
        Self: Sized;

    /// Search a buffer for mentions and parse out any that are encountered.
    ///
    /// Unlike [`parse`], this will not error if anything that is indicative of
    /// a mention is encountered but did not successfully parse, such as a `<`
    /// but with no trailing mention sigil.
    ///
    /// [`parse`]: Self::parse
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
        parse_id(buf, Self::SIGILS).map(|(id, _)| ChannelId(id))
    }
}

impl ParseMention for EmojiId {
    const SIGILS: &'static [&'static str] = &[":"];

    fn parse(buf: &str) -> Result<Self, ParseMentionError<'_>>
    where
        Self: Sized,
    {
        parse_id(buf, Self::SIGILS).map(|(id, _)| EmojiId(id))
    }
}

impl ParseMention for MentionType {
    /// Sigils for any type of mention.
    ///
    /// Contains all of the sigils of every other type of mention.
    const SIGILS: &'static [&'static str] = &["#", ":", "@&", "@!", "@"];

    fn parse(buf: &str) -> Result<Self, ParseMentionError<'_>>
    where
        Self: Sized,
    {
        let (id, found) = parse_id(buf, Self::SIGILS)?;

        for sigil in ChannelId::SIGILS {
            if *sigil == found {
                return Ok(MentionType::Channel(ChannelId(id)));
            }
        }

        for sigil in EmojiId::SIGILS {
            if *sigil == found {
                return Ok(MentionType::Emoji(EmojiId(id)));
            }
        }

        for sigil in RoleId::SIGILS {
            if *sigil == found {
                return Ok(MentionType::Role(RoleId(id)));
            }
        }

        for sigil in UserId::SIGILS {
            if *sigil == found {
                return Ok(MentionType::User(UserId(id)));
            }
        }

        unreachable!("mention type must have been found");
    }
}

impl ParseMention for RoleId {
    const SIGILS: &'static [&'static str] = &["@&"];

    fn parse(buf: &str) -> Result<Self, ParseMentionError<'_>>
    where
        Self: Sized,
    {
        parse_id(buf, Self::SIGILS).map(|(id, _)| RoleId(id))
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
        parse_id(buf, Self::SIGILS).map(|(id, _)| UserId(id))
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
fn parse_id<'a>(
    buf: &'a str,
    sigils: &'a [&'a str],
) -> Result<(u64, &'a str), ParseMentionError<'a>> {
    let mut chars = buf.chars();

    let c = chars.next();

    if c.map_or(true, |c| c != '<') {
        return Err(ParseMentionError {
            kind: ParseMentionErrorType::LeadingArrow { found: c },
            source: None,
        });
    }

    let maybe_sigil = sigils.iter().find(|sigil| {
        if chars.as_str().starts_with(*sigil) {
            for _ in 0..sigil.chars().count() {
                chars.next();
            }

            return true;
        }

        false
    });

    let sigil = if let Some(sigil) = maybe_sigil {
        *sigil
    } else {
        return Err(ParseMentionError {
            kind: ParseMentionErrorType::Sigil {
                expected: sigils,
                found: chars.next(),
            },
            source: None,
        });
    };

    if sigil == ":" && !emoji_sigil_present(&mut chars) {
        return Err(ParseMentionError {
            kind: ParseMentionErrorType::PartMissing {
                found: 1,
                expected: 2,
            },
            source: None,
        });
    }

    let remaining = chars
        .as_str()
        .find('>')
        .and_then(|idx| chars.as_str().get(..idx))
        .ok_or(ParseMentionError {
            kind: ParseMentionErrorType::TrailingArrow { found: None },
            source: None,
        })?;

    remaining
        .parse()
        .map(|id| (id, sigil))
        .map_err(|source| ParseMentionError {
            kind: ParseMentionErrorType::IdNotU64 { found: remaining },
            source: Some(Box::new(source)),
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
    use super::super::MentionType;
    use twilight_model::id::{ChannelId, EmojiId, RoleId, UserId};

    pub trait Sealed {}

    impl Sealed for ChannelId {}
    impl Sealed for EmojiId {}
    impl Sealed for MentionType {}
    impl Sealed for RoleId {}
    impl Sealed for UserId {}
}

#[cfg(test)]
mod tests {
    use super::{
        super::{MentionType, ParseMentionErrorType},
        private::Sealed,
        ParseMention,
    };
    use static_assertions::assert_impl_all;
    use twilight_model::id::{ChannelId, EmojiId, RoleId, UserId};

    assert_impl_all!(ChannelId: ParseMention, Sealed);
    assert_impl_all!(EmojiId: ParseMention, Sealed);
    assert_impl_all!(MentionType: ParseMention, Sealed);
    assert_impl_all!(RoleId: ParseMention, Sealed);
    assert_impl_all!(UserId: ParseMention, Sealed);

    #[test]
    fn test_sigils() {
        assert_eq!(&["#"], ChannelId::SIGILS);
        assert_eq!(&[":"], EmojiId::SIGILS);
        assert_eq!(&["#", ":", "@&", "@!", "@"], MentionType::SIGILS);
        assert_eq!(&["@&"], RoleId::SIGILS);
        assert_eq!(&["@!", "@"], UserId::SIGILS);
    }

    #[test]
    fn test_parse_channel_id() {
        assert_eq!(ChannelId(123), ChannelId::parse("<#123>").unwrap());
        assert_eq!(
            &ParseMentionErrorType::Sigil {
                expected: &["#"],
                found: Some('@'),
            },
            ChannelId::parse("<@123>").unwrap_err().kind(),
        );
    }

    #[test]
    fn test_parse_emoji_id() {
        assert_eq!(EmojiId(123), EmojiId::parse("<:name:123>").unwrap());
        assert_eq!(
            &ParseMentionErrorType::Sigil {
                expected: &[":"],
                found: Some('@'),
            },
            EmojiId::parse("<@123>").unwrap_err().kind(),
        );
    }

    #[test]
    fn test_parse_mention_type() {
        assert_eq!(
            MentionType::Channel(ChannelId(123)),
            MentionType::parse("<#123>").unwrap()
        );
        assert_eq!(
            MentionType::Emoji(EmojiId(123)),
            MentionType::parse("<:name:123>").unwrap()
        );
        assert_eq!(
            MentionType::Role(RoleId(123)),
            MentionType::parse("<@&123>").unwrap()
        );
        assert_eq!(
            MentionType::User(UserId(123)),
            MentionType::parse("<@123>").unwrap()
        );
        assert_eq!(
            &ParseMentionErrorType::Sigil {
                expected: &["#", ":", "@&", "@!", "@"],
                found: Some(';'),
            },
            MentionType::parse("<;123>").unwrap_err().kind(),
        );
    }

    #[test]
    fn test_parse_role_id() {
        assert_eq!(RoleId(123), RoleId::parse("<@&123>").unwrap());
        assert_eq!(
            &ParseMentionErrorType::Sigil {
                expected: &["@&"],
                found: Some('@'),
            },
            RoleId::parse("<@123>").unwrap_err().kind(),
        );
    }

    #[test]
    fn test_parse_user_id() {
        assert_eq!(UserId(123), UserId::parse("<@123>").unwrap());
        assert_eq!(
            &ParseMentionErrorType::IdNotU64 { found: "&123" },
            UserId::parse("<@&123>").unwrap_err().kind(),
        );
    }

    #[test]
    fn test_parse_id_wrong_sigil() {
        assert_eq!(
            &ParseMentionErrorType::Sigil {
                expected: &["@"],
                found: Some('#'),
            },
            super::parse_id("<#123>", &["@"]).unwrap_err().kind(),
        );
        assert_eq!(
            &ParseMentionErrorType::Sigil {
                expected: &["#"],
                found: None,
            },
            super::parse_id("<", &["#"]).unwrap_err().kind(),
        );
        assert_eq!(
            &ParseMentionErrorType::Sigil {
                expected: &["#"],
                found: None,
            },
            super::parse_id("<", &["#"]).unwrap_err().kind(),
        );
    }
}
