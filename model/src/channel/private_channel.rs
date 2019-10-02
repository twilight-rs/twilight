use crate::{
    channel::ChannelType,
    id::{ChannelId, MessageId},
    user::User,
};
use std::hash::{Hash, Hasher};

#[cfg_attr(
    feature = "serde-support",
    derive(serde::Deserialize, serde::Serialize)
)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PrivateChannel {
    pub id: ChannelId,
    pub last_message_id: Option<MessageId>,
    pub last_pin_timestamp: Option<String>,
    #[cfg_attr(feature = "serde-support", serde(rename = "type"))]
    pub kind: ChannelType,
    pub recipients: Vec<User>,
}

impl Hash for PrivateChannel {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}
