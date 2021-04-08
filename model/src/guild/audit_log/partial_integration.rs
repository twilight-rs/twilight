use crate::{guild::IntegrationAccount, id::IntegrationId, user::User};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct PartialGuildIntegration {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account: Option<IntegrationAccount>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expire_behavior: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expire_grace_period: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<IntegrationId>,
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_id: Option<IntegrationId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub synced_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub syncing: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<User>,
}
