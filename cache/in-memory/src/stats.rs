use twilight_model::id::{
    marker::{ChannelMarker, GuildMarker},
    Id,
};

use super::InMemoryCache;

/// Retrieve statistics about the number of entities of each resource in the
/// cache.
///
/// Statistics can be retrieved about the amount of resources on a cache-level
/// via a method such as [`users`] or in a particular channel via a method
/// such as [`channel_messages`].
///
/// # Examples
///
/// Retrieve the number of users stored in the cache:
///
/// ```no_run
/// use twilight_cache_inmemory::InMemoryCache;
///
/// let cache = InMemoryCache::new();
///
/// // later on...
/// println!("user count: {}", cache.stats().users());
/// ```
///
/// [`channel_messages`]: Self::channel_messages
/// [`users`]: Self::users
#[derive(Clone, Debug)]
pub struct InMemoryCacheStats<'a>(&'a InMemoryCache);

impl<'a> InMemoryCacheStats<'a> {
    pub(super) const fn new(cache: &'a InMemoryCache) -> Self {
        Self(cache)
    }

    /// Return an immutable reference to the underlying cache.
    pub const fn cache_ref(&'a self) -> &'a InMemoryCache {
        self.0
    }

    /// Consume the statistics interface, returning the underlying cache
    /// reference.
    pub const fn into_cache(self) -> &'a InMemoryCache {
        self.0
    }

    /// Number of channels in the cache.
    pub fn channels(&self) -> usize {
        self.0.channels.len()
    }

    /// Number of messages in a given channel in the cache.
    ///
    /// Returns `None` if the channel hasn't yet been cached or there are no
    /// messages in the channel. However, the provided number may still be 0
    /// if some number is returned.
    pub fn channel_messages(&self, channel_id: Id<ChannelMarker>) -> Option<usize> {
        let channel = self.0.channel_messages.get(&channel_id)?;

        Some(channel.len())
    }

    /// Number of voice states in a given channel in the cache.
    ///
    /// Returns `None` if the channel hasn't yet been cached or there are no
    /// voice states in the channel. However, the provided number may still be 0
    /// if some number is returned.
    pub fn channel_voice_states(&self, channel_id: Id<ChannelMarker>) -> Option<usize> {
        let channel = self.0.voice_state_channels.get(&channel_id)?;

        Some(channel.len())
    }

    /// Number of emojis in the cache.
    pub fn emojis(&self) -> usize {
        self.0.emojis.len()
    }

    /// Number of guilds in the cache.
    pub fn guilds(&self) -> usize {
        self.0.guilds.len()
    }

    /// Number of channels in a given guild in the cache.
    ///
    /// Returns `None` if the guild hasn't yet been cached.
    pub fn guild_channels(&self, guild_id: Id<GuildMarker>) -> Option<usize> {
        let guild = self.0.guild_channels.get(&guild_id)?;

        Some(guild.len())
    }

    /// Number of emojis in a given guild in the cache.
    ///
    /// Returns `None` if the guild hasn't yet been cached.
    pub fn guild_emojis(&self, guild_id: Id<GuildMarker>) -> Option<usize> {
        let guild = self.0.guild_emojis.get(&guild_id)?;

        Some(guild.len())
    }

    /// Number of members in a given guild in the cache.
    ///
    /// Returns `None` if the guild hasn't yet been cached.
    pub fn guild_members(&self, guild_id: Id<GuildMarker>) -> Option<usize> {
        let guild = self.0.guild_members.get(&guild_id)?;

        Some(guild.len())
    }

    /// Number of presences in a given guild in the cache.
    ///
    /// Returns `None` if the guild hasn't yet been cached.
    pub fn guild_presences(&self, guild_id: Id<GuildMarker>) -> Option<usize> {
        let guild = self.0.guild_presences.get(&guild_id)?;

        Some(guild.len())
    }

    /// Number of roles in a given guild in the cache.
    ///
    /// Returns `None` if the guild hasn't yet been cached.
    pub fn guild_roles(&self, guild_id: Id<GuildMarker>) -> Option<usize> {
        let guild = self.0.guild_roles.get(&guild_id)?;

        Some(guild.len())
    }

    /// Number of voice states in a given guild in the cache.
    ///
    /// Returns `None` if the guild hasn't yet been cached.
    pub fn guild_voice_states(&self, guild_id: Id<GuildMarker>) -> Option<usize> {
        let guild = self.0.voice_state_guilds.get(&guild_id)?;

        Some(guild.len())
    }

    /// Number of members in the cache.
    pub fn members(&self) -> usize {
        self.0.members.len()
    }

    /// Number of presences in the cache.
    pub fn presences(&self) -> usize {
        self.0.presences.len()
    }

    /// Number of roles in the cache.
    pub fn roles(&self) -> usize {
        self.0.roles.len()
    }

    /// Number of unavailable_guilds in the cache.
    pub fn unavailable_guilds(&self) -> usize {
        self.0.unavailable_guilds.len()
    }

    /// Number of users in the cache.
    pub fn users(&self) -> usize {
        self.0.users.len()
    }

    /// Number of voice_states in the cache.
    pub fn voice_states(&self) -> usize {
        self.0.voice_states.len()
    }
}

#[cfg(test)]
mod tests {
    use super::InMemoryCacheStats;
    use static_assertions::assert_impl_all;
    use std::fmt::Debug;

    assert_impl_all!(InMemoryCacheStats<'_>: Clone, Debug, Send, Sync);
}
