use crate::guild::PartialGuild;
use std::ops::{Deref, DerefMut};

#[cfg_attr(
    feature = "serde-support",
    derive(serde::Deserialize, serde::Serialize)
)]
#[derive(Clone, Debug, Eq, PartialEq)]
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
