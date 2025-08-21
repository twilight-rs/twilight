use serde::{Deserialize, Serialize};

use crate::{
    guild::SoundboardSound,
    id::{marker::GuildMarker, Id},
};

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct GuildSoundboardSoundsUpdate {
    pub guild_id: Id<GuildMarker>,
    pub soundboard_sounds: Vec<SoundboardSound>,
}
