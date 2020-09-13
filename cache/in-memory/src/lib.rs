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
//! use tokio::stream::StreamExt;
//! use twilight_cache_inmemory::InMemoryCache;
//! use twilight_gateway::Shard;
//!
//! # #[tokio::main] async fn main() -> Result<(), Box<dyn std::error::Error>> {
//! let token = env::var("DISCORD_TOKEN")?;
//! let mut shard = Shard::new(token);
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
//! [rust badge]: https://img.shields.io/badge/rust-stable-93450a.svg?style=for-the-badge&logo=rust

pub mod model;

mod builder;
mod config;
mod updates;

pub use self::{
    builder::InMemoryCacheBuilder,
    config::{Config, EventType},
    updates::UpdateCache,
};

use self::model::*;
use dashmap::{mapref::entry::Entry, DashMap, DashSet};
use std::{
    collections::{BTreeMap, BTreeSet, HashSet},
    hash::Hash,
    sync::{Arc, Mutex},
};
use twilight_model::{
    channel::{Group, GuildChannel, PrivateChannel},
    gateway::presence::{Presence, UserOrId},
    guild::{Emoji, Guild, Member, Role},
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
#[derive(Clone, Debug, Default)]
pub struct InMemoryCache(Arc<InMemoryCacheRef>);

/// Implemented methods and types for the cache.
impl InMemoryCache {
    /// Creates a new, empty cache.
    ///
    /// If you need to customize the cache, use the `From<InMemoryConfig>`
    /// implementation.
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
    /// This is an O(1) operation.
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
    /// This is an O(1) operation.
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
    /// This is an O(1) operation.
    pub fn guild(&self, guild_id: GuildId) -> Option<Arc<CachedGuild>> {
        self.0.guilds.get(&guild_id).map(|r| Arc::clone(r.value()))
    }

    /// Gets a member by guild ID and user ID.
    ///
    /// This is an O(1) operation.
    pub fn member(&self, guild_id: GuildId, user_id: UserId) -> Option<Arc<CachedMember>> {
        self.0
            .members
            .get(&(guild_id, user_id))
            .map(|r| Arc::clone(r.value()))
    }

    /// Gets a message by channel ID and message ID.
    ///
    /// This is an O(log n) operation.
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
    /// This is an O(1) operation.
    pub fn presence(&self, guild_id: GuildId, user_id: UserId) -> Option<Arc<CachedPresence>> {
        self.0
            .presences
            .get(&(guild_id, user_id))
            .map(|r| Arc::clone(r.value()))
    }

    /// Gets a private channel by ID.
    ///
    /// This is an O(1) operation.
    pub fn private_channel(&self, channel_id: ChannelId) -> Option<Arc<PrivateChannel>> {
        self.0
            .channels_private
            .get(&channel_id)
            .map(|r| Arc::clone(r.value()))
    }

    /// Gets a role by ID.
    ///
    /// This is an O(1) operation.
    pub fn role(&self, role_id: RoleId) -> Option<Arc<Role>> {
        self.0
            .roles
            .get(&role_id)
            .map(|role| Arc::clone(&role.data))
    }

    /// Gets a user by ID.
    ///
    /// This is an O(1) operation.
    pub fn user(&self, user_id: UserId) -> Option<Arc<User>> {
        self.0.users.get(&user_id).map(|r| Arc::clone(&r.0))
    }

    /// Gets the voice states within a voice channel.
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
    /// This is an O(1) operation.
    pub fn voice_state(&self, user_id: UserId, guild_id: GuildId) -> Option<Arc<VoiceState>> {
        self.0
            .voice_states
            .get(&(guild_id, user_id))
            .map(|r| Arc::clone(r.value()))
    }

    /// Clears the entire state of the Cache. This is equal to creating a new
    /// empty Cache.
    pub fn clear(&self) {
        self.0.channels_guild.clear();
        self.0
            .current_user
            .lock()
            .expect("current user poisoned")
            .take();
        self.0.emojis.clear();
        self.0.guilds.clear();
        self.0.presences.clear();
        self.0.roles.clear();
        self.0.users.clear();
        self.0.voice_state_guilds.clear();
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
    ) -> HashSet<ChannelId> {
        guild_channels
            .into_iter()
            .map(|channel| {
                let id = channel.id();
                self.cache_guild_channel(guild_id, channel);

                id
            })
            .collect()
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
        let user = match emoji.user {
            Some(u) => Some(self.cache_user(u, guild_id)),
            None => None,
        };
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
        cached
    }

    fn cache_emojis(
        &self,
        guild_id: GuildId,
        emojis: impl IntoIterator<Item = Emoji>,
    ) -> HashSet<EmojiId> {
        emojis
            .into_iter()
            .map(|emoji| {
                let id = emoji.id;
                self.cache_emoji(guild_id, emoji);

                id
            })
            .collect()
    }

    fn cache_group(&self, group: Group) -> Arc<Group> {
        upsert_item(&self.0.groups, group.id, group)
    }

    fn cache_guild(&self, guild: Guild) {
        // The map and set creation needs to occur first, so caching states and objects
        // always has a place to put them.
        self.0.guild_channels.insert(guild.id, HashSet::new());
        self.0.guild_emojis.insert(guild.id, HashSet::new());
        self.0.guild_members.insert(guild.id, HashSet::new());
        self.0.guild_presences.insert(guild.id, HashSet::new());
        self.0.guild_roles.insert(guild.id, HashSet::new());
        self.0.voice_state_guilds.insert(guild.id, HashSet::new());

        self.cache_guild_channels(guild.id, guild.channels.into_iter().map(|(_, v)| v));
        self.cache_emojis(guild.id, guild.emojis.into_iter().map(|(_, v)| v));
        self.cache_members(guild.id, guild.members.into_iter().map(|(_, v)| v));
        self.cache_presences(guild.id, guild.presences.into_iter().map(|(_, v)| v));
        self.cache_roles(guild.id, guild.roles.into_iter().map(|(_, v)| v));
        self.cache_voice_states(guild.voice_states.into_iter().map(|(_, v)| v));

        let guild = CachedGuild {
            id: guild.id,
            afk_channel_id: guild.afk_channel_id,
            afk_timeout: guild.afk_timeout,
            application_id: guild.application_id,
            banner: guild.banner,
            default_message_notifications: guild.default_message_notifications,
            description: guild.description,
            discovery_splash: guild.discovery_splash,
            embed_channel_id: guild.embed_channel_id,
            embed_enabled: guild.embed_enabled,
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
        let id = (guild_id, member.user.id);
        match self.0.members.get(&id) {
            Some(m) if **m == member => return Arc::clone(&m),
            Some(_) | None => {}
        }

        let user = self.cache_user(member.user, guild_id);
        let cached = Arc::new(CachedMember {
            deaf: member.deaf,
            guild_id,
            joined_at: member.joined_at,
            mute: member.mute,
            nick: member.nick,
            premium_since: member.premium_since,
            roles: member.roles,
            user,
        });
        self.0.members.insert(id, Arc::clone(&cached));
        cached
    }

    fn cache_members(
        &self,
        guild_id: GuildId,
        members: impl IntoIterator<Item = Member>,
    ) -> HashSet<UserId> {
        members
            .into_iter()
            .map(|member| {
                let id = member.user.id;
                self.cache_member(guild_id, member);

                id
            })
            .collect()
    }

    fn cache_presences(
        &self,
        guild_id: GuildId,
        presences: impl IntoIterator<Item = Presence>,
    ) -> HashSet<UserId> {
        presences
            .into_iter()
            .map(|presence| {
                let id = presence_user_id(&presence);
                self.cache_presence(guild_id, presence);

                id
            })
            .collect()
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

    fn cache_roles(
        &self,
        guild_id: GuildId,
        roles: impl IntoIterator<Item = Role>,
    ) -> HashSet<RoleId> {
        roles
            .into_iter()
            .map(|role| {
                let id = role.id;

                self.cache_role(guild_id, role);

                id
            })
            .collect()
    }

    fn cache_role(&self, guild_id: GuildId, role: Role) -> Arc<Role> {
        upsert_guild_item(&self.0.roles, guild_id, role.id, role)
    }

    fn cache_user(&self, user: User, guild_id: GuildId) -> Arc<User> {
        match self.0.users.get_mut(&user.id) {
            Some(mut u) if *u.0 == user => {
                u.1.insert(guild_id);

                return Arc::clone(&u.value().0);
            }
            Some(_) | None => {}
        }
        let user = Arc::new(user);
        let mut guild_id_set = BTreeSet::new();
        guild_id_set.insert(guild_id);
        self.0
            .users
            .insert(user.id, (Arc::clone(&user), guild_id_set));

        user
    }

    fn cache_voice_states(
        &self,
        voice_states: impl IntoIterator<Item = VoiceState>,
    ) -> HashSet<UserId> {
        voice_states
            .into_iter()
            .map(|voice_state| {
                let id = voice_state.user_id;
                self.cache_voice_state(voice_state);

                id
            })
            .collect()
    }

    fn cache_voice_state(&self, vs: VoiceState) -> Option<Arc<VoiceState>> {
        // This should always exist, but just incase use a match
        let guild_id = match vs.guild_id {
            Some(id) => id,
            None => return None,
        };

        let user_id = vs.user_id;

        if vs.channel_id.is_none() {
            {
                let remove = self
                    .0
                    .voice_state_guilds
                    .get_mut(&guild_id)
                    .map(|mut guild_users| {
                        guild_users.remove(&user_id);

                        guild_users.is_empty()
                    })
                    .unwrap_or_default();

                if remove {
                    self.0.voice_state_guilds.remove(&guild_id);
                }
            }

            let (_, state) = self.0.voice_states.remove(&(guild_id, user_id))?;

            if let Some(channel_id) = state.channel_id {
                let remove = self
                    .0
                    .voice_state_channels
                    .get_mut(&channel_id)
                    .map(|mut users| {
                        users.remove(&(guild_id, user_id));

                        users.is_empty()
                    })
                    .unwrap_or_default();

                if remove {
                    self.0.voice_state_channels.remove(&channel_id);
                }
            }

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
    use std::collections::HashMap;
    use twilight_model::{
        channel::{ChannelType, GuildChannel, TextChannel},
        gateway::payload::{MemberRemove, RoleDelete},
        guild::{
            DefaultMessageNotificationLevel, ExplicitContentFilter, Guild, MfaLevel, Permissions,
            PremiumTier, SystemChannelFlags, VerificationLevel,
        },
        id::{ChannelId, GuildId, RoleId, UserId},
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
            verified: true,
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

    fn voice_state(guild_id: u64, channel_id: Option<u64>, user_id: u64) -> VoiceState {
        VoiceState {
            channel_id: channel_id.map(ChannelId),
            deaf: false,
            guild_id: Some(GuildId(guild_id)),
            member: None,
            mute: true,
            self_deaf: false,
            self_mute: true,
            self_stream: false,
            session_id: "a".to_owned(),
            suppress: false,
            token: None,
            user_id: UserId(user_id),
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
        let mut channels = HashMap::new();
        channels.insert(
            ChannelId(111),
            GuildChannel::Text(TextChannel {
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
            }),
        );

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
            embed_channel_id: None,
            embed_enabled: None,
            emojis: HashMap::new(),
            explicit_content_filter: ExplicitContentFilter::AllMembers,
            features: vec![],
            icon: None,
            joined_at: Some("".to_owned()),
            large: false,
            lazy: Some(true),
            max_members: Some(50),
            max_presences: Some(100),
            member_count: Some(25),
            members: HashMap::new(),
            mfa_level: MfaLevel::Elevated,
            name: "this is a guild".to_owned(),
            owner: Some(false),
            owner_id: UserId(456),
            permissions: Some(Permissions::SEND_MESSAGES),
            preferred_locale: "en-GB".to_owned(),
            premium_subscription_count: Some(0),
            premium_tier: PremiumTier::None,
            presences: HashMap::new(),
            region: "us-east".to_owned(),
            roles: HashMap::new(),
            splash: None,
            system_channel_id: None,
            system_channel_flags: SystemChannelFlags::SUPPRESS_JOIN_NOTIFICATIONS,
            rules_channel_id: None,
            unavailable: false,
            verification_level: VerificationLevel::VeryHigh,
            voice_states: HashMap::new(),
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
            _ => assert!(false, "{:?}", channel),
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
        cache.cache_user(user(user_id), GuildId(1));

        // Test the guild's ID is the only one in the user's set of guilds.
        {
            let user = cache.0.users.get(&user_id).unwrap();
            assert!(user.1.contains(&GuildId(1)));
            assert_eq!(1, user.1.len());
        }

        // Test that a second guild will cause 2 in the set.
        cache.cache_user(user(user_id), GuildId(3));

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
        cache.cache_voice_state(voice_state(1, Some(2), 3));
        assert!(cache.0.voice_states.contains_key(&(GuildId(1), UserId(3))));
        assert_eq!(1, cache.0.voice_states.len());
        assert!(cache.0.voice_state_channels.contains_key(&ChannelId(2)));
        assert_eq!(1, cache.0.voice_state_channels.len());
        assert!(cache.0.voice_state_guilds.contains_key(&GuildId(1)));
        assert_eq!(1, cache.0.voice_state_guilds.len());

        // Inserting another voice state with a different guild ID, channel ID,
        // and user ID will make all of these 2.
        cache.cache_voice_state(voice_state(4, Some(5), 6));
        assert!(cache.0.voice_states.contains_key(&(GuildId(4), UserId(6))));
        assert_eq!(2, cache.0.voice_states.len());
        assert!(cache.0.voice_state_channels.contains_key(&ChannelId(5)));
        assert_eq!(2, cache.0.voice_state_channels.len());
        assert!(cache.0.voice_state_guilds.contains_key(&GuildId(4)));
        assert_eq!(2, cache.0.voice_state_guilds.len());

        // Inserting another voice state with the same guild ID and channel ID
        // but a different user will cause the guild to have 2 voice states,
        // the channel to have 2 voice states, and 3 overall voice states.
        cache.cache_voice_state(voice_state(1, Some(2), 7));
        assert!(cache.0.voice_states.contains_key(&(GuildId(4), UserId(6))));
        assert_eq!(3, cache.0.voice_states.len());
        assert!(cache.0.voice_state_channels.contains_key(&ChannelId(5)));
        assert_eq!(2, cache.0.voice_state_channels.len());
        assert!(cache.0.voice_state_guilds.contains_key(&GuildId(4)));
        assert_eq!(2, cache.0.voice_state_guilds.len());

        // Now test that deleting the last voice state in all of a channel and
        // guild causes both to be removed from the map.
        cache.cache_voice_state(voice_state(4, None, 6));
        assert!(!cache.0.voice_states.contains_key(&(GuildId(4), UserId(6))));
        assert_eq!(2, cache.0.voice_states.len());
        assert!(!cache.0.voice_state_channels.contains_key(&ChannelId(5)));
        assert_eq!(1, cache.0.voice_state_channels.len());
        assert!(!cache.0.voice_state_guilds.contains_key(&GuildId(4)));
        assert_eq!(1, cache.0.voice_state_guilds.len());

        // Now test that deleting one of two voice states in a guild and channel
        // causes the other to remain in all cases.
        cache.cache_voice_state(voice_state(1, None, 7));
        assert!(!cache.0.voice_states.contains_key(&(GuildId(1), UserId(7))));
        assert_eq!(1, cache.0.voice_states.len());
        assert!(cache.0.voice_state_channels.contains_key(&ChannelId(2)));
        assert_eq!(1, cache.0.voice_state_channels.len());
        assert!(cache.0.voice_state_guilds.contains_key(&GuildId(1)));
        assert_eq!(1, cache.0.voice_state_guilds.len());

        // And now that deleting the last voice state deletes all of the maps'
        // entries.
        cache.cache_voice_state(voice_state(1, None, 3));
        assert!(cache.0.voice_states.is_empty());
        assert!(cache.0.voice_state_channels.is_empty());
        assert!(cache.0.voice_state_guilds.is_empty());
    }

    #[test]
    fn test_voice_states() {
        let cache = InMemoryCache::new();
        cache.cache_voice_state(voice_state(1, Some(2), 3));
        cache.cache_voice_state(voice_state(1, Some(2), 4));

        // Returns both voice states for the channel that exists.
        assert_eq!(2, cache.voice_channel_states(ChannelId(2)).unwrap().len());

        // Returns None if the channel does not exist.
        assert!(cache.voice_channel_states(ChannelId(0)).is_none());
    }
}
