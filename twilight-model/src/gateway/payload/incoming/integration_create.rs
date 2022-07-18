use crate::guild::GuildIntegration;
use serde::{Deserialize, Serialize};
use std::ops::{Deref, DerefMut};

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct IntegrationCreate(pub GuildIntegration);

impl Deref for IntegrationCreate {
    type Target = GuildIntegration;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for IntegrationCreate {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
