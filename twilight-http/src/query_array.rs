use std::fmt::Display;

/// Provides a display implementation for serializing iterable objects into
/// query params.
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
