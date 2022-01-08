//! Timestamps with the ability to be formatted in clients based on the client's
//! local timezone and locale.
//!
//! Included is the [`TimestampStyle`] denoting how to format a timestamp and
//! the [`Timestamp`] itself, containing an optional style and a Unix timestamp.
//!
//! # Examples
//!
//! Format a [`Timestamp`] into a valid Discord Markdown markdown via its
//! implementation of [`Mention`]:
//!
//! ```
//! use twilight_mention::{timestamp::Timestamp, Mention};
//!
//! let timestamp = Timestamp::new(1624047064, None);
//!
//! println!("This action was performed at {}", timestamp.mention());
//! ```
//!
//! [`TimestampStyle`] implements [`Display`], which allows you to easily print
//! the display modifier of a style:
//!
//! ```
//! use twilight_mention::timestamp::TimestampStyle;
//!
//! println!("The modifier is '{}'", TimestampStyle::RelativeTime);
//! ```
//!
//! [`Display`]: core::fmt::Display
//! [`Mention`]: super::fmt::Mention

use std::{
    cmp::Ordering,
    error::Error,
    fmt::{Display, Formatter, Result as FmtResult},
};

/// Converting a [`TimestampStyle`] from a string slice failed.
#[derive(Debug)]
pub struct TimestampStyleConversionError {
    kind: TimestampStyleConversionErrorType,
}

impl TimestampStyleConversionError {
    /// Immutable reference to the type of error that occurred.
    #[must_use = "retrieving the type has no effect if left unused"]
    pub const fn kind(&self) -> &TimestampStyleConversionErrorType {
        &self.kind
    }

    /// Consume the error, returning the owned error type and the source error.
    #[must_use = "consuming the error into its parts has no effect if left unused"]
    pub fn into_parts(
        self,
    ) -> (
        TimestampStyleConversionErrorType,
        Option<Box<dyn Error + Send + Sync>>,
    ) {
        (self.kind, None)
    }
}

impl Display for TimestampStyleConversionError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match &self.kind {
            TimestampStyleConversionErrorType::StyleInvalid => {
                f.write_str("given value is not a valid style")
            }
        }
    }
}

impl Error for TimestampStyleConversionError {}

/// Type of [`TimestampStyleConversionError`] that occurred.
#[derive(Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum TimestampStyleConversionErrorType {
    /// Given value is not a valid style.
    StyleInvalid,
}

/// Timestamp representing a time to be formatted based on a client's current
/// local timezone and locale.
///
/// Timestamps can be compared based on their [`unix`] value.
///
/// # Examples
///
/// Compare two timestamps to determine which is more recent:
///
/// ```
/// use twilight_mention::timestamp::Timestamp;
///
/// let old = Timestamp::new(1_500_000_000, None);
/// let new = Timestamp::new(1_600_000_000, None);
///
/// assert!(new > old);
/// ```
///
/// [`unix`]: Self::unix
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct Timestamp {
    /// Display modifier style.
    ///
    /// When a style is not specified then [`TimestampStyle::ShortDateTime`] is
    /// the default; however, we do not implement `Default` for
    /// [`TimestampStyle`] because this is a third party implementation detail.
    style: Option<TimestampStyle>,
    /// Unix timestamp in seconds.
    unix: u64,
}

impl Timestamp {
    /// Create a new timestamp with a Unix timestamp and optionally a style.
    ///
    /// The Unix timestamp is in seconds.
    ///
    /// # Examples
    ///
    /// Create a timestamp without a display modifier and format it as a mention:
    ///
    /// ```
    /// use twilight_mention::{timestamp::Timestamp, Mention};
    ///
    /// let timestamp = Timestamp::new(1624044388, None);
    /// assert_eq!("<t:1624044388>", timestamp.mention().to_string());
    /// ```
    #[must_use = "creating a timestamp does nothing on its own"]
    pub const fn new(unix: u64, style: Option<TimestampStyle>) -> Self {
        Self { style, unix }
    }

    /// Style representing the display modifier.
    ///
    /// ```
    /// use twilight_mention::timestamp::{TimestampStyle, Timestamp};
    ///
    /// // When leaving a style unspecified a default is not provided.
    /// assert!(Timestamp::new(1624044388, None).style().is_none());
    ///
    /// // The same style is returned when a style is specified.
    /// let timestamp = Timestamp::new(
    ///     1_624_044_388,
    ///     Some(TimestampStyle::ShortDateTime),
    /// );
    /// assert_eq!(Some(TimestampStyle::ShortDateTime), timestamp.style());
    /// ```
    #[must_use = "retrieving the style does nothing on its own"]
    pub const fn style(&self) -> Option<TimestampStyle> {
        self.style
    }

    /// Unix timestamp.
    #[must_use = "retrieving the unix timestamp does nothing on its own"]
    pub const fn unix(&self) -> u64 {
        self.unix
    }
}

impl Ord for Timestamp {
    fn cmp(&self, other: &Timestamp) -> Ordering {
        self.unix.cmp(&other.unix)
    }
}

impl PartialOrd for Timestamp {
    fn partial_cmp(&self, other: &Timestamp) -> Option<Ordering> {
        self.unix.partial_cmp(&other.unix)
    }
}

/// Style modifier denoting how to display a timestamp.
///
/// The default variant is [`ShortDateTime`].
///
/// [`ShortDateTime`]: Self::ShortDateTime
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum TimestampStyle {
    /// Style modifier to display a timestamp as a long date/time.
    ///
    /// Correlates to the style `F`.
    ///
    /// Causes mentions to display in clients as `Tuesday, 1 April 2021 01:20`.
    LongDateTime,
    /// Style modifier to display a timestamp as a long date.
    ///
    /// Correlates to the style `D`.
    ///
    /// Causes mentions to display in clients as `1 April 2021`.
    LongDate,
    /// Style modifier to display a timestamp as a long date/time.
    ///
    /// Correlates to the style `T`.
    ///
    /// Causes mentions to display in clients as `01:20:30`.
    LongTime,
    /// Style modifier to display a timestamp as a relative timestamp.
    ///
    /// Correlates to the style `R`.
    ///
    /// Causes mentions to display in clients as `2 months ago`.
    RelativeTime,
    /// Style modifier to display a timestamp as a short date/time.
    ///
    /// Correlates to the style `f`.
    ///
    /// Causes mentions to display in clients as `1 April 2021 01:20`.
    ///
    /// This is the default style when left unspecified.
    ShortDateTime,
    /// Style modifier to display a timestamp as a short date/time.
    ///
    /// Correlates to the style `d`.
    ///
    /// Causes mentions to display in clients as `1/4/2021`.
    ShortDate,
    /// Style modifier to display a timestamp as a short date/time.
    ///
    /// Correlates to the style `t`.
    ///
    /// Causes mentions to display in clients as `01:20`.
    ShortTime,
}

impl TimestampStyle {
    /// Retrieve the display character of a style.
    ///
    /// # Examples
    ///
    /// ```
    /// use twilight_mention::timestamp::TimestampStyle;
    ///
    /// assert_eq!("F", TimestampStyle::LongDateTime.style());
    /// assert_eq!("R", TimestampStyle::RelativeTime.style());
    /// ```
    #[must_use = "retrieving the character of a style does nothing on its own"]
    pub const fn style(self) -> &'static str {
        match self {
            Self::LongDateTime => "F",
            Self::LongDate => "D",
            Self::LongTime => "T",
            Self::RelativeTime => "R",
            Self::ShortDateTime => "f",
            Self::ShortDate => "d",
            Self::ShortTime => "t",
        }
    }
}

impl Display for TimestampStyle {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        // Fastest way to convert a `char` to a `&str`.
        f.write_str(self.style())
    }
}

impl TryFrom<&str> for TimestampStyle {
    type Error = TimestampStyleConversionError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Ok(match value {
            "F" => Self::LongDateTime,
            "D" => Self::LongDate,
            "T" => Self::LongTime,
            "R" => Self::RelativeTime,
            "f" => Self::ShortDateTime,
            "d" => Self::ShortDate,
            "t" => Self::ShortTime,
            _ => {
                return Err(TimestampStyleConversionError {
                    kind: TimestampStyleConversionErrorType::StyleInvalid,
                })
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use super::{
        Timestamp, TimestampStyle, TimestampStyleConversionError, TimestampStyleConversionErrorType,
    };
    use static_assertions::assert_impl_all;
    use std::{cmp::Ordering, error::Error, fmt::Debug, hash::Hash};

    assert_impl_all!(TimestampStyleConversionErrorType: Debug, Send, Sync);
    assert_impl_all!(TimestampStyleConversionError: Debug, Error, Send, Sync);
    assert_impl_all!(
        TimestampStyle: Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        PartialEq,
        Send,
        Sync
    );
    assert_impl_all!(
        Timestamp: Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        PartialEq,
        Send,
        Sync
    );

    const TIMESTAMP_OLD_STYLED: Timestamp = Timestamp::new(1, Some(TimestampStyle::RelativeTime));
    const TIMESTAMP_OLD: Timestamp = Timestamp::new(1, None);
    const TIMESTAMP_NEW_STYLED: Timestamp = Timestamp::new(2, Some(TimestampStyle::ShortDate));
    const TIMESTAMP_NEW: Timestamp = Timestamp::new(2, None);

    /// Test the corresponding style modifiers.
    #[test]
    fn test_timestamp_style_modifiers() {
        assert_eq!("F", TimestampStyle::LongDateTime.style());
        assert_eq!("D", TimestampStyle::LongDate.style());
        assert_eq!("T", TimestampStyle::LongTime.style());
        assert_eq!("R", TimestampStyle::RelativeTime.style());
        assert_eq!("f", TimestampStyle::ShortDateTime.style());
        assert_eq!("d", TimestampStyle::ShortDate.style());
        assert_eq!("t", TimestampStyle::ShortTime.style());
    }

    /// Test that style modifiers correctly parse from their string slice variants.
    #[test]
    fn test_timestamp_style_try_from() -> Result<(), TimestampStyleConversionError> {
        assert_eq!(TimestampStyle::try_from("F")?, TimestampStyle::LongDateTime);
        assert_eq!(TimestampStyle::try_from("D")?, TimestampStyle::LongDate);
        assert_eq!(TimestampStyle::try_from("T")?, TimestampStyle::LongTime);
        assert_eq!(TimestampStyle::try_from("R")?, TimestampStyle::RelativeTime);
        assert_eq!(
            TimestampStyle::try_from("f")?,
            TimestampStyle::ShortDateTime
        );
        assert_eq!(TimestampStyle::try_from("d")?, TimestampStyle::ShortDate);
        assert_eq!(TimestampStyle::try_from("t")?, TimestampStyle::ShortTime);

        Ok(())
    }

    /// Test that timestamps are correctly compared based on their inner unix
    /// timestamp value.
    #[test]
    fn test_timestamp_cmp() {
        // Assert that a higher timestamp is greater than a lesser timestamp.
        assert!(TIMESTAMP_NEW > TIMESTAMP_OLD);

        // Assert that two timestamps with the same unix timestamp are equal.
        //
        // We make a new timestamp here to around Clippy's `eq_op` lint.
        assert!(Timestamp::new(2, None).cmp(&TIMESTAMP_NEW) == Ordering::Equal);

        // Assert that a lower timestamp is less than than a greater timestamp.
        assert!(TIMESTAMP_OLD < TIMESTAMP_NEW);
    }

    /// Test that whether a timestamp has a style incurs no effect on results.
    #[test]
    fn test_timestamp_cmp_styles() {
        // Assert that a higher timestamp is greater than a lesser timestamp
        // regardless of style combinations.
        assert!(TIMESTAMP_NEW_STYLED > TIMESTAMP_OLD);
        assert!(TIMESTAMP_NEW > TIMESTAMP_OLD_STYLED);
        assert!(TIMESTAMP_NEW_STYLED > TIMESTAMP_OLD_STYLED);

        // Assert that two timestamps with the same unix timestamp are equal
        // regardless of style combinations.
        //
        // We make new timestamps here to around Clippy's `eq_op` lint.
        assert!(TIMESTAMP_NEW_STYLED.cmp(&TIMESTAMP_NEW) == Ordering::Equal);
        assert!(
            Timestamp::new(2, Some(TimestampStyle::RelativeTime)).cmp(&TIMESTAMP_NEW_STYLED)
                == Ordering::Equal
        );

        // Assert that a lower timestamp is less than than a greater timestamp
        // regardless of style.
        assert!(TIMESTAMP_OLD_STYLED < TIMESTAMP_NEW);
        assert!(TIMESTAMP_OLD < TIMESTAMP_NEW_STYLED);
        assert!(TIMESTAMP_OLD_STYLED < TIMESTAMP_NEW_STYLED);
    }
}
