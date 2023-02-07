use crate::{config::ResourceType, model::CachedPresence, InMemoryCache, UpdateCache};
use twilight_model::{
    gateway::payload::incoming::PresenceUpdate,
    id::{marker::GuildMarker, Id},
};

impl InMemoryCache {
    pub(crate) fn cache_presences(
        &self,
        guild_id: Id<GuildMarker>,
        presences: impl IntoIterator<Item = CachedPresence>,
    ) {
        for presence in presences {
            self.cache_presence(guild_id, presence);
        }
    }

    fn cache_presence(&self, guild_id: Id<GuildMarker>, presence: CachedPresence) {
        self.guild_presences
            .entry(guild_id)
            .or_default()
            .insert(presence.user_id);

        self.presences
            .insert((guild_id, presence.user_id()), presence);
    }
}

impl UpdateCache for PresenceUpdate {
    fn update(&self, cache: &InMemoryCache) {
        if !cache.wants(ResourceType::PRESENCE) {
            return;
        }

        let presence = CachedPresence::from_model(self.0.clone());

        if let Some(guild_id) = self.guild_id {
            cache.cache_presence(guild_id, presence);
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{test, InMemoryCache};
    use twilight_model::{
        gateway::{
            event::Event,
            payload::incoming::PresenceUpdate,
            presence::{ClientStatus, Presence, Status, UserOrId},
        },
        id::Id,
    };

    #[test]
    fn presence_update() {
        let cache = InMemoryCache::new();

        let guild_id = Some(Id::new(1));
        let user_id = Id::new(1);

        let payload = PresenceUpdate(Presence {
            activities: Vec::new(),
            client_status: ClientStatus {
                desktop: Some(Status::Online),
                mobile: None,
                web: None,
            },
            guild_id,
            status: Status::Online,
            user: UserOrId::User(test::user(user_id)),
        });
        cache.update(&Event::PresenceUpdate(Box::new(payload)));

        assert_eq!(1, cache.presences.len());
        assert_eq!(1, cache.guild_presences.len());
        assert!(cache
            .guild_presences
            .get(&guild_id.unwrap())
            .unwrap()
            .contains(&user_id));
    }
}
