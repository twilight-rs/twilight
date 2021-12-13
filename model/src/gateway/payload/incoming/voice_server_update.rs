use crate::id::{marker, Id};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct VoiceServerUpdate {
    pub channel_id: Option<Id<marker::Channel>>,
    pub endpoint: Option<String>,
    pub guild_id: Option<Id<marker::Guild>>,
    pub token: String,
}
