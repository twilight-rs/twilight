use crate::guild::audit_log::AuditLogChangeKey;

#[cfg_attr(
    feature = "serde-support",
    derive(serde::Deserialize, serde::Serialize)
)]
#[derive(Clone, Debug)]
pub struct AuditLogChange {
    pub key: AuditLogChangeKey,
    #[cfg(feature = "serde-support")]
    pub new_value: Option<serde_json::Value>,
    #[cfg(feature = "serde-support")]
    pub old_value: Option<serde_json::Value>,
}
