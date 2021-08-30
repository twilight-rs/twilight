use crate::channel::StageInstance;
use serde::{Deserialize, Serialize};
use std::ops::{Deref, DerefMut};

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct StageInstanceDelete(pub StageInstance);

impl Deref for StageInstanceDelete {
    type Target = StageInstance;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for StageInstanceDelete {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
