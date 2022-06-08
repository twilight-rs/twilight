use crate::{config::ResourceType, InMemoryCache, UpdateCache};
use twilight_model::{
    gateway::payload::incoming::{RoleCreate, RoleDelete, RoleUpdate},
    guild::Role,
    id::{
        marker::{GuildMarker, RoleMarker},
        Id,
    },
};

impl InMemoryCache {
    pub(crate) fn cache_roles(
        &self,
        guild_id: Id<GuildMarker>,
        roles: impl IntoIterator<Item = Role>,
    ) {
        for role in roles {
            self.cache_role(guild_id, role);
        }
    }

    fn cache_role(&self, guild_id: Id<GuildMarker>, role: Role) {
        // Insert the role into the guild_roles map
        self.guild_roles
            .entry(guild_id)
            .or_default()
            .insert(role.id);

        // Insert the role into the all roles map
        crate::upsert_guild_item(&self.roles, guild_id, role.id, role);
    }

    fn delete_role(&self, role_id: Id<RoleMarker>) {
        if let Some((_, role)) = self.roles.remove(&role_id) {
            if let Some(mut roles) = self.guild_roles.get_mut(&role.guild_id) {
                roles.remove(&role_id);
            }
        }
    }
}

impl UpdateCache for RoleCreate {
    fn update(&self, cache: &InMemoryCache) {
        if !cache.wants(ResourceType::ROLE) {
            return;
        }

        cache.cache_role(self.guild_id, self.role.clone());
    }
}

impl UpdateCache for RoleDelete {
    fn update(&self, cache: &InMemoryCache) {
        if !cache.wants(ResourceType::ROLE) {
            return;
        }

        cache.delete_role(self.role_id);
    }
}

impl UpdateCache for RoleUpdate {
    fn update(&self, cache: &InMemoryCache) {
        if !cache.wants(ResourceType::ROLE) {
            return;
        }

        cache.cache_role(self.guild_id, self.role.clone());
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test;

    #[test]
    fn insert_role_on_event() {
        let cache = InMemoryCache::new();

        cache.update(&RoleCreate {
            guild_id: Id::new(1),
            role: test::role(Id::new(2)),
        });

        {
            assert_eq!(1, cache.guild_roles.get(&Id::new(1)).unwrap().len());
            assert_eq!(1, cache.roles.len());

            assert_eq!("test".to_string(), cache.role(Id::new(2)).unwrap().name);
        }
    }

    #[test]
    fn cache_role() {
        let cache = InMemoryCache::new();

        // Single inserts
        {
            // The role ids for the guild with id 1
            let guild_1_role_ids = (1..=10).map(Id::new).collect::<Vec<_>>();
            // Map the role ids to a test role
            let guild_1_roles = guild_1_role_ids
                .iter()
                .copied()
                .map(test::role)
                .collect::<Vec<_>>();
            // Cache all the roles using cache role
            for role in guild_1_roles.clone() {
                cache.cache_role(Id::new(1), role);
            }

            // Check for the cached guild role ids
            let cached_roles = cache.guild_roles(Id::new(1)).unwrap();
            assert_eq!(cached_roles.len(), guild_1_role_ids.len());
            assert!(guild_1_role_ids.iter().all(|id| cached_roles.contains(id)));

            // Check for the cached role
            assert!(guild_1_roles.into_iter().all(|role| cache
                .role(role.id)
                .expect("Role missing from cache")
                .resource()
                == &role));
        }

        // Bulk inserts
        {
            // The role ids for the guild with id 2
            let guild_2_role_ids = (101..=110).map(Id::new).collect::<Vec<_>>();
            // Map the role ids to a test role
            let guild_2_roles = guild_2_role_ids
                .iter()
                .copied()
                .map(test::role)
                .collect::<Vec<_>>();
            // Cache all the roles using cache roles
            cache.cache_roles(Id::new(2), guild_2_roles.clone());

            // Check for the cached guild role ids
            let cached_roles = cache.guild_roles(Id::new(2)).unwrap();
            assert_eq!(cached_roles.len(), guild_2_role_ids.len());
            assert!(guild_2_role_ids.iter().all(|id| cached_roles.contains(id)));

            // Check for the cached role
            assert!(guild_2_roles.into_iter().all(|role| cache
                .role(role.id)
                .expect("Role missing from cache")
                .resource()
                == &role));
        }
    }
}
