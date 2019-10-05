pub use dawn_cache_trait::{Cache, UpdateCache};

#[cfg(feature = "dawn-cache-inmemory")]
pub use dawn_cache_inmemory::InMemoryCache;
