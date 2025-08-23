use crate::id::{Id, marker::GuildMarker};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct GuildIntegrationsUpdate {
    pub guild_id: Id<GuildMarker>,
}
