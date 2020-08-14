use crate::id::{ChannelId, GuildId};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct ChannelPinsUpdate {
    pub channel_id: ChannelId,
    pub guild_id: Option<GuildId>,
    pub last_pin_timestamp: Option<String>,
}
