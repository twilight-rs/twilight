use crate::{config::ResourceType, CacheableModels, InMemoryCache, UpdateCache};
use twilight_model::gateway::payload::incoming::{
    ThreadCreate, ThreadDelete, ThreadListSync, ThreadUpdate,
};

impl<CacheModels: CacheableModels> UpdateCache<CacheModels> for ThreadCreate {
    fn update(&self, cache: &InMemoryCache<CacheModels>) {
        if !cache.wants(ResourceType::CHANNEL) {
            return;
        }

        cache.cache_channel(self.0.clone());
    }
}

impl<CacheModels: CacheableModels> UpdateCache<CacheModels> for ThreadDelete {
    fn update(&self, cache: &InMemoryCache<CacheModels>) {
        if !cache.wants(ResourceType::CHANNEL) {
            return;
        }

        cache.delete_channel(self.id);
    }
}

impl<CacheModels: CacheableModels> UpdateCache<CacheModels> for ThreadListSync {
    fn update(&self, cache: &InMemoryCache<CacheModels>) {
        if !cache.wants(ResourceType::CHANNEL) {
            return;
        }

        cache.cache_channels(self.threads.clone());
    }
}

impl<CacheModels: CacheableModels> UpdateCache<CacheModels> for ThreadUpdate {
    fn update(&self, cache: &InMemoryCache<CacheModels>) {
        if !cache.wants(ResourceType::CHANNEL) {
            return;
        }

        cache.cache_channel(self.0.clone());
    }
}
