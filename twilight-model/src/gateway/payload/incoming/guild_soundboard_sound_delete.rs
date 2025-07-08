use serde::{Deserialize, Serialize};

use crate::id::{
    marker::{GuildMarker, SoundboardSoundMarker},
    Id,
};

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct GuildSoundboardSoundDelete {
    pub guild_id: Id<GuildMarker>,
    pub sound_id: Id<SoundboardSoundMarker>,
}
