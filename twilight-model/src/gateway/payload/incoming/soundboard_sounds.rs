use crate::guild::SoundboardSound;
use crate::id::{marker::GuildMarker, Id};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct SoundboardSounds {
    pub guild_id: Id<GuildMarker>,
    pub soundboard_sounds: Vec<SoundboardSound>,
}
