//! Type definitions and trait implementations for a custom in-memory cache
//! that stores very little data about its entities.
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

/// Type alias for a cache that uses our custom cache types.
pub type CustomInMemoryCache = InMemoryCache<
    channel::MinimalCachedChannel,
    current_user::MinimalCachedCurrentUser,
    emoji::MinimalCachedEmoji,
    guild::MinimalCachedGuild,
    guild_integration::MinimalCachedGuildIntegration,
    member::MinimalCachedMember,
    message::MinimalCachedMessage,
    presence::MinimalCachedPresence,
    role::MinimalCachedRole,
    stage_instance::MinimalCachedStageInstance,
    sticker::MinimalCachedSticker,
    user::MinimalCachedUser,
    voice_state::MinimalCachedVoiceState,
>;
