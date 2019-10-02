use crate::{
    channel::ReactionType,
    id::{ChannelId, MessageId, UserId},
};

#[cfg_attr(
    feature = "serde-support",
    derive(serde::Deserialize, serde::Serialize)
)]
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Reaction {
    pub channel_id: ChannelId,
    pub emoji: ReactionType,
    pub message_id: MessageId,
    pub user_id: UserId,
}
