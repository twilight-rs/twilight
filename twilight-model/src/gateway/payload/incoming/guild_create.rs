use crate::guild::Guild;
use serde::{Deserialize, Serialize};
use std::ops::{Deref, DerefMut};

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct GuildCreate(pub Guild);

impl Deref for GuildCreate {
    type Target = Guild;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for GuildCreate {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
