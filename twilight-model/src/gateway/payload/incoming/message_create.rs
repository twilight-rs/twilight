use crate::channel::Message;
use serde::{Deserialize, Serialize};
use std::ops::{Deref, DerefMut};

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
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
