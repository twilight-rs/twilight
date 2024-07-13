use std::fmt::{Display, Formatter, Write};

/// A helper struct to write query paramseters to a formatter.
pub struct QueryStringFormatter<'w1, 'w2> {
    formatter: &'w1 mut Formatter<'w2>,
    is_first: bool,
}

impl<'w1, 'w2> QueryStringFormatter<'w1, 'w2> {
    pub fn new(formatter: &'w1 mut Formatter<'w2>) -> Self {
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

/// Provides a display implementation for serializing iterable objects into
/// query params.
#[derive(Debug)]
pub struct QueryArray<T>(pub T);

impl<T, U> Display for QueryArray<T>
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
    fn test_query_array() {
        let query_array = QueryArray([1, 2, 3]);
        assert_eq!(query_array.to_string(), "1,2,3");

        let params = vec!["a", "b", "c"];
        let query_array = QueryArray(&params);
        assert_eq!(query_array.to_string(), "a,b,c");
    }
}
