pub mod channel;
pub mod emoji;
pub mod guild;
pub mod guild_scheduled_events;
pub mod integration;
pub mod interaction;
pub mod member;
pub mod message;
pub mod presence;
pub mod reaction;
pub mod role;
pub mod stage_instance;
pub mod sticker;
pub mod thread;
pub mod voice_state;

use std::{borrow::Cow, collections::HashSet};

use crate::{CacheableModels, InMemoryCache, UpdateCache, config::ResourceType};
use twilight_model::{
    gateway::payload::incoming::{Ready, UnavailableGuild, UserUpdate},
    id::{Id, marker::GuildMarker},
    user::{CurrentUser, User},
};

impl<CacheModels: CacheableModels> InMemoryCache<CacheModels> {
    fn cache_current_user(&self, current_user: CurrentUser) {
        self.current_user
            .lock()
            .expect("current user poisoned")
            .replace(CacheModels::CurrentUser::from(current_user));
    }

    pub(crate) fn cache_user(&self, user: Cow<'_, User>, guild_id: Option<Id<GuildMarker>>) {
        if let Some(cached_user) = self.users.get_mut(&user.id)
            && cached_user.value() == user.as_ref()
            && let Some(guild_id) = guild_id
        {
            self.user_guilds
                .entry(user.id)
                .or_default()
                .insert(guild_id);

            return;
        }

        let user = user.into_owned();
        let user_id = user.id;

        self.users.insert(user_id, CacheModels::User::from(user));

        if let Some(guild_id) = guild_id {
            let mut guild_id_set = HashSet::new();
            guild_id_set.insert(guild_id);
            self.user_guilds.insert(user_id, guild_id_set);
        }
    }

    fn unavailable_guild(&self, guild_id: Id<GuildMarker>) {
        self.unavailable_guilds.insert(guild_id);
        self.delete_guild(guild_id, true);
    }
}

impl<CacheModels: CacheableModels> UpdateCache<CacheModels> for Ready {
    fn update(&self, cache: &InMemoryCache<CacheModels>) {
        if cache.wants(ResourceType::USER_CURRENT) {
            cache.cache_current_user(self.user.clone());
        }

        if cache.wants(ResourceType::GUILD) {
            for guild in &self.guilds {
                cache.unavailable_guild(guild.id);
            }
        }
    }
}

impl<CacheModels: CacheableModels> UpdateCache<CacheModels> for UnavailableGuild {
    fn update(&self, cache: &InMemoryCache<CacheModels>) {
        if cache.wants(ResourceType::GUILD) {
            cache.unavailable_guild(self.id);
        }
    }
}

impl<CacheModels: CacheableModels> UpdateCache<CacheModels> for UserUpdate {
    fn update(&self, cache: &InMemoryCache<CacheModels>) {
        if !cache.wants(ResourceType::USER_CURRENT) {
            return;
        }

        cache.cache_current_user(self.0.clone());
    }
}

#[cfg(test)]
mod tests {
    use crate::{DefaultInMemoryCache, test};

    /// Test retrieval of the current user, notably that it doesn't simply
    /// panic or do anything funny. This is the only synchronous mutex that we
    /// might have trouble with across await points if we're not careful.
    #[test]
    fn current_user_retrieval() {
        let cache = DefaultInMemoryCache::new();
        assert!(cache.current_user().is_none());
        cache.cache_current_user(test::current_user(1));
        assert!(cache.current_user().is_some());
    }
}
