//! Display implementation for formatting a [`Timestamp`].

use crate::datetime::constant::{
    self, DAYS_PER_YEAR, EPOCH_UNIX, MICROSECONDS_PER_SECOND, MINUTES_PER_HOUR, SECONDS_PER_HOUR,
    YEARS_PER_LEAP, YEAR_MAX,
};

use super::{
    constant::{Month, SECONDS_PER_DAY},
    Timestamp,
};
use serde::{Serialize, Serializer};
use std::fmt::{Display, Error as FmtError, Formatter, Result as FmtResult};

/// Display implementation to format a [`Timestamp`] in an ISO 8601 format.
///
/// Timestamps up to and including the year 2038 are supported.
///
/// # Examples
///
/// Format a timestamp as an ISO 8601 datetime both with microseconds:
///
/// ```
/// # fn foo() -> Option<()> {
/// use twilight_model::datetime::Timestamp;
///
/// let timestamp = Timestamp::from_micros(1_628_594_197_020_000)?;
/// assert_eq!(
///     "2021-08-10T11:16:37.020000+00:00",
///     timestamp.iso_8601().to_string(),
/// );
/// # None }
/// ```
#[derive(Debug)]
pub struct TimestampIso8601Display {
    /// Timestamp with the time stored as a Unix timestamp.
    timestamp: Timestamp,
    /// Whether to format the timestamp with microseconds included.
    with_microseconds: bool,
}

impl TimestampIso8601Display {
    /// Create a new ISO 8601 display formatter for a timestamp.
    pub(super) const fn new(timestamp: Timestamp) -> Self {
        Self {
            timestamp,
            with_microseconds: true,
        }
    }

    /// Get the inner timestamp.
    pub const fn get(self) -> Timestamp {
        self.timestamp
    }

    /// Whether to format the timestamp with microseconds.
    ///
    /// The ISO 8601 display formatter formats with microseconds by default.
    ///
    /// # Examples
    ///
    /// Format a timestamp with microseconds:
    ///
    /// ```
    /// # fn foo() -> Option<()> {
    /// use twilight_model::datetime::Timestamp;
    ///
    /// let timestamp = Timestamp::from_micros(1_628_594_197_020_000)?;
    /// let formatter = timestamp.iso_8601().with_microseconds(true);
    ///
    /// assert_eq!("2021-08-10T11:16:37.020000+00:00", formatter.to_string());
    /// # None }
    /// ```
    ///
    /// Format a timestamp without microseconds:
    ///
    /// ```
    /// # fn foo() -> Option<()> {
    /// use twilight_model::datetime::Timestamp;
    ///
    /// let timestamp = Timestamp::from_micros(1_628_594_197_020_000)?;
    /// let formatter = timestamp.iso_8601().with_microseconds(false);
    ///
    /// assert_eq!("2021-08-10T11:16:37+00:00", formatter.to_string());
    /// # None }
    /// ```
    pub const fn with_microseconds(mut self, with_microseconds: bool) -> Self {
        self.with_microseconds = with_microseconds;

        self
    }
}

impl Display for TimestampIso8601Display {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        /// Approximate number of seconds before we're too far in the future
        /// to care.
        pub const APPROXIMATE_MAX_SECONDS: u64 =
            (YEAR_MAX as u64 - EPOCH_UNIX as u64) * DAYS_PER_YEAR as u64 * SECONDS_PER_DAY;

        let total_seconds = self.timestamp.as_secs();
        let microseconds = self.timestamp.as_micros() % MICROSECONDS_PER_SECOND;

        if total_seconds >= APPROXIMATE_MAX_SECONDS {
            return Err(FmtError);
        }

        let seconds = total_seconds % SECONDS_PER_DAY;

        let Date { day, month, year } = date(total_seconds);

        // Years.
        //
        // Input: 2021-01-01T01:01:01.010000+00:00
        //       |----|
        Display::fmt(&(year / 1000), f)?;
        Display::fmt(&(year / 100 % 10), f)?;
        Display::fmt(&(year / 10 % 10), f)?;
        Display::fmt(&(year % 10), f)?;
        f.write_str("-")?;

        // Months.
        //
        // Input: 2021-01-01T01:01:01.010000+00:00
        //            |--|
        Display::fmt(&(month / 10), f)?;
        Display::fmt(&(month % 10), f)?;
        f.write_str("-")?;

        // Days.
        //
        // Input: 2021-01-01T01:01:01.010000+00:00
        //               |--|
        Display::fmt(&(day / 10), f)?;
        Display::fmt(&(day % 10), f)?;

        // Time designator.
        //
        // Input: 2021-01-01T01:01:01.010000+00:00
        //                 |-|
        f.write_str("T")?;

        // Hours.
        //
        // Input: 2021-01-01T01:01:01.010000+00:00
        //                  |--|
        Display::fmt(&(seconds / u64::from(SECONDS_PER_HOUR) / 10), f)?;
        Display::fmt(&(seconds / u64::from(SECONDS_PER_HOUR) % 10), f)?;
        f.write_str(":")?;

        // Minutes.
        //
        // Input: 2021-01-01T01:01:01.010000+00:00
        //                     |--|
        Display::fmt(&(seconds / u64::from(MINUTES_PER_HOUR) / 10 % 6), f)?;
        Display::fmt(&(seconds / u64::from(MINUTES_PER_HOUR) % 10), f)?;
        f.write_str(":")?;

        // Seconds.
        //
        // Input: 2021-01-01T01:01:01.010000+00:00
        //                        |--|
        Display::fmt(&(seconds / 10 % 6), f)?;
        Display::fmt(&(seconds % 10), f)?;

        if self.with_microseconds {
            // Subsecond designator.
            //
            // Input: 2021-01-01T01:01:01.010000+00:00
            //                          |-|
            f.write_str(".")?;

            // Microseconds.
            //
            // Input: 2021-01-01T01:01:01.010000+00:00
            //                           |------|
            Display::fmt(&(microseconds / 100_000), f)?;
            Display::fmt(&(microseconds / 10_000 % 10), f)?;
            Display::fmt(&(microseconds / 1_000 % 10), f)?;
            Display::fmt(&(microseconds / 100 % 10), f)?;
            Display::fmt(&(microseconds / 10 % 10), f)?;
            Display::fmt(&(microseconds % 10), f)?;
        }

        // Finish it all off.
        //
        // Input: 2021-01-01T01:01:01.010000+00:00
        //                                 |-----|
        //
        // The API doesn't operate in offsets other than +00:00, so we can just
        // fill that in.
        f.write_str("+00:00")
    }
}

impl Serialize for TimestampIso8601Display {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.collect_str(self)
    }
}

/// Parsed date of the timestamp.
struct Date {
    /// Day in the month.
    day: u64,
    /// Month in the year.
    month: u8,
    /// Year.
    year: u64,
}

/// Determine the date of a timestamp based on the total number of seconds
/// within it.
const fn date(total_seconds: u64) -> Date {
    /// Number of days between each leap day, exclusively from one and
    /// inclusive to the other.
    pub const DAYS_PER_LEAP: u16 = (DAYS_PER_YEAR * YEARS_PER_LEAP as u16) + 1;

    /// Closest leap year before the Discord epoch.
    pub const LEAP_EPOCH_YEAR: u64 = 2008;

    /// Closest date before the Discord epoch that is just after a leap day.
    ///
    /// This is equivalent to March 1st, [`LEAP_BEFORE_EPOCH`].
    pub const LEAP_EPOCH: u64 = 13880;

    // Convert the constants to u64s for simple repeated use.
    let days_per_leap = DAYS_PER_LEAP as u64;
    let days_per_year = DAYS_PER_YEAR as u64;

    let mut day = (total_seconds / SECONDS_PER_DAY) - LEAP_EPOCH;
    let leaps = day / days_per_leap;
    day -= leaps * days_per_leap;

    // Number of years since the most recent leap.
    let years_after_leap = {
        let value = day / days_per_year;

        if value >= YEARS_PER_LEAP as u64 {
            YEARS_PER_LEAP as u64 - 1
        } else {
            value
        }
    };
    day -= years_after_leap * days_per_year;

    let mut year = LEAP_EPOCH_YEAR + years_after_leap + (YEARS_PER_LEAP as u64 * leaps);

    let is_leap_year = constant::is_leap_year(year);

    let mut month = 0;

    loop {
        // `const`: Clippy suggests using `Option::map_or` which isn't const.
        #[allow(clippy::option_if_let_else)]
        let named_month = if let Some(named_month) = Month::new(month) {
            named_month
        } else {
            month = Month::January as u8;
            year += 1;

            Month::January
        };

        let month_days = named_month.days(is_leap_year) as u64;

        if day < month_days {
            break;
        }

        day -= month_days;
        month += 1;
    }

    if is_leap_year && month < Month::March as u8 {
        day += 1;
    }

    day += 1;

    if let Some(named_month) = Month::new(month) {
        let month_days = named_month.days(is_leap_year) as u64;

        if day > month_days {
            month += 1;
            day -= month_days;
        }

        if month as u8 > Month::December as u8 {
            month = Month::January as u8;
            year += 1;
        }
    }

    Date {
        day,
        month: month + 1,
        year,
    }
}

#[cfg(test)]
mod tests {
    use crate::datetime::constant::{YEAR_MAX, YEAR_MIN};

    use super::{
        super::{Timestamp, TimestampParseError},
        TimestampIso8601Display,
    };
    use serde::Serialize;
    use static_assertions::assert_impl_all;
    use std::{fmt::Debug, str::FromStr};

    assert_impl_all!(TimestampIso8601Display: Debug, Send, Serialize, Sync);

    #[test]
    fn test_display() {
        const LONG: &str = "2020-02-02T02:02:02.020000+00:00";
        const SHORT: &str = "2020-02-02T02:02:02+00:00";
        const TIME: u64 = 1_580_608_922_020_000;

        let mut formatter = Timestamp::from_micros(TIME).expect("non zero").iso_8601();

        // Default formatter should be with microseconds.
        assert_eq!(LONG, formatter.to_string());

        // Now with explicitly setting it to format with microseconds.
        formatter = formatter.with_microseconds(true);
        assert_eq!(LONG, formatter.to_string());

        // And now without microseconds.
        formatter = formatter.with_microseconds(false);
        assert_eq!(SHORT, formatter.to_string());
    }

    /// Test that the years we care about properly parse.
    #[test]
    fn test_century() -> Result<(), TimestampParseError> {
        for year in YEAR_MIN..YEAR_MAX {
            let input = format!("{}-02-01T02:02:02.000000+00:00", year);
            let timestamp = Timestamp::from_str(&input)?;

            assert_eq!(input, timestamp.iso_8601().to_string());
        }

        Ok(())
    }
}
