use crate::{config::ResourceType, InMemoryCache, UpdateCache};
use twilight_model::{
    channel::Channel,
    gateway::payload::incoming::{ChannelCreate, ChannelDelete, ChannelPinsUpdate, ChannelUpdate},
    id::{marker::ChannelMarker, Id},
};

impl InMemoryCache {
    pub(crate) fn cache_channels(&self, channels: impl IntoIterator<Item = Channel>) {
        for channel in channels {
            self.cache_channel(channel);
        }
    }

    pub(crate) fn cache_channel(&self, channel: Channel) {
        if let Some(guild_id) = channel.guild_id {
            self.guild_channels
                .entry(guild_id)
                .or_default()
                .insert(channel.id);
        }

        self.channels.insert(channel.id, channel);
    }

    /// Delete a guild channel from the cache.
    ///
    /// The guild channel data itself and the channel entry in its guild's list
    /// of channels will be deleted.
    pub(crate) fn delete_channel(&self, channel_id: Id<ChannelMarker>) {
        if let Some((_, channel)) = self.channels.remove(&channel_id) {
            if let Some(guild_id) = channel.guild_id {
                let maybe_channels = self.guild_channels.get_mut(&guild_id);

                if let Some(mut channels) = maybe_channels {
                    channels.remove(&channel_id);
                }
            }
        }
    }
}

impl UpdateCache for ChannelCreate {
    fn update(&self, cache: &InMemoryCache) {
        if !cache.wants(ResourceType::CHANNEL) {
            return;
        }

        cache.cache_channel(self.0.clone());
    }
}

impl UpdateCache for ChannelDelete {
    fn update(&self, cache: &InMemoryCache) {
        if !cache.wants(ResourceType::CHANNEL) {
            return;
        }

        cache.delete_channel(self.0.id);
    }
}

impl UpdateCache for ChannelPinsUpdate {
    fn update(&self, cache: &InMemoryCache) {
        if !cache.wants(ResourceType::CHANNEL) {
            return;
        }

        if let Some(mut channel) = cache.channels.get_mut(&self.channel_id) {
            channel.last_pin_timestamp = self.last_pin_timestamp;
        }
    }
}

impl UpdateCache for ChannelUpdate {
    fn update(&self, cache: &InMemoryCache) {
        if !cache.wants(ResourceType::CHANNEL) {
            return;
        }

        cache.cache_channel(self.0.clone());
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test;
    use twilight_model::gateway::event::Event;

    #[test]
    fn test_channel_delete_guild() {
        let cache = InMemoryCache::new();
        let (guild_id, channel_id, channel) = test::guild_channel_text();

        cache.cache_channel(channel.clone());
        assert_eq!(1, cache.channels.len());
        assert!(cache
            .guild_channels
            .get(&guild_id)
            .unwrap()
            .contains(&channel_id));

        cache.update(&Event::ChannelDelete(Box::new(ChannelDelete(channel))));
        assert!(cache.channels.is_empty());
        assert!(cache.guild_channels.get(&guild_id).unwrap().is_empty());
    }

    #[test]
    fn test_channel_update_guild() {
        let cache = InMemoryCache::new();
        let (guild_id, channel_id, channel) = test::guild_channel_text();

        cache.update(&ChannelUpdate(channel));
        assert_eq!(1, cache.channels.len());
        assert!(cache
            .guild_channels
            .get(&guild_id)
            .unwrap()
            .contains(&channel_id));
    }
}
