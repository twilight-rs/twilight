use crate::channel::thread::ThreadMember;
use serde::{Deserialize, Serialize};
use std::ops::{Deref, DerefMut};

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct ThreadMemberUpdate(pub ThreadMember);

impl Deref for ThreadMemberUpdate {
    type Target = ThreadMember;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for ThreadMemberUpdate {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
