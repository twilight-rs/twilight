use crate::{
    guild::{Guild, UnavailableGuild},
    id::GuildId,
};
use serde::{Deserialize, Serialize};
use serde_mappable_seq::Key;

#[allow(clippy::large_enum_variant)]
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum GuildStatus {
    OnlineGuild(Guild),
    Offline(UnavailableGuild),
}

impl Key<'_, GuildId> for GuildStatus {
    fn key(&self) -> GuildId {
        match self {
            GuildStatus::OnlineGuild(g) => g.id,
            GuildStatus::Offline(u) => u.id,
        }
    }
}
