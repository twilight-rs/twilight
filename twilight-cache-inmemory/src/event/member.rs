use std::borrow::Cow;

use crate::{
    CacheableModels, InMemoryCache, UpdateCache,
    config::ResourceType,
    model::member::ComputedInteractionMember,
    traits::{CacheableGuild, CacheableMember},
};
use twilight_model::{
    application::interaction::InteractionMember,
    gateway::payload::incoming::{MemberAdd, MemberChunk, MemberRemove, MemberUpdate},
    guild::{Member, PartialMember},
    id::{
        Id,
        marker::{GuildMarker, UserMarker},
    },
};

impl<CacheModels: CacheableModels> InMemoryCache<CacheModels> {
    pub(crate) fn cache_members(
        &self,
        guild_id: Id<GuildMarker>,
        members: impl IntoIterator<Item = Member>,
    ) {
        for member in members {
            self.cache_member(guild_id, member);
        }
    }

    pub(crate) fn cache_member(&self, guild_id: Id<GuildMarker>, member: Member) {
        let member_id = member.user.id;
        let id = (guild_id, member_id);

        if let Some(m) = self.members.get(&id)
            && *m == member
        {
            return;
        }

        self.cache_user(Cow::Borrowed(&member.user), Some(guild_id));
        let cached = CacheModels::Member::from(member);
        self.members.insert(id, cached);
        self.guild_members
            .entry(guild_id)
            .or_default()
            .insert(member_id);
    }

    pub(crate) fn cache_borrowed_partial_member(
        &self,
        guild_id: Id<GuildMarker>,
        member: &PartialMember,
        user_id: Id<UserMarker>,
    ) {
        let id = (guild_id, user_id);

        if let Some(m) = self.members.get(&id)
            && &*m == member
        {
            return;
        }

        self.guild_members
            .entry(guild_id)
            .or_default()
            .insert(user_id);

        let cached = CacheModels::Member::from((user_id, member.clone()));
        self.members.insert(id, cached);
    }

    pub(crate) fn cache_borrowed_interaction_member(
        &self,
        guild_id: Id<GuildMarker>,
        member: &InteractionMember,
        user_id: Id<UserMarker>,
    ) {
        let id = (guild_id, user_id);

        let (avatar, deaf, mute) = match self.members.get(&id) {
            Some(m) if &*m == member => return,
            Some(m) => (m.avatar(), m.deaf(), m.mute()),
            None => (None, None, None),
        };

        self.guild_members
            .entry(guild_id)
            .or_default()
            .insert(user_id);

        let cached = CacheModels::Member::from(ComputedInteractionMember {
            avatar,
            deaf,
            interaction_member: member.clone(),
            mute,
            user_id,
        });

        self.members.insert(id, cached);
    }
}

impl<CacheModels: CacheableModels> UpdateCache<CacheModels> for MemberAdd {
    fn update(&self, cache: &InMemoryCache<CacheModels>) {
        if cache.wants(ResourceType::GUILD)
            && let Some(mut guild) = cache.guilds.get_mut(&self.guild_id)
        {
            guild.increase_member_count(1);
        }

        if !cache.wants(ResourceType::MEMBER) {
            return;
        }

        cache.cache_member(self.guild_id, self.member.clone());
    }
}

impl<CacheModels: CacheableModels> UpdateCache<CacheModels> for MemberChunk {
    fn update(&self, cache: &InMemoryCache<CacheModels>) {
        if !cache.wants(ResourceType::MEMBER) {
            return;
        }

        if self.members.is_empty() {
            return;
        }

        cache.cache_members(self.guild_id, self.members.clone());
    }
}

impl<CacheModels: CacheableModels> UpdateCache<CacheModels> for MemberRemove {
    fn update(&self, cache: &InMemoryCache<CacheModels>) {
        if cache.wants(ResourceType::GUILD)
            && let Some(mut guild) = cache.guilds.get_mut(&self.guild_id)
        {
            guild.decrease_member_count(1);
        }

        if !cache.wants(ResourceType::MEMBER) {
            return;
        }

        cache.members.remove(&(self.guild_id, self.user.id));

        if let Some(mut members) = cache.guild_members.get_mut(&self.guild_id) {
            members.remove(&self.user.id);
        }

        // Avoid a deadlock by mutating the user, dropping the lock to the map,
        // and then removing the user later if they are in no guilds.
        let mut remove_user = false;

        if let Some(mut user_guilds) = cache.user_guilds.get_mut(&self.user.id) {
            user_guilds.remove(&self.guild_id);

            remove_user = user_guilds.is_empty();
        }

        if remove_user {
            cache.users.remove(&self.user.id);
        }
    }
}

impl<CacheModels: CacheableModels> UpdateCache<CacheModels> for MemberUpdate {
    fn update(&self, cache: &InMemoryCache<CacheModels>) {
        if !cache.wants(ResourceType::MEMBER) {
            return;
        }

        let key = (self.guild_id, self.user.id);

        if let Some(mut member) = cache.members.get_mut(&key) {
            member.update_with_member_update(self);
            cache.cache_user(Cow::Borrowed(&self.user), Some(self.guild_id));
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{DefaultInMemoryCache, test};
    use std::borrow::Cow;
    use twilight_model::{
        gateway::payload::incoming::{MemberRemove, MemberUpdate},
        guild::{Member, MemberFlags},
        id::Id,
    };

    #[test]
    fn cache_guild_member() {
        let cache = DefaultInMemoryCache::new();

        // Single inserts
        {
            let guild_1_user_ids = (1..=10).map(Id::new).collect::<Vec<_>>();
            let guild_1_members = guild_1_user_ids
                .iter()
                .copied()
                .map(test::member)
                .collect::<Vec<_>>();

            for member in guild_1_members {
                cache.cache_member(Id::new(1), member);
            }

            // Check for the cached guild members ids
            let cached_roles = cache.guild_members(Id::new(1)).unwrap();
            assert_eq!(cached_roles.len(), guild_1_user_ids.len());
            assert!(guild_1_user_ids.iter().all(|id| cached_roles.contains(id)));

            // Check for the cached members
            assert!(
                guild_1_user_ids
                    .iter()
                    .all(|id| cache.member(Id::new(1), *id).is_some())
            );

            // Check for the cached users
            assert!(guild_1_user_ids.iter().all(|id| cache.user(*id).is_some()));
        }

        // Bulk inserts
        {
            let guild_2_user_ids = (1..=10).map(Id::new).collect::<Vec<_>>();
            let guild_2_members = guild_2_user_ids
                .iter()
                .copied()
                .map(test::member)
                .collect::<Vec<_>>();
            cache.cache_members(Id::new(2), guild_2_members);

            // Check for the cached guild members ids
            let cached_roles = cache.guild_members(Id::new(1)).unwrap();
            assert_eq!(cached_roles.len(), guild_2_user_ids.len());
            assert!(guild_2_user_ids.iter().all(|id| cached_roles.contains(id)));

            // Check for the cached members
            assert!(
                guild_2_user_ids
                    .iter()
                    .copied()
                    .all(|id| cache.member(Id::new(1), id).is_some())
            );

            // Check for the cached users
            assert!(guild_2_user_ids.iter().all(|id| cache.user(*id).is_some()));
        }
    }

    #[test]
    fn cache_user_guild_state() {
        let user_id = Id::new(2);
        let cache = DefaultInMemoryCache::new();
        cache.cache_user(Cow::Owned(test::user(user_id)), Some(Id::new(1)));

        // Test the guild's ID is the only one in the user's set of guilds.
        {
            let user_guilds = cache.user_guilds(user_id).unwrap();
            assert!(user_guilds.contains(&Id::new(1)));
            assert_eq!(1, user_guilds.len());
        }

        // Test that a second guild will cause 2 in the set.
        cache.cache_user(Cow::Owned(test::user(user_id)), Some(Id::new(3)));

        {
            let user_guilds = cache.user_guilds(user_id).unwrap();
            assert!(user_guilds.contains(&Id::new(3)));
            assert_eq!(2, user_guilds.len());
        }

        // Test that removing a user from a guild will cause the ID to be
        // removed from the set, leaving the other ID.
        cache.update(&MemberRemove {
            guild_id: Id::new(3),
            user: test::user(user_id),
        });

        {
            let user_guilds = cache.user_guilds(user_id).unwrap();
            assert!(!user_guilds.contains(&Id::new(3)));
            assert_eq!(1, user_guilds.len());
        }

        // Test that removing the user from its last guild removes the user's
        // entry.
        cache.update(&MemberRemove {
            guild_id: Id::new(1),
            user: test::user(user_id),
        });
        assert!(!cache.users.contains_key(&user_id));
    }

    #[test]
    fn member_update_updates_user() {
        let user_id = Id::new(2);
        let guild_id = Id::new(3);
        let cache = DefaultInMemoryCache::new();

        let member = Member {
            avatar: None,
            communication_disabled_until: None,
            deaf: false,
            flags: MemberFlags::empty(),
            joined_at: None,
            mute: false,
            nick: None,
            pending: false,
            premium_since: None,
            roles: Vec::new(),
            user: test::user(user_id),
        };

        cache.cache_member(guild_id, member);

        let mut updated_user = test::user(user_id);
        updated_user.name = String::from("updated_username");

        // Test that a member update also updates the user.
        cache.update(&MemberUpdate {
            avatar: None,
            communication_disabled_until: None,
            guild_id,
            flags: None,
            deaf: None,
            joined_at: None,
            mute: None,
            nick: None,
            pending: false,
            premium_since: None,
            roles: Vec::new(),
            user: updated_user.clone(),
        });

        let cached_user = cache.user(user_id).unwrap();
        assert_eq!(cached_user.value(), &updated_user);
    }
}
