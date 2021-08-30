use crate::channel::Channel;
use serde::{Deserialize, Serialize};
use std::ops::{Deref, DerefMut};

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct ChannelCreate(pub Channel);

impl Deref for ChannelCreate {
    type Target = Channel;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for ChannelCreate {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
