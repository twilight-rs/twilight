//! Iterators over the various resources stored in the cache.
//!
//! The entry point to the module is [`InMemoryCacheIter`]. It exposes methods
//! for creating iterators over various resource types, such as
//! [`InMemoryCacheIter::integrations`] and [`InMemoryCacheIter::voice_states`].
//!
//! Creating an iterator returns a [`ResourceIter`]. The iterator implements the
//! [`std::iter::Iterator`] trait and returns [`IterReference`]s.
//!
//! [`IterReference`] exposes two methods: [`IterReference::key`] and
//! [`IterReference::value`], both returning immutable references to the
//! underlying key and value. It also implements [`std::ops::Deref`] and
//! dereferences to the value.

use crate::{CacheableModels, GuildResource, InMemoryCache};
use dashmap::{iter::Iter, mapref::multiple::RefMulti};
use std::{hash::Hash, ops::Deref};
use twilight_model::id::{
    marker::{
        ChannelMarker, EmojiMarker, GuildMarker, IntegrationMarker, MessageMarker, RoleMarker,
        StageMarker, StickerMarker, UserMarker,
    },
    Id,
};

/// Reference to a resource value being iterated over in the cache.
///
/// [`std::ops::Deref`] is implemented on this type and derefs to an immutable
/// reference of the underlying value.
// We need a separate type from [`Reference`] due to DashMap's iterators
// returning a different reference type from that of retrieval methods.
//
// [`Reference`]: super::Reference
pub struct IterReference<'a, K, V> {
    inner: RefMulti<'a, K, V>,
}

impl<'a, K, V> IterReference<'a, K, V> {
    /// Create a new iterator element reference.
    const fn new(inner: RefMulti<'a, K, V>) -> Self {
        Self { inner }
    }
}

impl<K: Eq + Hash, V> IterReference<'_, K, V> {
    /// Immutable reference to the resource's key.
    pub fn key(&self) -> &K {
        self.inner.key()
    }

    /// Immutable reference to the resource's value.
    pub fn value(&self) -> &V {
        self.inner.value()
    }
}

impl<K: Eq + Hash, V> Deref for IterReference<'_, K, V> {
    type Target = V;

    fn deref(&self) -> &Self::Target {
        self.value()
    }
}

/// Interface to create iterators over various resources.
///
/// The created iterators will iterate over *all* entities of a resource across
/// all channels and guilds.
///
/// The iteration order of all iterators are arbitrary.
///
/// # Examples
///
/// Count the number of users in the cache whose username begins with "twi":
///
/// ```no_run
/// use twilight_cache_inmemory::DefaultInMemoryCache;
///
/// let cache = DefaultInMemoryCache::new();
///
/// // later in the application...
/// let count = cache
///     .iter()
///     .users()
///     .filter(|user| user.name.starts_with("twi"))
///     .count();
///
/// println!("'twi' users: {count}");
/// ```
///
/// # Potential inefficiency
///
/// Resource iterators over the entire cache are inefficient when the goal is to
/// iterate over a resource in a specific guild. For example, when performing a
/// task such as iterating over the members of a specific guild, retrieving the
/// list of members via [`InMemoryCache::guild_members`] and then calling
/// [`InMemoryCache::member`] for each item is more efficient. That might look
/// like:
///
/// ```no_run
/// use twilight_cache_inmemory::DefaultInMemoryCache;
/// use twilight_model::id::Id;
///
/// let cache = DefaultInMemoryCache::new();
///
/// // later in the application...
/// let guild_id = Id::new(1);
/// let maybe_guild_members = cache.guild_members(guild_id);
///
/// if let Some(guild_members) = maybe_guild_members {
///     for user_id in guild_members.iter() {
///         if let Some(member) = cache.member(guild_id, *user_id) {
///             println!(
///                 "member id {}'s nickname: {:?}",
///                 member.user_id(),
///                 member.nick(),
///             );
///         }
///     }
/// }
/// ```
#[allow(clippy::type_complexity)]
#[derive(Debug)]
pub struct InMemoryCacheIter<'a, CacheModels: CacheableModels>(&'a InMemoryCache<CacheModels>);

impl<'a, CacheModels: CacheableModels> InMemoryCacheIter<'a, CacheModels> {
    /// Create a new interface to create iterators over various resource types.
    #[allow(clippy::type_complexity)]
    pub(super) const fn new(cache: &'a InMemoryCache<CacheModels>) -> Self {
        Self(cache)
    }

    /// Immutable reference to the underlying cache.
    #[allow(clippy::type_complexity)]
    pub const fn cache_ref(&'a self) -> &'a InMemoryCache<CacheModels> {
        self.0
    }

    /// Create an iterator over the channels in the cache.
    pub fn channels(&self) -> ResourceIter<'a, Id<ChannelMarker>, CacheModels::Channel> {
        ResourceIter::new(self.0.channels.iter())
    }

    /// Create an iterator over the emojis in the cache.
    pub fn emojis(&self) -> ResourceIter<'a, Id<EmojiMarker>, GuildResource<CacheModels::Emoji>> {
        ResourceIter::new(self.0.emojis.iter())
    }

    /// Create an iterator over the guilds in the cache.
    pub fn guilds(&self) -> ResourceIter<'a, Id<GuildMarker>, CacheModels::Guild> {
        ResourceIter::new(self.0.guilds.iter())
    }

    /// Create an iterator over the integrations in the cache.
    pub fn integrations(
        &self,
    ) -> ResourceIter<
        'a,
        (Id<GuildMarker>, Id<IntegrationMarker>),
        GuildResource<CacheModels::GuildIntegration>,
    > {
        ResourceIter::new(self.0.integrations.iter())
    }

    /// Create an iterator over the members across all guilds in the cache.
    pub fn members(
        &self,
    ) -> ResourceIter<'a, (Id<GuildMarker>, Id<UserMarker>), CacheModels::Member> {
        ResourceIter::new(self.0.members.iter())
    }

    /// Create an iterator over the messages in the cache.
    pub fn messages(&self) -> ResourceIter<'a, Id<MessageMarker>, CacheModels::Message> {
        ResourceIter::new(self.0.messages.iter())
    }

    /// Create an iterator over the presences in the cache.
    pub fn presences(
        &self,
    ) -> ResourceIter<'a, (Id<GuildMarker>, Id<UserMarker>), CacheModels::Presence> {
        ResourceIter::new(self.0.presences.iter())
    }

    /// Create an iterator over the roles in the cache.
    pub fn roles(&self) -> ResourceIter<'a, Id<RoleMarker>, GuildResource<CacheModels::Role>> {
        ResourceIter::new(self.0.roles.iter())
    }

    /// Create an iterator over the stage instances in the cache.
    pub fn stage_instances(
        &self,
    ) -> ResourceIter<'a, Id<StageMarker>, GuildResource<CacheModels::StageInstance>> {
        ResourceIter::new(self.0.stage_instances.iter())
    }

    /// Create an iterator over the stickers in the cache.
    pub fn stickers(
        &self,
    ) -> ResourceIter<'a, Id<StickerMarker>, GuildResource<CacheModels::Sticker>> {
        ResourceIter::new(self.0.stickers.iter())
    }

    /// Create an iterator over the users in the cache.
    pub fn users(&self) -> ResourceIter<'a, Id<UserMarker>, CacheModels::User> {
        ResourceIter::new(self.0.users.iter())
    }

    /// Create an iterator over the voice states in the cache.
    pub fn voice_states(
        &self,
    ) -> ResourceIter<'a, (Id<GuildMarker>, Id<UserMarker>), CacheModels::VoiceState> {
        ResourceIter::new(self.0.voice_states.iter())
    }
}

/// Generic iterator over key-value pairs of a resource.
///
/// The iteration order is arbitrary.
///
/// # Examples
///
/// Count how many users across all guilds are pending:
///
/// ```no_run
/// use twilight_cache_inmemory::DefaultInMemoryCache;
///
/// let cache = DefaultInMemoryCache::new();
///
/// // later in the application...
/// let count = cache
///     .iter()
///     .members()
///     .filter(|member| member.pending())
///     .count();
///
/// println!("pending users: {count}");
/// ```
pub struct ResourceIter<'a, K, V> {
    iter: Iter<'a, K, V>,
}

impl<'a, K, V> ResourceIter<'a, K, V> {
    /// Create a new iterator over a resource.
    pub(super) const fn new(iter: Iter<'a, K, V>) -> Self {
        Self { iter }
    }
}

impl<'a, K: Eq + Hash, V> Iterator for ResourceIter<'a, K, V> {
    type Item = IterReference<'a, K, V>;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(IterReference::new)
    }
}

#[cfg(test)]
mod tests {
    use super::{InMemoryCacheIter, IterReference, ResourceIter};
    use crate::{test, DefaultCacheModels, DefaultInMemoryCache};
    use static_assertions::assert_impl_all;
    use std::{borrow::Cow, fmt::Debug};
    use twilight_model::{
        id::{marker::UserMarker, Id},
        user::User,
    };

    assert_impl_all!(InMemoryCacheIter<'_, DefaultCacheModels>: Debug, Send, Sync);
    assert_impl_all!(IterReference<'_, Id<UserMarker>, User>: Send, Sync);
    assert_impl_all!(ResourceIter<'_, Id<UserMarker>, User>: Iterator, Send, Sync);

    #[test]
    fn iter() {
        let guild_id = Id::new(1);
        let users = &[
            (Id::new(2), Some(guild_id)),
            (Id::new(3), Some(guild_id)),
            (Id::new(4), None),
        ];
        let cache = DefaultInMemoryCache::new();

        for (user_id, maybe_guild_id) in users {
            cache.cache_user(Cow::Owned(test::user(*user_id)), *maybe_guild_id);
        }

        let mut actual = cache.iter().users().map(|user| user.id).collect::<Vec<_>>();
        actual.sort_unstable();

        let expected = users.iter().map(|(id, _)| *id).collect::<Vec<_>>();

        assert_eq!(actual, expected);
    }
}
