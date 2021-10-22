//! Error detail implementation for [`Timestamp`] parsing.
//!
//! [`Timestamp`]s can fail to parse for a few reasons, detailed by
//! [`TimestampParseErrorType`]'s variants. Of note is the
//! [`TimestampParseErrorType::Range`] variant detailing the field that was out
//! of range. This can occur when, for example, a given [hour] is 24 but the
//! acceptable range is from 0 to 23, inclusively.
//!
//! [`Timestamp`]: super::Timestamp
//! [hour]: RangeField::Hour

use super::constant::{self, Month};
use std::{
    error::Error,
    fmt::{Display, Formatter, Result as FmtResult},
};

/// Reason that an ISO 8601 format couldn't be parsed.
#[derive(Debug)]
pub struct TimestampParseError {
    /// Type of error that occurred.
    kind: TimestampParseErrorType,
}

impl TimestampParseError {
    /// Error that was caused by the datetime being of an improper format.
    pub(super) const FORMAT: TimestampParseError = TimestampParseError {
        kind: TimestampParseErrorType::Format,
    };

    /// Error that was caused by the datetime parsing to a unix timestamp of
    /// zero.
    pub(super) const ZERO: TimestampParseError = TimestampParseError {
        kind: TimestampParseErrorType::Zero,
    };

    /// Immutable reference to the type of error that occurred.
    #[must_use = "retrieving the type has no effect if left unused"]
    pub const fn kind(&self) -> &TimestampParseErrorType {
        &self.kind
    }

    /// Consume the error, returning the source error if there is any.
    #[allow(clippy::unused_self)]
    #[must_use = "consuming the error and retrieving the source has no effect if left unused"]
    pub fn into_source(self) -> Option<Box<dyn Error + Send + Sync>> {
        None
    }

    /// Consume the error, returning the owned error type and the source error.
    #[must_use = "consuming the error into its parts has no effect if left unused"]
    pub fn into_parts(
        self,
    ) -> (
        TimestampParseErrorType,
        Option<Box<dyn Error + Send + Sync>>,
    ) {
        (self.kind, None)
    }

    /// Create a new error with a year range issue.
    pub(super) const fn year(value: u16) -> Self {
        Self {
            kind: TimestampParseErrorType::Range {
                field: RangeField::Year { value },
            },
        }
    }

    /// Create a new error with a month range issue.
    pub(super) const fn month(value: u8) -> Self {
        Self {
            kind: TimestampParseErrorType::Range {
                field: RangeField::Month { value },
            },
        }
    }

    /// Create a new error with a day range issue.
    pub(super) const fn day(year: u16, month: u8, day: u8) -> Self {
        Self {
            kind: TimestampParseErrorType::Range {
                field: RangeField::Day {
                    value: day,
                    month,
                    year,
                },
            },
        }
    }

    /// Create a new error with an hour range issue.
    pub(super) const fn hour(value: u8) -> Self {
        Self {
            kind: TimestampParseErrorType::Range {
                field: RangeField::Hour { value },
            },
        }
    }

    /// Create a new error with a minute range issue.
    pub(super) const fn minute(value: u8) -> Self {
        Self {
            kind: TimestampParseErrorType::Range {
                field: RangeField::Minute { value },
            },
        }
    }

    /// Create a new error with a second range issue.
    pub(super) const fn second(value: u8) -> Self {
        Self {
            kind: TimestampParseErrorType::Range {
                field: RangeField::Second { value },
            },
        }
    }
}

impl Display for TimestampParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match &self.kind {
            TimestampParseErrorType::Format => {
                f.write_str("provided value is not in an iso 8601 format")
            }
            TimestampParseErrorType::Range { field } => match field {
                RangeField::Day { year, month, value } => {
                    if *value == 0 {
                        return f.write_str("day of 0 provided, but days are 1-indexed");
                    }

                    let is_leap_year = constant::is_leap_year(u64::from(*year));
                    let days_in_month = Month::new(*month as u8)
                        .map(|month| month.days(is_leap_year))
                        .unwrap_or_default();

                    f.write_str("day of ")?;
                    Display::fmt(&value, f)?;
                    f.write_str(" provided but month ")?;
                    Display::fmt(&month, f)?;
                    f.write_str(" only has ")?;
                    Display::fmt(&days_in_month, f)?;

                    f.write_str(" days")
                }
                RangeField::Hour { value } => {
                    f.write_str("hour is ")?;
                    Display::fmt(&value, f)?;

                    f.write_str(" but must be at most 23")
                }
                RangeField::Minute { value } => {
                    f.write_str("minute is ")?;
                    Display::fmt(&value, f)?;

                    f.write_str(" but must be at most 59")
                }
                RangeField::Month { value } => {
                    f.write_str("month is ")?;
                    Display::fmt(&value, f)?;

                    f.write_str(" but must be 1-12, inclusively")
                }
                RangeField::Second { value } => {
                    f.write_str("second is ")?;
                    Display::fmt(&value, f)?;

                    f.write_str(" but must at most 59, inclusively")
                }
                RangeField::Year { value } => {
                    f.write_str("year is ")?;
                    Display::fmt(&value, f)?;

                    f.write_str(" but must be at least 2010")
                }
            },
            TimestampParseErrorType::Zero => {
                f.write_str("datetime value is equivalent to zero; must be non-zero")
            }
        }
    }
}

impl Error for TimestampParseError {}

/// Type of [`TimestampParseError`] that occurred.
#[derive(Debug)]
pub enum TimestampParseErrorType {
    /// Format of the input datetime is invalid.
    ///
    /// A datetime can take two forms: with microseconds and without
    /// microseconds.
    Format,
    /// Value of a field is not in an acceptable range.
    Range {
        /// Field that is out of range and why.
        field: RangeField,
    },
    /// Provided datetime value is equivalent to zero.
    ///
    /// Datetime values must be non-zero.
    Zero,
}

/// Field that is out of range.
///
/// During parsing of timestamps the values of fields - such as the hour or day
/// portion of a timestamp - are validated. If one is outside of the accepted
/// range then the function is short-circuited an an error is returned.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum RangeField {
    /// Day field is out of the acceptable range.
    ///
    /// The acceptable ranges are:
    ///
    /// - [1, 31] in the case of January, March, May, July, August, October, and
    /// December;
    /// - [1, 30] in the case of April, June, September, and November;
    /// - [1, 29] in the case of February on a leap year;
    /// - [1, 28] in the case of February on a year that is not a leap year.
    Day {
        /// Year.
        ///
        /// Can be used in combination with [`month`] to determine the range of
        /// acceptable days.
        ///
        /// [`month`]: Self::Day::month
        year: u16,
        /// Month of the year.
        ///
        /// Can be used in combination with [`year`] to determine the range of
        /// acceptable days.
        ///
        /// [`year`]: Self::Day::year
        month: u8,
        /// Provided value.
        value: u8,
    },
    /// Hour field is out of the acceptable range.
    ///
    /// The acceptable range is [0, 23].
    Hour {
        /// Provided value.
        value: u8,
    },
    /// Minute field is out of the acceptable range.
    ///
    /// The acceptable range is [0, 59].
    Minute {
        /// Provided value.
        value: u8,
    },
    /// Month field is out of the acceptable range.
    ///
    /// The acceptable range is [1, 12].
    Month {
        /// Provided value.
        value: u8,
    },
    /// Second field is out of the acceptable range.
    ///
    /// The acceptable range is [0, 59], or [0, 60] in the case of a leap
    /// second.
    Second {
        /// Provided value.
        value: u8,
    },
    /// Year field is out of the acceptable range.
    ///
    /// The acceptable range is [2010, 2038].
    Year {
        /// Provided value.
        value: u16,
    },
}

#[cfg(test)]
mod tests {
    use super::{RangeField, TimestampParseError, TimestampParseErrorType};
    use static_assertions::assert_impl_all;
    use std::{error::Error, fmt::Debug, hash::Hash};

    assert_impl_all!(RangeField: Clone, Debug, Eq, Hash, PartialEq, Send, Sync);
    assert_impl_all!(TimestampParseErrorType: Debug, Send, Sync);
    assert_impl_all!(TimestampParseError: Error, Send, Sync);
}
