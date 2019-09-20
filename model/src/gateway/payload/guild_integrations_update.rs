use crate::id::GuildId;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct GuildIntegrationsUpdate {
    pub guild_id: GuildId,
}
