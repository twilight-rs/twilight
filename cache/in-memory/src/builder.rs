use super::{
    config::{Config, ResourceType},
    rule::Rule,
    InMemoryCache,
};

/// Builder to configure and construct an [`InMemoryCache`].
#[derive(Debug, Default)]
pub struct InMemoryCacheBuilder(Config);

impl InMemoryCacheBuilder {
    /// Creates a builder to configure and construct an [`InMemoryCache`].
    pub fn new() -> Self {
        Self(Config::new())
    }

    /// Consume the builder, returning a configured cache.
    pub fn build(self) -> InMemoryCache {
        InMemoryCache::new_with_config(self.0)
    }

    /// Sets the list of resource types for the cache to handle.
    ///
    /// Defaults to all types.
    pub const fn resource_types(mut self, resource_types: ResourceType) -> Self {
        self.0.resource_types = resource_types;

        self
    }

    /// Sets the list of rules associated with the cache.
    ///
    /// Defaults to none.
    ///
    /// # Examples
    ///
    /// Add a rule to the cache:
    ///
    /// ```
    /// use twilight_cache_inmemory::{config::{Entity, Rule}, InMemoryCache};
    ///
    /// #[derive(Clone, Debug, Eq, PartialEq)]
    /// pub struct HyphenatedRoleFilter;
    ///
    /// impl Rule for HyphenatedRoleFilter {
    ///     fn accept(&self, cache: &InMemoryCache, entity: &Entity) -> bool {
    ///         let role = if let Entity::Role(role) = entity {
    ///             role
    ///         } else {
    ///             return true;
    ///         };
    ///
    ///         role.name.starts_with('-')
    ///     }
    /// }
    ///
    /// let rules = Vec::from([Box::new(HyphenatedRoleFilter) as Box<dyn Rule>]);
    ///
    /// let cache = InMemoryCache::builder().rules(rules).build();
    /// assert_eq!(1, cache.config().rules().len());
    /// ```
    pub fn rules(mut self, rules: Vec<Box<dyn Rule>>) -> Self {
        self.0.rules = rules;

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

#[cfg(test)]
mod tests {
    use super::InMemoryCacheBuilder;
    use static_assertions::assert_impl_all;
    use std::fmt::Debug;

    assert_impl_all!(InMemoryCacheBuilder: Debug, Default, Send, Sync);
}
