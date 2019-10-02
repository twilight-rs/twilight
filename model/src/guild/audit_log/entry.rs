use crate::{
    guild::audit_log::{AuditLogChange, AuditLogEvent, AuditLogOptionalEntryInfo},
    id::{AuditLogEntryId, UserId},
};

#[cfg_attr(
    feature = "serde-support",
    derive(serde::Deserialize, serde::Serialize)
)]
#[derive(Clone, Debug)]
pub struct AuditLogEntry {
    pub action_type: AuditLogEvent,
    pub changes: Option<Vec<AuditLogChange>>,
    pub id: AuditLogEntryId,
    pub options: Option<AuditLogOptionalEntryInfo>,
    pub reason: Option<String>,
    pub target_id: Option<String>,
    pub user_id: UserId,
}
