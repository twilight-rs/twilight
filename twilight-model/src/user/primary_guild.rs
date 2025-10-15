use crate::id::{marker::GuildMarker, Id};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct PrimaryGuild {
    pub identity_guild_id: Option<Id<GuildMarker>>,
    pub identity_enabled: Option<bool>,
    pub tag: Option<String>,
    pub badge: Option<String>,
}
