use std::fmt::Debug;
use std::marker::PhantomData;

use twilight_model::{
    channel::{Channel, StageInstance},
    guild::{GuildIntegration, Role},
    user::{CurrentUser, User},
};

use crate::{
    model, CacheableChannel, CacheableCurrentUser, CacheableEmoji, CacheableGuild,
    CacheableGuildIntegration, CacheableMember, CacheableMessage, CacheablePresence, CacheableRole,
    CacheableStageInstance, CacheableSticker, CacheableUser, CacheableVoiceState,
};

use super::{
    config::{Config, ResourceType},
    InMemoryCache,
};

/// Builder to configure and construct an [`InMemoryCache`].
#[allow(clippy::type_complexity)]
#[must_use = "has no effect if not built"]
#[derive(Debug)]
pub struct InMemoryCacheBuilder<
    CachedChannel: CacheableChannel = Channel,
    CachedCurrentUser: CacheableCurrentUser = CurrentUser,
    CachedEmoji: CacheableEmoji = model::CachedEmoji,
    CachedGuild: CacheableGuild = model::CachedGuild,
    CachedGuildIntegration: CacheableGuildIntegration = GuildIntegration,
    CachedMember: CacheableMember = model::CachedMember,
    CachedMessage: CacheableMessage = model::CachedMessage,
    CachedPresence: CacheablePresence = model::CachedPresence,
    CachedRole: CacheableRole = Role,
    CachedStageInstance: CacheableStageInstance = StageInstance,
    CachedSticker: CacheableSticker = model::CachedSticker,
    CachedUser: CacheableUser = User,
    CachedVoiceState: CacheableVoiceState = model::CachedVoiceState,
>(
    Config,
    PhantomData<(
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
    )>,
);

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
    InMemoryCacheBuilder<
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
    /// Creates a builder to configure and construct an [`InMemoryCache`].
    pub const fn new() -> Self {
        Self(Config::new(), PhantomData)
    }

    /// Consume the builder, returning a configured cache.
    #[allow(clippy::type_complexity)]
    pub fn build(
        self,
    ) -> InMemoryCache<
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
    > {
        InMemoryCache::new_with_config(self.0)
    }

    /// Sets the list of resource types for the cache to handle.
    ///
    /// Defaults to all types.
    pub const fn resource_types(mut self, resource_types: ResourceType) -> Self {
        self.0.resource_types = resource_types;

        self
    }

    /// Sets the number of messages to cache per channel.
    ///
    /// Defaults to 100.
    pub const fn message_cache_size(mut self, message_cache_size: usize) -> Self {
        self.0.message_cache_size = message_cache_size;

        self
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
    > Default
    for InMemoryCacheBuilder<
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
    fn default() -> Self {
        Self(Config::default(), PhantomData)
    }
}

#[cfg(test)]
mod tests {
    use super::InMemoryCacheBuilder;
    use static_assertions::assert_impl_all;
    use std::fmt::Debug;

    assert_impl_all!(InMemoryCacheBuilder: Debug, Default, Send, Sync);
}
