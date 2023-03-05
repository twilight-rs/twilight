use crate::id::{marker::GuildMarker, Id};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
#[cfg_attr(
    feature = "rkyv",
    derive(rkyv::Archive, rkyv::Deserialize, rkyv::Serialize)
)]
pub struct VoiceServerUpdate {
    /// Discord voice server endpoint.
    pub endpoint: Option<String>,
    pub guild_id: Id<GuildMarker>,
    /// Voice authentication token to connect to the Discord voice server.
    pub token: String,
}
