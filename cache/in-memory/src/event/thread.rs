use crate::{config::ResourceType, InMemoryCache, UpdateCache};
use twilight_model::gateway::payload::incoming::{
    ThreadCreate, ThreadDelete, ThreadListSync, ThreadUpdate,
};

impl UpdateCache for ThreadCreate {
    fn update(&self, cache: &InMemoryCache) {
        if !cache.wants(ResourceType::CHANNEL) {
            return;
        }

        cache.cache_channel(self.0.clone());
    }
}

impl UpdateCache for ThreadDelete {
    fn update(&self, cache: &InMemoryCache) {
        if !cache.wants(ResourceType::CHANNEL) {
            return;
        }

        cache.delete_channel(self.id);
    }
}

impl UpdateCache for ThreadListSync {
    fn update(&self, cache: &InMemoryCache) {
        if !cache.wants(ResourceType::CHANNEL) {
            return;
        }

        cache.cache_channels(self.threads.clone());
    }
}

impl UpdateCache for ThreadUpdate {
    fn update(&self, cache: &InMemoryCache) {
        if !cache.wants(ResourceType::CHANNEL) {
            return;
        }

        cache.cache_channel(self.0.clone());
    }
}
