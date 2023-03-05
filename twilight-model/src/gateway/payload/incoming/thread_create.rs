use crate::channel::Channel;
use serde::{Deserialize, Serialize};
use std::ops::{Deref, DerefMut};

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
#[cfg_attr(
    feature = "rkyv",
    derive(rkyv::Archive, rkyv::Deserialize, rkyv::Serialize)
)]
pub struct ThreadCreate(pub Channel);

impl Deref for ThreadCreate {
    type Target = Channel;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for ThreadCreate {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
