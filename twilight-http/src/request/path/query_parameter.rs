//! Utilities for writing key-value query parameter pairs within an HTTP path.
//!
//! Query parameters are described by the [`QueryParameter`] trait and are
//! composed of two elements: the key and value. The key must be a string, and
//! so we represent this as a string slice. The value can be any type which
//! implements the [`QueryParameterValue`] formatter.
//!
//! Users will likely not need to implement their own [`QueryParameter`]
//! implementations when manually building paths and requests, but may find a
//! need to implement [`QueryParameterValue`] for novel data types. Refer to its
//! documentation for how to implement the trait.
//!
//! Three query parameter implementations are provided:
//!
//! - [`KeyValueQueryParameter`], which writes a single key-value pair;
//! - [`CsvQueryParameter`], which writes a single key with its values
//!   concatenated in a comma-separated value list; and
//! - [`RepeatingQueryParameter`], which writes a key and value for every entry
//!   in a list of values.
//!
//! # Examples
//!
//! Write a key-value and repeating query parameter:
//!
//! ```rust
//! # fn main() -> Result<(), Box<dyn std::error::Error>> {
//! use twilight_http::request::path::{
//!     Path,
//!     query_parameter::{CsvQueryParameter, KeyValueQueryParameter, RepeatingQueryParameter},
//! };
//! use twilight_model::id::{
//!     Id,
//!     marker::{ChannelMarker, GuildMarker},
//! };
//!
//! let channel_ids: &[Id<ChannelMarker>] = &[Id::new(123), Id::new(456)];
//! let guild_id = Id::<GuildMarker>::new(789);
//!
//! let path = Path::builder()
//!     .resource("guilds")
//!     .id(guild_id)
//!     .resource("messages")
//!     .action("search")
//!     .parameter(KeyValueQueryParameter::new("limit", 25u8))
//!     .parameter(CsvQueryParameter::new("channel_id", channel_ids))
//!     .build()?;
//!
//! assert_eq!(
//!     "guilds/789/messages/search?limit=25&channel_id=123,456",
//!     path
//! );
//! # Ok(()) }
//! ```

use percent_encoding::NON_ALPHANUMERIC;
use std::fmt::{self, Display, Formatter, Write as _};
use twilight_model::{channel::message::ReactionType, id::Id, util::Timestamp};

/// Writer [`QueryParameter`] implementations use to write one or more key-value
/// pairs to a query string.
///
/// The writer is a necessary construct because it has knowledge about whether
/// a query string has already been written, i.e. whether to begin appending
/// query pairs to a query string or initialize a new query string.
///
/// It has only one method: [`write_pair`][Self::write_pair], which **must** be
/// called **at least once** in a [`QueryParameter::write`] implementation up to
/// an unlimited number of times.
pub struct QueryParameterWriter<'a, 'b> {
    /// Whether a query parameter has been written yet.
    has_query_parameter: bool,
    /// Inner formatter.
    inner: &'a mut Formatter<'b>,
}

impl<'a, 'b> QueryParameterWriter<'a, 'b> {
    /// Create a new writer to provide to [`QueryParameter`]s.
    pub(super) const fn new(formatter: &'a mut Formatter<'b>, has_query_parameter: bool) -> Self {
        Self {
            has_query_parameter,
            inner: formatter,
        }
    }

    /// Write a key-value pair to the query parameter list.
    ///
    /// The same key can be written multiple times, but when doing so the value
    /// **should** be distinct.
    ///
    /// # Errors
    ///
    /// Returns a formatting error if the provided value could not be formatted.
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
        // specification and accepted by Discord at time of writing.
        // "?&foo=bar", "?foo&qux=quz", "?foo=&qux=quz", and
        // "?foo=bar&qux=quz" are all valid key-value pairs and keys with empty
        // or undefined values (in this example, "foo") will be ignored.
        //
        // And, even if it weren't accepted by Discord, there's not much we can
        // do here to "walk back" writing one portion if a latter portion fails
        // to write. Let's call this a compromise and leave everyone unsatisfied
        // with the potential subpar query string formatting.
        self.has_query_parameter = true;
        self.inner.write_str(key)?;
        self.inner.write_char(EQUALS_SIGN)?;
        T::fmt(&value, self.inner)?;

        Ok(())
    }
}

/// Formatter for a key and its value.
///
/// Each query parameter key name should be described once as a single query
/// parameter instance when building a [Path]. Each query parameter key can
/// write multiple times, which is necessary for writing query parameters with
/// multiple values.
///
/// # Provided Implementations
///
/// The primary implementation is [`KeyValueQueryParameter`], which accepts a
/// single key and a single value. It writes once, producing a query parameter
/// such as `foo=bar`. Refer to its documentation for more information and to
/// learn its shorthand syntax.
///
/// Some query parameter values are accepted by Discord in the form of a
/// Comma-Separated Value list. The [`CsvQueryParameter`] implementation serves
/// this use case, accepting a key and a list of values. It produces a query
/// parameter such as `foo=1,2,3`.
///
/// Other query parameters with list values accepted by Discord are in the form
/// of repeating query parameters, served by [`RepeatingQueryParameter`]. It has
/// one instance of a key for each of its values. It produces a query parameter
/// such as `foo=1&foo=2&foo=3`.
///
/// # Implementing Your Own Query Parameter
///
/// Although the provided implementations should serve all your needs you may
/// wish to use your own implementation if you're writing your own request
/// builders.
///
/// ```rust
/// use std::fmt;
/// use twilight_http::request::path::query_parameter::{QueryParameter, QueryParameterWriter};
/// use twilight_model::id::Id;
///
/// struct OnlyEvenIdsRepeatingQueryParameter<'a, T> {
///     key: &'a str,
///     values: &'a [Id<T>],
/// }
///
/// impl<T> QueryParameter for OnlyEvenIdsRepeatingQueryParameter<'_, T> {
///     fn will_write(&self) -> bool {
///         self.values.iter().any(|id| id.get() % 2 == 0)
///     }
///
///     fn write(&self, mut writer: QueryParameterWriter<'_, '_>) -> fmt::Result {
///         for value in self.values {
///             if value.get() % 2 == 0 {
///                 writer.write_pair(self.key, value)?;
///             }
///         }
///
///         Ok(())
///     }
/// }
/// ```
///
/// [Path]: super::Path
pub trait QueryParameter {
    /// Whether at least one key-value query parameter will be written.
    ///
    /// [`CsvQueryParameter`] accepts a single key and a list of values. If the
    /// list has at least one value, then this would return true; otherwise,
    /// there is nothing to write, and so it would return false.
    fn will_write(&self) -> bool;

    /// Write the query parameter key and value to the writer.
    ///
    /// # Errors
    ///
    /// Returns a formatting error if the provided value could not be formatted.
    fn write(&self, writer: QueryParameterWriter<'_, '_>) -> fmt::Result;
}

/// Key-value pair to be written in the query of a path.
///
/// Serializes as `key=value`.
impl<T: QueryParameterValue> QueryParameter for (&str, T) {
    fn will_write(&self) -> bool {
        T::will_write(&self.1)
    }

    fn write(&self, mut writer: QueryParameterWriter<'_, '_>) -> fmt::Result {
        writer.write_pair(self.0, &self.1)
    }
}

/// Key-value pair to be written in the query of a path.
///
/// Serializes as `key=value`.
///
/// # Shortform
///
/// There is a shortform of this: a tuple composed of a string slice and a value
/// (`(&str, T)`). The two implementations are identical.
///
/// # Examples
///
/// ```rust
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
/// use twilight_http::request::path::{Path, query_parameter::KeyValueQueryParameter};
/// use twilight_model::id::{Id, marker::GuildMarker};
///
/// let path = Path::builder()
///     .resource("guilds")
///     .id(Id::<GuildMarker>::new(123))
///     .resource("members")
///     .action("search")
///     // longform
///     .parameter(KeyValueQueryParameter::new("query", "foo"))
///     // shortform
///     .parameter(("limit", 25u8))
///     .build()?;
///
/// assert_eq!("guilds/123/members/search?query=foo&limit=25", path);
/// # Ok(()) }
/// ```
pub struct KeyValueQueryParameter<'a, T> {
    /// Representation of the key and value pair which contains the delegated
    /// implementations.
    inner: (&'a str, T),
}

impl<'a, T: QueryParameterValue> KeyValueQueryParameter<'a, T> {
    /// Create a new key-value query parameter.
    pub const fn new(key: &'a str, value: T) -> Self {
        Self {
            inner: (key, value),
        }
    }
}

impl<T: QueryParameterValue> QueryParameter for KeyValueQueryParameter<'_, T> {
    fn will_write(&self) -> bool {
        QueryParameter::will_write(&self.inner)
    }

    fn write<'b>(&self, writer: QueryParameterWriter<'_, '_>) -> fmt::Result {
        QueryParameter::write(&self.inner, writer)
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
///     Path,
///     query_parameter::{CsvQueryParameter, KeyValueQueryParameter},
/// };
/// use twilight_model::id::{Id, marker::ChannelMarker};
///
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
/// let channel_ids: &[Id<ChannelMarker>] = &[Id::new(1), Id::new(2), Id::new(3)];
///
/// let path = Path::builder()
///     .resource("example")
///     .parameter(KeyValueQueryParameter::new("key", "value"))
///     .parameter(CsvQueryParameter::new("channel_ids", channel_ids))
///     .build()?;
///
/// assert_eq!("example?key=value&channel_ids=1,2,3", path);
/// # Ok(()) }
/// ```
pub struct CsvQueryParameter<'a, T> {
    /// Key to be written in the key-value pair.
    key: &'a str,
    /// Values to be concatenated into a comma-separated value list and written
    /// as a single value in a key-value pair.
    value: &'a [T],
}

impl<'a, T: QueryParameterValue> CsvQueryParameter<'a, T> {
    /// Create a new query parameter with a list value written in CSV form.
    pub const fn new(key: &'a str, value: &'a [T]) -> Self {
        Self { key, value }
    }
}

impl<T: QueryParameterValue> QueryParameter for CsvQueryParameter<'_, T> {
    fn will_write(&self) -> bool {
        self.value.iter().any(T::will_write)
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
/// [comma-separated value parameters][`CsvQueryParameter`]; they can't be used
/// interchangeably, and which is used is particular to each endpoint and is
/// perhaps particular to the specific parameter.
///
/// # Examples
///
/// ```rust
/// use twilight_http::request::path::{
///     Path,
///     query_parameter::{KeyValueQueryParameter, RepeatingQueryParameter},
/// };
/// use twilight_model::id::{Id, marker::ChannelMarker};
///
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
/// let channel_ids: &[Id<ChannelMarker>] = &[Id::new(1), Id::new(2), Id::new(3)];
///
/// let path = Path::builder()
///     .resource("example")
///     .parameter(KeyValueQueryParameter::new("key", "value"))
///     .parameter(RepeatingQueryParameter::new("channel_id", channel_ids))
///     .build()?;
///
/// assert_eq!(
///     "example?key=value&channel_id=1&channel_id=2&channel_id=3",
///     path
/// );
/// # Ok(()) }
/// ```
pub struct RepeatingQueryParameter<'a, T> {
    /// Key to write once for every value.
    key: &'a str,
    /// List of values to write in a key-value pair with [`key`][Self::key] for
    /// every value in the list.
    ///
    /// Given a key of "foo" and a value of `&[1, 2, 3]`, the following
    /// key-value pairs will be written:
    ///
    /// - "foo=1"
    /// - "foo=2"
    /// - "foo=3"
    value: &'a [T],
}

impl<'a, T: QueryParameterValue> RepeatingQueryParameter<'a, T> {
    /// Create a new query parameter with a key-value pair written for every
    /// value in a list.
    pub const fn new(key: &'a str, value: &'a [T]) -> Self {
        Self { key, value }
    }
}

impl<T: QueryParameterValue> QueryParameter for RepeatingQueryParameter<'_, T> {
    fn will_write(&self) -> bool {
        self.value.iter().any(T::will_write)
    }

    /// Write the key and values to the query parameter writer, writing the key
    /// once per value.
    ///
    /// # Errors
    ///
    /// Returns a formatting error if the provided value could not be formatted.
    fn write(&self, mut writer: QueryParameterWriter<'_, '_>) -> fmt::Result {
        for value in self.value {
            writer.write_pair(self.key, value)?;
        }

        Ok(())
    }
}

/// Value formatter within a key-value pair.
pub trait QueryParameterValue {
    /// Write the Display representation of the value to the formatter.
    ///
    /// # Errors
    ///
    /// Returns a formatting error if the provided value could not be formatted.
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result;

    /// Whether a value will be written.
    ///
    /// This *must* be accurate; otherwise, a key will be written with an empty
    /// value, resulting in a potential query string such as `"?foo=&bar=1"`.
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

impl QueryParameterValue for ReactionType {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        Display::fmt(&u8::from(*self), f)
    }
}

impl QueryParameterValue for Timestamp {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        Display::fmt(&self.iso_8601(), f)
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
        self.as_ref().is_some_and(T::will_write)
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
    fn csv_query_parameter_write() {
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

    /// Assert that a `CsvQueryParameter`'s `will_write` is true if at
    /// least one value asserts it will write.
    #[test]
    fn csv_query_parameter_will_write_if_at_least_one_value_will() {
        let repeating = CsvQueryParameter::new("key", &[Some(1u64), None]);

        assert!(repeating.will_write())
    }

    /// Assert that a `CsvQueryParameter`'s `will_write` is false if it
    /// has multiple values but none of the values will write.
    #[test]
    fn csv_query_parameter_will_not_write_if_no_values_will() {
        let repeating = CsvQueryParameter::new("key", &[None::<u8>, None]);

        assert!(!repeating.will_write())
    }

    /// Assert that a `CsvQueryParameter` will not write if it has no values.
    #[test]
    fn csv_query_parameter_will_not_write_if_empty() {
        let repeating = CsvQueryParameter::new("key", &[] as &[u8]);

        assert!(!repeating.will_write())
    }

    #[test]
    fn repeating_query_parameter_write() {
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

    /// Assert that a `RepeatingQueryParameter`'s `will_write` is true if at
    /// least one value asserts it will write.
    #[test]
    fn repeating_query_parameter_will_write_if_at_least_one_value_will() {
        let repeating = RepeatingQueryParameter::new("key", &[Some(1u64), None]);

        assert!(repeating.will_write())
    }

    /// Assert that a `RepeatingQueryParameter`'s `will_write` is false if it
    /// has multiple values but none of the values will write.
    #[test]
    fn repeating_query_parameter_will_not_write_if_no_values_will() {
        let repeating = RepeatingQueryParameter::new("key", &[None::<u8>, None]);

        assert!(!repeating.will_write())
    }

    /// Assert that a `RepeatingQueryParameter` will not write if it has no values.
    #[test]
    fn repeating_query_parameter_will_not_write_if_empty() {
        let repeating = RepeatingQueryParameter::new("key", &[] as &[u8]);

        assert!(!repeating.will_write())
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
            QueryParameterValueDisplay(Id::<GuildMarker>::new(1_234_567_890_123_456_789))
                .to_string()
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
