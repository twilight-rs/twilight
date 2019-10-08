use crate::channel::Message;
use std::ops::{Deref, DerefMut};

#[cfg_attr(
    feature = "serde-support",
    derive(serde::Deserialize, serde::Serialize)
)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MessageCreate(pub Message);

impl Deref for MessageCreate {
    type Target = Message;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for MessageCreate {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
