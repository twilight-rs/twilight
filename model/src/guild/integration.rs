use crate::{guild::IntegrationAccount, id::IntegrationId, user::User};

#[cfg_attr(
    feature = "serde-support",
    derive(serde::Deserialize, serde::Serialize)
)]
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct GuildIntegration {
    pub id: IntegrationId,
    pub account: IntegrationAccount,
    pub enabled: bool,
    pub expire_behavior: u64,
    pub expire_grace_period: u64,
    #[cfg_attr(feature = "serde-support", serde(rename = "type"))]
    pub kind: String,
    pub name: String,
    pub role_id: IntegrationId,
    pub synced_at: String,
    pub syncing: bool,
    pub user: User,
}

#[cfg(feature = "serde-support")]
mod serde_support {
    use super::GuildIntegration;
    use crate::id::IntegrationId;
    use serde_mappable_seq::Key;

    impl Key<'_, IntegrationId> for GuildIntegration {
        fn key(&self) -> IntegrationId {
            self.id
        }
    }
}
