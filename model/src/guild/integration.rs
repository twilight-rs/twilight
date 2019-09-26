use crate::{guild::IntegrationAccount, id::IntegrationId, user::User};
use serde::{Deserialize, Serialize};
use serde_mappable_seq::Key;
use std::hash::{Hash, Hasher};

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
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

impl Hash for GuildIntegration {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

impl Key<'_, IntegrationId> for GuildIntegration {
    fn key(&self) -> IntegrationId {
        self.id
    }
}
