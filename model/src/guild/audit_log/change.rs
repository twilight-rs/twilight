use crate::guild::audit_log::AuditLogChangeKey;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AuditLogChange {
    pub key: AuditLogChangeKey,
    pub new_value: Option<Value>,
    pub old_value: Option<Value>,
}
