use percent_encoding::NON_ALPHANUMERIC;
use std::fmt::{self, Display, Formatter};
use twilight_model::id::Id;

pub struct QueryParameterDisplay<T: QueryParameter>(T);

impl<T: QueryParameter> QueryParameterDisplay<T> {
    pub fn new(query_parameter: T) -> Self {
        Self(query_parameter)
    }
}

impl<T: QueryParameter> Display for QueryParameterDisplay<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        T::fmt(&self.0, f)
    }
}

pub trait QueryParameter {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result;

    fn has_value(&self) -> bool {
        true
    }
}

impl QueryParameter for &str {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        Display::fmt(
            &percent_encoding::utf8_percent_encode(self, NON_ALPHANUMERIC),
            f,
        )
    }
}

impl QueryParameter for i8 {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        Display::fmt(self, f)
    }
}
impl QueryParameter for i16 {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        Display::fmt(self, f)
    }
}
impl QueryParameter for i32 {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        Display::fmt(self, f)
    }
}
impl QueryParameter for i64 {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        Display::fmt(self, f)
    }
}
impl QueryParameter for isize {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        Display::fmt(self, f)
    }
}
impl QueryParameter for u8 {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        Display::fmt(self, f)
    }
}
impl QueryParameter for u16 {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        Display::fmt(self, f)
    }
}
impl QueryParameter for u32 {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        Display::fmt(self, f)
    }
}
impl QueryParameter for u64 {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        Display::fmt(self, f)
    }
}
impl QueryParameter for usize {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        Display::fmt(self, f)
    }
}

impl QueryParameter for bool {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        Display::fmt(self, f)
    }
}

impl<T> QueryParameter for Id<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        Display::fmt(self, f)
    }
}

impl<T: QueryParameter> QueryParameter for Option<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        if let Some(value) = self {
            T::fmt(value, f)
        } else {
            Ok(())
        }
    }

    fn has_value(&self) -> bool {
        self.is_some()
    }
}

impl<T: QueryParameter> QueryParameter for &'_ [T] {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        for (index, item) in self.iter().enumerate() {
            if index > 0 {
                f.write_str(",")?;
            }

            T::fmt(item, f)?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::QueryParameterDisplay;

    #[test]
    fn format_u8() {
        assert_eq!("1", QueryParameterDisplay::new(1u8).to_string());
    }

    #[test]
    fn array_formats_as_csv() {
        let items = Vec::from([1u8, 2, 3]);

        assert_eq!("1,2,3", QueryParameterDisplay::new(&*items).to_string());
    }
}
