use crate::{
    channel::thread::ThreadMember,
    id::{
        marker::{ChannelMarker, GuildMarker, UserMarker},
        Id,
    },
};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct ThreadMembersUpdate {
    /// List of thread members.
    ///
    /// Includes the [`member`] key.
    ///
    /// [`member`]: ThreadMember::member
    #[serde(default)]
    pub added_members: Vec<ThreadMember>,
    pub guild_id: Id<GuildMarker>,
    pub id: Id<ChannelMarker>,
    /// Max value of 50.
    pub member_count: u8,
    #[serde(default)]
    pub removed_member_ids: Vec<Id<UserMarker>>,
}
