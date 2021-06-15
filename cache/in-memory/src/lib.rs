//! # twilight-cache-inmemory
//!
//! [![discord badge][]][discord link] [![github badge][]][github link] [![license badge][]][license link] ![rust badge]
//!
//! `twilight-cache-inmemory` is an in-process-memory cache for the
//! [`twilight-rs`] ecosystem. It's responsible for processing events and
//! caching things like guilds, channels, users, and voice states.
//!
//! ## Examples
//!
//! Update a cache with events that come in through the gateway:
//!
//! ```rust,no_run
//! use std::env;
//! use futures::stream::StreamExt;
//! use twilight_cache_inmemory::InMemoryCache;
//! use twilight_gateway::{Intents, Shard};
//!
//! # #[tokio::main] async fn main() -> Result<(), Box<dyn std::error::Error>> {
//! let token = env::var("DISCORD_TOKEN")?;
//! let (shard, mut events) = Shard::new(token, Intents::GUILD_MESSAGES);
//! shard.start().await?;
//!
//! // Create a cache, caching up to 10 messages per channel:
//! let cache = InMemoryCache::builder().message_cache_size(10).build();
//!
//! while let Some(event) = events.next().await {
//!     // Update the cache with the event.
//!     cache.update(&event);
//! }
//! # Ok(()) }
//! ```
//!
//! ## License
//!
//! All first-party crates are licensed under [ISC][LICENSE.md]
//!
//! [LICENSE.md]: https://github.com/twilight-rs/twilight/blob/main/LICENSE.md
//! [discord badge]: https://img.shields.io/discord/745809834183753828?color=%237289DA&label=discord%20server&logo=discord&style=for-the-badge
//! [discord link]: https://discord.gg/7jj8n7D
//! [docs:discord:sharding]: https://discord.com/developers/docs/topics/gateway#sharding
//! [github badge]: https://img.shields.io/badge/github-twilight-6f42c1.svg?style=for-the-badge&logo=github
//! [github link]: https://github.com/twilight-rs/twilight
//! [license badge]: https://img.shields.io/badge/license-ISC-blue.svg?style=for-the-badge&logo=pastebin
//! [license link]: https://github.com/twilight-rs/twilight/blob/main/LICENSE.md
//! [rust badge]: https://img.shields.io/badge/rust-1.49+-93450a.svg?style=for-the-badge&logo=rust

#![deny(
    clippy::missing_const_for_fn,
    broken_intra_doc_links,
    rust_2018_idioms,
    unused,
    warnings
)]

pub mod model;

mod builder;
mod config;
mod event;
mod stats;
mod test;

pub use self::{
    builder::InMemoryCacheBuilder,
    config::{Config, ResourceType},
    stats::InMemoryCacheStats,
};

use self::model::*;
use dashmap::{mapref::entry::Entry, DashMap, DashSet};
use std::{
    collections::{BTreeSet, HashSet, VecDeque},
    hash::Hash,
    ops::Deref,
    sync::{Arc, Mutex},
};
use twilight_model::{
    channel::{Group, GuildChannel, PrivateChannel, StageInstance},
    gateway::event::Event,
    guild::{GuildIntegration, Role},
    id::{ChannelId, EmojiId, GuildId, IntegrationId, RoleId, StageId, UserId},
    user::{CurrentUser, User},
    voice::VoiceState,
};

#[derive(Debug)]
struct GuildItem<T> {
    data: T,
    guild_id: GuildId,
}

fn upsert_guild_item<K: Eq + Hash, V: PartialEq>(
    map: &DashMap<K, GuildItem<V>>,
    guild_id: GuildId,
    key: K,
    value: V,
) {
    match map.entry(key) {
        Entry::Occupied(entry) if entry.get().data == value => {}
        Entry::Occupied(mut entry) => {
            entry.insert(GuildItem {
                data: value,
                guild_id,
            });
        }
        Entry::Vacant(entry) => {
            entry.insert(GuildItem {
                data: value,
                guild_id,
            });
        }
    }
}

fn upsert_item<K: Eq + Hash, V: PartialEq>(map: &DashMap<K, V>, k: K, v: V) {
    map.insert(k, v);
}

// When adding a field here, be sure to add it to `InMemoryCache::clear` if
// necessary.
#[derive(Debug, Default)]
struct InMemoryCacheRef {
    config: Config,
    channels_guild: DashMap<ChannelId, GuildItem<GuildChannel>>,
    channels_private: DashMap<ChannelId, PrivateChannel>,
    // So long as the lock isn't held across await or panic points this is fine.
    current_user: Mutex<Option<CurrentUser>>,
    emojis: DashMap<EmojiId, GuildItem<CachedEmoji>>,
    groups: DashMap<ChannelId, Group>,
    guilds: DashMap<GuildId, CachedGuild>,
    guild_channels: DashMap<GuildId, HashSet<ChannelId>>,
    guild_emojis: DashMap<GuildId, HashSet<EmojiId>>,
    guild_integrations: DashMap<GuildId, HashSet<IntegrationId>>,
    guild_members: DashMap<GuildId, HashSet<UserId>>,
    guild_presences: DashMap<GuildId, HashSet<UserId>>,
    guild_roles: DashMap<GuildId, HashSet<RoleId>>,
    guild_stage_instances: DashMap<GuildId, HashSet<StageId>>,
    integrations: DashMap<(GuildId, IntegrationId), GuildItem<GuildIntegration>>,
    members: DashMap<(GuildId, UserId), CachedMember>,
    messages: DashMap<ChannelId, VecDeque<CachedMessage>>,
    presences: DashMap<(GuildId, UserId), CachedPresence>,
    roles: DashMap<RoleId, GuildItem<Role>>,
    stage_instances: DashMap<StageId, GuildItem<StageInstance>>,
    unavailable_guilds: DashSet<GuildId>,
    users: DashMap<UserId, (User, BTreeSet<GuildId>)>,
    /// Mapping of channels and the users currently connected.
    voice_state_channels: DashMap<ChannelId, HashSet<(GuildId, UserId)>>,
    /// Mapping of guilds and users currently connected to its voice channels.
    voice_state_guilds: DashMap<GuildId, HashSet<UserId>>,
    /// Mapping of guild ID and user ID pairs to their voice states.
    voice_states: DashMap<(GuildId, UserId), VoiceState>,
}

/// A thread-safe, in-memory-process cache of Discord data. It can be cloned and
/// sent to other threads.
///
/// This is an implementation of a cache designed to be used by only the
/// current process.
///
/// Events will only be processed if they are properly expressed with
/// [`Intents`]; refer to function-level documentation for more details.
///
/// # Cloning
///
/// The cache internally wraps its data within an Arc. This means that the cache
/// can be cloned and passed around tasks and threads cheaply.
///
/// # Design and Performance
///
/// The defining characteristic of this cache is that returned types (such as a
/// guild or user) do not use locking for access. The internals of the cache use
/// a concurrent map for mutability and the returned types are clones of the
/// cached data. If a user is retrieved from the cache, then a clone of the user
/// *at that point in time* is returned. If the cache updates the user, then the
/// returned user  held by you will be outdated.
///
/// The intended use is that data is held outside the cache for only as long
/// as necessary, where the state of the value at that point time doesn't need
/// to be up-to-date. If you need to ensure you always have the most up-to-date
/// "version" of a cached resource, then you can re-retrieve it whenever you use
/// it: retrieval operations are extremely cheap.
///
/// For example, say you're deleting some of the guilds of a channel. You'll
/// probably need the guild to do that, so you retrieve it from the cache. You
/// can then use the guild to update all of the channels, because for most use
/// cases you don't need the guild to be up-to-date in real time, you only need
/// its state at that *point in time* or maybe across the lifetime of an
/// operation. If you need the guild to always be up-to-date between operations,
/// then the intent is that you keep getting it from the cache.
///
/// [`Intents`]: ::twilight_model::gateway::Intents
#[derive(Clone, Debug, Default)]
pub struct InMemoryCache(Arc<InMemoryCacheRef>);

/// Implemented methods and types for the cache.
impl InMemoryCache {
    /// Creates a new, empty cache.
    ///
    /// # Examples
    ///
    /// Creating a new `InMemoryCache` with a custom configuration, limiting
    /// the message cache to 50 messages per channel:
    ///
    /// ```
    /// use twilight_cache_inmemory::InMemoryCache;
    ///
    /// let cache = InMemoryCache::builder().message_cache_size(50).build();
    /// ```
    pub fn new() -> Self {
        Self::default()
    }

    fn new_with_config(config: Config) -> Self {
        Self(Arc::new(InMemoryCacheRef {
            config,
            ..Default::default()
        }))
    }

    /// Create a new builder to configure and construct an in-memory cache.
    pub const fn builder() -> InMemoryCacheBuilder {
        InMemoryCacheBuilder::new()
    }

    /// Clear the state of the Cache.
    ///
    /// This is equal to creating a new empty cache.
    pub fn clear(&self) {
        self.0.channels_guild.clear();
        self.0.channels_private.clear();
        self.0
            .current_user
            .lock()
            .expect("current user poisoned")
            .take();
        self.0.emojis.clear();
        self.0.groups.clear();
        self.0.guilds.clear();
        self.0.guild_channels.clear();
        self.0.guild_emojis.clear();
        self.0.guild_integrations.clear();
        self.0.guild_members.clear();
        self.0.guild_presences.clear();
        self.0.guild_roles.clear();
        self.0.guild_stage_instances.clear();
        self.0.integrations.clear();
        self.0.members.clear();
        self.0.messages.clear();
        self.0.presences.clear();
        self.0.roles.clear();
        self.0.unavailable_guilds.clear();
        self.0.users.clear();
        self.0.voice_state_channels.clear();
        self.0.voice_state_guilds.clear();
        self.0.voice_states.clear();
    }

    /// Returns a copy of the config cache.
    pub fn config(&self) -> Config {
        self.0.config.clone()
    }

    /// Create an interface for retrieving statistics about the cache.
    ///
    /// # Examples
    ///
    /// Print the number of guilds in a cache:
    ///
    /// ```
    /// use twilight_cache_inmemory::InMemoryCache;
    ///
    /// let cache = InMemoryCache::new();
    ///
    /// // later on...
    /// let guilds = cache.stats().guilds();
    /// println!("guild count: {}", guilds);
    /// ```
    pub const fn stats(&self) -> InMemoryCacheStats<'_> {
        InMemoryCacheStats::new(self)
    }

    /// Update the cache with an event from the gateway.
    pub fn update(&self, value: &impl UpdateCache) {
        value.update(self);
    }

    /// Determine whether the configured cache wants a specific resource to be
    /// processed.
    fn wants(&self, resource_type: ResourceType) -> bool {
        self.0.config.resource_types().contains(resource_type)
    }
}

pub trait UpdateCache {
    // Allow this for presentation purposes in documentation.
    #[allow(unused_variables)]
    fn update(&self, cache: &InMemoryCache) {}
}

impl UpdateCache for Event {
    #[allow(clippy::cognitive_complexity)]
    fn update(&self, c: &InMemoryCache) {
        use Event::*;

        match self {
            BanAdd(_) => {}
            BanRemove(_) => {}
            ChannelCreate(v) => c.update(v),
            ChannelDelete(v) => c.update(v),
            ChannelPinsUpdate(v) => c.update(v),
            ChannelUpdate(v) => c.update(v),
            GatewayHeartbeat(_) => {}
            GatewayHeartbeatAck => {}
            GatewayHello(_) => {}
            GatewayInvalidateSession(_v) => {}
            GatewayReconnect => {}
            GiftCodeUpdate => {}
            GuildCreate(v) => c.update(v.deref()),
            GuildDelete(v) => c.update(v.deref()),
            GuildEmojisUpdate(v) => c.update(v),
            GuildIntegrationsUpdate(_) => {}
            GuildUpdate(v) => c.update(v.deref()),
            IntegrationCreate(v) => c.update(v.deref()),
            IntegrationDelete(v) => c.update(v.deref()),
            IntegrationUpdate(v) => c.update(v.deref()),
            InteractionCreate(v) => c.update(v.deref()),
            InviteCreate(_) => {}
            InviteDelete(_) => {}
            MemberAdd(v) => c.update(v.deref()),
            MemberRemove(v) => c.update(v),
            MemberUpdate(v) => c.update(v.deref()),
            MemberChunk(v) => c.update(v),
            MessageCreate(v) => c.update(v.deref()),
            MessageDelete(v) => c.update(v),
            MessageDeleteBulk(v) => c.update(v),
            MessageUpdate(v) => c.update(v.deref()),
            PresenceUpdate(v) => c.update(v.deref()),
            PresencesReplace => {}
            ReactionAdd(v) => c.update(v.deref()),
            ReactionRemove(v) => c.update(v.deref()),
            ReactionRemoveAll(v) => c.update(v),
            ReactionRemoveEmoji(v) => c.update(v),
            Ready(v) => c.update(v.deref()),
            Resumed => {}
            RoleCreate(v) => c.update(v),
            RoleDelete(v) => c.update(v),
            RoleUpdate(v) => c.update(v),
            ShardConnected(_) => {}
            ShardConnecting(_) => {}
            ShardDisconnected(_) => {}
            ShardIdentifying(_) => {}
            ShardReconnecting(_) => {}
            ShardPayload(_) => {}
            ShardResuming(_) => {}
            StageInstanceCreate(v) => c.update(v),
            StageInstanceDelete(v) => c.update(v),
            StageInstanceUpdate(v) => c.update(v),
            TypingStart(_) => {}
            UnavailableGuild(v) => c.update(v),
            UserUpdate(v) => c.update(v),
            VoiceServerUpdate(_) => {}
            VoiceStateUpdate(v) => c.update(v.deref()),
            WebhooksUpdate(_) => {}
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{test, InMemoryCache};
    use twilight_model::{
        gateway::payload::RoleDelete,
        id::{EmojiId, GuildId, RoleId, UserId},
    };

    #[test]
    fn test_syntax_update() {
        let cache = InMemoryCache::new();
        cache.update(&RoleDelete {
            guild_id: GuildId(0),
            role_id: RoleId(1),
        });
    }

    #[test]
    fn test_clear() {
        let cache = InMemoryCache::new();
        cache.cache_emoji(GuildId(1), test::emoji(EmojiId(3), None));
        cache.cache_member(GuildId(2), test::member(UserId(4), GuildId(2)));
        cache.clear();
        assert!(cache.0.emojis.is_empty());
        assert!(cache.0.members.is_empty());
    }
}
