use crate::{
    channel::thread::ThreadMember,
    id::{Id, marker::GuildMarker},
};
use serde::{Deserialize, Serialize};
use std::ops::{Deref, DerefMut};

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct ThreadMemberUpdate {
    pub guild_id: Id<GuildMarker>,
    #[serde(flatten)]
    pub member: ThreadMember,
}

impl Deref for ThreadMemberUpdate {
    type Target = ThreadMember;

    fn deref(&self) -> &Self::Target {
        &self.member
    }
}

impl DerefMut for ThreadMemberUpdate {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.member
    }
}
