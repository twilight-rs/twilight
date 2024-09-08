use twilight_model::{
    gateway::payload::incoming::{
        GuildScheduledEventCreate, GuildScheduledEventDelete, GuildScheduledEventUpdate,
        GuildScheduledEventUserAdd, GuildScheduledEventUserRemove,
    },
    guild::scheduled_event::GuildScheduledEvent,
    id::{marker::GuildMarker, Id},
};

use crate::{
    traits::CacheableGuildScheduledEvent, CacheableModels, InMemoryCache, ResourceType, UpdateCache,
};

impl<CacheModels: CacheableModels> InMemoryCache<CacheModels> {
    pub(crate) fn cache_guild_scheduled_events(
        &self,
        guild_id: Id<GuildMarker>,
        guild_scheduled_events: impl IntoIterator<Item = GuildScheduledEvent>,
    ) {
        for event in guild_scheduled_events {
            self.cache_guild_scheduled_event(guild_id, event);
        }
    }

    fn cache_guild_scheduled_event(
        &self,
        guild_id: Id<GuildMarker>,
        guild_scheduled_event: GuildScheduledEvent,
    ) {
        self.guild_scheduled_events
            .entry(guild_id)
            .or_default()
            .insert(guild_scheduled_event.id);

        crate::upsert_guild_item(
            &self.scheduled_events,
            guild_id,
            guild_scheduled_event.id,
            CacheModels::GuildScheduledEvent::from(guild_scheduled_event),
        );
    }
}

impl<CacheModels: CacheableModels> UpdateCache<CacheModels> for GuildScheduledEventCreate {
    fn update(&self, cache: &InMemoryCache<CacheModels>) {
        if !cache.wants(ResourceType::GUILD_SCHEDULED_EVENT) {
            return;
        }

        cache.cache_guild_scheduled_event(self.guild_id, self.0.clone());
    }
}

impl<CacheModels: CacheableModels> UpdateCache<CacheModels> for GuildScheduledEventDelete {
    fn update(&self, cache: &InMemoryCache<CacheModels>) {
        if !cache.wants(ResourceType::GUILD_SCHEDULED_EVENT) {
            return;
        }

        if cache.scheduled_events.remove(&self.id).is_some() {
            if let Some(mut events) = cache.guild_scheduled_events.get_mut(&self.guild_id) {
                events.remove(&self.id);
            }
        }
    }
}

impl<CacheModels: CacheableModels> UpdateCache<CacheModels> for GuildScheduledEventUpdate {
    fn update(&self, cache: &InMemoryCache<CacheModels>) {
        if !cache.wants(ResourceType::GUILD_SCHEDULED_EVENT) {
            return;
        }

        cache.cache_guild_scheduled_event(self.guild_id, self.0.clone());
    }
}

impl<CacheModels: CacheableModels> UpdateCache<CacheModels> for GuildScheduledEventUserAdd {
    fn update(&self, cache: &InMemoryCache<CacheModels>) {
        cache
            .scheduled_events
            .entry(self.guild_scheduled_event_id)
            .and_modify(|event| {
                event
                    .value
                    .add_user(self.guild_id, self.guild_scheduled_event_id, self.user_id);
            });
    }
}

impl<CacheModels: CacheableModels> UpdateCache<CacheModels> for GuildScheduledEventUserRemove {
    fn update(&self, cache: &InMemoryCache<CacheModels>) {
        cache
            .scheduled_events
            .entry(self.guild_scheduled_event_id)
            .and_modify(|event| {
                event
                    .value
                    .remove_user(self.guild_id, self.guild_scheduled_event_id, self.user_id);
            });
    }
}

#[cfg(test)]
mod tests {
    use crate::{test, DefaultInMemoryCache};
    use twilight_model::{
        gateway::payload::incoming::{
            GuildScheduledEventCreate, GuildScheduledEventUserAdd, GuildScheduledEventUserRemove,
        },
        id::Id,
    };

    #[test]
    fn guild_event_on_event() {
        let cache = DefaultInMemoryCache::new();

        let id = Id::new(1);
        let guild_id = Id::new(2);
        let user_id = Id::new(3);

        cache.update(&GuildScheduledEventCreate(test::guild_schduled_event(
            id,
            guild_id,
            Some(41),
        )));

        assert_eq!(
            1,
            cache.guild_scheduled_events.get(&guild_id).unwrap().len()
        );
        assert_eq!(1, cache.scheduled_events.len());
        assert_eq!(
            41,
            cache.scheduled_events.get(&id).unwrap().user_count.unwrap()
        );

        cache.update(&GuildScheduledEventUserAdd {
            guild_id,
            guild_scheduled_event_id: id,
            user_id,
        });

        assert_eq!(
            42,
            cache.scheduled_events.get(&id).unwrap().user_count.unwrap()
        );

        cache.update(&GuildScheduledEventUserRemove {
            guild_id,
            guild_scheduled_event_id: id,
            user_id,
        });

        assert_eq!(
            41,
            cache.scheduled_events.get(&id).unwrap().user_count.unwrap()
        );
    }
}
