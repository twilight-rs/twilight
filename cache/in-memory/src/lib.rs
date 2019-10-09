pub mod config;
pub mod model;

mod updates;

use self::model::*;
use async_trait::async_trait;
use config::Config;
use dawn_cache_trait::Cache;
use dawn_model::{
    channel::{Group, GuildChannel, PrivateChannel},
    gateway::presence::{Presence, UserOrId},
    guild::{Emoji, Guild, Member, Role},
    id::{ChannelId, EmojiId, GuildId, MessageId, RoleId, UserId},
    user::{CurrentUser, User},
    voice::VoiceState,
};
use futures_util::{future, lock::Mutex};
use std::{
    collections::{
        hash_map::{Entry, HashMap},
        BTreeMap,
        HashSet,
    },
    hash::Hash,
    iter::FromIterator,
    sync::Arc,
};

struct GuildItem<T> {
    data: Arc<T>,
    guild_id: GuildId,
}

async fn upsert_guild_item<K: Eq + Hash, V: PartialEq>(
    map: &Mutex<HashMap<K, GuildItem<V>>>,
    guild_id: GuildId,
    k: K,
    v: V,
) -> Arc<V> {
    match map.lock().await.entry(k) {
        Entry::Occupied(e) if *e.get().data == v => Arc::clone(&e.get().data),
        Entry::Occupied(mut e) => {
            let v = Arc::new(v);
            e.insert(GuildItem {
                data: Arc::clone(&v),
                guild_id,
            });

            v
        },
        Entry::Vacant(e) => Arc::clone(
            &e.insert(GuildItem {
                data: Arc::new(v),
                guild_id,
            })
            .data,
        ),
    }
}

async fn upsert_item<K: Eq + Hash, V: PartialEq>(
    map: &Mutex<HashMap<K, Arc<V>>>,
    k: K,
    v: V,
) -> Arc<V> {
    match map.lock().await.entry(k) {
        Entry::Occupied(e) if **e.get() == v => Arc::clone(e.get()),
        Entry::Occupied(mut e) => {
            let v = Arc::new(v);
            e.insert(Arc::clone(&v));

            v
        },
        Entry::Vacant(e) => Arc::clone(e.insert(Arc::new(v))),
    }
}

/// Error type for [`InMemoryCache`] operations.
///
/// Currently this is empty as no error can occur. This exists only to satisfy
/// the `Cache`'s associated type [`Cache::Error`].
///
/// [`Cache::Error`]: trait.Cache.html#type.Error
/// [`InMemoryCache`]: struct.InMemoryCache.html
#[derive(Clone, Debug)]
pub enum InMemoryCacheError {}

type LockedArcMap<T, U> = Mutex<HashMap<T, Arc<U>>>;

#[derive(Debug, Default)]
struct InMemoryCacheRef {
    config: Arc<Config>,
    channels_guild: Mutex<HashMap<ChannelId, GuildItem<GuildChannel>>>,
    channels_private: LockedArcMap<ChannelId, PrivateChannel>,
    current_user: Mutex<Option<Arc<CurrentUser>>>,
    emojis: Mutex<HashMap<EmojiId, GuildItem<CachedEmoji>>>,
    groups: LockedArcMap<ChannelId, Group>,
    guilds: LockedArcMap<GuildId, CachedGuild>,
    guild_channels: Mutex<HashMap<GuildId, HashSet<ChannelId>>>,
    guild_emojis: Mutex<HashMap<GuildId, HashSet<EmojiId>>>,
    guild_members: Mutex<HashMap<GuildId, HashSet<UserId>>>,
    guild_presences: Mutex<HashMap<GuildId, HashSet<UserId>>>,
    guild_roles: Mutex<HashMap<GuildId, HashSet<RoleId>>>,
    guild_voice_states: Mutex<HashMap<GuildId, HashSet<(ChannelId, UserId)>>>,
    members: LockedArcMap<(GuildId, UserId), CachedMember>,
    messages: Mutex<HashMap<ChannelId, BTreeMap<MessageId, Arc<CachedMessage>>>>,
    presences: LockedArcMap<(Option<GuildId>, UserId), CachedPresence>,
    roles: Mutex<HashMap<RoleId, GuildItem<Role>>>,
    unavailable_guilds: Mutex<HashSet<GuildId>>,
    users: LockedArcMap<UserId, User>,
    voice_states: LockedArcMap<(ChannelId, UserId), CachedVoiceState>,
}

/// A thread-safe, in-memory-process cache of Discord data. It can be cloned and
/// sent to other threads.
///
/// This is an implementation of [`Cache`] designed to be used by only the
/// current process. If the cache needs to be used by other processes, consider
/// using [`dawn-cache-redis`] or another cache.
///
/// # Public Immutability
///
/// The defining characteristic of this cache is that returned types (such as a
/// guild or user) do not use locking for access. Although the internals of the
/// cache use asynchronous locking for mutability, the returned types themselves
/// are immutable. If a user is retrieved from the cache, an `Arc<User>` is
/// returned. If a reference to that user is held but the cache updates the
/// user, the reference held by you will be outdated, but still exist.
///
/// The intended use is that data is held outside the cache for only as long
/// as necessary, where the state of the value at that time doesn't need to be
/// up-to-date.
///
/// Say you're deleting some of the guilds of a channel. You'll probably need
/// the guild to do that, so you retrieve it from the cache. You can then use
/// the guild to update all of the channels, because for most use cases you
/// don't need the guild to be up-to-date in real time, you only need its state
/// at that *point in time*. If you need the guild to always be up-to-date
/// between operations, the intent is that you keep getting it from the cache.
///
/// Getting something from the cache is cheap and has low contention, so public
/// immutability is preferred over using mutexes, read-write locks, or other
/// smart atomic updating cells. Refer to the crate-level documentation for
/// a list of known first-party and third-party cache implementations.
///
/// # Caveats
///
/// - The "last pin timestamp" and "last message id" fields of channels will
/// *not* be kept up to date as pins and messages come in.
///
/// [`Cache`]: trait.Cache.html
/// [`dawn-cache-redis`]: https://github.com/dawn-rs/cache-redis
#[derive(Clone, Debug, Default)]
pub struct InMemoryCache(Arc<InMemoryCacheRef>);

/// Implemented methods and types for the cache.
///
/// **Note**: This section may *appear* empty. Please read the implementation
/// in the [`Cache` trait implementation].
///
/// [`Cache` trait implementation]: #impl-Cache
impl InMemoryCache {
    /// Creates a new, empty cache.
    ///
    /// If you need to customize the cache, use the `From<Config>`
    /// implementation.
    ///
    /// # Examples
    ///
    /// Creating a new `InMemoryCache` with a custom configuration, limiting
    /// the message cache to 50 messages per channel:
    ///
    /// ```
    /// use dawn_cache_inmemory::{
    ///     config::Config,
    ///     InMemoryCache,
    /// };
    ///
    /// let config = Config::builder().message_cache_size(50);
    /// let cache = InMemoryCache::from(config);
    /// ```
    pub fn new() -> Self {
        Self::default()
    }

    /// Returns a copy of the config cache.
    pub fn config(&self) -> Config {
        (*self.0.config).clone()
    }

    async fn cache_current_user(&self, mut current_user: CurrentUser) {
        let mut user = self.0.current_user.lock().await;

        if let Some(mut user) = user.as_mut() {
            if let Some(user) = Arc::get_mut(&mut user) {
                std::mem::swap(user, &mut current_user);

                return;
            }
        }

        *user = Some(Arc::new(current_user));
    }

    async fn cache_guild_channels(
        &self,
        guild_id: GuildId,
        guild_channels: impl IntoIterator<Item = GuildChannel>,
    ) -> HashSet<ChannelId> {
        let pairs = future::join_all(guild_channels.into_iter().map(|channel| {
            async {
                let id = *guild_channel_id(&channel);
                self.cache_guild_channel(guild_id, channel).await;

                id
            }
        }))
        .await;

        HashSet::from_iter(pairs)
    }

    async fn cache_guild_channel(
        &self,
        guild_id: GuildId,
        channel: GuildChannel,
    ) -> Arc<GuildChannel> {
        let id = *guild_channel_id(&channel);

        upsert_guild_item(&self.0.channels_guild, guild_id, id, channel).await
    }

    async fn cache_emoji(&self, guild_id: GuildId, emoji: Emoji) -> Arc<CachedEmoji> {
        match self.0.emojis.lock().await.get(&emoji.id) {
            Some(e) if *e.data == emoji => Arc::clone(&e.data),
            Some(_) | None => {
                let user = match emoji.user {
                    Some(u) => Some(self.cache_user(u).await),
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
                });
                self.0.emojis.lock().await.insert(
                    cached.id,
                    GuildItem {
                        data: Arc::clone(&cached),
                        guild_id,
                    },
                );

                cached
            },
        }
    }

    async fn cache_emojis(
        &self,
        guild_id: GuildId,
        emojis: impl IntoIterator<Item = Emoji>,
    ) -> HashSet<EmojiId> {
        let pairs = future::join_all(emojis.into_iter().map(|emoji| {
            async {
                let id = emoji.id;
                self.cache_emoji(guild_id, emoji).await;

                id
            }
        }))
        .await;

        HashSet::from_iter(pairs)
    }

    async fn cache_group(&self, group: Group) -> Arc<Group> {
        upsert_item(&self.0.groups, group.id, group).await
    }

    async fn cache_guild(&self, guild: Guild) {
        self.cache_guild_channels(guild.id, guild.channels.into_iter().map(|(_, v)| v))
            .await;
        self.cache_emojis(guild.id, guild.emojis.into_iter().map(|(_, v)| v))
            .await;
        self.cache_members(guild.id, guild.members.into_iter().map(|(_, v)| v))
            .await;
        self.cache_presences(Some(guild.id), guild.presences.into_iter().map(|(_, v)| v))
            .await;
        self.cache_roles(guild.id, guild.roles.into_iter().map(|(_, v)| v))
            .await;
        self.cache_voice_states(guild.voice_states.into_iter().map(|(_, v)| v))
            .await;
        self.0
            .guild_channels
            .lock()
            .await
            .insert(guild.id, HashSet::new());
        self.0
            .guild_emojis
            .lock()
            .await
            .insert(guild.id, HashSet::new());
        self.0
            .guild_members
            .lock()
            .await
            .insert(guild.id, HashSet::new());
        self.0
            .guild_presences
            .lock()
            .await
            .insert(guild.id, HashSet::new());
        self.0
            .guild_roles
            .lock()
            .await
            .insert(guild.id, HashSet::new());
        self.0
            .guild_voice_states
            .lock()
            .await
            .insert(guild.id, HashSet::new());

        let guild = CachedGuild {
            id: guild.id,
            afk_channel_id: guild.afk_channel_id,
            afk_timeout: guild.afk_timeout,
            application_id: guild.application_id,
            banner: guild.banner,
            default_message_notifications: guild.default_message_notifications,
            description: guild.description,
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
            splash: guild.splash,
            system_channel_id: guild.system_channel_id,
            unavailable: guild.unavailable,
            verification_level: guild.verification_level,
            vanity_url_code: guild.vanity_url_code,
            widget_channel_id: guild.widget_channel_id,
            widget_enabled: guild.widget_enabled,
        };

        self.0.unavailable_guilds.lock().await.remove(&guild.id);
        self.0.guilds.lock().await.insert(guild.id, Arc::new(guild));
    }

    async fn cache_member(&self, guild_id: GuildId, member: Member) -> Arc<CachedMember> {
        let id = (guild_id, member.user.id);

        match self.0.members.lock().await.get(&id) {
            Some(m) if **m == member => Arc::clone(&m),
            Some(_) | None => {
                let user = self.cache_user(member.user).await;

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

                self.0.members.lock().await.insert(id, Arc::clone(&cached));

                cached
            },
        }
    }

    async fn cache_members(
        &self,
        guild_id: GuildId,
        members: impl IntoIterator<Item = Member>,
    ) -> HashSet<UserId> {
        let ids = future::join_all(members.into_iter().map(|member| {
            async {
                let id = member.user.id;
                self.cache_member(guild_id, member).await;

                id
            }
        }))
        .await;

        HashSet::from_iter(ids)
    }

    async fn cache_presences(
        &self,
        guild_id: Option<GuildId>,
        presences: impl IntoIterator<Item = Presence>,
    ) -> HashSet<UserId> {
        let ids = future::join_all(presences.into_iter().map(|presence| {
            async {
                let id = presence_user_id(&presence);
                self.cache_presence(guild_id, presence).await;

                id
            }
        }))
        .await;

        HashSet::from_iter(ids)
    }

    async fn cache_presence(
        &self,
        guild_id: Option<GuildId>,
        presence: Presence,
    ) -> Arc<CachedPresence> {
        let k = (guild_id, presence_user_id(&presence));

        match self.0.presences.lock().await.get(&k) {
            Some(p) if **p == presence => Arc::clone(&p),
            Some(_) | None => {
                let cached = Arc::new(CachedPresence::from(&presence));

                self.0.presences.lock().await.insert(k, Arc::clone(&cached));

                cached
            },
        }
    }

    async fn cache_private_channel(&self, private_channel: PrivateChannel) -> Arc<PrivateChannel> {
        let id = private_channel.id;

        match self.0.channels_private.lock().await.get(&id) {
            Some(c) if **c == private_channel => Arc::clone(&c),
            Some(_) | None => {
                let v = Arc::new(private_channel);
                self.0
                    .channels_private
                    .lock()
                    .await
                    .insert(id, Arc::clone(&v));

                v
            },
        }
    }

    async fn cache_roles(
        &self,
        guild_id: GuildId,
        roles: impl IntoIterator<Item = Role>,
    ) -> HashSet<RoleId> {
        let ids = future::join_all(roles.into_iter().map(|role| {
            async {
                let id = role.id;

                self.cache_role(guild_id, role).await;

                id
            }
        }))
        .await;

        HashSet::from_iter(ids)
    }

    async fn cache_role(&self, guild_id: GuildId, role: Role) -> Arc<Role> {
        upsert_guild_item(&self.0.roles, guild_id, role.id, role).await
    }

    async fn cache_user(&self, user: User) -> Arc<User> {
        match self.0.users.lock().await.get(&user.id) {
            Some(u) if **u == user => Arc::clone(&u),
            Some(_) | None => {
                let user = Arc::new(user);
                self.0.users.lock().await.insert(user.id, Arc::clone(&user));

                user
            },
        }
    }

    async fn cache_voice_states(
        &self,
        voice_states: impl IntoIterator<Item = VoiceState>,
    ) -> HashSet<UserId> {
        let ids = future::join_all(voice_states.into_iter().map(|vs| {
            async {
                let id = vs.user_id;
                self.cache_voice_state(vs).await;

                id
            }
        }))
        .await;

        HashSet::from_iter(ids)
    }

    async fn cache_voice_state(&self, vs: VoiceState) -> Arc<CachedVoiceState> {
        let k = (vs.channel_id.unwrap(), vs.user_id);

        match self.0.voice_states.lock().await.get(&k) {
            Some(v) if **v == vs => Arc::clone(v),
            Some(_) | None => {
                let state = Arc::new(CachedVoiceState {
                    channel_id: vs.channel_id,
                    deaf: vs.deaf,
                    guild_id: vs.guild_id,
                    mute: vs.mute,
                    self_deaf: vs.self_deaf,
                    self_mute: vs.self_mute,
                    self_stream: vs.self_stream,
                    session_id: vs.session_id,
                    suppress: vs.suppress,
                    token: vs.token,
                    user_id: vs.user_id,
                });
                self.0
                    .voice_states
                    .lock()
                    .await
                    .insert(k, Arc::clone(&state));

                state
            },
        }
    }

    async fn delete_group(&self, channel_id: ChannelId) -> Option<Arc<Group>> {
        self.0.groups.lock().await.remove(&channel_id)
    }

    async fn unavailable_guild(&self, guild_id: GuildId) {
        self.0.unavailable_guilds.lock().await.insert(guild_id);
        self.0.guilds.lock().await.remove(&guild_id);
    }

    async fn delete_guild_channel(&self, channel_id: ChannelId) -> Option<Arc<GuildChannel>> {
        self.0
            .channels_guild
            .lock()
            .await
            .remove(&channel_id)
            .map(|x| x.data)
    }

    async fn delete_role(&self, role_id: RoleId) -> Option<Arc<Role>> {
        let role = self.0.roles.lock().await.remove(&role_id)?;
        let mut guild_roles = self.0.guild_roles.lock().await;

        if let Some(roles) = guild_roles.get_mut(&role.guild_id) {
            roles.remove(&role_id);
        }

        Some(role.data)
    }
}

impl<T: Into<Config>> From<T> for InMemoryCache {
    fn from(config: T) -> Self {
        InMemoryCache(Arc::new(InMemoryCacheRef {
            config: Arc::new(config.into()),
            ..Default::default()
        }))
    }
}

#[async_trait]
impl Cache for InMemoryCache {
    type Error = InMemoryCacheError;
    type CurrentUser = Arc<CurrentUser>;
    type Emoji = Arc<CachedEmoji>;
    type Group = Arc<Group>;
    type Guild = Arc<CachedGuild>;
    type GuildChannel = Arc<GuildChannel>;
    type Member = Arc<CachedMember>;
    type Message = Arc<CachedMessage>;
    type Presence = Arc<CachedPresence>;
    type PrivateChannel = Arc<PrivateChannel>;
    type Role = Arc<Role>;
    type User = Arc<User>;
    type VoiceState = Arc<CachedVoiceState>;

    /// Gets a channel by ID.
    ///
    /// This is an O(1) operation.
    async fn guild_channel(
        &self,
        channel_id: ChannelId,
    ) -> Result<Option<Self::GuildChannel>, Self::Error> {
        Ok(self
            .0
            .channels_guild
            .lock()
            .await
            .get(&channel_id)
            .map(|x| Arc::clone(&x.data)))
    }

    /// Gets the current user.
    ///
    /// This is an O(1) operation.
    async fn current_user(&self) -> Result<Option<Self::CurrentUser>, Self::Error> {
        Ok(self.0.current_user.lock().await.clone())
    }

    /// Gets an emoji by ID.
    ///
    /// This is an O(1) operation.
    async fn emoji(&self, emoji_id: EmojiId) -> Result<Option<Self::Emoji>, Self::Error> {
        Ok(self
            .0
            .emojis
            .lock()
            .await
            .get(&emoji_id)
            .map(|x| Arc::clone(&x.data)))
    }

    /// Gets a group by ID.
    ///
    /// This is an O(1) operation.
    async fn group(&self, channel_id: ChannelId) -> Result<Option<Self::Group>, Self::Error> {
        Ok(self.0.groups.lock().await.get(&channel_id).cloned())
    }

    /// Gets a guild by ID.
    ///
    /// This is an O(1) operation.
    async fn guild(&self, guild_id: GuildId) -> Result<Option<Self::Guild>, Self::Error> {
        Ok(self.0.guilds.lock().await.get(&guild_id).cloned())
    }

    /// Gets a member by guild ID and user ID.
    ///
    /// This is an O(1) operation.
    async fn member(
        &self,
        guild_id: GuildId,
        user_id: UserId,
    ) -> Result<Option<Self::Member>, Self::Error> {
        Ok(self
            .0
            .members
            .lock()
            .await
            .get(&(guild_id, user_id))
            .cloned())
    }

    /// Gets a message by channel ID and message ID.
    ///
    /// This is an O(log n) operation.
    async fn message(
        &self,
        channel_id: ChannelId,
        message_id: MessageId,
    ) -> Result<Option<Self::Message>, Self::Error> {
        let channels = self.0.messages.lock().await;
        let channel = match channels.get(&channel_id) {
            Some(channel) => channel,
            None => return Ok(None),
        };

        Ok(channel.get(&message_id).cloned())
    }

    /// Gets a presence by, optionally, guild ID, and user ID.
    ///
    /// This is an O(1) operation.
    async fn presence(
        &self,
        guild_id: Option<GuildId>,
        user_id: UserId,
    ) -> Result<Option<Self::Presence>, Self::Error> {
        Ok(self
            .0
            .presences
            .lock()
            .await
            .get(&(guild_id, user_id))
            .cloned())
    }

    /// Gets a private channel by ID.
    ///
    /// This is an O(1) operation.
    async fn private_channel(
        &self,
        channel_id: ChannelId,
    ) -> Result<Option<Self::PrivateChannel>, Self::Error> {
        Ok(self
            .0
            .channels_private
            .lock()
            .await
            .get(&channel_id)
            .cloned())
    }

    /// Gets a role by ID.
    ///
    /// This is an O(1) operation.
    async fn role(&self, role_id: RoleId) -> Result<Option<Self::Role>, Self::Error> {
        Ok(self
            .0
            .roles
            .lock()
            .await
            .get(&role_id)
            .map(|x| Arc::clone(&x.data)))
    }

    /// Gets a user by ID.
    ///
    /// This is an O(1) operation.
    async fn user(&self, user_id: UserId) -> Result<Option<Self::User>, Self::Error> {
        Ok(self.0.users.lock().await.get(&user_id).cloned())
    }

    /// Gets a voice state by channel ID and user ID.
    ///
    /// This is an O(1) operation.
    async fn voice_state(
        &self,
        channel_id: ChannelId,
        user_id: UserId,
    ) -> Result<Option<Self::VoiceState>, Self::Error> {
        Ok(self
            .0
            .voice_states
            .lock()
            .await
            .get(&(channel_id, user_id))
            .cloned())
    }

    /// Clears the entire state of the Cache. This is equal to creating a new
    /// empty Cache.
    async fn clear(&self) -> Result<(), Self::Error> {
        self.0.channels_guild.lock().await.clear();
        self.0.current_user.lock().await.take();
        self.0.emojis.lock().await.clear();
        self.0.guilds.lock().await.clear();
        self.0.presences.lock().await.clear();
        self.0.roles.lock().await.clear();
        self.0.users.lock().await.clear();
        self.0.voice_states.lock().await.clear();

        Ok(())
    }
}

fn guild_channel_guild_id(channel: &GuildChannel) -> Option<&GuildId> {
    match channel {
        GuildChannel::Category(c) => c.guild_id.as_ref(),
        GuildChannel::Text(c) => c.guild_id.as_ref(),
        GuildChannel::Voice(c) => c.guild_id.as_ref(),
    }
}

fn guild_channel_id(channel: &GuildChannel) -> &ChannelId {
    match channel {
        GuildChannel::Category(c) => &c.id,
        GuildChannel::Text(c) => &c.id,
        GuildChannel::Voice(c) => &c.id,
    }
}

fn presence_user_id(presence: &Presence) -> UserId {
    match presence.user {
        UserOrId::User(ref u) => u.id,
        UserOrId::UserId {
            id,
        } => id,
    }
}
