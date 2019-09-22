use crate::id::ChannelId;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct ChannelPinsUpdate {
    pub channel_id: ChannelId,
    #[cfg(feature = "chrono")]
    pub last_pin_timestamp: Option<chrono::DateTime<chrono::FixedOffset>>,
    #[cfg(not(feature = "chrono"))]
    pub last_pin_timestamp: Option<String>,
}
