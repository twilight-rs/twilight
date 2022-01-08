use crate::{config::ResourceType, InMemoryCache, UpdateCache};
use twilight_model::{
    gateway::payload::incoming::{IntegrationCreate, IntegrationDelete, IntegrationUpdate},
    guild::GuildIntegration,
    id::{
        marker::{GuildMarker, IntegrationMarker},
        Id,
    },
};

impl InMemoryCache {
    fn cache_integration(&self, guild_id: Id<GuildMarker>, integration: GuildIntegration) {
        self.guild_integrations
            .entry(guild_id)
            .or_default()
            .insert(integration.id);

        crate::upsert_guild_item(
            &self.integrations,
            guild_id,
            (guild_id, integration.id),
            integration,
        );
    }

    fn delete_integration(&self, guild_id: Id<GuildMarker>, integration_id: Id<IntegrationMarker>) {
        if self
            .integrations
            .remove(&(guild_id, integration_id))
            .is_some()
        {
            if let Some(mut integrations) = self.guild_integrations.get_mut(&guild_id) {
                integrations.remove(&integration_id);
            }
        }
    }
}

impl UpdateCache for Box<IntegrationCreate> {
    fn update(self, cache: &InMemoryCache) {
        if !cache.wants(ResourceType::INTEGRATION) {
            return;
        }

        if let Some(guild_id) = self.guild_id {
            crate::upsert_guild_item(&cache.integrations, guild_id, (guild_id, self.id), self.0);
        }
    }
}

impl UpdateCache for IntegrationDelete {
    fn update(self, cache: &InMemoryCache) {
        if !cache.wants(ResourceType::INTEGRATION) {
            return;
        }

        cache.delete_integration(self.guild_id, self.id);
    }
}

impl UpdateCache for Box<IntegrationUpdate> {
    fn update(self, cache: &InMemoryCache) {
        if !cache.wants(ResourceType::INTEGRATION) {
            return;
        }

        if let Some(guild_id) = self.guild_id {
            cache.cache_integration(guild_id, self.0);
        }
    }
}
