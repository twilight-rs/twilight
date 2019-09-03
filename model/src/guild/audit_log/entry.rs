use crate::{
    guild::audit_log::{
        AuditLogChange,
        AuditLogEvent,
        AuditLogOptionalEntryInfo,
    },
    id::{AuditLogEntryId, GenericId, UserId},
};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AuditLogEntry {
    pub action_type: AuditLogEvent,
    pub changes: Option<Vec<AuditLogChange>>,
    pub id: AuditLogEntryId,
    pub options: Option<AuditLogOptionalEntryInfo>,
    pub reason: Option<String>,
    pub target_id: Option<GenericId>,
    pub user_id: UserId,
}
