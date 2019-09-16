use crate::{
    guild::GuildIntegration,
    id::IntegrationId,
    user::ConnectionVisibility,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Connection {
    pub id: String,
    pub friend_sync: bool,
    #[serde(with = "serde_mappable_seq")]
    pub integrations: HashMap<IntegrationId, GuildIntegration>,
    #[serde(rename = "type")]
    pub kind: String,
    pub name: String,
    pub revoked: bool,
    pub show_activity: bool,
    pub verified: bool,
    pub visibility: ConnectionVisibility,
}
