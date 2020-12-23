use crate::applications::GuildInteraction;
use serde::{Deserialize, Serialize};
use std::ops::{Deref, DerefMut};

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct InteractionCreate(pub GuildInteraction);

impl Deref for InteractionCreate {
    type Target = GuildInteraction;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for InteractionCreate {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
