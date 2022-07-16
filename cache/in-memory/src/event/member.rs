use crate::{
    config::ResourceType,
    model::{member::ComputedInteractionMemberFields, CachedMember},
    InMemoryCache, UpdateCache,
};
use std::borrow::Cow;
use twilight_model::{
    application::interaction::application_command::InteractionMember,
    gateway::payload::incoming::{MemberAdd, MemberChunk, MemberRemove, MemberUpdate},
    guild::{Member, PartialMember},
    id::{
        marker::{GuildMarker, UserMarker},
        Id,
    },
};

impl InMemoryCache {
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

        if let Some(m) = self.members.get(&id) {
            if *m == member {
                return;
            }
        }

        self.cache_user(Cow::Borrowed(&member.user), Some(guild_id));
        let cached = CachedMember::from_model(member);
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

        if let Some(m) = self.members.get(&id) {
            if &*m == member {
                return;
            }
        }

        self.guild_members
            .entry(guild_id)
            .or_default()
            .insert(user_id);

        let cached = CachedMember::from_partial_member(guild_id, user_id, member.clone());
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

        let cached = CachedMember::from_interaction_member(
            guild_id,
            user_id,
            member.clone(),
            ComputedInteractionMemberFields { avatar, deaf, mute },
        );

        self.members.insert(id, cached);
    }
}

impl UpdateCache for MemberAdd {
    fn update(&self, cache: &InMemoryCache) {
        if cache.wants(ResourceType::GUILD) {
            if let Some(mut guild) = cache.guilds.get_mut(&self.guild_id) {
                guild.member_count = guild.member_count.map(|count| count + 1);
            }
        }

        if !cache.wants(ResourceType::MEMBER) {
            return;
        }

        cache.cache_member(self.guild_id, self.0.clone());

        cache
            .guild_members
            .entry(self.guild_id)
            .or_default()
            .insert(self.0.user.id);
    }
}

impl UpdateCache for MemberChunk {
    fn update(&self, cache: &InMemoryCache) {
        if !cache.wants(ResourceType::MEMBER) {
            return;
        }

        if self.members.is_empty() {
            return;
        }

        cache.cache_members(self.guild_id, self.members.clone());
        let mut guild = cache.guild_members.entry(self.guild_id).or_default();
        guild.extend(self.members.iter().map(|member| member.user.id));
    }
}

impl UpdateCache for MemberRemove {
    fn update(&self, cache: &InMemoryCache) {
        if cache.wants(ResourceType::GUILD) {
            if let Some(mut guild) = cache.guilds.get_mut(&self.guild_id) {
                guild.member_count = guild.member_count.map(|count| count - 1);
            }
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

impl UpdateCache for MemberUpdate {
    fn update(&self, cache: &InMemoryCache) {
        if !cache.wants(ResourceType::MEMBER) {
            return;
        }

        let key = (self.guild_id, self.user.id);

        let mut member = if let Some(member) = cache.members.get_mut(&key) {
            member
        } else {
            return;
        };

        member.avatar = self.avatar;
        member.deaf = self.deaf.or_else(|| member.deaf());
        member.mute = self.mute.or_else(|| member.mute());
        member.nick = self.nick.clone();
        member.roles = self.roles.clone();
        member.joined_at = self.joined_at;
        member.pending = self.pending;
        member.communication_disabled_until = self.communication_disabled_until;
    }
}

#[cfg(test)]
mod tests {
    use crate::{test, InMemoryCache};
    use std::borrow::Cow;
    use twilight_model::{gateway::payload::incoming::MemberRemove, id::Id};

    #[test]
    fn cache_guild_member() {
        let cache = InMemoryCache::new();

        // Single inserts
        {
            let guild_1_user_ids = (1..=10).map(Id::new).collect::<Vec<_>>();
            let guild_1_members = guild_1_user_ids
                .iter()
                .copied()
                .map(|id| test::member(id, Id::new(1)))
                .collect::<Vec<_>>();

            for member in guild_1_members {
                cache.cache_member(Id::new(1), member);
            }

            // Check for the cached guild members ids
            let cached_roles = cache.guild_members(Id::new(1)).unwrap();
            assert_eq!(cached_roles.len(), guild_1_user_ids.len());
            assert!(guild_1_user_ids.iter().all(|id| cached_roles.contains(id)));

            // Check for the cached members
            assert!(guild_1_user_ids
                .iter()
                .all(|id| cache.member(Id::new(1), *id).is_some()));

            // Check for the cached users
            assert!(guild_1_user_ids.iter().all(|id| cache.user(*id).is_some()));
        }

        // Bulk inserts
        {
            let guild_2_user_ids = (1..=10).map(Id::new).collect::<Vec<_>>();
            let guild_2_members = guild_2_user_ids
                .iter()
                .copied()
                .map(|id| test::member(id, Id::new(2)))
                .collect::<Vec<_>>();
            cache.cache_members(Id::new(2), guild_2_members);

            // Check for the cached guild members ids
            let cached_roles = cache.guild_members(Id::new(1)).unwrap();
            assert_eq!(cached_roles.len(), guild_2_user_ids.len());
            assert!(guild_2_user_ids.iter().all(|id| cached_roles.contains(id)));

            // Check for the cached members
            assert!(guild_2_user_ids
                .iter()
                .copied()
                .all(|id| cache.member(Id::new(1), id).is_some()));

            // Check for the cached users
            assert!(guild_2_user_ids.iter().all(|id| cache.user(*id).is_some()));
        }
    }

    #[test]
    fn cache_user_guild_state() {
        let user_id = Id::new(2);
        let cache = InMemoryCache::new();
        cache.cache_user(Cow::Owned(test::user(user_id)), Some(Id::new(1)));

        // Test the guild's ID is the only one in the user's set of guilds.
        {
            let user_guilds = cache.user_guilds.get(&user_id).unwrap();
            assert!(user_guilds.contains(&Id::new(1)));
            assert_eq!(1, user_guilds.len());
        }

        // Test that a second guild will cause 2 in the set.
        cache.cache_user(Cow::Owned(test::user(user_id)), Some(Id::new(3)));

        {
            let user_guilds = cache.user_guilds.get(&user_id).unwrap();
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
            let user_guilds = cache.user_guilds.get(&user_id).unwrap();
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
}
