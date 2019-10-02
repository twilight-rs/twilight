use crate::{id::GuildId, voice::VoiceState};

#[cfg_attr(
    feature = "serde-support",
    derive(serde::Deserialize, serde::Serialize)
)]
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct VoiceStateUpdate {
    pub guild_id: Option<GuildId>,
    pub voice_state: VoiceState,
}
