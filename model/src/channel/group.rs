use crate::{
    channel::ChannelType,
    id::{ApplicationId, ChannelId, MessageId, UserId},
    user::User,
};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Group {
    pub id: ChannelId,
    pub application_id: Option<ApplicationId>,
    pub icon: Option<String>,
    #[serde(rename = "type")]
    pub kind: ChannelType,
    pub last_message_id: Option<MessageId>,
    #[cfg(feature = "chrono")]
    pub last_pin_timestamp: Option<chrono::DateTime<chrono::FixedOffset>>,
    #[cfg(not(feature = "chrono"))]
    pub last_pin_timestamp: Option<String>,
    pub name: Option<String>,
    pub owner_id: UserId,
    pub recipients: Vec<User>,
}
