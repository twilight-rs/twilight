use crate::application::interaction::Interaction;
use serde::{Deserialize, Serialize};
use std::ops::{Deref, DerefMut};

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[cfg_attr(
    feature = "rkyv",
    derive(rkyv::Archive, rkyv::Deserialize, rkyv::Serialize)
)]
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
