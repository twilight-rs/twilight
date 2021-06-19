pub mod channel;
pub mod emoji;
pub mod guild;
pub mod integration;
pub mod interaction;
pub mod member;
pub mod message;
pub mod presence;
pub mod reaction;
pub mod role;
pub mod stage_instance;
pub mod voice_state;

use crate::{config::ResourceType, InMemoryCache, UpdateCache};
use dashmap::mapref::one::Ref;
use std::{borrow::Cow, collections::BTreeSet};
use twilight_model::{
    gateway::payload::{Ready, UnavailableGuild, UserUpdate},
    id::{GuildId, UserId},
    user::{CurrentUser, User},
};

impl InMemoryCache {
    /// Gets the current user.
    ///
    /// This is an O(1) operation.
    pub fn current_user(&self) -> Option<CurrentUser> {
        self.0
            .current_user
            .lock()
            .expect("current user poisoned")
            .clone()
    }

    /// Gets a user by ID.
    ///
    /// This is an O(1) operation. This requires the [`GUILD_MEMBERS`] intent.
    ///
    /// [`GUILD_MEMBERS`]: ::twilight_model::gateway::Intents::GUILD_MEMBERS
    pub fn user(&self, user_id: UserId) -> Option<User> {
        self.0.users.get(&user_id).map(|r| r.0.clone())
    }

    /// Gets a user by ID.
    ///
    /// This is an O(1) operation. This requires the [`GUILD_MEMBERS`] intent.
    ///
    /// [`GUILD_MEMBERS`]: ::twilight_model::gateway::Intents::GUILD_MEMBERS
    pub fn user_ref(&self, user_id: UserId) -> Option<Ref<'_, UserId, (User, BTreeSet<GuildId>)>> {
        self.0.users.get(&user_id)
    }

    fn cache_current_user(&self, current_user: CurrentUser) {
        self.0
            .current_user
            .lock()
            .expect("current user poisoned")
            .replace(current_user);
    }

    fn cache_user(&self, user: Cow<'_, User>, guild_id: Option<GuildId>) {
        match self.0.users.get_mut(&user.id) {
            Some(mut u) if u.0 == *user => {
                if let Some(guild_id) = guild_id {
                    u.1.insert(guild_id);
                }

                return;
            }
            Some(_) | None => {}
        }
        let user = user.into_owned();

        if let Some(guild_id) = guild_id {
            let mut guild_id_set = BTreeSet::new();
            guild_id_set.insert(guild_id);
            self.0.users.insert(user.id, (user, guild_id_set));
        }
    }

    fn unavailable_guild(&self, guild_id: GuildId) {
        self.0.unavailable_guilds.insert(guild_id);
        self.0.guilds.remove(&guild_id);
    }
}

impl UpdateCache for Ready {
    fn update(&self, cache: &InMemoryCache) {
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

impl UpdateCache for UnavailableGuild {
    fn update(&self, cache: &InMemoryCache) {
        if !cache.wants(ResourceType::GUILD) {
            return;
        }

        cache.0.guilds.remove(&self.id);
        cache.0.unavailable_guilds.insert(self.id);
    }
}

impl UpdateCache for UserUpdate {
    fn update(&self, cache: &InMemoryCache) {
        if !cache.wants(ResourceType::USER_CURRENT) {
            return;
        }

        cache.cache_current_user(self.0.clone());
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test;
    use twilight_model::{id::ApplicationId, oauth::PartialApplication, user::UserFlags};

    /// Test retrieval of the current user, notably that it doesn't simply
    /// panic or do anything funny. This is the only synchronous mutex that we
    /// might have trouble with across await points if we're not careful.
    #[test]
    fn test_current_user_retrieval() {
        let cache = InMemoryCache::new();
        assert!(cache.current_user().is_none());
        cache.cache_current_user(test::current_user(1));
        assert!(cache.current_user().is_some());
    }

    #[test]
    fn test_current_user_lifecycle() {
        let cache = InMemoryCache::new();

        let current_user = CurrentUser {
            avatar: None,
            bot: true,
            discriminator: "1111".into(),
            email: None,
            flags: None,
            id: UserId(2),
            locale: None,
            mfa_enabled: false,
            name: "bot".into(),
            premium_type: None,
            public_flags: None,
            verified: None,
        };

        let event = Ready {
            application: PartialApplication {
                flags: UserFlags::empty(),
                id: ApplicationId(1),
            },
            guilds: Vec::new(),
            session_id: "session_id".into(),
            shard: Some([1, 1]),
            user: current_user.clone(),
            version: 1,
        };
        cache.update(&event);

        {
            let current_user = cache.current_user().unwrap();

            assert_eq!("bot".to_string(), current_user.name);
            assert!(current_user.avatar.is_none());
        }

        let event = UserUpdate(CurrentUser {
            avatar: Some("avatar".into()),
            ..current_user
        });
        cache.update(&event);

        {
            let current_user = cache.current_user().unwrap();

            assert_eq!("avatar".to_string(), current_user.avatar.unwrap());
        }
    }
}
