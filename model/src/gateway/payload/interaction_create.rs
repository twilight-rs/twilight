use crate::applications::Interaction;
use serde::{Deserialize, Serialize};
use std::ops::{Deref, DerefMut};

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct InteractionCreate(pub Interaction);

impl Deref for InteractionCreate {
    type Target = Interaction;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for InteractionCreate {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
