//! Models built for utilizing efficient caching.

pub mod message;

mod emoji;
mod guild;
pub(crate) mod member;
mod presence;
mod sticker;
mod voice_state;

pub use self::{
    emoji::CachedEmoji, guild::CachedGuild, member::CachedMember, message::CachedMessage,
    presence::CachedPresence, sticker::CachedSticker, voice_state::CachedVoiceState,
};
