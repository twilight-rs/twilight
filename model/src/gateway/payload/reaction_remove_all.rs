use crate::id::{ChannelId, MessageId};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct ReactionRemoveAll {
    pub channel_id: ChannelId,
    pub message_id: MessageId,
}
