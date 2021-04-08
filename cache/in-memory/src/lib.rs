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
//! let mut shard = Shard::new(token, Intents::GUILD_MESSAGES);
//! shard.start().await?;
//!
//! // Create a cache, caching up to 10 messages per channel:
//! let cache = InMemoryCache::builder().message_cache_size(10).build();
//!
//! let mut events = shard.events();
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
//! [LICENSE.md]: https://github.com/twilight-rs/twilight/blob/trunk/LICENSE.md
//! [discord badge]: https://img.shields.io/discord/745809834183753828?color=%237289DA&label=discord%20server&logo=discord&style=for-the-badge
//! [discord link]: https://discord.gg/7jj8n7D
//! [docs:discord:sharding]: https://discord.com/developers/docs/topics/gateway#sharding
//! [github badge]: https://img.shields.io/badge/github-twilight-6f42c1.svg?style=for-the-badge&logo=github
//! [github link]: https://github.com/twilight-rs/twilight
//! [license badge]: https://img.shields.io/badge/license-ISC-blue.svg?style=for-the-badge&logo=pastebin
//! [license link]: https://github.com/twilight-rs/twilight/blob/trunk/LICENSE.md
//! [rust badge]: https://img.shields.io/badge/rust-1.48+-93450a.svg?style=for-the-badge&logo=rust

#![deny(rust_2018_idioms, broken_intra_doc_links, unused, warnings)]

pub mod model;

mod builder;
mod config;
mod updates;

pub use self::{
    builder::InMemoryCacheBuilder,
    config::{Config, ResourceType},
    updates::UpdateCache,
};

use self::model::*;
use dashmap::{mapref::entry::Entry, DashMap, DashSet};
use std::{
    borrow::Cow,
    collections::{BTreeMap, BTreeSet, HashSet},
    hash::Hash,
    sync::{Arc, Mutex},
};
use twilight_model::{
    channel::{Group, GuildChannel, PrivateChannel},
    gateway::presence::{Presence, UserOrId},
    guild::{Emoji, Guild, Member, PartialMember, Role},
    id::{ChannelId, EmojiId, GuildId, MessageId, RoleId, UserId},
    user::{CurrentUser, User},
    voice::VoiceState,
};

#[derive(Debug)]
struct GuildItem<T> {
    data: Arc<T>,
    guild_id: GuildId,
}

fn upsert_guild_item<K: Eq + Hash, V: PartialEq>(
    map: &DashMap<K, GuildItem<V>>,
    guild_id: GuildId,
    k: K,
    v: V,
) -> Arc<V> {
    match map.entry(k) {
        Entry::Occupied(e) if *e.get().data == v => Arc::clone(&e.get().data),
        Entry::Occupied(mut e) => {
            let v = Arc::new(v);
            e.insert(GuildItem {
                data: Arc::clone(&v),
                guild_id,
            });

            v
        }
        Entry::Vacant(e) => Arc::clone(
            &e.insert(GuildItem {
                data: Arc::new(v),
                guild_id,
            })
            .data,
        ),
    }
}

fn upsert_item<K: Eq + Hash, V: PartialEq>(map: &DashMap<K, Arc<V>>, k: K, v: V) -> Arc<V> {
    match map.entry(k) {
        Entry::Occupied(e) if **e.get() == v => Arc::clone(e.get()),
        Entry::Occupied(mut e) => {
            let v = Arc::new(v);
            e.insert(Arc::clone(&v));

            v
        }
        Entry::Vacant(e) => {
            let v = Arc::new(v);
            e.insert(Arc::clone(&v));

            v
        }
    }
}

// When adding a field here, be sure to add it to `InMemoryCache::clear` if
// necessary.
#[derive(Debug, Default)]
struct InMemoryCacheRef {
    config: Arc<Config>,
    channels_guild: DashMap<ChannelId, GuildItem<GuildChannel>>,
    channels_private: DashMap<ChannelId, Arc<PrivateChannel>>,
    // So long as the lock isn't held across await or panic points this is fine.
    current_user: Mutex<Option<Arc<CurrentUser>>>,
    emojis: DashMap<EmojiId, GuildItem<CachedEmoji>>,
    groups: DashMap<ChannelId, Arc<Group>>,
    guilds: DashMap<GuildId, Arc<CachedGuild>>,
    guild_channels: DashMap<GuildId, HashSet<ChannelId>>,
    guild_emojis: DashMap<GuildId, HashSet<EmojiId>>,
    guild_members: DashMap<GuildId, HashSet<UserId>>,
    guild_presences: DashMap<GuildId, HashSet<UserId>>,
    guild_roles: DashMap<GuildId, HashSet<RoleId>>,
    members: DashMap<(GuildId, UserId), Arc<CachedMember>>,
    messages: DashMap<ChannelId, BTreeMap<MessageId, Arc<CachedMessage>>>,
    presences: DashMap<(GuildId, UserId), Arc<CachedPresence>>,
    roles: DashMap<RoleId, GuildItem<Role>>,
    unavailable_guilds: DashSet<GuildId>,
    users: DashMap<UserId, (Arc<User>, BTreeSet<GuildId>)>,
    /// Mapping of channels and the users currently connected.
    voice_state_channels: DashMap<ChannelId, HashSet<(GuildId, UserId)>>,
    /// Mapping of guilds and users currently connected to its voice channels.
    voice_state_guilds: DashMap<GuildId, HashSet<UserId>>,
    /// Mapping of guild ID and user ID pairs to their voice states.
    voice_states: DashMap<(GuildId, UserId), Arc<VoiceState>>,
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
/// a concurrent map for mutability and the returned types themselves are Arcs.
/// If a user is retrieved from the cache, an `Arc<User>` is returned. If a
/// reference to that user is held but the cache updates the user, the reference
/// held by you will be outdated, but still exist.
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
            config: Arc::new(config),
            ..Default::default()
        }))
    }

    /// Create a new builder to configure and construct an in-memory cache.
    pub fn builder() -> InMemoryCacheBuilder {
        InMemoryCacheBuilder::new()
    }

    /// Returns a copy of the config cache.
    pub fn config(&self) -> Config {
        (*self.0.config).clone()
    }

    /// Update the cache with an event from the gateway.
    pub fn update(&self, value: &impl UpdateCache) {
        value.update(self);
    }

    /// Gets a channel by ID.
    ///
    /// This is an O(1) operation. This requires the [`GUILDS`] intent.
    ///
    /// [`GUILDS`]: ::twilight_model::gateway::Intents::GUILDS
    pub fn guild_channel(&self, channel_id: ChannelId) -> Option<Arc<GuildChannel>> {
        self.0
            .channels_guild
            .get(&channel_id)
            .map(|x| Arc::clone(&x.data))
    }

    /// Gets the current user.
    ///
    /// This is an O(1) operation.
    pub fn current_user(&self) -> Option<Arc<CurrentUser>> {
        self.0
            .current_user
            .lock()
            .expect("current user poisoned")
            .clone()
    }

    /// Gets an emoji by ID.
    ///
    /// This is an O(1) operation. This requires the [`GUILD_EMOJIS`] intent.
    ///
    /// [`GUILD_EMOJIS`]: ::twilight_model::gateway::Intents::GUILD_EMOJIS
    pub fn emoji(&self, emoji_id: EmojiId) -> Option<Arc<CachedEmoji>> {
        self.0.emojis.get(&emoji_id).map(|x| Arc::clone(&x.data))
    }

    /// Gets a group by ID.
    ///
    /// This is an O(1) operation.
    pub fn group(&self, channel_id: ChannelId) -> Option<Arc<Group>> {
        self.0
            .groups
            .get(&channel_id)
            .map(|r| Arc::clone(r.value()))
    }

    /// Gets a guild by ID.
    ///
    /// This is an O(1) operation. This requires the [`GUILDS`] intent.
    ///
    /// [`GUILDS`]: ::twilight_model::gateway::Intents::GUILDS
    pub fn guild(&self, guild_id: GuildId) -> Option<Arc<CachedGuild>> {
        self.0.guilds.get(&guild_id).map(|r| Arc::clone(r.value()))
    }

    /// Gets the set of channels in a guild.
    ///
    /// This is a O(m) operation, where m is the amount of channels in the
    /// guild. This requires the [`GUILDS`] intent.
    ///
    /// [`GUILDS`]: ::twilight_model::gateway::Intents::GUILDS
    pub fn guild_channels(&self, guild_id: GuildId) -> Option<HashSet<ChannelId>> {
        self.0
            .guild_channels
            .get(&guild_id)
            .map(|r| r.value().clone())
    }

    /// Gets the set of emojis in a guild.
    ///
    /// This is a O(m) operation, where m is the amount of emojis in the guild.
    /// This requires both the [`GUILDS`] and [`GUILD_EMOJIS`] intents.
    ///
    /// [`GUILDS`]: ::twilight_model::gateway::Intents::GUILDS
    /// [`GUILD_EMOJIS`]: ::twilight_model::gateway::Intents::GUILD_EMOJIS
    pub fn guild_emojis(&self, guild_id: GuildId) -> Option<HashSet<EmojiId>> {
        self.0
            .guild_emojis
            .get(&guild_id)
            .map(|r| r.value().clone())
    }

    /// Gets the set of members in a guild.
    ///
    /// This list may be incomplete if not all members have been cached.
    ///
    /// This is a O(m) operation, where m is the amount of members in the guild.
    /// This requires the [`GUILD_MEMBERS`] intent.
    ///
    /// [`GUILD_MEMBERS`]: ::twilight_model::gateway::Intents::GUILD_MEMBERS
    pub fn guild_members(&self, guild_id: GuildId) -> Option<HashSet<UserId>> {
        self.0
            .guild_members
            .get(&guild_id)
            .map(|r| r.value().clone())
    }

    /// Gets the set of presences in a guild.
    ///
    /// This list may be incomplete if not all members have been cached.
    ///
    /// This is a O(m) operation, where m is the amount of members in the guild.
    /// This requires the [`GUILD_PRESENCES`] intent.
    ///
    /// [`GUILD_PRESENCES`]: ::twilight_model::gateway::Intents::GUILD_PRESENCES
    pub fn guild_presences(&self, guild_id: GuildId) -> Option<HashSet<UserId>> {
        self.0
            .guild_presences
            .get(&guild_id)
            .map(|r| r.value().clone())
    }

    /// Gets the set of roles in a guild.
    ///
    /// This is a O(m) operation, where m is the amount of roles in the guild.
    /// This requires the [`GUILDS`] intent.
    ///
    /// [`GUILDS`]: ::twilight_model::gateway::Intents::GUILDS
    pub fn guild_roles(&self, guild_id: GuildId) -> Option<HashSet<RoleId>> {
        self.0.guild_roles.get(&guild_id).map(|r| r.value().clone())
    }

    /// Gets a member by guild ID and user ID.
    ///
    /// This is an O(1) operation. This requires the [`GUILD_MEMBERS`] intent.
    ///
    /// [`GUILD_MEMBERS`]: ::twilight_model::gateway::Intents::GUILD_MEMBERS
    pub fn member(&self, guild_id: GuildId, user_id: UserId) -> Option<Arc<CachedMember>> {
        self.0
            .members
            .get(&(guild_id, user_id))
            .map(|r| Arc::clone(r.value()))
    }

    /// Gets a message by channel ID and message ID.
    ///
    /// This is an O(log n) operation. This requires one or both of the
    /// [`GUILD_MESSAGES`] or [`DIRECT_MESSAGES`] intents.
    ///
    /// [`GUILD_MESSAGES`]: ::twilight_model::gateway::Intents::GUILD_MESSAGES
    /// [`DIRECT_MESSAGES`]: ::twilight_model::gateway::Intents::DIRECT_MESSAGES
    pub fn message(
        &self,
        channel_id: ChannelId,
        message_id: MessageId,
    ) -> Option<Arc<CachedMessage>> {
        let channel = self.0.messages.get(&channel_id)?;

        channel.get(&message_id).cloned()
    }

    /// Gets a presence by, optionally, guild ID, and user ID.
    ///
    /// This is an O(1) operation. This requires the [`GUILD_PRESENCES`] intent.
    ///
    /// [`GUILD_PRESENCES`]: ::twilight_model::gateway::Intents::GUILD_PRESENCES
    pub fn presence(&self, guild_id: GuildId, user_id: UserId) -> Option<Arc<CachedPresence>> {
        self.0
            .presences
            .get(&(guild_id, user_id))
            .map(|r| Arc::clone(r.value()))
    }

    /// Gets a private channel by ID.
    ///
    /// This is an O(1) operation. This requires the [`DIRECT_MESSAGES`] intent.
    ///
    /// [`DIRECT_MESSAGES`]: ::twilight_model::gateway::Intents::DIRECT_MESSAGES
    pub fn private_channel(&self, channel_id: ChannelId) -> Option<Arc<PrivateChannel>> {
        self.0
            .channels_private
            .get(&channel_id)
            .map(|r| Arc::clone(r.value()))
    }

    /// Gets a role by ID.
    ///
    /// This is an O(1) operation. This requires the [`GUILDS`] intent.
    ///
    /// [`GUILDS`]: ::twilight_model::gateway::Intents::GUILDS
    pub fn role(&self, role_id: RoleId) -> Option<Arc<Role>> {
        self.0
            .roles
            .get(&role_id)
            .map(|role| Arc::clone(&role.data))
    }

    /// Gets a user by ID.
    ///
    /// This is an O(1) operation. This requires the [`GUILD_MEMBERS`] intent.
    ///
    /// [`GUILD_MEMBERS`]: ::twilight_model::gateway::Intents::GUILD_MEMBERS
    pub fn user(&self, user_id: UserId) -> Option<Arc<User>> {
        self.0.users.get(&user_id).map(|r| Arc::clone(&r.0))
    }

    /// Gets the voice states within a voice channel.
    ///
    /// This requires both the [`GUILDS`] and [`GUILD_VOICE_STATES`] intents.
    ///
    /// [`GUILDS`]: ::twilight_model::gateway::Intents::GUILDS
    /// [`GUILD_VOICE_STATES`]: ::twilight_model::gateway::Intents::GUILD_VOICE_STATES
    pub fn voice_channel_states(&self, channel_id: ChannelId) -> Option<Vec<Arc<VoiceState>>> {
        let user_ids = self.0.voice_state_channels.get(&channel_id)?;

        Some(
            user_ids
                .iter()
                .filter_map(|key| self.0.voice_states.get(&key).map(|r| Arc::clone(r.value())))
                .collect(),
        )
    }

    /// Gets a voice state by user ID and Guild ID.
    ///
    /// This is an O(1) operation. This requires both the [`GUILDS`] and
    /// [`GUILD_VOICE_STATES`] intents.
    ///
    /// [`GUILDS`]: ::twilight_model::gateway::Intents::GUILDS
    /// [`GUILD_VOICE_STATES`]: ::twilight_model::gateway::Intents::GUILD_VOICE_STATES
    pub fn voice_state(&self, user_id: UserId, guild_id: GuildId) -> Option<Arc<VoiceState>> {
        self.0
            .voice_states
            .get(&(guild_id, user_id))
            .map(|r| Arc::clone(r.value()))
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
        self.0.guild_members.clear();
        self.0.guild_presences.clear();
        self.0.guild_roles.clear();
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

    fn cache_current_user(&self, mut current_user: CurrentUser) {
        let mut user = self.0.current_user.lock().expect("current user poisoned");

        if let Some(mut user) = user.as_mut() {
            if let Some(user) = Arc::get_mut(&mut user) {
                std::mem::swap(user, &mut current_user);

                return;
            }
        }

        *user = Some(Arc::new(current_user));
    }

    fn cache_guild_channels(
        &self,
        guild_id: GuildId,
        guild_channels: impl IntoIterator<Item = GuildChannel>,
    ) {
        for channel in guild_channels {
            self.cache_guild_channel(guild_id, channel);
        }
    }

    fn cache_guild_channel(
        &self,
        guild_id: GuildId,
        mut channel: GuildChannel,
    ) -> Arc<GuildChannel> {
        match channel {
            GuildChannel::Category(ref mut c) => {
                c.guild_id.replace(guild_id);
            }
            GuildChannel::Text(ref mut c) => {
                c.guild_id.replace(guild_id);
            }
            GuildChannel::Voice(ref mut c) => {
                c.guild_id.replace(guild_id);
            }
        }

        let id = channel.id();
        self.0
            .guild_channels
            .entry(guild_id)
            .or_default()
            .insert(id);

        upsert_guild_item(&self.0.channels_guild, guild_id, id, channel)
    }

    fn cache_emoji(&self, guild_id: GuildId, emoji: Emoji) -> Arc<CachedEmoji> {
        match self.0.emojis.get(&emoji.id) {
            Some(e) if *e.data == emoji => return Arc::clone(&e.data),
            Some(_) | None => {}
        }

        let user = emoji
            .user
            .map(|u| self.cache_user(Cow::Owned(u), Some(guild_id)));

        let cached = Arc::new(CachedEmoji {
            id: emoji.id,
            animated: emoji.animated,
            name: emoji.name,
            managed: emoji.managed,
            require_colons: emoji.require_colons,
            roles: emoji.roles,
            user,
            available: emoji.available,
        });

        self.0.emojis.insert(
            cached.id,
            GuildItem {
                data: Arc::clone(&cached),
                guild_id,
            },
        );

        self.0
            .guild_emojis
            .entry(guild_id)
            .or_default()
            .insert(emoji.id);

        cached
    }

    fn cache_emojis(&self, guild_id: GuildId, emojis: Vec<Emoji>) {
        if let Some(mut guild_emojis) = self.0.guild_emojis.get_mut(&guild_id) {
            let incoming: Vec<EmojiId> = emojis.iter().map(|e| e.id).collect();

            let removal_filter: Vec<EmojiId> = guild_emojis
                .iter()
                .copied()
                .filter(|e| !incoming.contains(e))
                .collect();

            for to_remove in &removal_filter {
                guild_emojis.remove(to_remove);
            }

            for to_remove in &removal_filter {
                self.0.emojis.remove(to_remove);
            }
        }

        for emoji in emojis {
            self.cache_emoji(guild_id, emoji);
        }
    }

    fn cache_group(&self, group: Group) -> Arc<Group> {
        upsert_item(&self.0.groups, group.id, group)
    }

    fn cache_guild(&self, guild: Guild) {
        // The map and set creation needs to occur first, so caching states and
        // objects always has a place to put them.
        if self.wants(ResourceType::CHANNEL) {
            self.0.guild_channels.insert(guild.id, HashSet::new());
            self.cache_guild_channels(guild.id, guild.channels);
        }

        if self.wants(ResourceType::EMOJI) {
            self.0.guild_emojis.insert(guild.id, HashSet::new());
            self.cache_emojis(guild.id, guild.emojis);
        }

        if self.wants(ResourceType::MEMBER) {
            self.0.guild_members.insert(guild.id, HashSet::new());
            self.cache_members(guild.id, guild.members);
        }

        if self.wants(ResourceType::PRESENCE) {
            self.0.guild_presences.insert(guild.id, HashSet::new());
            self.cache_presences(guild.id, guild.presences);
        }

        if self.wants(ResourceType::ROLE) {
            self.0.guild_roles.insert(guild.id, HashSet::new());
            self.cache_roles(guild.id, guild.roles);
        }

        if self.wants(ResourceType::VOICE_STATE) {
            self.0.voice_state_guilds.insert(guild.id, HashSet::new());
            self.cache_voice_states(guild.voice_states);
        }

        let guild = CachedGuild {
            id: guild.id,
            afk_channel_id: guild.afk_channel_id,
            afk_timeout: guild.afk_timeout,
            application_id: guild.application_id,
            banner: guild.banner,
            default_message_notifications: guild.default_message_notifications,
            description: guild.description,
            discovery_splash: guild.discovery_splash,
            explicit_content_filter: guild.explicit_content_filter,
            features: guild.features,
            icon: guild.icon,
            joined_at: guild.joined_at,
            large: guild.large,
            lazy: guild.lazy,
            max_members: guild.max_members,
            max_presences: guild.max_presences,
            member_count: guild.member_count,
            mfa_level: guild.mfa_level,
            name: guild.name,
            owner: guild.owner,
            owner_id: guild.owner_id,
            permissions: guild.permissions,
            preferred_locale: guild.preferred_locale,
            premium_subscription_count: guild.premium_subscription_count,
            premium_tier: guild.premium_tier,
            region: guild.region,
            rules_channel_id: guild.rules_channel_id,
            splash: guild.splash,
            system_channel_id: guild.system_channel_id,
            system_channel_flags: guild.system_channel_flags,
            unavailable: guild.unavailable,
            verification_level: guild.verification_level,
            vanity_url_code: guild.vanity_url_code,
            widget_channel_id: guild.widget_channel_id,
            widget_enabled: guild.widget_enabled,
        };

        self.0.unavailable_guilds.remove(&guild.id);
        self.0.guilds.insert(guild.id, Arc::new(guild));
    }

    fn cache_member(&self, guild_id: GuildId, member: Member) -> Arc<CachedMember> {
        let member_id = member.user.id;
        let id = (guild_id, member_id);
        match self.0.members.get(&id) {
            Some(m) if **m == member => return Arc::clone(&m),
            Some(_) | None => {}
        }

        let user = self.cache_user(Cow::Owned(member.user), Some(guild_id));
        let cached = Arc::new(CachedMember {
            deaf: member.deaf,
            guild_id,
            joined_at: member.joined_at,
            mute: member.mute,
            nick: member.nick,
            pending: member.pending,
            premium_since: member.premium_since,
            roles: member.roles,
            user,
        });
        self.0.members.insert(id, Arc::clone(&cached));
        self.0
            .guild_members
            .entry(guild_id)
            .or_default()
            .insert(member_id);
        cached
    }

    fn cache_borrowed_partial_member(
        &self,
        guild_id: GuildId,
        member: &PartialMember,
        user: Arc<User>,
    ) -> Arc<CachedMember> {
        let id = (guild_id, user.id);
        match self.0.members.get(&id) {
            Some(m) if **m == member => return Arc::clone(&m),
            Some(_) | None => {}
        }

        self.0
            .guild_members
            .entry(guild_id)
            .or_default()
            .insert(user.id);

        let cached = Arc::new(CachedMember {
            deaf: member.deaf,
            guild_id,
            joined_at: member.joined_at.to_owned(),
            mute: member.mute,
            nick: member.nick.to_owned(),
            pending: false,
            premium_since: None,
            roles: member.roles.to_owned(),
            user,
        });
        self.0.members.insert(id, Arc::clone(&cached));

        cached
    }

    fn cache_members(&self, guild_id: GuildId, members: impl IntoIterator<Item = Member>) {
        for member in members {
            self.cache_member(guild_id, member);
        }
    }

    fn cache_presences(&self, guild_id: GuildId, presences: impl IntoIterator<Item = Presence>) {
        for presence in presences {
            self.cache_presence(guild_id, presence);
        }
    }

    fn cache_presence(&self, guild_id: GuildId, presence: Presence) -> Arc<CachedPresence> {
        let k = (guild_id, presence_user_id(&presence));

        match self.0.presences.get(&k) {
            Some(p) if **p == presence => return Arc::clone(&p),
            Some(_) | None => {}
        }
        let cached = Arc::new(CachedPresence::from(&presence));

        self.0.presences.insert(k, Arc::clone(&cached));

        cached
    }

    fn cache_private_channel(&self, private_channel: PrivateChannel) -> Arc<PrivateChannel> {
        let id = private_channel.id;

        match self.0.channels_private.get(&id) {
            Some(c) if **c == private_channel => Arc::clone(&c),
            Some(_) | None => {
                let v = Arc::new(private_channel);
                self.0.channels_private.insert(id, Arc::clone(&v));

                v
            }
        }
    }

    fn cache_roles(&self, guild_id: GuildId, roles: impl IntoIterator<Item = Role>) {
        for role in roles {
            self.cache_role(guild_id, role);
        }
    }

    fn cache_role(&self, guild_id: GuildId, role: Role) -> Arc<Role> {
        // Insert the role into the guild_roles map
        self.0
            .guild_roles
            .entry(guild_id)
            .or_default()
            .insert(role.id);

        // Insert the role into the all roles map
        upsert_guild_item(&self.0.roles, guild_id, role.id, role)
    }

    fn cache_user(&self, user: Cow<'_, User>, guild_id: Option<GuildId>) -> Arc<User> {
        match self.0.users.get_mut(&user.id) {
            Some(mut u) if *u.0 == *user => {
                if let Some(guild_id) = guild_id {
                    u.1.insert(guild_id);
                }

                return Arc::clone(&u.value().0);
            }
            Some(_) | None => {}
        }
        let user = Arc::new(user.into_owned());
        if let Some(guild_id) = guild_id {
            let mut guild_id_set = BTreeSet::new();
            guild_id_set.insert(guild_id);
            self.0
                .users
                .insert(user.id, (Arc::clone(&user), guild_id_set));
        }

        user
    }

    fn cache_voice_states(&self, voice_states: impl IntoIterator<Item = VoiceState>) {
        for voice_state in voice_states {
            self.cache_voice_state(voice_state);
        }
    }

    fn cache_voice_state(&self, vs: VoiceState) -> Option<Arc<VoiceState>> {
        // This should always exist, but just incase use a match
        let guild_id = match vs.guild_id {
            Some(id) => id,
            None => return None,
        };

        let user_id = vs.user_id;

        // Check if the user is switching channels in the same guild (ie. they already have a voice state entry)
        if let Some(voice_state) = self.0.voice_states.get(&(guild_id, user_id)) {
            if let Some(channel_id) = voice_state.channel_id {
                let remove_channel_mapping = self
                    .0
                    .voice_state_channels
                    .get_mut(&channel_id)
                    .map(|mut channel_voice_states| {
                        channel_voice_states.remove(&(guild_id, user_id));

                        channel_voice_states.is_empty()
                    })
                    .unwrap_or_default();

                if remove_channel_mapping {
                    self.0.voice_state_channels.remove(&channel_id);
                }
            }
        }

        // Check if the voice channel_id does not exist, signifying that the user has left
        if vs.channel_id.is_none() {
            {
                let remove_guild = self
                    .0
                    .voice_state_guilds
                    .get_mut(&guild_id)
                    .map(|mut guild_users| {
                        guild_users.remove(&user_id);

                        guild_users.is_empty()
                    })
                    .unwrap_or_default();

                if remove_guild {
                    self.0.voice_state_guilds.remove(&guild_id);
                }
            }

            let (_, state) = self.0.voice_states.remove(&(guild_id, user_id))?;

            return Some(state);
        }

        let state = Arc::new(vs);

        self.0
            .voice_states
            .insert((guild_id, user_id), Arc::clone(&state));

        self.0
            .voice_state_guilds
            .entry(guild_id)
            .or_default()
            .insert(user_id);

        if let Some(channel_id) = state.channel_id {
            self.0
                .voice_state_channels
                .entry(channel_id)
                .or_default()
                .insert((guild_id, user_id));
        }

        Some(state)
    }

    fn delete_group(&self, channel_id: ChannelId) -> Option<Arc<Group>> {
        self.0.groups.remove(&channel_id).map(|(_, v)| v)
    }

    fn unavailable_guild(&self, guild_id: GuildId) {
        self.0.unavailable_guilds.insert(guild_id);
        self.0.guilds.remove(&guild_id);
    }

    /// Delete a guild channel from the cache.
    ///
    /// The guild channel data itself and the channel entry in its guild's list
    /// of channels will be deleted.
    fn delete_guild_channel(&self, channel_id: ChannelId) -> Option<Arc<GuildChannel>> {
        let GuildItem { data, guild_id } = self.0.channels_guild.remove(&channel_id)?.1;

        if let Some(mut guild_channels) = self.0.guild_channels.get_mut(&guild_id) {
            guild_channels.remove(&channel_id);
        }

        Some(data)
    }

    fn delete_role(&self, role_id: RoleId) -> Option<Arc<Role>> {
        let role = self.0.roles.remove(&role_id).map(|(_, v)| v)?;

        if let Some(mut roles) = self.0.guild_roles.get_mut(&role.guild_id) {
            roles.remove(&role_id);
        }

        Some(role.data)
    }

    /// Determine whether the configured cache wants a specific resource to be
    /// processed.
    fn wants(&self, resource_type: ResourceType) -> bool {
        self.0.config.resource_types().contains(resource_type)
    }
}

fn presence_user_id(presence: &Presence) -> UserId {
    match presence.user {
        UserOrId::User(ref u) => u.id,
        UserOrId::UserId { id } => id,
    }
}

#[cfg(test)]
mod tests {
    use crate::InMemoryCache;
    use std::borrow::Cow;
    use twilight_model::{
        channel::{ChannelType, GuildChannel, TextChannel},
        gateway::payload::{GuildEmojisUpdate, MemberRemove, RoleDelete},
        guild::{
            DefaultMessageNotificationLevel, Emoji, ExplicitContentFilter, Guild, Member, MfaLevel,
            Permissions, PremiumTier, Role, SystemChannelFlags, VerificationLevel,
        },
        id::{ChannelId, EmojiId, GuildId, RoleId, UserId},
        user::{CurrentUser, User},
        voice::VoiceState,
    };

    fn current_user(id: u64) -> CurrentUser {
        CurrentUser {
            avatar: None,
            bot: true,
            discriminator: "9876".to_owned(),
            email: None,
            id: UserId(id),
            mfa_enabled: true,
            name: "test".to_owned(),
            verified: Some(true),
            premium_type: None,
            public_flags: None,
            flags: None,
            locale: None,
        }
    }

    fn emoji(id: EmojiId, user: Option<User>) -> Emoji {
        Emoji {
            animated: false,
            available: true,
            id,
            managed: false,
            name: "test".to_owned(),
            require_colons: true,
            roles: Vec::new(),
            user,
        }
    }

    fn member(id: UserId, guild_id: GuildId) -> Member {
        Member {
            deaf: false,
            guild_id,
            hoisted_role: None,
            joined_at: None,
            mute: false,
            nick: None,
            pending: false,
            premium_since: None,
            roles: Vec::new(),
            user: user(id),
        }
    }

    fn role(id: RoleId) -> Role {
        Role {
            color: 0,
            hoist: false,
            id,
            managed: false,
            mentionable: false,
            name: "test".to_owned(),
            permissions: Permissions::empty(),
            position: 0,
            tags: None,
        }
    }

    fn user(id: UserId) -> User {
        User {
            avatar: None,
            bot: false,
            discriminator: "0001".to_owned(),
            email: None,
            flags: None,
            id,
            locale: None,
            mfa_enabled: None,
            name: "user".to_owned(),
            premium_type: None,
            public_flags: None,
            system: None,
            verified: None,
        }
    }

    fn voice_state(
        guild_id: GuildId,
        channel_id: Option<ChannelId>,
        user_id: UserId,
    ) -> VoiceState {
        VoiceState {
            channel_id,
            deaf: false,
            guild_id: Some(guild_id),
            member: None,
            mute: true,
            self_deaf: false,
            self_mute: true,
            self_stream: false,
            session_id: "a".to_owned(),
            suppress: false,
            token: None,
            user_id,
        }
    }

    /// Test retrieval of the current user, notably that it doesn't simply
    /// panic or do anything funny. This is the only synchronous mutex that we
    /// might have trouble with across await points if we're not careful.
    #[test]
    fn test_current_user_retrieval() {
        let cache = InMemoryCache::new();
        assert!(cache.current_user().is_none());
        cache.cache_current_user(current_user(1));
        assert!(cache.current_user().is_some());
    }

    #[test]
    fn test_guild_create_channels_have_guild_ids() {
        let channels = Vec::from([GuildChannel::Text(TextChannel {
            id: ChannelId(111),
            guild_id: None,
            kind: ChannelType::GuildText,
            last_message_id: None,
            last_pin_timestamp: None,
            name: "guild channel with no guild id".to_owned(),
            nsfw: true,
            permission_overwrites: Vec::new(),
            parent_id: None,
            position: 1,
            rate_limit_per_user: None,
            topic: None,
        })]);

        let guild = Guild {
            id: GuildId(123),
            afk_channel_id: None,
            afk_timeout: 300,
            application_id: None,
            banner: None,
            channels,
            default_message_notifications: DefaultMessageNotificationLevel::Mentions,
            description: None,
            discovery_splash: None,
            emojis: Vec::new(),
            explicit_content_filter: ExplicitContentFilter::AllMembers,
            features: vec![],
            icon: None,
            joined_at: Some("".to_owned()),
            large: false,
            lazy: Some(true),
            max_members: Some(50),
            max_presences: Some(100),
            member_count: Some(25),
            members: Vec::new(),
            mfa_level: MfaLevel::Elevated,
            name: "this is a guild".to_owned(),
            owner: Some(false),
            owner_id: UserId(456),
            permissions: Some(Permissions::SEND_MESSAGES),
            preferred_locale: "en-GB".to_owned(),
            premium_subscription_count: Some(0),
            premium_tier: PremiumTier::None,
            presences: Vec::new(),
            region: "us-east".to_owned(),
            roles: Vec::new(),
            splash: None,
            system_channel_id: None,
            system_channel_flags: SystemChannelFlags::SUPPRESS_JOIN_NOTIFICATIONS,
            rules_channel_id: None,
            unavailable: false,
            verification_level: VerificationLevel::VeryHigh,
            voice_states: Vec::new(),
            vanity_url_code: None,
            widget_channel_id: None,
            widget_enabled: None,
            max_video_channel_users: None,
            approximate_member_count: None,
            approximate_presence_count: None,
        };

        let cache = InMemoryCache::new();
        cache.cache_guild(guild);

        let channel = cache.guild_channel(ChannelId(111)).unwrap();

        // The channel was given to the cache without a guild ID, but because
        // it's part of a guild create, the cache can automatically attach the
        // guild ID to it. So now, the channel's guild ID is present with the
        // correct value.
        match *channel {
            GuildChannel::Text(ref c) => {
                assert_eq!(Some(GuildId(123)), c.guild_id);
            }
            _ => panic!("{:?}", channel),
        }
    }

    #[test]
    fn test_syntax_update() {
        let cache = InMemoryCache::new();
        cache.update(&RoleDelete {
            guild_id: GuildId(0),
            role_id: RoleId(1),
        });
    }

    #[test]
    fn test_cache_user_guild_state() {
        let user_id = UserId(2);
        let cache = InMemoryCache::new();
        cache.cache_user(Cow::Owned(user(user_id)), Some(GuildId(1)));

        // Test the guild's ID is the only one in the user's set of guilds.
        {
            let user = cache.0.users.get(&user_id).unwrap();
            assert!(user.1.contains(&GuildId(1)));
            assert_eq!(1, user.1.len());
        }

        // Test that a second guild will cause 2 in the set.
        cache.cache_user(Cow::Owned(user(user_id)), Some(GuildId(3)));

        {
            let user = cache.0.users.get(&user_id).unwrap();
            assert!(user.1.contains(&GuildId(3)));
            assert_eq!(2, user.1.len());
        }

        // Test that removing a user from a guild will cause the ID to be
        // removed from the set, leaving the other ID.
        cache.update(&MemberRemove {
            guild_id: GuildId(3),
            user: user(user_id),
        });

        {
            let user = cache.0.users.get(&user_id).unwrap();
            assert!(!user.1.contains(&GuildId(3)));
            assert_eq!(1, user.1.len());
        }

        // Test that removing the user from its last guild removes the user's
        // entry.
        cache.update(&MemberRemove {
            guild_id: GuildId(1),
            user: user(user_id),
        });
        assert!(!cache.0.users.contains_key(&user_id));
    }

    #[test]
    fn test_voice_state_inserts_and_removes() {
        let cache = InMemoryCache::new();

        // Note: Channel ids are `<guildid><idx>` where idx is the index of the channel id
        // This is done to prevent channel id collisions between guilds
        // The other 2 ids are not special since they cant overlap

        // User 1 joins guild 1's channel 11 (1 channel, 1 guild)
        {
            // Ids for this insert
            let (guild_id, channel_id, user_id) = (GuildId(1), ChannelId(11), UserId(1));
            cache.cache_voice_state(voice_state(guild_id, Some(channel_id), user_id));

            // The new user should show up in the global voice states
            assert!(cache.0.voice_states.contains_key(&(guild_id, user_id)));
            // There should only be the one new voice state in there
            assert_eq!(1, cache.0.voice_states.len());

            // The new channel should show up in the voice states by channel lookup
            assert!(cache.0.voice_state_channels.contains_key(&channel_id));
            assert_eq!(1, cache.0.voice_state_channels.len());

            // The new guild should also show up in the voice states by guild lookup
            assert!(cache.0.voice_state_guilds.contains_key(&guild_id));
            assert_eq!(1, cache.0.voice_state_guilds.len());
        }

        // User 2 joins guild 2's channel 21 (2 channels, 2 guilds)
        {
            // Ids for this insert
            let (guild_id, channel_id, user_id) = (GuildId(2), ChannelId(21), UserId(2));
            cache.cache_voice_state(voice_state(guild_id, Some(channel_id), user_id));

            // The new voice state should show up in the global voice states
            assert!(cache.0.voice_states.contains_key(&(guild_id, user_id)));
            // There should be two voice states now that we have inserted another
            assert_eq!(2, cache.0.voice_states.len());

            // The new channel should also show up in the voice states by channel lookup
            assert!(cache.0.voice_state_channels.contains_key(&channel_id));
            assert_eq!(2, cache.0.voice_state_channels.len());

            // The new guild should also show up in the voice states by guild lookup
            assert!(cache.0.voice_state_guilds.contains_key(&guild_id));
            assert_eq!(2, cache.0.voice_state_guilds.len());
        }

        // User 3 joins guild 1's channel 12  (3 channels, 2 guilds)
        {
            // Ids for this insert
            let (guild_id, channel_id, user_id) = (GuildId(1), ChannelId(12), UserId(3));
            cache.cache_voice_state(voice_state(guild_id, Some(channel_id), user_id));

            // The new voice state should show up in the global voice states
            assert!(cache.0.voice_states.contains_key(&(guild_id, user_id)));
            assert_eq!(3, cache.0.voice_states.len());

            // The new channel should also show up in the voice states by channel lookup
            assert!(cache.0.voice_state_channels.contains_key(&channel_id));
            assert_eq!(3, cache.0.voice_state_channels.len());

            // The guild should still show up in the voice states by guild lookup
            assert!(cache.0.voice_state_guilds.contains_key(&guild_id));
            // Since we have used a guild that has been inserted into the cache already, there
            // should not be a new guild in the map
            assert_eq!(2, cache.0.voice_state_guilds.len());
        }

        // User 3 moves to guild 1's channel 11 (2 channels, 2 guilds)
        {
            // Ids for this insert
            let (guild_id, channel_id, user_id) = (GuildId(1), ChannelId(11), UserId(3));
            cache.cache_voice_state(voice_state(guild_id, Some(channel_id), user_id));

            // The new voice state should show up in the global voice states
            assert!(cache.0.voice_states.contains_key(&(guild_id, user_id)));
            // The amount of global voice states should not change since it was a move, not a join
            assert_eq!(3, cache.0.voice_states.len());

            // The new channel should show up in the voice states by channel lookup
            assert!(cache.0.voice_state_channels.contains_key(&channel_id));
            // The old channel should be removed from the lookup table
            assert_eq!(2, cache.0.voice_state_channels.len());

            // The guild should still show up in the voice states by guild lookup
            assert!(cache.0.voice_state_guilds.contains_key(&guild_id));
            assert_eq!(2, cache.0.voice_state_guilds.len());
        }

        // User 3 dcs (2 channels, 2 guilds)
        {
            let (guild_id, channel_id, user_id) = (GuildId(1), ChannelId(11), UserId(3));
            cache.cache_voice_state(voice_state(guild_id, None, user_id));

            // Now that the user left, they should not show up in the voice states
            assert!(!cache.0.voice_states.contains_key(&(guild_id, user_id)));
            assert_eq!(2, cache.0.voice_states.len());

            // Since they were not alone in their channel, the channel and guild mappings should not disappear
            assert!(cache.0.voice_state_channels.contains_key(&channel_id));
            // assert_eq!(2, cache.0.voice_state_channels.len());
            assert!(cache.0.voice_state_guilds.contains_key(&guild_id));
            assert_eq!(2, cache.0.voice_state_guilds.len());
        }

        // User 2 dcs (1 channel, 1 guild)
        {
            let (guild_id, channel_id, user_id) = (GuildId(2), ChannelId(21), UserId(2));
            cache.cache_voice_state(voice_state(guild_id, None, user_id));

            // Now that the user left, they should not show up in the voice states
            assert!(!cache.0.voice_states.contains_key(&(guild_id, user_id)));
            assert_eq!(1, cache.0.voice_states.len());

            // Since they were the last in their channel, the mapping should disappear
            assert!(!cache.0.voice_state_channels.contains_key(&channel_id));
            assert_eq!(1, cache.0.voice_state_channels.len());

            // Since they were the last in their guild, the mapping should disappear
            assert!(!cache.0.voice_state_guilds.contains_key(&guild_id));
            assert_eq!(1, cache.0.voice_state_guilds.len());
        }

        // User 1 dcs (0 channels, 0 guilds)
        {
            let (guild_id, _channel_id, user_id) = (GuildId(1), ChannelId(11), UserId(1));
            cache.cache_voice_state(voice_state(guild_id, None, user_id));

            // Since the last person has disconnected, the global voice states, guilds, and channels should all be gone
            assert!(cache.0.voice_states.is_empty());
            assert!(cache.0.voice_state_channels.is_empty());
            assert!(cache.0.voice_state_guilds.is_empty());
        }
    }

    #[test]
    fn test_voice_states() {
        let cache = InMemoryCache::new();
        cache.cache_voice_state(voice_state(GuildId(1), Some(ChannelId(2)), UserId(3)));
        cache.cache_voice_state(voice_state(GuildId(1), Some(ChannelId(2)), UserId(4)));

        // Returns both voice states for the channel that exists.
        assert_eq!(2, cache.voice_channel_states(ChannelId(2)).unwrap().len());

        // Returns None if the channel does not exist.
        assert!(cache.voice_channel_states(ChannelId(0)).is_none());
    }

    #[test]
    fn test_cache_role() {
        let cache = InMemoryCache::new();

        // Single inserts
        {
            // The role ids for the guild with id 1
            let guild_1_role_ids = (1..=10).map(RoleId).collect::<Vec<_>>();
            // Map the role ids to a test role
            let guild_1_roles = guild_1_role_ids
                .iter()
                .copied()
                .map(role)
                .collect::<Vec<_>>();
            // Cache all the roles using cache role
            for role in guild_1_roles.clone() {
                cache.cache_role(GuildId(1), role);
            }

            // Check for the cached guild role ids
            let cached_roles = cache.guild_roles(GuildId(1)).unwrap();
            assert_eq!(cached_roles.len(), guild_1_role_ids.len());
            assert!(guild_1_role_ids.iter().all(|id| cached_roles.contains(id)));

            // Check for the cached role
            assert!(guild_1_roles
                .into_iter()
                .all(|role| *cache.role(role.id).expect("Role missing from cache") == role))
        }

        // Bulk inserts
        {
            // The role ids for the guild with id 2
            let guild_2_role_ids = (101..=110).map(RoleId).collect::<Vec<_>>();
            // Map the role ids to a test role
            let guild_2_roles = guild_2_role_ids
                .iter()
                .copied()
                .map(role)
                .collect::<Vec<_>>();
            // Cache all the roles using cache roles
            cache.cache_roles(GuildId(2), guild_2_roles.clone());

            // Check for the cached guild role ids
            let cached_roles = cache.guild_roles(GuildId(2)).unwrap();
            assert_eq!(cached_roles.len(), guild_2_role_ids.len());
            assert!(guild_2_role_ids.iter().all(|id| cached_roles.contains(id)));

            // Check for the cached role
            assert!(guild_2_roles
                .into_iter()
                .all(|role| *cache.role(role.id).expect("Role missing from cache") == role))
        }
    }

    #[test]
    fn test_cache_guild_member() {
        let cache = InMemoryCache::new();

        // Single inserts
        {
            let guild_1_user_ids = (1..=10).map(UserId).collect::<Vec<_>>();
            let guild_1_members = guild_1_user_ids
                .iter()
                .copied()
                .map(|id| member(id, GuildId(1)))
                .collect::<Vec<_>>();

            for member in guild_1_members {
                cache.cache_member(GuildId(1), member);
            }

            // Check for the cached guild members ids
            let cached_roles = cache.guild_members(GuildId(1)).unwrap();
            assert_eq!(cached_roles.len(), guild_1_user_ids.len());
            assert!(guild_1_user_ids.iter().all(|id| cached_roles.contains(id)));

            // Check for the cached members
            assert!(guild_1_user_ids
                .iter()
                .all(|id| cache.member(GuildId(1), *id).is_some()));

            // Check for the cached users
            assert!(guild_1_user_ids.iter().all(|id| cache.user(*id).is_some()));
        }

        // Bulk inserts
        {
            let guild_2_user_ids = (1..=10).map(UserId).collect::<Vec<_>>();
            let guild_2_members = guild_2_user_ids
                .iter()
                .copied()
                .map(|id| member(id, GuildId(2)))
                .collect::<Vec<_>>();
            cache.cache_members(GuildId(2), guild_2_members);

            // Check for the cached guild members ids
            let cached_roles = cache.guild_members(GuildId(1)).unwrap();
            assert_eq!(cached_roles.len(), guild_2_user_ids.len());
            assert!(guild_2_user_ids.iter().all(|id| cached_roles.contains(id)));

            // Check for the cached members
            assert!(guild_2_user_ids
                .iter()
                .copied()
                .all(|id| cache.member(GuildId(1), id).is_some()));

            // Check for the cached users
            assert!(guild_2_user_ids.iter().all(|id| cache.user(*id).is_some()));
        }
    }

    #[test]
    fn test_cache_emoji() {
        let cache = InMemoryCache::new();

        // The user to do some of the inserts
        fn user_mod(id: EmojiId) -> Option<User> {
            if id.0 % 2 == 0 {
                // Only use user for half
                Some(user(UserId(1)))
            } else {
                None
            }
        }

        // Single inserts
        {
            let guild_1_emoji_ids = (1..=10).map(EmojiId).collect::<Vec<_>>();
            let guild_1_emoji = guild_1_emoji_ids
                .iter()
                .copied()
                .map(|id| emoji(id, user_mod(id)))
                .collect::<Vec<_>>();

            for emoji in guild_1_emoji {
                cache.cache_emoji(GuildId(1), emoji);
            }

            for id in guild_1_emoji_ids.iter().cloned() {
                let global_emoji = cache.emoji(id);
                assert!(global_emoji.is_some());
            }

            // Ensure the emoji has been added to the per-guild lookup map to prevent
            // issues like #551 from returning
            let guild_emojis = cache.guild_emojis(GuildId(1));
            assert!(guild_emojis.is_some());
            let guild_emojis = guild_emojis.unwrap();

            assert_eq!(guild_1_emoji_ids.len(), guild_emojis.len());
            assert!(guild_1_emoji_ids.iter().all(|id| guild_emojis.contains(id)));
        }

        // Bulk inserts
        {
            let guild_2_emoji_ids = (11..=20).map(EmojiId).collect::<Vec<_>>();
            let guild_2_emojis = guild_2_emoji_ids
                .iter()
                .copied()
                .map(|id| emoji(id, user_mod(id)))
                .collect::<Vec<_>>();
            cache.cache_emojis(GuildId(2), guild_2_emojis);

            for id in guild_2_emoji_ids.iter().cloned() {
                let global_emoji = cache.emoji(id);
                assert!(global_emoji.is_some());
            }

            let guild_emojis = cache.guild_emojis(GuildId(2));

            assert!(guild_emojis.is_some());
            let guild_emojis = guild_emojis.unwrap();
            assert_eq!(guild_2_emoji_ids.len(), guild_emojis.len());
            assert!(guild_2_emoji_ids.iter().all(|id| guild_emojis.contains(id)));
        }
    }

    #[test]
    fn test_clear() {
        let cache = InMemoryCache::new();
        cache.cache_emoji(GuildId(1), emoji(EmojiId(3), None));
        cache.cache_member(GuildId(2), member(UserId(4), GuildId(2)));
        cache.clear();
        assert!(cache.0.emojis.is_empty());
        assert!(cache.0.members.is_empty());
    }

    #[test]
    fn test_emoji_removal() {
        let cache = InMemoryCache::new();

        let guild_id = GuildId(1);

        let emote = emoji(EmojiId(1), None);
        let emote_2 = emoji(EmojiId(2), None);
        let emote_3 = emoji(EmojiId(3), None);

        cache.cache_emoji(guild_id, emote.clone());
        cache.cache_emoji(guild_id, emote_2.clone());
        cache.cache_emoji(guild_id, emote_3.clone());

        cache.update(&GuildEmojisUpdate {
            emojis: vec![emote.clone(), emote_3.clone()],
            guild_id,
        });

        assert_eq!(cache.0.emojis.len(), 2);
        assert_eq!(cache.0.guild_emojis.get(&guild_id).unwrap().len(), 2);
        assert!(cache.emoji(emote.id).is_some());
        assert!(cache.emoji(emote_2.id).is_none());
        assert!(cache.emoji(emote_3.id).is_some());

        cache.update(&GuildEmojisUpdate {
            emojis: vec![emote.clone()],
            guild_id,
        });

        assert_eq!(cache.0.emojis.len(), 1);
        assert_eq!(cache.0.guild_emojis.get(&guild_id).unwrap().len(), 1);
        assert!(cache.emoji(emote.id).is_some());
        assert!(cache.emoji(emote_2.id).is_none());

        let emote_4 = emoji(EmojiId(4), None);

        cache.update(&GuildEmojisUpdate {
            emojis: vec![emote_4.clone()],
            guild_id,
        });

        assert_eq!(cache.0.emojis.len(), 1);
        assert_eq!(cache.0.guild_emojis.get(&guild_id).unwrap().len(), 1);
        assert!(cache.emoji(emote_4.id).is_some());
        assert!(cache.emoji(emote.id).is_none());

        cache.update(&GuildEmojisUpdate {
            emojis: vec![],
            guild_id,
        });

        assert!(cache.0.emojis.is_empty());
        assert!(cache.0.guild_emojis.get(&guild_id).unwrap().is_empty());
    }
}
