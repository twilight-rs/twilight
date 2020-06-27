use crate::{guild::IntegrationAccount, id::IntegrationId, user::User};
use serde::{Deserialize, Serialize};
use serde_mappable_seq::Key;

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct GuildIntegration {
    pub account: IntegrationAccount,
    pub enabled: bool,
    pub expire_behavior: u64,
    pub expire_grace_period: u64,
    pub id: IntegrationId,
    #[serde(rename = "type")]
    pub kind: String,
    pub name: String,
    pub role_id: IntegrationId,
    pub synced_at: String,
    pub syncing: bool,
    pub user: User,
}

impl Key<'_, IntegrationId> for GuildIntegration {
    fn key(&self) -> IntegrationId {
        self.id
    }
}
