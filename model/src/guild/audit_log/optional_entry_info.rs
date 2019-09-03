use crate::id::{ChannelId, GenericId};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AuditLogOptionalEntryInfo {
    pub channel_id: Option<ChannelId>,
    pub count: Option<String>,
    pub delete_member_days: Option<String>,
    pub id: Option<GenericId>,
    pub members_removed: Option<String>,
    #[serde(rename = "type")]
    pub kind: Option<String>,
    pub role_name: Option<String>,
}
