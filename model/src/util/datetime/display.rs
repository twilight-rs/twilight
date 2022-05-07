//! Display implementation for formatting a [`Timestamp`].

use super::Timestamp;
use serde::{Serialize, Serializer};
use std::fmt::{Display, Formatter, Result as FmtResult};

/// Display implementation to format a [`Timestamp`] in an ISO 8601 format.
///
/// # Examples
///
/// Format a timestamp as an ISO 8601 datetime both with microseconds:
///
/// ```
/// # use std::error::Error;
/// # fn foo() -> Result<(), Box<dyn Error>> {
/// use twilight_model::util::Timestamp;
///
/// let timestamp = Timestamp::from_micros(1_628_594_197_020_000)?;
/// assert_eq!(
///     "2021-08-10T11:16:37.020000+00:00",
///     timestamp.iso_8601().to_string(),
/// );
/// # Ok(()) }
/// ```
#[derive(Debug)]
pub struct TimestampIso8601Display {
    /// Timestamp.
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
    /// # use std::error::Error;
    /// # fn foo() -> Result<(), Box<dyn Error>> {
    /// use twilight_model::util::Timestamp;
    ///
    /// let timestamp = Timestamp::from_micros(1_628_594_197_020_000)?;
    /// let formatter = timestamp.iso_8601().with_microseconds(true);
    ///
    /// assert_eq!("2021-08-10T11:16:37.020000+00:00", formatter.to_string());
    /// # Ok(()) }
    /// ```
    ///
    /// Format a timestamp without microseconds:
    ///
    /// ```
    /// # use std::error::Error;
    /// # fn foo() -> Result<(), Box<dyn Error>> {
    /// use twilight_model::util::Timestamp;
    ///
    /// let timestamp = Timestamp::from_micros(1_628_594_197_020_000)?;
    /// let formatter = timestamp.iso_8601().with_microseconds(false);
    ///
    /// assert_eq!("2021-08-10T11:16:37+00:00", formatter.to_string());
    /// # Ok(()) }
    /// ```
    #[must_use]
    pub const fn with_microseconds(mut self, with_microseconds: bool) -> Self {
        self.with_microseconds = with_microseconds;

        self
    }
}

impl Display for TimestampIso8601Display {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        // Years.
        //
        // Input: 2021-01-01T01:01:01.010000+00:00
        //       |----|
        let year = self.timestamp.0.year();

        Display::fmt(&(year / 1000), f)?;
        Display::fmt(&(year / 100 % 10), f)?;
        Display::fmt(&(year / 10 % 10), f)?;
        Display::fmt(&(year % 10), f)?;

        f.write_str("-")?;

        // Months.
        //
        // Input: 2021-01-01T01:01:01.010000+00:00
        //            |--|
        let month = self.timestamp.0.month() as u8;

        Display::fmt(&(month / 10), f)?;
        Display::fmt(&(month % 10), f)?;
        f.write_str("-")?;

        // Days.
        //
        // Input: 2021-01-01T01:01:01.010000+00:00
        //               |--|
        let day = self.timestamp.0.day();

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
        let hour = self.timestamp.0.hour();

        Display::fmt(&(hour / 10), f)?;
        Display::fmt(&(hour % 10), f)?;

        f.write_str(":")?;

        // Minutes.
        //
        // Input: 2021-01-01T01:01:01.010000+00:00
        //                     |--|
        let minute = self.timestamp.0.minute();

        Display::fmt(&(minute / 10 % 6), f)?;
        Display::fmt(&(minute % 10), f)?;

        f.write_str(":")?;

        // Seconds.
        //
        // Input: 2021-01-01T01:01:01.010000+00:00
        //                        |--|
        let second = self.timestamp.0.second();

        Display::fmt(&(second / 10 % 6), f)?;
        Display::fmt(&(second % 10), f)?;

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
            let microsecond = self.timestamp.0.microsecond();

            Display::fmt(&(microsecond / 100_000), f)?;
            Display::fmt(&(microsecond / 10_000 % 10), f)?;
            Display::fmt(&(microsecond / 1_000 % 10), f)?;
            Display::fmt(&(microsecond / 100 % 10), f)?;
            Display::fmt(&(microsecond / 10 % 10), f)?;
            Display::fmt(&(microsecond % 10), f)?;
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

#[cfg(test)]
mod tests {
    use super::{super::Timestamp, TimestampIso8601Display};
    use serde::Serialize;
    use static_assertions::assert_impl_all;
    use std::fmt::Debug;

    assert_impl_all!(TimestampIso8601Display: Debug, Send, Serialize, Sync);

    #[test]
    fn test_display() {
        const LONG: &str = "2020-02-02T02:02:02.020000+00:00";
        const SHORT: &str = "2020-02-02T02:02:02+00:00";
        const TIME: i64 = 1_580_608_922_020_000;

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
}
