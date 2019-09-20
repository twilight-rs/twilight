use crate::id::{ChannelId, MessageId};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct MessageDelete {
    pub channel_id: ChannelId,
    pub id: MessageId,
}
