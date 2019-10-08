use crate::channel::Channel;
use std::ops::{Deref, DerefMut};

#[cfg_attr(
    feature = "serde-support",
    derive(serde::Deserialize, serde::Serialize)
)]
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct ChannelDelete(pub Channel);

impl Deref for ChannelDelete {
    type Target = Channel;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for ChannelDelete {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
