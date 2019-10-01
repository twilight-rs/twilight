use crate::{
    channel::ChannelType,
    id::{ApplicationId, ChannelId, MessageId, UserId},
    user::User,
};
use serde::{Deserialize, Serialize};
use std::hash::{Hash, Hasher};

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Group {
    pub id: ChannelId,
    pub application_id: Option<ApplicationId>,
    pub icon: Option<String>,
    #[serde(rename = "type")]
    pub kind: ChannelType,
    pub last_message_id: Option<MessageId>,
    pub last_pin_timestamp: Option<String>,
    pub name: Option<String>,
    pub owner_id: UserId,
    pub recipients: Vec<User>,
}

impl Hash for Group {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}
