use crate::id::{ChannelId, UserId};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct TypingStart {
    pub channel_id: ChannelId,
    pub timestamp: u64,
    pub user_id: UserId,
}
