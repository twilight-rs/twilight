use crate::channel::Reaction;
use std::ops::{Deref, DerefMut};

#[cfg_attr(
    feature = "serde-support",
    derive(serde::Deserialize, serde::Serialize)
)]
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct ReactionRemove(pub Reaction);

impl Deref for ReactionRemove {
    type Target = Reaction;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for ReactionRemove {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
