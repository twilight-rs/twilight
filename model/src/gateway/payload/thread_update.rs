use crate::channel::Channel;
use serde::{Deserialize, Serialize};
use std::ops::{Deref, DerefMut};

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct ThreadUpdate(pub Channel);

impl Deref for ThreadUpdate {
    type Target = Channel;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for ThreadUpdate {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
