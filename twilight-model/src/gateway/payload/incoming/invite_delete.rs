use crate::id::{
    Id,
    marker::{ChannelMarker, GuildMarker},
};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct InviteDelete {
    pub channel_id: Id<ChannelMarker>,
    pub code: String,
    pub guild_id: Id<GuildMarker>,
}
