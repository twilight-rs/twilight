use std::fmt::Display;

/// Provides a display implementation for serializing arrays-like objects into
/// query params.
pub struct QueryArray<T>(pub T);

impl<T, U> Display for QueryArray<T>
where
    T: IntoIterator<Item = U> + Copy,
    U: Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut iter = self.0.into_iter().peekable();

        while let Some(item) = iter.next() {
            item.fmt(f)?;
            if iter.peek().is_some() {
                f.write_str(",")?;
            }
        }

        Ok(())
    }
}

impl<T, U> From<QueryArray<T>> for String
where
    T: IntoIterator<Item = U> + Copy,
    U: Display,
{
    fn from(val: QueryArray<T>) -> Self {
        val.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::QueryArray;

    #[test]
    fn test_query_array() {
        let query_array = QueryArray([1, 2, 3]);
        assert_eq!(query_array.to_string(), "1,2,3");

        let params = vec!["a", "b", "c"];
        let query_array = QueryArray(&params);
        assert_eq!(query_array.to_string(), "a,b,c");
    }
}
