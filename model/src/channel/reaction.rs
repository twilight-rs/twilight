use crate::{
    channel::ReactionType,
    id::{ChannelId, MessageId, UserId},
};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Reaction {
    pub channel_id: ChannelId,
    pub emoji: ReactionType,
    pub message_id: MessageId,
    pub user_id: UserId,
}
