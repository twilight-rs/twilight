//! Utilities for parsing and format ISO 8601 timestamps returned by the API.
//!
//! # Examples
//!
//! Parse an acceptable ISO 8601 timestamp into a [`Timestamp`]:
//!
//! ```
//! # fn main() -> Result<(), Box<dyn std::error::Error>> {
//! use std::str::FromStr;
//! use twilight_model::datetime::Timestamp;
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
//! # fn foo() -> Option<()> {
//! use twilight_model::datetime::Timestamp;
//!
//! let timestamp = Timestamp::from_secs(1_580_608_922)?;
//!
//! assert_eq!(
//!     "2020-02-02T02:02:02.000000+00:00",
//!     timestamp.iso_8601().to_string(),
//! );
//! # None }
//! ```

#![deny(clippy::missing_docs_in_private_items)]

pub mod error;

mod constant;
mod display;

use crate::datetime::constant::{MINUTES_PER_HOUR, SECONDS_PER_MINUTE};

pub use self::{display::TimestampIso8601Display, error::TimestampParseError};

use self::constant::{
    Month, DAYS_PER_YEAR, EPOCH_UNIX, MICROSECONDS_PER_SECOND, SECONDS_PER_DAY, SECONDS_PER_HOUR,
    YEAR_MAX, YEAR_MIN,
};
use serde::{
    de::{Deserialize, Deserializer, Error as DeError, Visitor},
    ser::{Serialize, Serializer},
};
use std::{
    convert::TryFrom,
    fmt::{Formatter, Result as FmtResult},
    num::NonZeroU64,
    str::FromStr,
};

/// Byte value of the number 0.
const ZERO_BYTE: u8 = b'0';

/// Representation of a Unix timestamp with milliseconds.
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
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct Timestamp(NonZeroU64);

impl Timestamp {
    /// Create a non-zero timestamp from a Unix timestamp with microseconds
    /// precision if the value is not zero.
    pub const fn from_micros(unix_microseconds: u64) -> Option<Self> {
        // `const`: Clippy suggests using `Option::map_or` which isn't const.
        #[allow(clippy::option_if_let_else)]
        if let Some(value) = NonZeroU64::new(unix_microseconds) {
            Some(Self(value))
        } else {
            None
        }
    }

    /// Create a non-zero timestamp from a Unix timestamp with seconds precision
    /// if the value is not zero.
    pub const fn from_secs(unix_seconds: u64) -> Option<Self> {
        let micros = unix_seconds * MICROSECONDS_PER_SECOND;

        // `const`: Clippy suggests using `Option::map_or` which isn't const.
        #[allow(clippy::option_if_let_else)]
        if let Some(value) = NonZeroU64::new(micros) {
            Some(Self(value))
        } else {
            None
        }
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
    /// use twilight_model::datetime::Timestamp;
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
    /// Returns a [`TimestampParseErrorType::Range`] error type if one of the
    /// provided datetime segment values is not within an acceptable range.
    ///
    /// [`TimestampParseErrorType::Format`]: error::TimestampParseErrorType::Format
    /// [`TimestampParseErrorType::Range`]: error::TimestampParseErrorType::Range
    pub const fn parse(datetime: &str) -> Result<Self, TimestampParseError> {
        let micros = match parse_iso8601(datetime) {
            Ok(micros) => micros,
            Err(source) => return Err(source),
        };

        // `const`: Clippy suggests using `Option::map_or` which isn't const.
        #[allow(clippy::option_if_let_else)]
        if let Some(timestamp) = Timestamp::from_micros(micros) {
            Ok(timestamp)
        } else {
            Err(TimestampParseError::ZERO)
        }
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
    /// use twilight_model::datetime::Timestamp;
    ///
    /// let timestamp = Timestamp::from_str("2021-08-10T11:16:37.020000+00:00")?;
    /// assert_eq!(1_628_594_197, timestamp.as_secs());
    /// # Ok(()) }
    /// ```
    pub const fn as_secs(self) -> u64 {
        self.0.get() / MICROSECONDS_PER_SECOND
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
    /// use twilight_model::datetime::Timestamp;
    ///
    /// let timestamp = Timestamp::from_str("2021-08-10T11:16:37.123456+00:00")?;
    /// assert_eq!(1_628_594_197_123_456, timestamp.as_micros());
    /// # Ok(()) }
    /// ```
    pub const fn as_micros(self) -> u64 {
        self.0.get()
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
    /// use twilight_model::datetime::Timestamp;
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
/// Returns a [`TimestampParseErrorType::Format`] error type if the provided string
/// is too short to be an ISO 8601 datetime without a time offset.
///
/// Returns a [`TimestampParseErrorType::Range`] error type if one of the provided
/// datetime segment values is not within an acceptable range.
///
/// [`TimestampParseErrorType::Format`]: error::TimestampParseErrorType::Format
/// [`TimestampParseErrorType::Range`]: error::TimestampParseErrorType::Range
const fn parse_iso8601(input: &str) -> Result<u64, TimestampParseError> {
    /// Discord sends some timestamps with the microseconds and some without.
    const TIMESTAMP_LENGTH: usize = "2021-01-01T01:01:01+00:00".len();

    if input.len() < TIMESTAMP_LENGTH {
        return Err(TimestampParseError::FORMAT);
    }

    let bytes = input.as_bytes();

    // Years.
    //
    // Input: 2021-01-01T01:01:01.010000+00:00
    //       |----|
    //
    // First we take the first two digits - the thousands and hundreds - and
    // multiply them by 100, followed by adding the tens and ones.
    let year = digits(bytes[0], bytes[1]) as u16 * 100 + digits(bytes[2], bytes[3]) as u16;

    if year > YEAR_MAX || year < YEAR_MIN {
        return Err(TimestampParseError::year(year));
    }

    let month = {
        let value = digits(bytes[5], bytes[6]) - 1;

        match Month::new(value) {
            Some(month) => month,
            None => {
                return Err(TimestampParseError::month(value));
            }
        }
    };

    let day = digits(bytes[8], bytes[9]);

    let is_leap_year = constant::is_leap_year(year as u64);

    if day > month.days(is_leap_year) {
        return Err(TimestampParseError::day(year, month.legible(), day));
    }

    let hour = digits(bytes[11], bytes[12]);

    if hour > 23 {
        return Err(TimestampParseError::hour(hour));
    }

    let minute = digits(bytes[14], bytes[15]);

    if minute >= MINUTES_PER_HOUR {
        return Err(TimestampParseError::minute(minute));
    }
    let second = digits(bytes[17], bytes[18]);

    if second > SECONDS_PER_MINUTE {
        return Err(TimestampParseError::second(second));
    }

    // 10 leap years before the Discord epoch.
    let leap_years = 10 + (year - 2009) / 4;

    let mut running_year_days = month.running_total(is_leap_year);

    // We subtract one because the calendar is 1-indexed, but we're
    // computationally working off of a 0-indexed day.
    running_year_days += day as u16 - 1;

    let days = (year - EPOCH_UNIX) * DAYS_PER_YEAR + leap_years + running_year_days;

    // Active multiplier for the current byte.
    //
    // The first byte will be the hundred-thousands, so we need to multiply it by
    // that. The second is ten-thousands, so after the first iteration it needs
    // to be divided by 10, and so on.
    let mut multiplier = MICROSECONDS_PER_SECOND / 10;

    // Milliseconds.
    //
    // 2021-01-01T01:01:01.010000+00:00
    //                    |---|
    //
    // The API only works in milliseconds - not microseconds - so we can ignore
    // the last three digits due to them always being zeroes.
    let mut microseconds = 0;

    if bytes[19] == b'.' {
        // `const`: `for` loops can't be used so we need to use a `while`.
        //
        // The microseconds starts at index 20 of the input.
        let mut milliseconds_idx = 20;

        while milliseconds_idx < 26 {
            let byte = bytes[milliseconds_idx];

            microseconds += multiplier * (byte - ZERO_BYTE) as u64;

            // Remove a magnitude from the multiplier.
            multiplier /= 10;

            milliseconds_idx += 1;
        }
    }

    let time = second as u64
        + minute as u64 * MINUTES_PER_HOUR as u64
        + hour as u64 * (SECONDS_PER_HOUR as u64);

    // The Discord API doesn't send timestamps in any offset other than +00:00,
    // so we can just add up what we've got and call good.
    let seconds = time + days as u64 * SECONDS_PER_DAY;

    Ok(seconds * MICROSECONDS_PER_SECOND + microseconds)
}

/// Parse two bytes into digits and then add them together, multiplying the
/// first by a magnitude.
const fn digits(first: u8, second: u8) -> u8 {
    let tens = (first - ZERO_BYTE) * 10;
    let ones = second - ZERO_BYTE;

    tens + ones
}

#[cfg(test)]
mod tests {
    use super::{Timestamp, TimestampParseError};
    use serde::{Deserialize, Serialize};
    use static_assertions::{assert_impl_all, const_assert_eq};
    use std::{convert::TryFrom, fmt::Debug, hash::Hash, str::FromStr};

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
    const_assert_eq!(17, super::digits(b'1', b'7'));
    const_assert_eq!(3, super::digits(b'0', b'3'));

    /// Test a variety of supported ISO 8601 datetime formats.
    #[test]
    fn test_parse_iso8601() -> Result<(), TimestampParseError> {
        // With milliseconds.
        assert_eq!(
            1_580_608_922_020_000,
            super::parse_iso8601("2020-02-02T02:02:02.020000+00:00")?
        );

        // Without milliseconds.
        assert_eq!(
            1_580_608_922_000_000,
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
    fn test_parse_iso8601_boundaries() -> Result<(), TimestampParseError> {
        fn test(input: &str) -> Result<(), TimestampParseError> {
            assert_eq!(input, Timestamp::from_str(input)?.iso_8601().to_string());

            Ok(())
        }

        test("2021-12-31T23:59:59.999999+00:00")?;
        test("2021-01-01T00:00:00.000000+00:00")?;

        Ok(())
    }

    /// Test that 2016-03-01T07:33:19.000000+00:00 (1456817599) will parse and
    /// format back out the same value.
    ///
    /// The issue arose because in [`super::parse_iso8601`] we had a check for
    /// whether the year was a leap year and if the month was after February; if
    /// this was true then we added a day to the result. However, we already
    /// calculate the additional day in another part of the parsing function.
    #[test]
    fn test_march_1st_leap_year() -> Result<(), TimestampParseError> {
        const INPUT: &str = "2016-03-01T07:33:19.000000+00:00";
        const SECONDS: u64 = 1_456_817_599;

        let timestamp = Timestamp::from_str(INPUT)?;
        assert_eq!(SECONDS, timestamp.as_secs());
        let formatted = timestamp.iso_8601().to_string();

        assert_eq!(INPUT, formatted);
        assert_eq!(SECONDS, Timestamp::from_str(&formatted)?.as_secs());

        Ok(())
    }
}
