use crate::{config::ResourceType, model::CachedPresence, InMemoryCache, UpdateCache};
use twilight_model::{gateway::payload::incoming::PresenceUpdate, id::GuildId};

impl InMemoryCache {
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
        self.guild_presences.entry(guild_id).or_default().insert(presence.user_id);

        self.presences
            .insert((guild_id, presence.user_id()), presence);
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
            user_id: self.user.id(),
        };

        cache.cache_presence(self.guild_id, presence);
    }
}
