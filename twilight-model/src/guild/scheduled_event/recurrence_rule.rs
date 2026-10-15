//! Definitions for how often [`GuildScheduledEvent`]s should recur.
//!
//! Discord's recurrence rule is a subset of the behaviors defined in the
//! [iCalendar RFC] and implemented by [python's dateutil rrule].
//!
//! # Limitations
//!
//! Discord's implementation of recurrence rules has a number of limitations
//! which are not set in stone. Examples of possible limitations are mutually
//! exclusive fields or fields that can't be set by applications.
//!
//! Refer to [Discord Docs/System Limitations] for up-to-date information on
//! these limitations.
//!
//! [Discord Docs/System Limitations]: https://docs.discord.com/developers/resources/guild-scheduled-event#system-limitations
//! [`GuildScheduledEvent`]: super::GuildScheduledEvent
//! [iCalendar RFC]: https://datatracker.ietf.org/doc/html/rfc5545
//! [python's dateutil rrule]: https://dateutil.readthedocs.io/en/stable/rrule.html

use crate::util::Timestamp;
use serde::{Deserialize, Serialize};

/// Definition for how often a [`GuildScheduledEvent`] should recur.
///
/// Refer to the module-level documentation for more information.
///
/// [`GuildScheduledEvent`]: super::GuildScheduledEvent
#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct RecurrenceRule {
    /// Set of specific months to recur on.
    pub by_month: Option<Vec<RecurrenceRuleMonth>>,
    /// Set of specific dates within a month to recur on.
    pub by_month_day: Option<Vec<u8>>,
    /// List of specific days within a specific week (1 through 5, inclusively)
    /// to recur on.
    pub by_n_weekday: Option<Vec<RecurrenceRuleNWeekday>>,
    /// Set of specific days within a week for the event to recur on.
    pub by_weekday: Option<Vec<RecurrenceRuleWeekday>>,
    /// Set of days within a year to recur on.
    ///
    /// Valid values are in the range 1 through 364, inclusively.
    ///
    /// This field is currently not able to be set externally.
    pub by_year_day: Option<Vec<u16>>,
    /// Total amount of times that the event is allowed to recur before
    /// stopping.
    ///
    /// This field is currently not able to be set externally.
    pub count: Option<u64>,
    /// Ending time of the recurrence interval.
    ///
    /// This field is currently not able to be set externally.
    pub end: Option<Timestamp>,
    /// How often the event occurs.
    pub frequency: RecurrenceRuleFrequency,
    /// Spacing between the events, defined by frequency. For example,
    /// [`frequency`][Self::frequency] of
    /// [weekly][RecurrenceRuleFrequency::Weekly] and an interval of 2 would be
    /// "every-other week".
    pub interval: u64,
    /// Starting time of the recurrence interval.
    pub start: Timestamp,
}

/// How often the event recurs.
///
/// The numerical representation of this is 0-indexed, with value 0 being
/// [`Yearly`][Self::Yearly].
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
#[non_exhaustive]
#[serde(from = "u8", into = "u8")]
pub enum RecurrenceRuleFrequency {
    /// Recurrence frequency is daily.
    Daily,
    /// Recurrence frequency is weekly.
    Weekly,
    /// Recurrence frequency is monthly.
    Monthly,
    /// Recurrence frequency is yearly.
    Yearly,
    /// Recurrence frequency is an unknown value.
    Unknown(u8),
}

impl From<u8> for RecurrenceRuleFrequency {
    fn from(value: u8) -> Self {
        match value {
            0 => Self::Yearly,
            1 => Self::Monthly,
            2 => Self::Weekly,
            3 => Self::Daily,
            unknown => Self::Unknown(unknown),
        }
    }
}

impl From<RecurrenceRuleFrequency> for u8 {
    fn from(value: RecurrenceRuleFrequency) -> Self {
        match value {
            RecurrenceRuleFrequency::Daily => 3,
            RecurrenceRuleFrequency::Weekly => 2,
            RecurrenceRuleFrequency::Monthly => 1,
            RecurrenceRuleFrequency::Yearly => 0,
            RecurrenceRuleFrequency::Unknown(unknown) => unknown,
        }
    }
}

/// Month to recur on.
///
/// The numerical representation of this is 1-indexed, with value 1 being
/// [`January`][Self::January].
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
#[non_exhaustive]
#[serde(from = "u8", into = "u8")]
pub enum RecurrenceRuleMonth {
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

impl From<u8> for RecurrenceRuleMonth {
    /// Convert from a numerical representation to a defined month.
    ///
    /// # Panics
    ///
    /// Panics if the value is invalid; that is, 0 or greater than 12.
    fn from(value: u8) -> Self {
        match value {
            1 => Self::January,
            2 => Self::February,
            3 => Self::March,
            4 => Self::April,
            5 => Self::May,
            6 => Self::June,
            7 => Self::July,
            8 => Self::August,
            9 => Self::September,
            10 => Self::October,
            11 => Self::November,
            12 => Self::December,
            other => {
                // It all started on the 13th hour of the 13th day of the 13th
                // month. We were there to discuss the misprinted calendars the
                // school had purchased.
                panic!("unknown recurrence rule month: {other}");
            }
        }
    }
}

impl From<RecurrenceRuleMonth> for u8 {
    fn from(value: RecurrenceRuleMonth) -> Self {
        match value {
            RecurrenceRuleMonth::January => 1,
            RecurrenceRuleMonth::February => 2,
            RecurrenceRuleMonth::March => 3,
            RecurrenceRuleMonth::April => 4,
            RecurrenceRuleMonth::May => 5,
            RecurrenceRuleMonth::June => 6,
            RecurrenceRuleMonth::July => 7,
            RecurrenceRuleMonth::August => 8,
            RecurrenceRuleMonth::September => 9,
            RecurrenceRuleMonth::October => 10,
            RecurrenceRuleMonth::November => 11,
            RecurrenceRuleMonth::December => 12,
        }
    }
}
/// Weekday to recur on.
///
/// The numerical representation of this is 0-indexed, with value 0 being
/// [`Monday`][Self::Monday].
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
#[non_exhaustive]
#[serde(from = "u8", into = "u8")]
pub enum RecurrenceRuleWeekday {
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
    Sunday,
}

impl From<u8> for RecurrenceRuleWeekday {
    fn from(value: u8) -> Self {
        match value {
            0 => Self::Monday,
            1 => Self::Tuesday,
            2 => Self::Wednesday,
            3 => Self::Thursday,
            4 => Self::Friday,
            5 => Self::Saturday,
            6 => Self::Sunday,
            other => {
                // We're unlikely to move away from the Gregorian calendar soon.
                panic!("unknown recurrence rule weekday: {other}");
            }
        }
    }
}

impl From<RecurrenceRuleWeekday> for u8 {
    fn from(value: RecurrenceRuleWeekday) -> Self {
        match value {
            RecurrenceRuleWeekday::Monday => 0,
            RecurrenceRuleWeekday::Tuesday => 1,
            RecurrenceRuleWeekday::Wednesday => 2,
            RecurrenceRuleWeekday::Thursday => 3,
            RecurrenceRuleWeekday::Friday => 4,
            RecurrenceRuleWeekday::Saturday => 5,
            RecurrenceRuleWeekday::Sunday => 6,
        }
    }
}

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct RecurrenceRuleNWeekday {
    /// Day within the week to reoccur on.
    pub day: RecurrenceRuleWeekday,
    /// Week to reoccur on.
    ///
    /// Must be a value between 1 through 5, inclusively.
    pub n: u8,
}

#[cfg(test)]
mod tests {
    use super::{RecurrenceRuleMonth, RecurrenceRuleWeekday};

    /// Test that `RecurrenceRuleMonth` is 1-indexed and converting from 0
    /// panics.
    #[should_panic(expected = "unknown recurrence rule month: 0")]
    #[test]
    fn month_one_indexed() {
        let _ = RecurrenceRuleMonth::from(0u8);
    }

    /// Test that `RecurrenceRuleMonth` does not have 13 months.
    #[should_panic(expected = "unknown recurrence rule month: 13")]
    #[test]
    fn month_thirteen_invalid() {
        let _ = RecurrenceRuleMonth::from(13u8);
    }

    /// Test that `RecurrenceRuleWeekday` does not have 8 days.
    #[should_panic(expected = "unknown recurrence rule weekday: 7")]
    #[test]
    fn weekday_eight_invalid() {
        let _ = RecurrenceRuleWeekday::from(7u8);
    }
}
