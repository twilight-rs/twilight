use crate::id::GuildId;
use std::hash::{Hash, Hasher};

#[cfg_attr(
    feature = "serde-support",
    derive(serde::Deserialize, serde::Serialize)
)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UnavailableGuild {
    pub id: GuildId,
    pub unavailable: bool,
}

impl Hash for UnavailableGuild {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}
