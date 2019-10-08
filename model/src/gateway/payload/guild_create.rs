use crate::guild::Guild;
use std::ops::{Deref, DerefMut};

#[cfg_attr(
    feature = "serde-support",
    derive(serde::Deserialize, serde::Serialize)
)]
#[derive(Clone, Debug, Eq, PartialEq)]
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
