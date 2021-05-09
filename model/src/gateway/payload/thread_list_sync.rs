use crate::{
    channel::{Channel, ThreadMember},
    id::GuildId,
};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct ThreadListSync {
    pub guild_id: GuildId,
    #[serde(default)]
    pub channel_ids: Vec<GuildId>,
    pub threads: Vec<Channel>,
    pub members: Vec<ThreadMember>,
}
