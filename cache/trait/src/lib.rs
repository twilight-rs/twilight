use async_trait::async_trait;
use std::fmt::Debug;

pub trait Cache: Debug + Send + Sync {}

#[async_trait]
pub trait UpdateCache<T: Cache, Err> {
    async fn update(&self, item: &T) -> Result<(), Err>;
}

#[cfg(test)]
mod tests {
    use super::{Cache, UpdateCache};
    use static_assertions::assert_impl_all;
    use std::fmt::Debug;

    #[test]
    fn test_cache_bounds() {
        static_assertions::assert_obj_safe!(Cache);
        assert_impl_all!(dyn Cache: Debug, Send, Sync);
    }

    #[test]
    fn test_cache_update_bounds() {
        static_assertions::assert_obj_safe!(UpdateCache<(), ()>);
    }
}
