use crate::guild::PartialGuild;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct GuildDelete {
    pub guild: PartialGuild,
}
