use crate::{
    channel::ChannelType,
    id::{ChannelId, MessageId},
    user::User,
};
use serde::{Deserialize, Serialize};
use std::hash::{Hash, Hasher};

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct PrivateChannel {
    pub id: ChannelId,
    pub last_message_id: Option<MessageId>,
    pub last_pin_timestamp: Option<String>,
    #[serde(rename = "type")]
    pub kind: ChannelType,
    pub recipients: Vec<User>,
}

impl Hash for PrivateChannel {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}
