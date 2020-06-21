use crate::channel::Reaction;
use serde::{Deserialize, Serialize};
use std::ops::{Deref, DerefMut};

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
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
