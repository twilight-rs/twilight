use twilight_cache_inmemory::CacheableGuildIntegration;
use twilight_model::{
    guild::GuildIntegration,
    id::{Id, marker::IntegrationMarker},
};

#[derive(Clone, Debug, PartialEq)]
pub struct MinimalCachedGuildIntegration {
    pub id: Id<IntegrationMarker>,
}

impl From<GuildIntegration> for MinimalCachedGuildIntegration {
    fn from(integration: GuildIntegration) -> Self {
        Self { id: integration.id }
    }
}

impl PartialEq<GuildIntegration> for MinimalCachedGuildIntegration {
    fn eq(&self, other: &GuildIntegration) -> bool {
        self.id == other.id
    }
}

impl CacheableGuildIntegration for MinimalCachedGuildIntegration {}
