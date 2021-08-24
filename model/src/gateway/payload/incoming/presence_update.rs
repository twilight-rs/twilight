use crate::{
    gateway::presence::{Activity, ClientStatus, Status, UserOrId},
    id::GuildId,
};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct PresenceUpdate {
    #[serde(default)]
    pub activities: Vec<Activity>,
    pub client_status: ClientStatus,
    pub game: Option<Activity>,
    pub guild_id: GuildId,
    pub status: Status,
    pub user: UserOrId,
}
