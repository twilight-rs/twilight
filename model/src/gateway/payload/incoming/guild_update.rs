use crate::guild::PartialGuild;
use serde::{Deserialize, Serialize};
use std::ops::{Deref, DerefMut};

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct GuildUpdate(pub PartialGuild);

impl Deref for GuildUpdate {
    type Target = PartialGuild;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for GuildUpdate {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
