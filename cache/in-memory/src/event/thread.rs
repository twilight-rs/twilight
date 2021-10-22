use crate::{config::ResourceType, InMemoryCache, UpdateCache};
use twilight_model::{
    channel::{Channel, GuildChannel},
    gateway::payload::incoming::{ThreadCreate, ThreadDelete, ThreadListSync, ThreadUpdate},
};

impl UpdateCache for ThreadCreate {
    fn update(&self, cache: &InMemoryCache) {
        if !cache.wants(ResourceType::CHANNEL) {
            return;
        }

        if let Channel::Guild(c) = &self.0 {
            if let Some(gid) = c.guild_id() {
                cache.cache_guild_channel(gid, c.clone());
            }
        }
    }
}

impl UpdateCache for ThreadDelete {
    fn update(&self, cache: &InMemoryCache) {
        if !cache.wants(ResourceType::CHANNEL) {
            return;
        }

        cache.delete_guild_channel(self.0.id());
    }
}

impl UpdateCache for ThreadListSync {
    fn update(&self, cache: &InMemoryCache) {
        if !cache.wants(ResourceType::CHANNEL) {
            return;
        }

        let threads: Vec<GuildChannel> = self
            .threads
            .iter()
            .filter_map(|c| match &c {
                Channel::Guild(c) => Some(c.clone()),
                _ => None,
            })
            .collect();

        cache.cache_guild_channels(self.guild_id, threads);
    }
}

impl UpdateCache for ThreadUpdate {
    fn update(&self, cache: &InMemoryCache) {
        if !cache.wants(ResourceType::CHANNEL) {
            return;
        }

        if let Channel::Guild(c) = &self.0 {
            if let Some(gid) = c.guild_id() {
                cache.cache_guild_channel(gid, c.clone());
            }
        }
    }
}
