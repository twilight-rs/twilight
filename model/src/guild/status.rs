use crate::guild::{Guild, UnavailableGuild};
use serde::{Deserialize, Serialize};

#[allow(clippy::large_enum_variant)]
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum GuildStatus {
    OnlineGuild(Guild),
    Offline(UnavailableGuild),
}
