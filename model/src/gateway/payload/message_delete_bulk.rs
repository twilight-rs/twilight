use crate::id::{ChannelId, MessageId};

#[cfg_attr(
    feature = "serde-support",
    derive(serde::Deserialize, serde::Serialize)
)]
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct MessageDeleteBulk {
    pub channel_id: ChannelId,
    pub ids: Vec<MessageId>,
}
