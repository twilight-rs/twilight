use crate::id::{ChannelId, GuildId};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct InviteDelete {
    pub channel_id: ChannelId,
    pub code: String,
    pub guild_id: GuildId,
}
