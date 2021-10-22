use crate::{config::ResourceType, model::CachedPresence, InMemoryCache, UpdateCache};
use twilight_model::{
    gateway::{payload::incoming::PresenceUpdate, presence::UserOrId},
    id::{GuildId, UserId},
};

const fn presence_user_id(user_or_id: &UserOrId) -> UserId {
    match user_or_id {
        UserOrId::User(u) => u.id,
        UserOrId::UserId { id } => *id,
    }
}

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
            user_id: presence_user_id(&self.user),
        };

        cache.cache_presence(self.guild_id, presence);
    }
}
