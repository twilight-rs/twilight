use crate::id::{ChannelId, GuildId, MessageId};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct MessageDelete {
    pub channel_id: ChannelId,
    pub guild_id: Option<GuildId>,
    pub id: MessageId,
}
