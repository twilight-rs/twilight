use crate::{
    channel::{thread::ThreadMember, Channel},
    id::GuildId,
};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct ThreadListSync {
    #[serde(default)]
    pub channel_ids: Vec<GuildId>,
    pub guild_id: GuildId,
    pub members: Vec<ThreadMember>,
    pub threads: Vec<Channel>,
}
