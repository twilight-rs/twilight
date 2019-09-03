use crate::{
    guild::GuildIntegration,
    user::ConnectionVisibility,
};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Connection {
    pub id: String,
    pub friend_sync: bool,
    pub integrations: Vec<GuildIntegration>,
    #[serde(rename = "type")]
    pub kind: String,
    pub name: String,
    pub revoked: bool,
    pub show_activity: bool,
    pub verified: bool,
    pub visibility: ConnectionVisibility,
}
