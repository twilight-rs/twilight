use serde::{Deserialize, Serialize};
use crate::id::{Id, marker::GuildMarker};

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct PrimaryGuild {
    pub identity_guild_id: Option<Id<GuildMarker>>,
    pub identity_enabled: Option<bool>,
    pub tag: Option<String>,
    pub badge: Option<String>,
}
