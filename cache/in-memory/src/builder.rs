use super::{
    config::{Config, EventType},
    InMemoryCache,
};

/// Builder to configure and construct an [`InMemoryCache`].
///
/// [`InMemoryCache`]: struct.InMemoryCache.html
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct InMemoryCacheBuilder(Config);

impl InMemoryCacheBuilder {
    /// Creates a builder to configure and construct an [`InMemoryCache`].
    ///
    /// [`InMemoryCache`]: struct.InMemoryCache.html
    pub fn new() -> Self {
        Self::default()
    }

    /// Consume the builder, returning a configured cache.
    pub fn build(self) -> InMemoryCache {
        InMemoryCache::new_with_config(self.0)
    }

    /// Sets the list of event types for the cache to handle.
    ///
    /// Defaults to all types.
    pub fn event_types(mut self, event_types: EventType) -> Self {
        self.0.event_types = event_types;

        self
    }

    /// Sets the number of messages to cache per channel.
    ///
    /// Defaults to 100.
    pub fn message_cache_size(mut self, message_cache_size: usize) -> Self {
        self.0.message_cache_size = message_cache_size;

        self
    }
}

#[cfg(test)]
mod tests {
    use super::InMemoryCacheBuilder;
    use static_assertions::assert_impl_all;
    use std::fmt::Debug;

    assert_impl_all!(InMemoryCacheBuilder: Clone, Debug, Default, Send, Sync);
}
