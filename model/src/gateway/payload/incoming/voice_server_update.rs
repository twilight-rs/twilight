use crate::id::{marker::GuildMarker, Id};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct VoiceServerUpdate {
    pub endpoint: Option<String>,
    pub guild_id: Option<Id<GuildMarker>>,
    pub token: String,
}
