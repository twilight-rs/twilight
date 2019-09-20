use crate::id::{ChannelId, MessageId};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct MessageDeleteBulk {
    pub channel_id: ChannelId,
    pub ids: Vec<MessageId>,
}
