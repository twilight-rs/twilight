use std::fmt::{Display, Formatter, Write};

/// A helper struct to write query parameters to a formatter.
pub struct QueryStringFormatter<'w1, 'w2> {
    formatter: &'w1 mut Formatter<'w2>,
    is_first: bool,
}

impl<'w1, 'w2> QueryStringFormatter<'w1, 'w2> {
    pub const fn new(formatter: &'w1 mut Formatter<'w2>) -> Self {
        Self {
            formatter,
            is_first: true,
        }
    }

    /// Writes a query parameter to the formatter.
    ///
    /// # Errors
    ///
    /// This returns a [`std::fmt::Error`] if the formatter returns an error.
    pub fn write_param(&mut self, key: &str, value: &impl Display) -> std::fmt::Result {
        if self.is_first {
            self.formatter.write_char('?')?;
            self.is_first = false;
        } else {
            self.formatter.write_char('&')?;
        }

        self.formatter.write_str(key)?;
        self.formatter.write_char('=')?;
        Display::fmt(value, self.formatter)
    }

    /// Writes a repeating query parameter to the formatter.
    ///
    /// The formatted query parameter will be in the format of "?foo=1&foo=2".
    /// For CSV parameter values ("?foo=1,2") use [`QueryCsvArray`].
    ///
    /// # Errors
    ///
    /// This returns a [`std::fmt::Error`] if the formatter returns an error.
    pub fn write_repeating_param<T: Display>(
        &mut self,
        key: &str,
        values: impl IntoIterator<Item = T>,
    ) -> std::fmt::Result {
        for value in values {
            self.write_param(key, &value)?;
        }

        Ok(())
    }

    /// Writes a query parameter to the formatter.
    ///
    /// # Errors
    ///
    /// This returns a [`std::fmt::Error`] if the formatter returns an error.
    pub fn write_opt_param(&mut self, key: &str, value: Option<&impl Display>) -> std::fmt::Result {
        if let Some(value) = value {
            self.write_param(key, value)
        } else {
            Ok(())
        }
    }
}

/// Provides a display implementation for serializing iterable objects into a
/// query parameter with a comma-separated value.
///
/// For repeating values query parameters (`?foo=1&foo=2`) use
/// [`QueryFormatter::write_repeating_param`].
#[derive(Debug)]
pub struct QueryCsvArray<T>(pub T);

impl<T, U> Display for QueryCsvArray<T>
where
    T: IntoIterator<Item = U> + Clone,
    U: Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut iter = self.0.clone().into_iter().peekable();

        while let Some(item) = iter.next() {
            Display::fmt(&item, f)?;
            if iter.peek().is_some() {
                f.write_str(",")?;
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct Test {
        a: Option<u32>,
        b: Option<String>,
    }

    impl Display for Test {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            let mut writer = QueryStringFormatter::new(f);
            writer.write_opt_param("a", self.a.as_ref())?;
            writer.write_opt_param("b", self.b.as_ref())
        }
    }

    #[test]
    fn test_query_string_formatter_filled() {
        let test = Test {
            a: Some(1),
            b: Some("hello".to_string()),
        };

        assert_eq!(test.to_string(), "?a=1&b=hello");
    }

    #[test]
    fn test_query_string_formatter_empty() {
        let test = Test { a: None, b: None };

        assert_eq!(test.to_string(), "");
    }

    #[test]
    fn test_query_string_formatter_single() {
        let test = Test {
            a: Some(1),
            b: None,
        };

        assert_eq!(test.to_string(), "?a=1");
    }

    #[test]
    fn test_query_csv_array() {
        let query_array = QueryCsvArray([1, 2, 3]);
        assert_eq!(query_array.to_string(), "1,2,3");

        let params = vec!["a", "b", "c"];
        let query_array = QueryCsvArray(&params);
        assert_eq!(query_array.to_string(), "a,b,c");
    }

    #[test]
    fn test_query_string_formatter_repeating() {
        struct Repeating;

        impl Display for Repeating {
            fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
                let mut writer = QueryStringFormatter::new(f);
                writer.write_repeating_param("foo", [1, 2, 3])?;
                writer.write_repeating_param("bar", ["baz", "qux"])?;

                Ok(())
            }
        }

        assert_eq!("?foo=1&foo=2&foo=3&bar=baz&bar=qux", Repeating.to_string());
    }
}
