use crate::{
    gateway::presence::{Activity, ClientStatus, Status, UserOrId},
    id::{GuildId, RoleId},
};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct PresenceUpdate {
    #[serde(default)]
    pub activities: Vec<Activity>,
    pub client_status: ClientStatus,
    pub game: Option<Activity>,
    pub guild_id: Option<GuildId>,
    pub nick: Option<String>,
    pub premium_since: Option<String>,
    pub roles: Option<Vec<RoleId>>,
    pub status: Status,
    pub user: UserOrId,
}
