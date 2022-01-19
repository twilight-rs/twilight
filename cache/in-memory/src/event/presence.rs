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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test;
    use twilight_model::{
        gateway::{
            event::Event,
            presence::{ClientStatus, Status, UserOrId},
        },
        id::UserId,
    };

    #[test]
    fn test_presence_update() {
        let cache = InMemoryCache::new();

        let guild_id = GuildId::new(1).expect("non zero");
        let user_id = UserId::new(1).expect("non zero");

        cache.update(&Event::PresenceUpdate(Box::new(PresenceUpdate {
            activities: Vec::new(),
            client_status: ClientStatus {
                desktop: Some(Status::Online),
                mobile: None,
                web: None,
            },
            game: None,
            guild_id,
            status: Status::Online,
            user: UserOrId::User(test::user(user_id)),
        })));

        assert_eq!(1, cache.presences.len());
        assert_eq!(1, cache.guild_presences.len());
        assert!(cache
            .guild_presences
            .get(&guild_id)
            .unwrap()
            .contains(&user_id));
    }
}
