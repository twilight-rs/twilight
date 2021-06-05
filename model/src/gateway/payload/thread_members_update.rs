use crate::{
    channel::ThreadMember,
    id::{ChannelId, GuildId, UserId},
};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct ThreadMembersUpdate {
    #[serde(default)]
    pub added_members: Vec<ThreadMember>,
    pub guild_id: GuildId,
    pub id: ChannelId,
    /// Max value of 50.
    pub member_count: u8,
    #[serde(default)]
    pub removed_member_ids: Vec<UserId>,
}
