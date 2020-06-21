use crate::id::ChannelId;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct GuildWidget {
    pub channel_id: ChannelId,
    pub enabled: bool,
}
