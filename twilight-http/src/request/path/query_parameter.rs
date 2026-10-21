//! Utilities for writing key-value query parameter pairs within an HTTP path.
//!
//! Query parameters are composed of two items within a pair: the key and value.
//! The key must be a string, and so we represent this as a non-negotiable
//! string slice. The value can more or less be anything.
//!
//! TODO

use percent_encoding::NON_ALPHANUMERIC;
use std::fmt::{self, Display, Formatter, Write as _};
use twilight_model::id::Id;

pub struct QueryParameterWriter<'a, 'b> {
    has_query_parameter: bool,
    inner: &'a mut Formatter<'b>,
}

impl<'a, 'b> QueryParameterWriter<'a, 'b> {
    pub(super) fn new(formatter: &'a mut Formatter<'b>, has_query_parameter: bool) -> Self {
        Self {
            has_query_parameter,
            inner: formatter,
        }
    }

    /// Write a single key-value pair to the query parameter list.
    pub fn write_pair<T: QueryParameterValue>(&mut self, key: &str, value: T) -> fmt::Result {
        /// Character for a combiner between two key-value pairs.
        const AMPERSAND: char = '&';

        /// Character for a combiner between the key and value within a
        /// key-value pair.
        const EQUALS_SIGN: char = '=';

        /// Character denoting the start of the query within a path.
        const QUESTION_MARK: char = '?';

        self.inner.write_char(if self.has_query_parameter {
            AMPERSAND
        } else {
            QUESTION_MARK
        })?;

        // A partially-formed query parameter is both valid according to
        // specification and accepted by Discord. "?&foo=bar", "?foo&qux=quz",
        // "?foo=&qux=quz", and "?foo=bar&qux=quz" are all valid key-value pairs
        // and keys with empty or undefined values (in this example, "foo") will
        // be ignored.
        //
        // And, even if it weren't accepted by Discord, there's not much we can
        // *reasonably* do here to "walk back" writing one portion if a latter
        // portion fails to write. Let's call this a compromise and leave
        // everyone unsatisfied with the potential subpar query string
        // formatting.
        self.has_query_parameter = true;
        self.inner.write_str(key)?;
        self.inner.write_char(EQUALS_SIGN)?;
        T::fmt(&value, self.inner)?;

        Ok(())
    }
}

pub trait QueryParameter {
    /// Whether at least one key-value query parameter will be written.
    ///
    /// [`CsvQueryParameter`] accepts a single key and a list of values. If the
    /// list has at least one value, then this would return true; otherwise,
    /// there is nothing to write, and so it would return false.
    fn will_write(&self) -> bool;

    fn write<'a>(&self, f: QueryParameterWriter<'_, '_>) -> fmt::Result;
}

impl<'a, T: QueryParameterValue> QueryParameter for (&'a str, T) {
    fn will_write(&self) -> bool {
        T::will_write(&self.1)
    }

    fn write(&self, mut writer: QueryParameterWriter<'_, '_>) -> fmt::Result {
        writer.write_pair(&self.0, &self.1)
    }
}

/// Key-value pair to be written in the query of a path.
///
/// This is a structure identical to passing [`(&str, T)`] as a query parameter,
/// but may be preferable to use under some circumstances such as when there is
/// heavy use of other query parameters like [`CsvQueryParameter`] and
/// [`RepeatingQueryParameter`].
pub struct KeyValueQueryParameter<'a, T> {
    inner: (&'a str, T),
}

impl<'a, T: QueryParameterValue> KeyValueQueryParameter<'a, T> {
    pub fn new(key: &'a str, value: T) -> Self {
        Self {
            inner: (key, value),
        }
    }
}

impl<'a, T: QueryParameterValue> QueryParameter for KeyValueQueryParameter<'a, T> {
    fn will_write(&self) -> bool {
        QueryParameter::will_write(&self.inner)
    }

    fn write<'b>(&self, f: QueryParameterWriter<'_, '_>) -> fmt::Result {
        QueryParameter::write(&self.inner, f)
    }
}

/// Write a query parameter with a Comma-Separated Value.
///
/// The value for the key will be written with a comma separating each value.
///
/// Endpoints require parameters with list values to either be written with
/// comma-separated value query parameters or
/// [repeating parameters][`RepeatingQueryParameter`]; they can't be used
/// interchangeably, and which is used is particular to each endpoint and is
/// perhaps particular to the specific parameter.
///
/// # Examples
///
/// ```rust
/// use twilight_http::request::path::{
///     query_parameter::{CsvQueryParameter, KeyValueQueryParameter},
///     Path,
/// };
/// use twilight_model::id::{marker::ChannelMarker, Id};
///
/// let channel_ids: &[Id<ChannelMarker>] = &[Id::new(1), Id::new(2), Id::new(3)];
///
/// let path = Path::builder()
///     .resource("example")
///     .parameter(KeyValueQueryParameter::new("key", "value"))
///     .parameter(CsvQueryParameter::new("channel_ids", channel_ids))
///     .build();
///
/// assert_eq!("example?key=value&channel_ids=1,2,3", path);
/// ```
pub struct CsvQueryParameter<'a, T> {
    key: &'a str,
    value: &'a [T],
}

impl<'a, T: QueryParameterValue> CsvQueryParameter<'a, T> {
    pub fn new(key: &'a str, value: &'a [T]) -> Self {
        Self { key, value }
    }
}

impl<'a, T: QueryParameterValue> QueryParameter for CsvQueryParameter<'a, T> {
    fn will_write(&self) -> bool {
        !self.value.is_empty()
    }

    fn write<'b>(&self, mut writer: QueryParameterWriter<'_, '_>) -> fmt::Result {
        struct CsvValueDisplay<'a, T>(&'a [T]);

        impl<T: QueryParameterValue> QueryParameterValue for CsvValueDisplay<'_, T> {
            fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
                const COMMA: char = ',';

                for (index, item) in self.0.iter().enumerate() {
                    if index > 0 {
                        f.write_char(COMMA)?;
                    }

                    T::fmt(item, f)?;
                }

                Ok(())
            }
        }

        writer.write_pair(self.key, CsvValueDisplay(self.value))
    }
}

/// Write a query parameter with a repeated key for each value in a list.
///
/// Endpoints require parameters with list values to either be written with
/// repeating query parameters or
/// [comma-separated value parameters][`CsvQueryparameter`]; they can't be used
/// interchangeably, and which is used is particular to each endpoint and is
/// perhaps particular to the specific parameter.
///
/// # Examples
///
/// ```rust
/// use twilight_http::request::path::{
///     query_parameter::{KeyValueQueryParameter, RepeatingQueryParameter},
///     Path,
/// };
/// use twilight_model::id::{marker::ChannelMarker, Id};
///
/// let channel_ids: &[Id<ChannelMarker>] = &[Id::new(1), Id::new(2), Id::new(3)];
///
/// let path = Path::builder()
///     .resource("example")
///     .parameter(KeyValueQueryParameter::new("key", "value"))
///     .parameter(RepeatingQueryParameter::new("channel_id", channel_ids))
///     .build();
///
/// assert_eq!("example?key=value&channel_id=1&channel_id=2&channel_id=3", path);
/// ```
pub struct RepeatingQueryParameter<'a, T> {
    key: &'a str,
    value: &'a [T],
}

impl<'a, T: QueryParameterValue> RepeatingQueryParameter<'a, T> {
    pub fn new(key: &'a str, value: &'a [T]) -> Self {
        Self { key, value }
    }
}

impl<'a, T: QueryParameterValue> QueryParameter for RepeatingQueryParameter<'a, T> {
    fn will_write(&self) -> bool {
        !self.value.is_empty()
    }

    fn write<'b>(&self, mut writer: QueryParameterWriter<'_, '_>) -> fmt::Result {
        for value in self.value.iter() {
            writer.write_pair(self.key, value)?;
        }

        Ok(())
    }
}

pub trait QueryParameterValue {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result;

    fn will_write(&self) -> bool {
        true
    }
}

impl QueryParameterValue for &str {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        Display::fmt(
            &percent_encoding::utf8_percent_encode(self, NON_ALPHANUMERIC),
            f,
        )
    }

    fn will_write(&self) -> bool {
        !self.is_empty()
    }
}

impl QueryParameterValue for i8 {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        Display::fmt(self, f)
    }
}

impl QueryParameterValue for i16 {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        Display::fmt(self, f)
    }
}

impl QueryParameterValue for i32 {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        Display::fmt(self, f)
    }
}

impl QueryParameterValue for i64 {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        Display::fmt(self, f)
    }
}

impl QueryParameterValue for isize {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        Display::fmt(self, f)
    }
}

impl QueryParameterValue for u8 {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        Display::fmt(self, f)
    }
}

impl QueryParameterValue for u16 {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        Display::fmt(self, f)
    }
}

impl QueryParameterValue for u32 {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        Display::fmt(self, f)
    }
}

impl QueryParameterValue for u64 {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        Display::fmt(self, f)
    }
}

impl QueryParameterValue for usize {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        Display::fmt(self, f)
    }
}

impl QueryParameterValue for bool {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        Display::fmt(self, f)
    }
}

impl<T> QueryParameterValue for Id<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        Display::fmt(self, f)
    }
}

impl<T: QueryParameterValue> QueryParameterValue for Option<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        if let Some(value) = self {
            T::fmt(value, f)?;
        }

        Ok(())
    }

    fn will_write(&self) -> bool {
        self.as_ref().is_some_and(|inner| inner.will_write())
    }
}

impl<T: QueryParameterValue> QueryParameterValue for &'_ T {
    fn will_write(&self) -> bool {
        T::will_write(*self)
    }

    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        T::fmt(self, f)
    }
}

#[cfg(test)]
mod tests {
    use super::{
        CsvQueryParameter, QueryParameter, QueryParameterValue, QueryParameterWriter,
        RepeatingQueryParameter,
    };
    use std::fmt::{Display, Error as FmtError, Formatter};
    use twilight_model::id::{
        Id,
        marker::{ChannelMarker, GuildMarker, UserMarker},
    };

    struct QueryParameterDisplay<T> {
        has_query_parameter: bool,
        query_parameter: T,
    }

    impl<T: QueryParameter> QueryParameterDisplay<T> {
        fn new(query_parameter: T, has_query_parameter: bool) -> Self {
            Self {
                has_query_parameter,
                query_parameter,
            }
        }
    }

    impl<T: QueryParameter> Display for QueryParameterDisplay<T> {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), FmtError> {
            T::write(
                &self.query_parameter,
                QueryParameterWriter::new(f, self.has_query_parameter),
            )
        }
    }

    struct QueryParameterValueDisplay<T>(T);

    impl<T: QueryParameterValue> Display for QueryParameterValueDisplay<T> {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), FmtError> {
            T::fmt(&self.0, f)
        }
    }

    #[test]
    fn query_parameter_key_value_tuple() {
        assert_eq!(
            "?key=value",
            QueryParameterDisplay::new(("key", "value"), false).to_string()
        );
        assert_eq!(
            "&key=value",
            QueryParameterDisplay::new(("key", "value"), true).to_string()
        );
    }

    #[test]
    fn query_parameter_csv_parameter() {
        let user_ids = &[Id::<UserMarker>::new(1), Id::new(2), Id::new(3)];

        assert_eq!(
            "?user_ids=1,2,3",
            QueryParameterDisplay::new(CsvQueryParameter::new("user_ids", user_ids), false)
                .to_string()
        );
        assert_eq!(
            "&user_ids=1,2,3",
            QueryParameterDisplay::new(CsvQueryParameter::new("user_ids", user_ids), true)
                .to_string()
        );
    }

    #[test]
    fn query_parameter_repeating_parameter() {
        let channel_ids = &[Id::<ChannelMarker>::new(1), Id::new(2), Id::new(3)];

        assert_eq!(
            "?channel_id=1&channel_id=2&channel_id=3",
            QueryParameterDisplay::new(
                RepeatingQueryParameter::new("channel_id", channel_ids),
                false
            )
            .to_string()
        );
        assert_eq!(
            "&channel_id=1&channel_id=2&channel_id=3",
            QueryParameterDisplay::new(
                RepeatingQueryParameter::new("channel_id", channel_ids),
                true
            )
            .to_string()
        );
    }

    #[test]
    fn query_parameter_value_format_bool() {
        assert_eq!("true", QueryParameterValueDisplay(true).to_string());
        assert_eq!("false", QueryParameterValueDisplay(false).to_string());
    }

    #[test]
    fn query_parameter_value_str_should_write() {
        assert!(
            <&str as QueryParameterValue>::will_write(&"test"),
            "non-empty strings should write"
        );
        assert!(
            !<&str as QueryParameterValue>::will_write(&""),
            "empty strings should not write"
        );
    }

    #[test]
    fn query_parameter_value_str_format() {
        assert_eq!("1", QueryParameterValueDisplay("1").to_string());
        assert_eq!("foo", QueryParameterValueDisplay("foo").to_string());
    }

    #[test]
    fn query_parameter_value_u8() {
        assert!(QueryParameterValue::will_write(&0u8));
        assert!(QueryParameterValue::will_write(&0u8));
        assert_eq!("0", QueryParameterValueDisplay(0u8).to_string());
        assert_eq!("12", QueryParameterValueDisplay(12u8).to_string());
        assert_eq!("123", QueryParameterValueDisplay(123u8).to_string());
    }

    #[test]
    fn query_parameter_value_u16() {
        assert!(QueryParameterValue::will_write(&0u16));
        assert_eq!("0", QueryParameterValueDisplay(0u16).to_string());
        assert_eq!("12", QueryParameterValueDisplay(12u16).to_string());
        assert_eq!("123", QueryParameterValueDisplay(123u16).to_string());
    }

    #[test]
    fn query_parameter_value_u32() {
        assert!(QueryParameterValue::will_write(&0u32));
        assert_eq!("0", QueryParameterValueDisplay(0u32).to_string());
        assert_eq!("12", QueryParameterValueDisplay(12u32).to_string());
        assert_eq!("123", QueryParameterValueDisplay(123u32).to_string());
    }

    #[test]
    fn query_parameter_value_u64() {
        assert!(QueryParameterValue::will_write(&0u64));
        assert_eq!("0", QueryParameterValueDisplay(0u64).to_string());
        assert_eq!("12", QueryParameterValueDisplay(12u64).to_string());
        assert_eq!("123", QueryParameterValueDisplay(123u64).to_string());
    }

    #[test]
    fn query_parameter_value_usize() {
        assert!(QueryParameterValue::will_write(&0usize));
        assert_eq!("0", QueryParameterValueDisplay(0usize).to_string());
        assert_eq!("12", QueryParameterValueDisplay(12usize).to_string());
        assert_eq!("123", QueryParameterValueDisplay(123usize).to_string());
    }

    #[test]
    fn query_parameter_value_i8() {
        assert!(QueryParameterValue::will_write(&0i8));
        assert_eq!("-123", QueryParameterValueDisplay(-123i8).to_string());
        assert_eq!("-12", QueryParameterValueDisplay(-12i8).to_string());
        assert_eq!("-1", QueryParameterValueDisplay(-1i8).to_string());
        assert_eq!("0", QueryParameterValueDisplay(0i8).to_string());
        assert_eq!("12", QueryParameterValueDisplay(12i8).to_string());
        assert_eq!("123", QueryParameterValueDisplay(123i8).to_string());
    }

    #[test]
    fn query_parameter_value_i16() {
        assert!(QueryParameterValue::will_write(&0i16));
        assert_eq!("-123", QueryParameterValueDisplay(-123i16).to_string());
        assert_eq!("-12", QueryParameterValueDisplay(-12i16).to_string());
        assert_eq!("-1", QueryParameterValueDisplay(-1i16).to_string());
        assert_eq!("0", QueryParameterValueDisplay(0i16).to_string());
        assert_eq!("12", QueryParameterValueDisplay(12i16).to_string());
        assert_eq!("123", QueryParameterValueDisplay(123i16).to_string());
    }

    #[test]
    fn query_parameter_value_i32() {
        assert!(QueryParameterValue::will_write(&0i32));
        assert_eq!("-123", QueryParameterValueDisplay(-123i32).to_string());
        assert_eq!("-12", QueryParameterValueDisplay(-12i32).to_string());
        assert_eq!("-1", QueryParameterValueDisplay(-1i32).to_string());
        assert_eq!("0", QueryParameterValueDisplay(0i32).to_string());
        assert_eq!("12", QueryParameterValueDisplay(12i32).to_string());
        assert_eq!("123", QueryParameterValueDisplay(123i32).to_string());
    }

    #[test]
    fn query_parameter_value_i64() {
        assert!(QueryParameterValue::will_write(&0i64));
        assert_eq!("-123", QueryParameterValueDisplay(-123i64).to_string());
        assert_eq!("-12", QueryParameterValueDisplay(-12i64).to_string());
        assert_eq!("-1", QueryParameterValueDisplay(-1i64).to_string());
        assert_eq!("0", QueryParameterValueDisplay(0i64).to_string());
        assert_eq!("12", QueryParameterValueDisplay(12i64).to_string());
        assert_eq!("123", QueryParameterValueDisplay(123i64).to_string());
    }

    #[test]
    fn query_parameter_value_isize() {
        assert!(QueryParameterValue::will_write(&0isize));
        assert_eq!("-123", QueryParameterValueDisplay(-123isize).to_string());
        assert_eq!("-12", QueryParameterValueDisplay(-12isize).to_string());
        assert_eq!("-1", QueryParameterValueDisplay(-1isize).to_string());
        assert_eq!("0", QueryParameterValueDisplay(0isize).to_string());
        assert_eq!("12", QueryParameterValueDisplay(12isize).to_string());
        assert_eq!("123", QueryParameterValueDisplay(123isize).to_string());
    }

    #[test]
    fn query_parameter_value_id() {
        assert!(QueryParameterValue::will_write(&Id::<GuildMarker>::new(1)));
        assert_eq!(
            "1234567890123456789",
            QueryParameterValueDisplay(Id::<GuildMarker>::new(1234567890123456789)).to_string()
        );
    }

    #[test]
    fn query_parameter_value_option() {
        assert!(QueryParameterValue::will_write(&Some("test")));
        assert!(!QueryParameterValue::will_write(&Some("")));
        assert!(!QueryParameterValue::will_write(&None::<&str>));
        assert_eq!("test", QueryParameterValueDisplay(Some("test")).to_string());
    }

    #[test]
    fn query_parameter_value_reference() {
        assert!(QueryParameterValue::will_write(&Some(&"test")));
        assert!(!QueryParameterValue::will_write(&Some(&"")));
        assert!(!QueryParameterValue::will_write(&None::<&&str>));
        assert_eq!(
            "test",
            QueryParameterValueDisplay(Some(&"test")).to_string()
        );
    }
}
