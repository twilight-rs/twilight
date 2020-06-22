use crate::{guild::IntegrationAccount, id::IntegrationId, user::User};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct PartialGuildIntegration {
    pub id: Option<IntegrationId>,
    pub account: Option<IntegrationAccount>,
    pub enabled: Option<bool>,
    pub expire_behavior: Option<u64>,
    pub expire_grace_period: Option<u64>,
    #[serde(rename = "type")]
    pub kind: Option<String>,
    pub name: Option<String>,
    pub role_id: Option<IntegrationId>,
    pub synced_at: Option<String>,
    pub syncing: Option<bool>,
    pub user: Option<User>,
}
