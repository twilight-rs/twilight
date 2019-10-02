use crate::{guild::GuildIntegration, id::IntegrationId, user::ConnectionVisibility};
use std::{
    collections::HashMap,
    hash::{Hash, Hasher},
};

#[cfg_attr(
    feature = "serde-support",
    derive(serde::Deserialize, serde::Serialize)
)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Connection {
    pub id: String,
    pub friend_sync: bool,
    #[cfg_attr(feature = "serde-support", serde(with = "serde_mappable_seq"))]
    pub integrations: HashMap<IntegrationId, GuildIntegration>,
    #[cfg_attr(feature = "serde-support", serde(rename = "type"))]
    pub kind: String,
    pub name: String,
    pub revoked: bool,
    pub show_activity: bool,
    pub verified: bool,
    pub visibility: ConnectionVisibility,
}

impl Hash for Connection {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}
