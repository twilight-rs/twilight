use crate::channel::StageInstance;
use serde::{Deserialize, Serialize};
use std::ops::{Deref, DerefMut};

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct StageInstanceCreate(pub StageInstance);

impl Deref for StageInstanceCreate {
    type Target = StageInstance;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for StageInstanceCreate {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
