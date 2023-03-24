use crate::{
    config::ResourceType,
    traits::{
        CacheableChannel, CacheableCurrentUser, CacheableEmoji, CacheableGuild,
        CacheableGuildIntegration, CacheableMember, CacheableMessage, CacheablePresence,
        CacheableRole, CacheableStageInstance, CacheableSticker, CacheableUser,
        CacheableVoiceState,
    },
    InMemoryCache, UpdateCache,
};
use twilight_model::{
    gateway::payload::incoming::{IntegrationCreate, IntegrationDelete, IntegrationUpdate},
    guild::GuildIntegration,
    id::{
        marker::{GuildMarker, IntegrationMarker},
        Id,
    },
};

impl<
        CachedChannel: CacheableChannel,
        CachedCurrentUser: CacheableCurrentUser,
        CachedEmoji: CacheableEmoji,
        CachedGuild: CacheableGuild,
        CachedGuildIntegration: CacheableGuildIntegration,
        CachedMember: CacheableMember,
        CachedMessage: CacheableMessage,
        CachedPresence: CacheablePresence,
        CachedRole: CacheableRole,
        CachedStageInstance: CacheableStageInstance,
        CachedSticker: CacheableSticker,
        CachedUser: CacheableUser,
        CachedVoiceState: CacheableVoiceState,
    >
    InMemoryCache<
        CachedChannel,
        CachedCurrentUser,
        CachedEmoji,
        CachedGuild,
        CachedGuildIntegration,
        CachedMember,
        CachedMessage,
        CachedPresence,
        CachedRole,
        CachedStageInstance,
        CachedSticker,
        CachedUser,
        CachedVoiceState,
    >
{
    fn cache_integration(&self, guild_id: Id<GuildMarker>, integration: GuildIntegration) {
        self.guild_integrations
            .entry(guild_id)
            .or_default()
            .insert(integration.id);

        crate::upsert_guild_item(
            &self.integrations,
            guild_id,
            (guild_id, integration.id),
            CachedGuildIntegration::from(integration),
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

impl<
        CachedChannel: CacheableChannel,
        CachedCurrentUser: CacheableCurrentUser,
        CachedEmoji: CacheableEmoji,
        CachedGuild: CacheableGuild,
        CachedGuildIntegration: CacheableGuildIntegration,
        CachedMember: CacheableMember,
        CachedMessage: CacheableMessage,
        CachedPresence: CacheablePresence,
        CachedRole: CacheableRole,
        CachedStageInstance: CacheableStageInstance,
        CachedSticker: CacheableSticker,
        CachedUser: CacheableUser,
        CachedVoiceState: CacheableVoiceState,
    >
    UpdateCache<
        CachedChannel,
        CachedCurrentUser,
        CachedEmoji,
        CachedGuild,
        CachedGuildIntegration,
        CachedMember,
        CachedMessage,
        CachedPresence,
        CachedRole,
        CachedStageInstance,
        CachedSticker,
        CachedUser,
        CachedVoiceState,
    > for IntegrationCreate
{
    fn update(
        &self,
        cache: &InMemoryCache<
            CachedChannel,
            CachedCurrentUser,
            CachedEmoji,
            CachedGuild,
            CachedGuildIntegration,
            CachedMember,
            CachedMessage,
            CachedPresence,
            CachedRole,
            CachedStageInstance,
            CachedSticker,
            CachedUser,
            CachedVoiceState,
        >,
    ) {
        if !cache.wants(ResourceType::INTEGRATION) {
            return;
        }

        if let Some(guild_id) = self.guild_id {
            crate::upsert_guild_item(
                &cache.integrations,
                guild_id,
                (guild_id, self.id),
                CachedGuildIntegration::from(self.0.clone()),
            );
        }
    }
}

impl<
        CachedChannel: CacheableChannel,
        CachedCurrentUser: CacheableCurrentUser,
        CachedEmoji: CacheableEmoji,
        CachedGuild: CacheableGuild,
        CachedGuildIntegration: CacheableGuildIntegration,
        CachedMember: CacheableMember,
        CachedMessage: CacheableMessage,
        CachedPresence: CacheablePresence,
        CachedRole: CacheableRole,
        CachedStageInstance: CacheableStageInstance,
        CachedSticker: CacheableSticker,
        CachedUser: CacheableUser,
        CachedVoiceState: CacheableVoiceState,
    >
    UpdateCache<
        CachedChannel,
        CachedCurrentUser,
        CachedEmoji,
        CachedGuild,
        CachedGuildIntegration,
        CachedMember,
        CachedMessage,
        CachedPresence,
        CachedRole,
        CachedStageInstance,
        CachedSticker,
        CachedUser,
        CachedVoiceState,
    > for IntegrationDelete
{
    fn update(
        &self,
        cache: &InMemoryCache<
            CachedChannel,
            CachedCurrentUser,
            CachedEmoji,
            CachedGuild,
            CachedGuildIntegration,
            CachedMember,
            CachedMessage,
            CachedPresence,
            CachedRole,
            CachedStageInstance,
            CachedSticker,
            CachedUser,
            CachedVoiceState,
        >,
    ) {
        if !cache.wants(ResourceType::INTEGRATION) {
            return;
        }

        cache.delete_integration(self.guild_id, self.id);
    }
}

impl<
        CachedChannel: CacheableChannel,
        CachedCurrentUser: CacheableCurrentUser,
        CachedEmoji: CacheableEmoji,
        CachedGuild: CacheableGuild,
        CachedGuildIntegration: CacheableGuildIntegration,
        CachedMember: CacheableMember,
        CachedMessage: CacheableMessage,
        CachedPresence: CacheablePresence,
        CachedRole: CacheableRole,
        CachedStageInstance: CacheableStageInstance,
        CachedSticker: CacheableSticker,
        CachedUser: CacheableUser,
        CachedVoiceState: CacheableVoiceState,
    >
    UpdateCache<
        CachedChannel,
        CachedCurrentUser,
        CachedEmoji,
        CachedGuild,
        CachedGuildIntegration,
        CachedMember,
        CachedMessage,
        CachedPresence,
        CachedRole,
        CachedStageInstance,
        CachedSticker,
        CachedUser,
        CachedVoiceState,
    > for IntegrationUpdate
{
    fn update(
        &self,
        cache: &InMemoryCache<
            CachedChannel,
            CachedCurrentUser,
            CachedEmoji,
            CachedGuild,
            CachedGuildIntegration,
            CachedMember,
            CachedMessage,
            CachedPresence,
            CachedRole,
            CachedStageInstance,
            CachedSticker,
            CachedUser,
            CachedVoiceState,
        >,
    ) {
        if !cache.wants(ResourceType::INTEGRATION) {
            return;
        }

        if let Some(guild_id) = self.guild_id {
            cache.cache_integration(guild_id, self.0.clone());
        }
    }
}
