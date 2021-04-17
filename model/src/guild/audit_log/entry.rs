use crate::{
    guild::audit_log::{AuditLogChange, AuditLogEvent, AuditLogOptionalEntryInfo},
    id::{AuditLogEntryId, UserId},
};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AuditLogEntry {
    pub action_type: AuditLogEvent,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub changes: Option<Vec<AuditLogChange>>,
    pub id: AuditLogEntryId,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<AuditLogOptionalEntryInfo>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    pub target_id: Option<String>,
    pub user_id: Option<UserId>,
}
