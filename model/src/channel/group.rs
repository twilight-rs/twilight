use crate::{
    channel::ChannelType,
    id::{ApplicationId, ChannelId, MessageId, UserId},
    user::User,
};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct Group {
    pub application_id: Option<ApplicationId>,
    pub icon: Option<String>,
    pub id: ChannelId,
    #[serde(rename = "type")]
    pub kind: ChannelType,
    pub last_message_id: Option<MessageId>,
    pub last_pin_timestamp: Option<String>,
    pub name: Option<String>,
    pub owner_id: UserId,
    pub recipients: Vec<User>,
}
