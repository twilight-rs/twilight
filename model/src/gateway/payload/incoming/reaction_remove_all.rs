use crate::id::{
    marker::{ChannelMarker, GuildMarker, MessageMarker},
    Id,
};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct ReactionRemoveAll {
    pub channel_id: Id<ChannelMarker>,
    pub message_id: Id<MessageMarker>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub guild_id: Option<Id<GuildMarker>>,
}
