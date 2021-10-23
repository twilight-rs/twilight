//! Constants and constant functions for use in datetime calculations across
//! multiple functions.
//!
//! This module primarily exists to avoid the use of magic numbers and to
//! centralize date related constant math, which in turn reduces the complexity
//! of learning the [`datetime`] module.
//!
//! [`datetime`]: super

/// Number of days in a standard year.
pub const DAYS_PER_YEAR: u16 = 365;

/// Year of the Unix epoch.
pub const EPOCH_UNIX: u16 = 1970;

/// Number of microseconds in a second.
pub const MICROSECONDS_PER_SECOND: u64 = 1_000_000;

/// Number of minutes in an hour.
pub const MINUTES_PER_HOUR: u8 = 60;

/// Number of seconds in a standard day.
pub const SECONDS_PER_DAY: u64 = (SECONDS_PER_HOUR as u64) * 24;

/// Number of seconds in an hour.
pub const SECONDS_PER_HOUR: u16 = SECONDS_PER_MINUTE as u16 * MINUTES_PER_HOUR as u16;

/// Number of seconds in a minute.
pub const SECONDS_PER_MINUTE: u8 = 60;

/// Number of years in a leap year, inclusively.
pub const YEARS_PER_LEAP: u8 = 4;

/// Maximum number of acceptable years.
///
/// When updating this value be sure to update [`TimestampIso8601Display`]'s
/// documentation.
///
/// [`TimestampIso8601Display`]: super::TimestampIso8601Display
pub const YEAR_MAX: u16 = 2038;

/// Minimum number of acceptable years.
pub const YEAR_MIN: u16 = 2010;

/// Length of each standard month.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(u8)]
pub enum Month {
    /// First month of the year.
    January,
    /// Second month of the year.
    February,
    /// Third month of the year.
    March,
    /// Fourth month of the year.
    April,
    /// Fifth month of the year.
    May,
    /// Sixth month of the year.
    June,
    /// Seventh month of the year.
    July,
    /// Eighth month of the year.
    August,
    /// Ninth month of the year.
    September,
    /// Tenth month of the year.
    October,
    /// Eleventh month of the year.
    November,
    /// Twelfth month of the year.
    December,
}

impl Month {
    /// List of months in their correct calendar order.
    pub const MONTHS: [Month; 12] = [
        Month::January,
        Month::February,
        Month::March,
        Month::April,
        Month::May,
        Month::June,
        Month::July,
        Month::August,
        Month::September,
        Month::October,
        Month::November,
        Month::December,
    ];

    /// Create a new month from its raw value.
    ///
    /// The provided argument must be 0-indexed.
    pub const fn new(month: u8) -> Option<Self> {
        let mut idx = 0;

        while idx < Self::MONTHS.len() {
            let typed_month = Self::MONTHS[idx];

            if typed_month as u8 == month {
                return Some(typed_month);
            }

            idx += 1;
        }

        None
    }

    /// Number of days in the month, 1-indexed.
    pub const fn days(self, leap_year: bool) -> u8 {
        match self {
            Self::February if leap_year => 29,
            Self::February => 28,
            Self::January
            | Self::March
            | Self::May
            | Self::July
            | Self::August
            | Self::October
            | Self::December => 31,
            Self::April | Self::June | Self::September | Self::November => 30,
        }
    }

    /// Legible number of the month, 1-indexed.
    ///
    /// Although the programmatic number of say, [`April`], is 3 the
    /// human-legible number is 4.
    ///
    /// [`April`]: Month::April
    pub const fn legible(self) -> u8 {
        self as u8 + 1
    }

    /// Running total of days leading up to, but excluding, the month.
    pub const fn running_total(self, leap_year: bool) -> u16 {
        let mut total = 0;
        let mut idx = 0;

        while idx < self as usize {
            let month = Month::MONTHS[idx];
            total += month.days(leap_year) as u16;

            idx += 1;
        }

        total
    }
}

/// Whether the given year is a leap year.
///
/// This only does the simple check that a year is divisible by four and does
/// not take into account century leaps and quadricentennial (400 years) leaps.
pub const fn is_leap_year(year: u64) -> bool {
    year % 4 == 0
}

#[cfg(test)]
mod tests {
    use super::Month;
    use static_assertions::{const_assert, const_assert_eq};

    // Test creating a month from its index. The index must be 0-indexed.
    //
    // `const`: we must use `matches!` instead of `const_assert_eq!` because
    // equality checks on non-primitives aren't const.
    const_assert!(matches!(Month::new(0), Some(Month::January)));
    const_assert!(matches!(Month::new(11), Some(Month::December)));
    const_assert!(Month::new(12).is_none());

    // Check that we've got a few of the months' days right. It needs to know
    // whether the year is a leap year because February has a variable number
    // of days.
    const_assert_eq!(31, Month::January.days(false));
    const_assert_eq!(29, Month::February.days(true));
    const_assert_eq!(28, Month::February.days(false));
    // Whether the year is a leap year should make no difference for months other
    // than February.
    const_assert_eq!(31, Month::August.days(true));
    const_assert_eq!(31, Month::August.days(false));

    // Check the running total of months based on whether the year is a leap
    // year.
    const_assert_eq!(59, Month::March.running_total(false));
    const_assert_eq!(60, Month::March.running_total(true));
    const_assert_eq!(334, Month::December.running_total(false));
    const_assert_eq!(335, Month::December.running_total(true));

    // Do some simple checks to *really make sure* we've got the complicated
    // logic to determine a leap year down.
    const_assert!(!super::is_leap_year(2010));
    const_assert!(!super::is_leap_year(2011));
    const_assert!(super::is_leap_year(2012));
    const_assert!(!super::is_leap_year(2013));
    const_assert!(super::is_leap_year(2016));
}
