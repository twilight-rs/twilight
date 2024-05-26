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

    pub fn write_opt_param(&mut self, key: &str, value: Option<&impl Display>) -> std::fmt::Result {
        if let Some(value) = value {
            self.write_param(key, value)
        } else {
            Ok(())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct Test {
        a: u32,
        b: String,
    }

    impl Display for Test {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            let mut writer = QueryStringFormatter::new(f);
            writer.write_param("a", &self.a)?;
            writer.write_param("b", &self.b)
        }
    }

    #[test]
    fn test_query_string_writer() {
        let test = Test {
            a: 1,
            b: "hello".to_string(),
        };

        let mut output = String::new();
        write!(output, "{}", test).unwrap();
        assert_eq!(output, "?a=1&b=hello");
    }
}
