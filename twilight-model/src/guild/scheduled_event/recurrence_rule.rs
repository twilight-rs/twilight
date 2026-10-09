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
    pub frequence: RecurrenceRuleFrequency,
    /// Starting time of the recurrence interval.
    pub start: Timestamp,
}

/// How often the event recurs.
// n.b.: This enum is 0-indexed.
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
            1 => Self::Weekly,
            2 => Self::Monthly,
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
// n.b.: This enum is 1-indexed.
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
#[non_exhaustive]
#[serde(from = "u8", into = "u8")]
pub enum RecurrenceRuleMonth {
    January,
    February,
    March,
    April,
    May,
    June,
    July,
    August,
    September,
    October,
    November,
    December,
}

impl From<u8> for RecurrenceRuleMonth {
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
// n.b.: This enum is 0-indexed. The 0th day of the week is Monday.
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
