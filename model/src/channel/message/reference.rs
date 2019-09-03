use crate::id::{ChannelId, GuildId, MessageId};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct MessageReference {
    pub channel_id: ChannelId,
    pub guild_id: Option<GuildId>,
    pub message_id: Option<MessageId>,
}
