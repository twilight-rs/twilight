use serde::{Deserialize, Serialize};

use crate::guild::SoundboardSound;

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct GuildSoundboardSoundCreate(pub SoundboardSound);
