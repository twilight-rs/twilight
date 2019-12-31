use crate::{guild::IntegrationAccount, id::IntegrationId, user::User};

#[cfg_attr(
    feature = "serde-support",
    derive(serde::Deserialize, serde::Serialize)
)]
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct PartialGuildIntegration {
    pub id: Option<IntegrationId>,
    pub account: Option<IntegrationAccount>,
    pub enabled: Option<bool>,
    pub expire_behavior: Option<u64>,
    pub expire_grace_period: Option<u64>,
    #[cfg_attr(feature = "serde-support", serde(rename = "type"))]
    pub kind: Option<String>,
    pub name: Option<String>,
    pub role_id: Option<IntegrationId>,
    pub synced_at: Option<String>,
    pub syncing: Option<bool>,
    pub user: Option<User>,
}

#[cfg(feature = "serde-support")]
mod serde_support {
    use super::GuildIntegration;
    use crate::id::IntegrationId;
    use serde_mappable_seq::Key;

    impl Key<'_, IntegrationId> for PartialGuildIntegration {
        fn key(&self) -> IntegrationId {
            self.id
        }
    }
}
