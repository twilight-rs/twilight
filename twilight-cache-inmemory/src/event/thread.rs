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
use twilight_model::gateway::payload::incoming::{
    ThreadCreate, ThreadDelete, ThreadListSync, ThreadUpdate,
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
    > for ThreadCreate
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
        if !cache.wants(ResourceType::CHANNEL) {
            return;
        }

        cache.cache_channel(self.0.clone());
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
    > for ThreadDelete
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
        if !cache.wants(ResourceType::CHANNEL) {
            return;
        }

        cache.delete_channel(self.id);
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
    > for ThreadListSync
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
        if !cache.wants(ResourceType::CHANNEL) {
            return;
        }

        cache.cache_channels(self.threads.clone());
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
    > for ThreadUpdate
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
        if !cache.wants(ResourceType::CHANNEL) {
            return;
        }

        cache.cache_channel(self.0.clone());
    }
}
