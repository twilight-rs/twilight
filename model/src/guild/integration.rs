use crate::{
    guild::IntegrationAccount,
    id::IntegrationId,
    user::User,
};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct GuildIntegration {
    pub id: IntegrationId,
    pub account: IntegrationAccount,
    pub enabled: bool,
    pub expire_behavior: u64,
    pub expire_grace_period: u64,
    #[serde(rename = "type")]
    pub kind: String,
    pub name: String,
    pub role_id: IntegrationId,
    #[cfg(feature = "chrono")]
    pub synced_at: chrono::DateTime<chrono::FixedOffset>,
    #[cfg(not(feature = "chrono"))]
    pub synced_at: String,
    pub syncing: bool,
    pub user: User,
}
