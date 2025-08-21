use crate::{
    gateway::OpCode,
    id::{marker::GuildMarker, Id},
};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct RequestSoundboardSounds {
    pub d: RequestSoundboardSoundsInfo,
    pub op: OpCode,
}

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct RequestSoundboardSoundsInfo {
    /// Guild IDs to request soundboard sounds for.
    pub guild_ids: Vec<Id<GuildMarker>>,
}
