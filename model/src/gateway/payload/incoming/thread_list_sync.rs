use crate::{
    channel::{thread::ThreadMember, Channel},
    id::{marker::GuildMarker, Id},
};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct ThreadListSync {
    #[serde(default)]
    pub channel_ids: Vec<Id<GuildMarker>>,
    pub guild_id: Id<GuildMarker>,
    pub members: Vec<ThreadMember>,
    pub threads: Vec<Channel>,
}
