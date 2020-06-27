use crate::id::{ChannelId, GenericId, MessageId, UserId};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct AuditLogOptionalEntryInfo {
    pub channel_id: Option<ChannelId>,
    pub count: Option<String>,
    pub delete_member_days: Option<String>,
    pub id: Option<GenericId>,
    #[serde(rename = "type")]
    pub kind: Option<String>,
    pub member_id: Option<UserId>,
    pub members_removed: Option<String>,
    pub message_id: Option<MessageId>,
    pub role_name: Option<String>,
}
