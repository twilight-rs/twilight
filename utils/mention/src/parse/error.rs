use std::{
    error::Error,
    fmt::{Display, Formatter, Result as FmtResult},
    num::ParseIntError,
};

/// Parsing a mention failed due to invalid syntax.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ParseMentionError<'a> {
    /// ID portion of the mention isn't a u64.
    IdNotU64 {
        /// String that could not be parsed into a u64.
        found: &'a str,
        /// Reason for the error.
        source: ParseIntError,
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
        /// Sigil that is expected for the mention type.
        expected: &'a str,
        /// Character that was instead found where the sigil should be.
        found: Option<char>,
    },
    /// Trailing arrow (`>`) is not present.
    TrailingArrow {
        /// Character that was instead found where the trailing arrow should be.
        found: Option<char>,
    },
}

impl Display for ParseMentionError<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            Self::IdNotU64 { found, .. } => f.write_fmt(format_args!(
                "id portion ('{}') of mention is not a u64",
                found,
            )),
            Self::LeadingArrow { found } => {
                f.write_str("expected to find a leading arrow ('<') but instead ")?;

                if let Some(c) = found {
                    f.write_fmt(format_args!("found '{}'", c))
                } else {
                    f.write_str("found nothing")
                }
            }
            Self::PartMissing { expected, found } => f.write_fmt(format_args!(
                "
                    expected {} parts but only found {}",
                expected, found,
            )),
            Self::Sigil { expected, found } => {
                f.write_fmt(format_args!(
                    "expected to find a mention sigil ('{}') but instead ",
                    expected,
                ))?;
                f.write_str("found ")?;

                if let Some(c) = found {
                    f.write_fmt(format_args!("'{}'", c))
                } else {
                    f.write_str("nothing")
                }
            }
            Self::TrailingArrow { found } => {
                f.write_str("expected to find a trailing arrow ('>') but instead ")?;

                if let Some(c) = found {
                    f.write_fmt(format_args!("found '{}'", c))
                } else {
                    f.write_str("found nothing")
                }
            }
        }
    }
}

impl Error for ParseMentionError<'_> {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::IdNotU64 { source, .. } => Some(source),
            Self::LeadingArrow { .. }
            | Self::PartMissing { .. }
            | Self::Sigil { .. }
            | Self::TrailingArrow { .. } => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::ParseMentionError;
    use static_assertions::{assert_fields, assert_impl_all};
    use std::{error::Error, fmt::Debug};

    assert_fields!(ParseMentionError::IdNotU64: found, source);
    assert_fields!(ParseMentionError::LeadingArrow: found);
    assert_fields!(ParseMentionError::Sigil: expected, found);
    assert_fields!(ParseMentionError::TrailingArrow: found);
    assert_impl_all!(ParseMentionError<'static>: Clone, Debug, Error, Eq, PartialEq, Send, Sync);

    #[test]
    fn test_display() {
        let mut expected = "id portion ('abcd') of mention is not a u64";
        assert_eq!(
            expected,
            ParseMentionError::IdNotU64 {
                found: "abcd",
                source: "abcd".parse::<u64>().unwrap_err(),
            }
            .to_string(),
        );
        expected = "expected to find a leading arrow ('<') but instead found 'a'";
        assert_eq!(
            expected,
            ParseMentionError::LeadingArrow { found: Some('a') }.to_string(),
        );

        expected = "expected to find a leading arrow ('<') but instead found nothing";
        assert_eq!(
            expected,
            ParseMentionError::LeadingArrow { found: None }.to_string(),
        );

        expected = "expected to find a mention sigil ('@') but instead found '#'";
        assert_eq!(
            expected,
            ParseMentionError::Sigil {
                expected: "@",
                found: Some('#')
            }
            .to_string(),
        );

        expected = "expected to find a mention sigil ('@') but instead found nothing";
        assert_eq!(
            expected,
            ParseMentionError::Sigil {
                expected: "@",
                found: None
            }
            .to_string(),
        );

        expected = "expected to find a trailing arrow ('>') but instead found 'a'";
        assert_eq!(
            expected,
            ParseMentionError::TrailingArrow { found: Some('a') }.to_string(),
        );

        expected = "expected to find a trailing arrow ('>') but instead found nothing";
        assert_eq!(
            expected,
            ParseMentionError::TrailingArrow { found: None }.to_string(),
        );
    }
}
