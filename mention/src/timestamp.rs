//! Timestamps with the ability to be formatted in clients based on the client's
//! local timezone and locale.
//!
//! Included is the [`TimestampFlag`] denoting how to format a timestamp and the
//! [`Timestamp`] itself, containing an optional flag and a Unix timestamp.
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
//! [`TimestampFlag`] implements [`Display`], which allows you to easily print
//! the display modifier of a flag:
//!
//! ```
//! use twilight_mention::timestamp::TimestampFlag;
//!
//! println!("The modifier is '{}'", TimestampFlag::RelativeTime);
//! ```
//!
//! [`Display`]: core::fmt::Display
//! [`Mention`]: super::fmt::Mention

use std::{
    cmp::Ordering,
    convert::TryFrom,
    error::Error,
    fmt::{Display, Formatter, Result as FmtResult},
};

/// Converting a [`TimestampFlag`] from a string slice failed.
#[derive(Debug)]
pub struct TimestampFlagConversionError {
    kind: TimestampFlagConversionErrorType,
}

impl TimestampFlagConversionError {
    /// Immutable reference to the type of error that occurred.
    #[must_use = "retrieving the type has no effect if left unused"]
    pub const fn kind(&self) -> &TimestampFlagConversionErrorType {
        &self.kind
    }

    /// Consume the error, returning the owned error type and the source error.
    #[must_use = "consuming the error into its parts has no effect if left unused"]
    pub fn into_parts(
        self,
    ) -> (
        TimestampFlagConversionErrorType,
        Option<Box<dyn Error + Send + Sync>>,
    ) {
        (self.kind, None)
    }
}

impl Display for TimestampFlagConversionError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match &self.kind {
            TimestampFlagConversionErrorType::FlagInvalid => {
                f.write_str("given value is not a valid flag")
            }
        }
    }
}

impl Error for TimestampFlagConversionError {}

/// Type of [`TimestampFlagConversionError`] that occurred.
#[derive(Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum TimestampFlagConversionErrorType {
    /// Given value is not a valid flag.
    FlagInvalid,
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
    /// Display modifier flag.
    ///
    /// When a flag is not specified then [`TimestampFlag::ShortDateTime`] is
    /// the default; however, we do not implement `Default` for
    /// [`TimestampFlag`] because this is a third party implementation detail.
    flag: Option<TimestampFlag>,
    /// Unix timestamp in seconds.
    unix: u64,
}

impl Timestamp {
    /// Create a new timestamp with a Unix timestamp and optionally a flag.
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
    pub const fn new(unix: u64, flag: Option<TimestampFlag>) -> Self {
        Self { flag, unix }
    }

    /// Flag representing the display modifier.
    ///
    /// ```
    /// use twilight_mention::timestamp::{TimestampFlag, Timestamp};
    ///
    /// // When leaving a flag unspecified a default is not provided.
    /// assert!(Timestamp::new(1624044388, None).flag().is_none());
    ///
    /// // The same flag is returned when a flag is specified.
    /// let timestamp = Timestamp::new(
    ///     1_624_044_388,
    ///     Some(TimestampFlag::ShortDateTime),
    /// );
    /// assert_eq!(Some(TimestampFlag::ShortDateTime), timestamp.flag());
    /// ```
    #[must_use = "retrieving the flag does nothing on its own"]
    pub const fn flag(&self) -> Option<TimestampFlag> {
        self.flag
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

/// Flag modifier denoting how to display a timestamp.
///
/// The default variant is [`ShortDateTime`].
///
/// [`ShortDateTime`]: Self::ShortDateTime
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum TimestampFlag {
    /// Flag modifier to display a timestamp as a long date/time.
    ///
    /// Correlates to the flag `F`.
    ///
    /// Causes mentions to display in clients as `Tuesday, 1 April 2021 01:20`.
    LongDateTime,
    /// Flag modifier to display a timestamp as a long date.
    ///
    /// Correlates to the flag `D`.
    ///
    /// Causes mentions to display in clients as `1 April 2021`.
    LongDate,
    /// Flag modifier to display a timestamp as a long date/time.
    ///
    /// Correlates to the flag `T`.
    ///
    /// Causes mentions to display in clients as `01:20:30`.
    LongTime,
    /// Flag modifier to display a timestamp as a relative timestamp.
    ///
    /// Correlates to the flag `R`.
    ///
    /// Causes mentions to display in clients as `2 months ago`.
    RelativeTime,
    /// Flag modifier to display a timestamp as a short date/time.
    ///
    /// Correlates to the flag `f`.
    ///
    /// Causes mentions to display in clients as `1 April 2021 01:20`.
    ///
    /// This is the default flag when left unspecified.
    ShortDateTime,
    /// Flag modifier to display a timestamp as a short date/time.
    ///
    /// Correlates to the flag `d`.
    ///
    /// Causes mentions to display in clients as `1/4/2021`.
    ShortDate,
    /// Flag modifier to display a timestamp as a short date/time.
    ///
    /// Correlates to the flag `t`.
    ///
    /// Causes mentions to display in clients as `01:20`.
    ShortTime,
}

impl TimestampFlag {
    /// Retrieve the display character of a flag.
    ///
    /// # Examples
    ///
    /// ```
    /// use twilight_mention::timestamp::TimestampFlag;
    ///
    /// assert_eq!("F", TimestampFlag::LongDateTime.flag());
    /// assert_eq!("R", TimestampFlag::RelativeTime.flag());
    /// ```
    #[must_use = "retrieving the character of a flag does nothing on its own"]
    pub const fn flag(self) -> &'static str {
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

impl Display for TimestampFlag {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        // Fastest way to convert a `char` to a `&str`.
        f.write_str(self.flag())
    }
}

impl TryFrom<&str> for TimestampFlag {
    type Error = TimestampFlagConversionError;

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
                return Err(TimestampFlagConversionError {
                    kind: TimestampFlagConversionErrorType::FlagInvalid,
                })
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use super::{
        Timestamp, TimestampFlag, TimestampFlagConversionError, TimestampFlagConversionErrorType,
    };
    use static_assertions::assert_impl_all;
    use std::{convert::TryFrom, error::Error, fmt::Debug, hash::Hash};

    assert_impl_all!(TimestampFlagConversionErrorType: Debug, Send, Sync);
    assert_impl_all!(TimestampFlagConversionError: Debug, Error, Send, Sync);
    assert_impl_all!(
        TimestampFlag: Clone,
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

    const TIMESTAMP_OLD_FLAGGED: Timestamp = Timestamp::new(1, Some(TimestampFlag::RelativeTime));
    const TIMESTAMP_OLD: Timestamp = Timestamp::new(1, None);
    const TIMESTAMP_NEW_FLAGGED: Timestamp = Timestamp::new(2, Some(TimestampFlag::ShortDate));
    const TIMESTAMP_NEW: Timestamp = Timestamp::new(2, None);

    /// Test the corresponding flag modifiers.
    #[test]
    fn test_timestamp_flags_modifiers() {
        assert_eq!("F", TimestampFlag::LongDateTime.flag());
        assert_eq!("D", TimestampFlag::LongDate.flag());
        assert_eq!("T", TimestampFlag::LongTime.flag());
        assert_eq!("R", TimestampFlag::RelativeTime.flag());
        assert_eq!("f", TimestampFlag::ShortDateTime.flag());
        assert_eq!("d", TimestampFlag::ShortDate.flag());
        assert_eq!("t", TimestampFlag::ShortTime.flag());
    }

    /// Test that flag modifiers correctly parse from their string slice variants.
    #[test]
    fn test_timestamp_flag_try_from() -> Result<(), TimestampFlagConversionError> {
        assert_eq!(TimestampFlag::try_from("F")?, TimestampFlag::LongDateTime);
        assert_eq!(TimestampFlag::try_from("D")?, TimestampFlag::LongDate);
        assert_eq!(TimestampFlag::try_from("T")?, TimestampFlag::LongTime);
        assert_eq!(TimestampFlag::try_from("R")?, TimestampFlag::RelativeTime);
        assert_eq!(TimestampFlag::try_from("f")?, TimestampFlag::ShortDateTime);
        assert_eq!(TimestampFlag::try_from("d")?, TimestampFlag::ShortDate);
        assert_eq!(TimestampFlag::try_from("t")?, TimestampFlag::ShortTime);

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
        assert!(Timestamp::new(2, None).cmp(&TIMESTAMP_NEW).is_eq());

        // Assert that a lower timestamp is less than than a greater timestamp.
        assert!(TIMESTAMP_OLD < TIMESTAMP_NEW);
    }

    /// Test that whether a timestamp has a flag incurs no effect on results.
    #[test]
    fn test_timestamp_cmp_flags() {
        // Assert that a higher timestamp is greater than a lesser timestamp
        // regardless of flag combinations.
        assert!(TIMESTAMP_NEW_FLAGGED > TIMESTAMP_OLD);
        assert!(TIMESTAMP_NEW > TIMESTAMP_OLD_FLAGGED);
        assert!(TIMESTAMP_NEW_FLAGGED > TIMESTAMP_OLD_FLAGGED);

        // Assert that two timestamps with the same unix timestamp are equal
        // regardless of flag combinations.
        //
        // We make new timestamps here to around Clippy's `eq_op` lint.
        assert!(TIMESTAMP_NEW_FLAGGED.cmp(&TIMESTAMP_NEW).is_eq());
        assert!(Timestamp::new(2, Some(TimestampFlag::RelativeTime))
            .cmp(&TIMESTAMP_NEW_FLAGGED)
            .is_eq());

        // Assert that a lower timestamp is less than than a greater timestamp
        // regardless of flag.
        assert!(TIMESTAMP_OLD_FLAGGED < TIMESTAMP_NEW);
        assert!(TIMESTAMP_OLD < TIMESTAMP_NEW_FLAGGED);
        assert!(TIMESTAMP_OLD_FLAGGED < TIMESTAMP_NEW_FLAGGED);
    }
}
