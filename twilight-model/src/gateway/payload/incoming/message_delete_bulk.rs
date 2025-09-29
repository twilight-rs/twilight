use crate::id::{
    Id,
    marker::{ChannelMarker, GuildMarker, MessageMarker},
};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct MessageDeleteBulk {
    pub channel_id: Id<ChannelMarker>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub guild_id: Option<Id<GuildMarker>>,
    pub ids: Vec<Id<MessageMarker>>,
}
