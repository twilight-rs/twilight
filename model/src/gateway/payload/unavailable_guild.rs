use crate::id::GuildId;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct UnavailableGuild {
    pub id: GuildId,
}
