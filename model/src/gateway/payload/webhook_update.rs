use crate::id::{ChannelId, GuildId};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct WebhookUpdate {
    pub channel_id: ChannelId,
    pub guild_id: GuildId,
}
