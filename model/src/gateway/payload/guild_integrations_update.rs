use crate::id::GuildId;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct GuildIntegrationsUpdate {
    pub guild_id: GuildId,
}
