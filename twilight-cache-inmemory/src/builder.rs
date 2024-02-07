use std::fmt::Debug;
use std::marker::PhantomData;

use crate::CacheableModels;

use super::{
    config::{Config, ResourceType},
    InMemoryCache,
};

/// Builder to configure and construct an [`InMemoryCache`].
#[allow(clippy::type_complexity)]
#[must_use = "has no effect if not built"]
#[derive(Debug)]
pub struct InMemoryCacheBuilder<CacheModels: CacheableModels>(Config, PhantomData<CacheModels>);

impl<CacheModels: CacheableModels> InMemoryCacheBuilder<CacheModels> {
    /// Creates a builder to configure and construct an [`InMemoryCache`].
    pub const fn new() -> Self {
        Self(Config::new(), PhantomData)
    }

    /// Consume the builder, returning a configured cache.
    #[allow(clippy::type_complexity)]
    pub fn build(self) -> InMemoryCache<CacheModels> {
        InMemoryCache::new_with_config(self.0)
    }

    /// Sets the list of resource types for the cache to handle.
    ///
    /// Defaults to all types.
    pub const fn resource_types(mut self, resource_types: ResourceType) -> Self {
        self.0.resource_types = resource_types;

        self
    }

    /// Sets the number of messages to cache per channel.
    ///
    /// Defaults to 100.
    pub const fn message_cache_size(mut self, message_cache_size: usize) -> Self {
        self.0.message_cache_size = message_cache_size;

        self
    }
}

impl<CacheModels: CacheableModels> Default for InMemoryCacheBuilder<CacheModels> {
    fn default() -> Self {
        Self(Config::default(), PhantomData)
    }
}

#[cfg(test)]
mod tests {
    use super::InMemoryCacheBuilder;
    use static_assertions::assert_impl_all;
    use std::fmt::Debug;

    assert_impl_all!(InMemoryCacheBuilder: Debug, Default, Send, Sync);
}
