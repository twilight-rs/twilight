use crate::channel::ChannelType;
use crate::id::{ChannelId, GuildId};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct ThreadDelete {
    pub guild_id: GuildId,
    pub id: ChannelId,
    #[serde(rename = "type")]
    pub kind: ChannelType,
    pub parent_id: ChannelId,
}
