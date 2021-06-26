use crate::{config::ResourceType, model::CachedPresence, InMemoryCache, UpdateCache};
use std::collections::HashSet;
use twilight_model::{
    gateway::{payload::PresenceUpdate, presence::UserOrId},
    id::{GuildId, UserId},
};

const fn presence_user_id(user_or_id: &UserOrId) -> UserId {
    match user_or_id {
        UserOrId::User(u) => u.id,
        UserOrId::UserId { id } => *id,
    }
}

impl InMemoryCache {
    /// Gets the set of presences in a guild.
    ///
    /// This list may be incomplete if not all members have been cached.
    ///
    /// This is a O(m) operation, where m is the amount of members in the guild.
    /// This requires the [`GUILD_PRESENCES`] intent.
    ///
    /// [`GUILD_PRESENCES`]: ::twilight_model::gateway::Intents::GUILD_PRESENCES
    pub fn guild_presences(&self, guild_id: GuildId) -> Option<HashSet<UserId>> {
        self.0.guild_presences.get(&guild_id).map(|r| r.clone())
    }

    /// Gets a presence by, optionally, guild ID, and user ID.
    ///
    /// This is an O(1) operation. This requires the [`GUILD_PRESENCES`] intent.
    ///
    /// [`GUILD_PRESENCES`]: ::twilight_model::gateway::Intents::GUILD_PRESENCES
    pub fn presence(&self, guild_id: GuildId, user_id: UserId) -> Option<CachedPresence> {
        self.0
            .presences
            .get(&(guild_id, user_id))
            .map(|r| r.clone())
    }

    pub(crate) fn cache_presences(
        &self,
        guild_id: GuildId,
        presences: impl IntoIterator<Item = CachedPresence>,
    ) {
        for presence in presences {
            self.cache_presence(guild_id, presence);
        }
    }

    fn cache_presence(&self, guild_id: GuildId, presence: CachedPresence) {
        self.0
            .presences
            .insert((guild_id, presence.user_id), presence);
    }
}

impl UpdateCache for PresenceUpdate {
    fn update(&self, cache: &InMemoryCache) {
        if !cache.wants(ResourceType::PRESENCE) {
            return;
        }

        let presence = CachedPresence {
            activities: self.activities.clone(),
            client_status: self.client_status.clone(),
            guild_id: self.guild_id,
            status: self.status,
            user_id: presence_user_id(&self.user),
        };

        cache.cache_presence(self.guild_id, presence);
    }
}
