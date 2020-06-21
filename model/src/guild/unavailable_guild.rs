use crate::id::GuildId;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct UnavailableGuild {
    pub id: GuildId,
    pub unavailable: bool,
}
