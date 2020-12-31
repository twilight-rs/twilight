use crate::id::{ChannelId, GenericId, MessageId, UserId};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct AuditLogOptionalEntryInfo {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel_id: Option<ChannelId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delete_member_days: Option<String>,
    pub id: Option<GenericId>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_id: Option<UserId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub members_removed: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_id: Option<MessageId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_name: Option<String>,
}
