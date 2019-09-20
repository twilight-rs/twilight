use crate::id::{GuildId, RoleId};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct RoleDelete {
    pub guild_id: GuildId,
    pub role_id: RoleId,
}
