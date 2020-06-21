use crate::{guild::{Guild, UnavailableGuild}, id::GuildId};
use serde::{Deserialize, Serialize};
use serde_mappable_seq::Key;

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(untagged)]
pub enum GuildStatus {
    Online(Guild),
    Offline(UnavailableGuild),
}

impl Key<'_, GuildId> for GuildStatus {
    fn key(&self) -> GuildId {
        match self {
            Self::Online(g) => g.id,
            Self::Offline(u) => u.id,
        }
    }
}
