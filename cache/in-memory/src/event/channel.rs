use crate::{config::ResourceType, InMemoryCache, UpdateCache};
use std::collections::HashSet;
use twilight_model::{
    channel::{Channel, Group, GuildChannel, PrivateChannel},
    gateway::payload::{ChannelCreate, ChannelDelete, ChannelPinsUpdate, ChannelUpdate},
    id::{ChannelId, GuildId},
};

impl InMemoryCache {
    /// Gets a channel by ID.
    ///
    /// This is an O(1) operation. This requires the [`GUILDS`] intent.
    ///
    /// [`GUILDS`]: ::twilight_model::gateway::Intents::GUILDS
    pub fn guild_channel(&self, channel_id: ChannelId) -> Option<GuildChannel> {
        self.0
            .channels_guild
            .get(&channel_id)
            .map(|r| r.data.clone())
    }

    /// Gets the set of channels in a guild.
    ///
    /// This is a O(m) operation, where m is the amount of channels in the
    /// guild. This requires the [`GUILDS`] intent.
    ///
    /// [`GUILDS`]: ::twilight_model::gateway::Intents::GUILDS
    pub fn guild_channels(&self, guild_id: GuildId) -> Option<HashSet<ChannelId>> {
        self.0.guild_channels.get(&guild_id).map(|r| r.clone())
    }

    /// Gets a group by ID.
    ///
    /// This is an O(1) operation.
    pub fn group(&self, channel_id: ChannelId) -> Option<Group> {
        self.0.groups.get(&channel_id).map(|r| r.clone())
    }

    /// Gets a private channel by ID.
    ///
    /// This is an O(1) operation. This requires the [`DIRECT_MESSAGES`] intent.
    ///
    /// [`DIRECT_MESSAGES`]: ::twilight_model::gateway::Intents::DIRECT_MESSAGES
    pub fn private_channel(&self, channel_id: ChannelId) -> Option<PrivateChannel> {
        self.0.channels_private.get(&channel_id).map(|r| r.clone())
    }

    pub(crate) fn cache_guild_channels(
        &self,
        guild_id: GuildId,
        guild_channels: impl IntoIterator<Item = GuildChannel>,
    ) {
        for channel in guild_channels {
            self.cache_guild_channel(guild_id, channel);
        }
    }

    fn cache_guild_channel(&self, guild_id: GuildId, mut channel: GuildChannel) {
        match channel {
            GuildChannel::Category(ref mut c) => {
                c.guild_id.replace(guild_id);
            }
            GuildChannel::Text(ref mut c) => {
                c.guild_id.replace(guild_id);
            }
            GuildChannel::Voice(ref mut c) => {
                c.guild_id.replace(guild_id);
            }
            GuildChannel::Stage(ref mut c) => {
                c.guild_id.replace(guild_id);
            }
        }

        let id = channel.id();
        self.0
            .guild_channels
            .entry(guild_id)
            .or_default()
            .insert(id);

        crate::upsert_guild_item(&self.0.channels_guild, guild_id, id, channel);
    }

    fn cache_group(&self, group: Group) {
        crate::upsert_item(&self.0.groups, group.id, group)
    }

    fn cache_private_channel(&self, private_channel: PrivateChannel) {
        self.0
            .channels_private
            .insert(private_channel.id, private_channel);
    }

    /// Delete a guild channel from the cache.
    ///
    /// The guild channel data itself and the channel entry in its guild's list
    /// of channels will be deleted.
    fn delete_guild_channel(&self, channel_id: ChannelId) {
        if let Some((_, item)) = self.0.channels_guild.remove(&channel_id) {
            if let Some(mut guild_channels) = self.0.guild_channels.get_mut(&item.guild_id) {
                guild_channels.remove(&channel_id);
            }
        }
    }

    fn delete_group(&self, channel_id: ChannelId) {
        self.0.groups.remove(&channel_id);
    }
}

impl UpdateCache for ChannelCreate {
    fn update(&self, cache: &InMemoryCache) {
        if !cache.wants(ResourceType::CHANNEL) {
            return;
        }

        match &self.0 {
            Channel::Group(c) => {
                crate::upsert_item(&cache.0.groups, c.id, c.clone());
            }
            Channel::Guild(c) => {
                if let Some(gid) = c.guild_id() {
                    cache.cache_guild_channel(gid, c.clone());
                }
            }
            Channel::Private(c) => {
                cache.cache_private_channel(c.clone());
            }
        }
    }
}

impl UpdateCache for ChannelDelete {
    fn update(&self, cache: &InMemoryCache) {
        if !cache.wants(ResourceType::CHANNEL) {
            return;
        }

        match self.0 {
            Channel::Group(ref c) => {
                cache.delete_group(c.id);
            }
            Channel::Guild(ref c) => {
                cache.delete_guild_channel(c.id());
            }
            Channel::Private(ref c) => {
                cache.0.channels_private.remove(&c.id);
            }
        }
    }
}

impl UpdateCache for ChannelPinsUpdate {
    fn update(&self, cache: &InMemoryCache) {
        if !cache.wants(ResourceType::CHANNEL) {
            return;
        }

        if let Some(mut r) = cache.0.channels_guild.get_mut(&self.channel_id) {
            let value = r.value_mut();

            if let GuildChannel::Text(ref mut text) = value.data {
                text.last_pin_timestamp = self.last_pin_timestamp.clone();
            }

            return;
        }

        if let Some(mut channel) = cache.0.channels_private.get_mut(&self.channel_id) {
            channel.last_pin_timestamp = self.last_pin_timestamp.clone();

            return;
        }

        if let Some(mut group) = cache.0.groups.get_mut(&self.channel_id) {
            group.last_pin_timestamp = self.last_pin_timestamp.clone();
        }
    }
}

impl UpdateCache for ChannelUpdate {
    fn update(&self, cache: &InMemoryCache) {
        if !cache.wants(ResourceType::CHANNEL) {
            return;
        }

        match self.0.clone() {
            Channel::Group(c) => {
                cache.cache_group(c);
            }
            Channel::Guild(c) => {
                if let Some(gid) = c.guild_id() {
                    cache.cache_guild_channel(gid, c);
                }
            }
            Channel::Private(c) => {
                cache.cache_private_channel(c);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test;

    #[test]
    fn test_channel_lifecycle() {
        let cache = InMemoryCache::new();
        let (guild_id, channel_id, channel) = test::guild_channel_text("name".to_string());

        let event = ChannelCreate(Channel::Guild(channel.clone()));
        cache.update(&event);

        {
            let cached_channel = cache.guild_channel(channel_id).unwrap();

            assert_eq!(channel.id(), cached_channel.id());
            assert_eq!(channel.name(), cached_channel.name());

            assert!(cache
                .guild_channels(guild_id)
                .unwrap()
                .contains(&channel_id));
        }

        let (_, _, new_channel) = test::guild_channel_text("new name".to_string());

        let event = ChannelUpdate(Channel::Guild(new_channel.clone()));
        cache.update(&event);

        {
            let cached_channel = cache.guild_channel(channel_id).unwrap();

            assert_eq!(channel.id(), cached_channel.id());
            assert_eq!(new_channel.name(), cached_channel.name());

            if let GuildChannel::Text(text_channel) = cached_channel {
                assert_eq!(None, text_channel.last_pin_timestamp);
            }
        }

        let event = ChannelPinsUpdate {
            channel_id,
            guild_id: Some(guild_id),
            last_pin_timestamp: Some("new last pin".into()),
        };
        cache.update(&event);

        {
            let cached_channel = cache.guild_channel(channel_id).unwrap();
            if let GuildChannel::Text(text_channel) = cached_channel {
                assert_eq!(
                    Some("new last pin".to_string()),
                    text_channel.last_pin_timestamp
                );
            }
        }

        let event = ChannelDelete(Channel::Guild(new_channel.clone()));
        cache.update(&event);

        {
            assert_eq!(None, cache.guild_channel(channel_id));
            assert!(!cache
                .guild_channels(guild_id)
                .unwrap()
                .contains(&channel_id));
        }
    }
}
