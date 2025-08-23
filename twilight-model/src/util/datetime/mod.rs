//! Utilities for parsing and formatting ISO 8601 timestamps.
//!
//! # Examples
//!
//! Parse an acceptable ISO 8601 timestamp into a [`Timestamp`]:
//!
//! ```
//! # fn main() -> Result<(), Box<dyn std::error::Error>> {
//! use std::str::FromStr;
//! use twilight_model::util::Timestamp;
//!
//! let timestamp = Timestamp::from_str("2020-02-02T02:02:02.020000+00:00")?;
//!
//! // Check the Unix timestamp, which includes microseconds.
//! assert_eq!(1_580_608_922_020_000, timestamp.as_micros());
//! # Ok(()) }
//! ```
//!
//! Format a timestamp as an ISO 8601 string used by the Discord API:
//!
//! ```
//! # use std::error::Error;
//! # fn foo() -> Result<(), Box<dyn Error>> {
//! use twilight_model::util::Timestamp;
//!
//! let timestamp = Timestamp::from_secs(1_580_608_922)?;
//!
//! assert_eq!(
//!     "2020-02-02T02:02:02.000000+00:00",
//!     timestamp.iso_8601().to_string(),
//! );
//! # Ok(()) }
//! ```

#![warn(clippy::missing_docs_in_private_items)]

mod display;
mod error;

pub use self::{
    display::TimestampIso8601Display,
    error::{TimestampParseError, TimestampParseErrorType},
};

use serde::{
    de::{Deserialize, Deserializer, Error as DeError, Visitor},
    ser::{Serialize, Serializer},
};
use std::{
    fmt::{Formatter, Result as FmtResult},
    str::FromStr,
};
use time::{OffsetDateTime, PrimitiveDateTime, format_description::well_known::Rfc3339};

/// Number of microseconds in a second.
const MICROSECONDS_PER_SECOND: i64 = 1_000_000;

/// Number of nanoseconds in a microsecond.
const NANOSECONDS_PER_MICROSECOND: i64 = 1_000;

/// Representation of a Unix timestamp.
///
/// # Display
///
/// The timestamp does not itself implement [`core::fmt::Display`]. It could
/// have two possible display implementations: that of the Unix timestamp or
/// that of the timestamp in ISO 8601 format. Therefore, the preferred
/// implementation may be chosen by explicitly retrieving the Unix timestamp
/// with [seconds precision], with [microseconds precision], or
/// [retrieving an ISO 8601 formatter].
///
/// [retrieving an ISO 8601 formatter]: Self::iso_8601
/// [microseconds precision]: Self::as_micros
/// [seconds precision]: Self::as_secs
// We use a [`PrimitiveDateTime`] here since it does not store an offset, and
// the API only operates in UTC. Additionally, it is four bytes smaller than an
// [`OffsetDateTime`].
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct Timestamp(PrimitiveDateTime);

impl Timestamp {
    /// Create a timestamp from a Unix timestamp with microseconds precision.
    ///
    /// # Errors
    ///
    /// Returns a [`TimestampParseErrorType::Parsing`] error type if the parsing
    /// failed.
    ///
    /// [`TimestampParseErrorType::Parsing`]: self::error::TimestampParseErrorType::Parsing
    pub fn from_micros(unix_microseconds: i64) -> Result<Self, TimestampParseError> {
        let nanoseconds = i128::from(unix_microseconds) * i128::from(NANOSECONDS_PER_MICROSECOND);

        OffsetDateTime::from_unix_timestamp_nanos(nanoseconds)
            .map(|offset| Self(PrimitiveDateTime::new(offset.date(), offset.time())))
            .map_err(TimestampParseError::from_component_range)
    }

    /// Create a timestamp from a Unix timestamp with seconds precision.
    ///
    /// # Errors
    ///
    /// Returns a [`TimestampParseErrorType::Parsing`] error type if the parsing
    /// failed.
    ///
    /// [`TimestampParseErrorType::Parsing`]: self::error::TimestampParseErrorType::Parsing
    pub fn from_secs(unix_seconds: i64) -> Result<Self, TimestampParseError> {
        OffsetDateTime::from_unix_timestamp(unix_seconds)
            .map(|offset| Self(PrimitiveDateTime::new(offset.date(), offset.time())))
            .map_err(TimestampParseError::from_component_range)
    }

    /// Parse a timestamp from an ISO 8601 datetime string emitted by Discord.
    ///
    /// Discord emits two ISO 8601 valid formats of datetimes: with microseconds
    /// (2021-01-01T01:01:01.010000+00:00) and without microseconds
    /// (2021-01-01T01:01:01+00:00). This supports parsing from either.
    ///
    /// Supports parsing dates between the Discord epoch year (2010) and 2038.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::str::FromStr;
    /// use twilight_model::util::Timestamp;
    ///
    /// // Date and time in UTC with +00:00 offsets are supported:
    /// assert!(Timestamp::parse("2021-01-01T01:01:01.010000+00:00").is_ok());
    /// assert!(Timestamp::parse("2021-01-01T01:01:01+00:00").is_ok());
    ///
    /// // Other formats, such as dates, weeks, zero UTC offset designators, or
    /// // ordinal dates are not supported:
    /// assert!(Timestamp::parse("2021-08-10T18:19:59Z").is_err());
    /// assert!(Timestamp::parse("2021-01-01").is_err());
    /// assert!(Timestamp::parse("2021-W32-2").is_err());
    /// assert!(Timestamp::parse("2021-222").is_err());
    /// ```
    ///
    /// # Errors
    ///
    /// Returns a [`TimestampParseErrorType::Format`] error type if the provided
    /// string is too short to be an ISO 8601 datetime without a time offset.
    ///
    /// Returns a [`TimestampParseErrorType::Parsing`] error type if the parsing
    /// failed.
    ///
    /// [`TimestampParseErrorType::Format`]: self::error::TimestampParseErrorType::Format
    /// [`TimestampParseErrorType::Parsing`]: self::error::TimestampParseErrorType::Parsing
    pub fn parse(datetime: &str) -> Result<Self, TimestampParseError> {
        parse_iso8601(datetime).map(Self)
    }

    /// Total number of seconds within the timestamp.
    ///
    /// # Examples
    ///
    /// Parse a formatted timestamp and then get its Unix timestamp value with
    /// seconds precision:
    ///
    /// ```
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// use std::str::FromStr;
    /// use twilight_model::util::Timestamp;
    ///
    /// let timestamp = Timestamp::from_str("2021-08-10T11:16:37.020000+00:00")?;
    /// assert_eq!(1_628_594_197, timestamp.as_secs());
    /// # Ok(()) }
    /// ```
    pub const fn as_secs(self) -> i64 {
        self.0.assume_utc().unix_timestamp()
    }

    /// Total number of microseconds within the timestamp.
    ///
    /// # Examples
    ///
    /// Parse a formatted timestamp and then get its Unix timestamp value with
    /// microseconds precision:
    ///
    /// ```
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// use std::str::FromStr;
    /// use twilight_model::util::Timestamp;
    ///
    /// let timestamp = Timestamp::from_str("2021-08-10T11:16:37.123456+00:00")?;
    /// assert_eq!(1_628_594_197_123_456, timestamp.as_micros());
    /// # Ok(()) }
    /// ```
    pub const fn as_micros(self) -> i64 {
        let utc = self.0.assume_utc();

        (utc.unix_timestamp() * MICROSECONDS_PER_SECOND) + (utc.microsecond() as i64)
    }

    /// Create a Display implementation to format the timestamp as an ISO 8601
    /// datetime.
    pub const fn iso_8601(self) -> TimestampIso8601Display {
        TimestampIso8601Display::new(self)
    }
}

impl FromStr for Timestamp {
    type Err = TimestampParseError;

    /// Parse a timestamp from an ISO 8601 datetime string emitted by Discord.
    ///
    /// Discord emits two ISO 8601 valid formats of datetimes: with microseconds
    /// (2021-01-01T01:01:01.010000+00:00) and without microseconds
    /// (2021-01-01T01:01:01+00:00). This supports parsing from either.
    ///
    /// Supports parsing dates between the Discord epoch year (2010) and 2038.
    ///
    /// # Examples
    ///
    /// Refer to the documentation for [`Timestamp::parse`] for more examples.
    ///
    /// ```
    /// use std::str::FromStr;
    /// use twilight_model::util::Timestamp;
    ///
    /// assert!(Timestamp::from_str("2021-01-01T01:01:01.010000+00:00").is_ok());
    /// assert!(Timestamp::from_str("2021-01-01T01:01:01+00:00").is_ok());
    /// ```
    ///
    /// # Errors
    ///
    /// Refer to the documentation for [`Timestamp::parse`] for a list of
    /// errors.
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Timestamp::parse(s)
    }
}

impl<'de> Deserialize<'de> for Timestamp {
    /// Parse a timestamp from an ISO 8601 datetime string emitted by Discord.
    ///
    /// Discord emits two ISO 8601 valid formats of datetimes: with microseconds
    /// (2021-01-01T01:01:01.010000+00:00) and without microseconds
    /// (2021-01-01T01:01:01+00:00). This supports parsing from either.
    ///
    /// # Errors
    ///
    /// Refer to the documentation for [`Timestamp::parse`] for a list of
    /// errors.
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        /// Visitor for the [`Timestamp`] deserialize implementation.
        struct TimestampVisitor;

        impl Visitor<'_> for TimestampVisitor {
            type Value = Timestamp;

            fn expecting(&self, f: &mut Formatter<'_>) -> FmtResult {
                f.write_str("iso 8601 datetime format")
            }

            fn visit_str<E: DeError>(self, v: &str) -> Result<Self::Value, E> {
                Timestamp::parse(v).map_err(DeError::custom)
            }
        }

        deserializer.deserialize_any(TimestampVisitor)
    }
}

impl Serialize for Timestamp {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.collect_str(&self.iso_8601())
    }
}

impl TryFrom<&'_ str> for Timestamp {
    type Error = TimestampParseError;

    /// Parse a timestamp from an ISO 8601 datetime string emitted by Discord.
    ///
    /// Discord emits two ISO 8601 valid formats of datetimes: with microseconds
    /// (2021-01-01T01:01:01.010000+00:00) and without microseconds
    /// (2021-01-01T01:01:01+00:00). This supports parsing from either.
    ///
    /// # Examples
    ///
    /// Refer to the documentation for [`Timestamp::parse`] for examples.
    ///
    /// # Errors
    ///
    /// Refer to the documentation for [`Timestamp::parse`] for a list of
    /// errors.
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::from_str(value)
    }
}

/// Parse an input ISO 8601 timestamp into a Unix timestamp with microseconds.
///
/// Input in the format of "2021-01-01T01:01:01.010000+00:00" is acceptable.
///
/// # Errors
///
/// Returns a [`TimestampParseErrorType::Parsing`] if the parsing failed.
fn parse_iso8601(input: &str) -> Result<PrimitiveDateTime, TimestampParseError> {
    /// Discord sends some timestamps with the microseconds and some without.
    const TIMESTAMP_LENGTH: usize = "2021-01-01T01:01:01+00:00".len();

    if input.len() < TIMESTAMP_LENGTH {
        return Err(TimestampParseError::FORMAT);
    }

    OffsetDateTime::parse(input, &Rfc3339)
        .map(|offset| PrimitiveDateTime::new(offset.date(), offset.time()))
        .map_err(TimestampParseError::from_parse)
}

#[cfg(test)]
mod tests {
    use super::{Timestamp, TimestampParseError};
    use serde::{Deserialize, Serialize};
    use static_assertions::assert_impl_all;
    use std::{fmt::Debug, hash::Hash, str::FromStr};
    use time::{OffsetDateTime, PrimitiveDateTime};

    assert_impl_all!(
        Timestamp: Clone,
        Copy,
        Debug,
        Deserialize<'static>,
        Eq,
        FromStr,
        Hash,
        PartialEq,
        Send,
        Serialize,
        Sync,
        TryFrom<&'static str>,
    );

    /// Test a variety of supported ISO 8601 datetime formats.
    #[test]
    fn parse_iso8601() -> Result<(), TimestampParseError> {
        // With milliseconds.
        let offset = OffsetDateTime::from_unix_timestamp_nanos(1_580_608_922_020_000_000).unwrap();

        assert_eq!(
            PrimitiveDateTime::new(offset.date(), offset.time()),
            super::parse_iso8601("2020-02-02T02:02:02.020000+00:00")?
        );

        // Without milliseconds.
        let offset = OffsetDateTime::from_unix_timestamp_nanos(1_580_608_922_000_000_000).unwrap();

        assert_eq!(
            PrimitiveDateTime::new(offset.date(), offset.time()),
            super::parse_iso8601("2020-02-02T02:02:02+00:00")?
        );

        // And a couple not in leap years.
        assert_eq!(
            "2021-03-16T14:29:19.046000+00:00",
            Timestamp::from_str("2021-03-16T14:29:19.046000+00:00")?
                .iso_8601()
                .to_string(),
        );
        assert_eq!(
            "2022-03-16T14:29:19.046000+00:00",
            Timestamp::from_str("2022-03-16T14:29:19.046000+00:00")?
                .iso_8601()
                .to_string(),
        );
        assert_eq!(
            "2023-03-16T14:29:19.046000+00:00",
            Timestamp::from_str("2023-03-16T14:29:19.046000+00:00")?
                .iso_8601()
                .to_string(),
        );

        Ok(())
    }

    /// Test the boundaries of valid ISO 8601 datetime boundaries.
    #[test]
    fn parse_iso8601_boundaries() -> Result<(), TimestampParseError> {
        fn test(input: &str) -> Result<(), TimestampParseError> {
            assert_eq!(input, Timestamp::from_str(input)?.iso_8601().to_string());

            Ok(())
        }

        test("2021-12-31T23:59:59.999999+00:00")?;
        test("2021-01-01T00:00:00.000000+00:00")?;

        Ok(())
    }
}
