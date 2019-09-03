use crate::{
    guild::Role,
    id::GuildId,
};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RoleUpdate {
    pub guild_id: GuildId,
    pub role: Role,
}
