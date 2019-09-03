use crate::id::ChannelId;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct GuildEmbed {
    pub channel_id: ChannelId,
    pub enabled: bool,
}
