use crate::id::GuildId;
use serde::{Deserialize, Serialize};
use std::hash::{Hash, Hasher};

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct UnavailableGuild {
    pub id: GuildId,
    pub unavailable: bool,
}

impl Hash for UnavailableGuild {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}
