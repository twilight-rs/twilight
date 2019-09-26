use crate::{id::GuildId, voice::VoiceState};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct VoiceStateUpdate {
    pub guild_id: Option<GuildId>,
    pub voice_state: VoiceState,
}
