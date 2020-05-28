use crate::{guild::GuildIntegration, id::IntegrationId, user::ConnectionVisibility};
use std::collections::HashMap;

#[cfg_attr(
    feature = "serde-support",
    derive(serde::Deserialize, serde::Serialize)
)]
#[allow(clippy::struct_excessive_bools)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Connection {
    pub id: String,
    pub friend_sync: bool,
    #[cfg_attr(feature = "serde-support", serde(with = "serde_mappable_seq", default))]
    pub integrations: HashMap<IntegrationId, GuildIntegration>,
    #[cfg_attr(feature = "serde-support", serde(rename = "type"))]
    pub kind: String,
    pub name: String,
    pub revoked: Option<bool>,
    pub show_activity: bool,
    pub verified: bool,
    pub visibility: ConnectionVisibility,
}
