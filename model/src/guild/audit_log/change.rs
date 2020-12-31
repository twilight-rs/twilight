use crate::guild::audit_log::AuditLogChangeKey;
use serde::{Deserialize, Serialize};
use serde_value::Value;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AuditLogChange {
    pub key: AuditLogChangeKey,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub new_value: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub old_value: Option<Value>,
}
