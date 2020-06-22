use crate::id::ChannelId;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct ChannelPinsUpdate {
    pub channel_id: ChannelId,
    pub last_pin_timestamp: Option<String>,
}
