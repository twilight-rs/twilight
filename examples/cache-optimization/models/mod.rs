//! Type definitions and trait implementations for a custom in-memory cache
//! that stores very little data about its entities.
use twilight_cache_inmemory::CacheableModels;
use twilight_cache_inmemory::InMemoryCache;

pub mod channel;
pub mod current_user;
pub mod emoji;
pub mod guild;
pub mod guild_integration;
pub mod member;
pub mod message;
pub mod presence;
pub mod role;
pub mod stage_instance;
pub mod sticker;
pub mod user;
pub mod voice_state;

#[derive(Clone, Debug)]
pub struct CustomCacheModels;

impl CacheableModels for CustomCacheModels {
    type Channel = channel::MinimalCachedChannel;
    type CurrentUser = current_user::MinimalCachedCurrentUser;
    type Emoji = emoji::MinimalCachedEmoji;
    type Guild = guild::MinimalCachedGuild;
    type GuildIntegration = guild_integration::MinimalCachedGuildIntegration;
    type Member = member::MinimalCachedMember;
    type Message = message::MinimalCachedMessage;
    type Presence = presence::MinimalCachedPresence;
    type Role = role::MinimalCachedRole;
    type StageInstance = stage_instance::MinimalCachedStageInstance;
    type Sticker = sticker::MinimalCachedSticker;
    type User = user::MinimalCachedUser;
    type VoiceState = voice_state::MinimalCachedVoiceState;
}

/// Type alias for a cache that uses our custom cache types.
pub type CustomInMemoryCache = InMemoryCache<CustomCacheModels>;
