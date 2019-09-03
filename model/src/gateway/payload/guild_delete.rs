use crate::guild::PartialGuild;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct GuildDelete {
    pub guild: PartialGuild,
}
