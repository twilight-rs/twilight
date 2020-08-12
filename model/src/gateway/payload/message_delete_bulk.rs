use crate::id::{ChannelId, GuildId, MessageId};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct MessageDeleteBulk {
    pub channel_id: ChannelId,
    pub guild_id: Option<GuildId>,
    pub ids: Vec<MessageId>,
}
