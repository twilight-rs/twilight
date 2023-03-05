use crate::guild::GuildIntegration;
use serde::{Deserialize, Serialize};
use std::ops::{Deref, DerefMut};

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
#[cfg_attr(
    feature = "rkyv",
    derive(rkyv::Archive, rkyv::Deserialize, rkyv::Serialize)
)]
pub struct IntegrationUpdate(pub GuildIntegration);

impl Deref for IntegrationUpdate {
    type Target = GuildIntegration;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for IntegrationUpdate {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
