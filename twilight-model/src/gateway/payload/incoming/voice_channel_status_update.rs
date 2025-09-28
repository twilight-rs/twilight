use crate::id::{
    marker::{ChannelMarker, GuildMarker},
    Id,
};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct VoiceChannelStatusUpdate {
    pub guild_id: Id<GuildMarker>,
    pub id: Id<ChannelMarker>,
    pub status: Option<String>,
}
