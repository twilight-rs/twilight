use crate::{
    id::{ChannelId, MessageId},
    channel::ChannelType,
    user::User,
};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PrivateChannel {
    pub id: ChannelId,
    pub last_message_id: Option<MessageId>,
    #[cfg(feature = "chrono")]
    pub last_pin_timestamp: Option<chrono::DateTime<chrono::FixedOffset>>,
    #[cfg(not(feature = "chrono"))]
    pub last_pin_timestamp: Option<String>,
    #[serde(rename = "type")]
    pub kind: ChannelType,
    pub recipients: Vec<User>,
}
