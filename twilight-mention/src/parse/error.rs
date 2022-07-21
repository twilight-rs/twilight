use std::{
    error::Error,
    fmt::{Display, Formatter, Result as FmtResult},
};

/// Parsing a mention failed due to invalid syntax.
#[derive(Debug)]
pub struct ParseMentionError<'a> {
    pub(super) kind: ParseMentionErrorType<'a>,
    pub(super) source: Option<Box<dyn Error + Send + Sync>>,
}

impl<'a> ParseMentionError<'a> {
    /// Immutable reference to the type of error that occurred.
    #[must_use = "retrieving the type has no effect if left unused"]
    pub const fn kind(&self) -> &ParseMentionErrorType<'_> {
        &self.kind
    }

    /// Consume the error, returning the source error if there is any.
    #[must_use = "consuming the error and retrieving the source has no effect if left unused"]
    pub fn into_source(self) -> Option<Box<dyn Error + Send + Sync>> {
        self.source
    }

    /// Consume the error, returning the owned error type and the source error.
    #[must_use = "consuming the error into its parts has no effect if left unused"]
    pub fn into_parts(
        self,
    ) -> (
        ParseMentionErrorType<'a>,
        Option<Box<dyn Error + Send + Sync>>,
    ) {
        (self.kind, self.source)
    }

    pub(super) fn trailing_arrow(found: Option<char>) -> Self {
        Self {
            kind: ParseMentionErrorType::TrailingArrow { found },
            source: None,
        }
    }
}

impl Display for ParseMentionError<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match &self.kind {
            ParseMentionErrorType::IdNotU64 { found, .. } => {
                f.write_str("id portion ('")?;
                Display::fmt(found, f)?;

                f.write_str("') of mention is not a u64")
            }
            ParseMentionErrorType::LeadingArrow { found } => {
                f.write_str("expected to find a leading arrow ('<') but instead found ")?;

                if let Some(c) = found {
                    f.write_str("'")?;
                    f.write_str(c.encode_utf8(&mut [0; 4]))?;

                    f.write_str("'")
                } else {
                    f.write_str("nothing")
                }
            }
            ParseMentionErrorType::PartMissing { expected, found } => {
                f.write_str("expected ")?;
                Display::fmt(expected, f)?;
                f.write_str(" parts but only found ")?;

                Display::fmt(found, f)
            }
            ParseMentionErrorType::Sigil { expected, found } => {
                f.write_str("expected to find a mention sigil (")?;

                for (idx, sigil) in expected.iter().enumerate() {
                    f.write_str("'")?;
                    f.write_str(sigil)?;
                    f.write_str("'")?;

                    if idx < expected.len() - 1 {
                        f.write_str(", ")?;
                    }
                }

                f.write_str(") but instead found ")?;

                if let Some(c) = found {
                    f.write_str("'")?;
                    f.write_str(c.encode_utf8(&mut [0; 4]))?;

                    f.write_str("'")
                } else {
                    f.write_str("nothing")
                }
            }
            ParseMentionErrorType::TimestampStyleInvalid { found } => {
                f.write_str("timestamp style value '")?;
                f.write_str(found)?;

                f.write_str("' is invalid")
            }
            ParseMentionErrorType::TrailingArrow { found } => {
                f.write_str("expected to find a trailing arrow ('>') but instead found ")?;

                if let Some(c) = found {
                    f.write_str("'")?;
                    f.write_str(c.encode_utf8(&mut [0; 4]))?;

                    f.write_str("'")
                } else {
                    f.write_str("nothing")
                }
            }
        }
    }
}

impl Error for ParseMentionError<'_> {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        self.source
            .as_ref()
            .map(|source| &**source as &(dyn Error + 'static))
    }
}

/// Type of [`ParseMentionError`] that occurred.
#[derive(Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum ParseMentionErrorType<'a> {
    /// ID portion of the mention isn't a u64.
    IdNotU64 {
        /// String that could not be parsed into a u64.
        found: &'a str,
    },
    /// Leading arrow (`<`) is not present.
    LeadingArrow {
        /// Character that was instead found where the leading arrow should be.
        found: Option<char>,
    },
    /// One or more parts of the mention are missing.
    ///
    /// For example, an emoji mention - `<:name:id>` - has two parts: the `name`
    /// and the `id`, separated by the sigil (`:`). If the second sigil denoting
    /// the second part can't be found, then it is missing.
    PartMissing {
        /// Number of parts that are expected.
        expected: usize,
        /// Number of parts that have been found.
        found: usize,
    },
    /// Mention's sigil is not present.
    ///
    /// Users, for example, have the sigil `@`.
    Sigil {
        /// Possible sigils that were expected for the mention type.
        expected: &'a [&'a str],
        /// Character that was instead found where the sigil should be.
        found: Option<char>,
    },
    /// Timestamp style value is invalid.
    TimestampStyleInvalid {
        /// Value of the style.
        found: &'a str,
    },
    /// Trailing arrow (`>`) is not present.
    TrailingArrow {
        /// Character that was instead found where the trailing arrow should be.
        found: Option<char>,
    },
}

#[cfg(test)]
mod tests {
    use super::{ParseMentionError, ParseMentionErrorType};
    use static_assertions::{assert_fields, assert_impl_all};
    use std::{error::Error, fmt::Debug};

    assert_fields!(ParseMentionErrorType::IdNotU64: found);
    assert_fields!(ParseMentionErrorType::LeadingArrow: found);
    assert_fields!(ParseMentionErrorType::Sigil: expected, found);
    assert_fields!(ParseMentionErrorType::TimestampStyleInvalid: found);
    assert_fields!(ParseMentionErrorType::TrailingArrow: found);
    assert_impl_all!(ParseMentionErrorType<'_>: Debug, Send, Sync);
    assert_impl_all!(ParseMentionError<'_>: Debug, Error, Send, Sync);

    #[allow(clippy::too_many_lines)]
    #[test]
    fn display() {
        let mut expected = "id portion ('abcd') of mention is not a u64";
        assert_eq!(
            expected,
            ParseMentionError {
                kind: ParseMentionErrorType::IdNotU64 { found: "abcd" },
                source: Some(Box::new("abcd".parse::<u64>().unwrap_err())),
            }
            .to_string(),
        );
        expected = "expected to find a leading arrow ('<') but instead found 'a'";
        assert_eq!(
            expected,
            ParseMentionError {
                kind: ParseMentionErrorType::LeadingArrow { found: Some('a') },
                source: None,
            }
            .to_string(),
        );

        expected = "expected to find a leading arrow ('<') but instead found nothing";
        assert_eq!(
            expected,
            ParseMentionError {
                kind: ParseMentionErrorType::LeadingArrow { found: None },
                source: None,
            }
            .to_string(),
        );

        expected = "expected to find a mention sigil ('@') but instead found '#'";
        assert_eq!(
            expected,
            ParseMentionError {
                kind: ParseMentionErrorType::Sigil {
                    expected: &["@"],
                    found: Some('#')
                },
                source: None,
            }
            .to_string(),
        );

        expected = "expected to find a mention sigil ('@') but instead found nothing";
        assert_eq!(
            expected,
            ParseMentionError {
                kind: ParseMentionErrorType::Sigil {
                    expected: &["@"],
                    found: None
                },
                source: None,
            }
            .to_string(),
        );

        expected = "expected to find a mention sigil ('@') but instead found '#'";
        assert_eq!(
            expected,
            ParseMentionError {
                kind: ParseMentionErrorType::Sigil {
                    expected: &["@"],
                    found: Some('#'),
                },
                source: None,
            }
            .to_string(),
        );

        expected = "expected to find a mention sigil ('@') but instead found nothing";
        assert_eq!(
            expected,
            ParseMentionError {
                kind: ParseMentionErrorType::Sigil {
                    expected: &["@"],
                    found: None
                },
                source: None,
            }
            .to_string(),
        );
        expected = "timestamp style value 'E' is invalid";
        assert_eq!(
            expected,
            ParseMentionError {
                kind: ParseMentionErrorType::TimestampStyleInvalid { found: "E" },
                source: None,
            }
            .to_string(),
        );

        expected = "expected to find a trailing arrow ('>') but instead found 'a'";
        assert_eq!(
            expected,
            ParseMentionError {
                kind: ParseMentionErrorType::TrailingArrow { found: Some('a') },
                source: None,
            }
            .to_string(),
        );

        expected = "expected to find a trailing arrow ('>') but instead found nothing";
        assert_eq!(
            expected,
            ParseMentionError {
                kind: ParseMentionErrorType::TrailingArrow { found: None },
                source: None,
            }
            .to_string(),
        );
    }
}
