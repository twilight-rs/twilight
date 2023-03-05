use crate::gateway::GatewayReaction;
use serde::{Deserialize, Serialize};
use std::ops::{Deref, DerefMut};

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
#[cfg_attr(
    feature = "rkyv",
    derive(rkyv::Archive, rkyv::Deserialize, rkyv::Serialize)
)]
pub struct ReactionRemove(pub GatewayReaction);

impl Deref for ReactionRemove {
    type Target = GatewayReaction;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for ReactionRemove {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
