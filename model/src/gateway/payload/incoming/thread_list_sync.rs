use crate::{
    channel::{thread::ThreadMember, Channel},
    id::{
        marker::{ChannelMarker, GuildMarker},
        Id,
    },
};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct ThreadListSync {
    #[serde(default)]
    pub channel_ids: Vec<Id<ChannelMarker>>,
    pub guild_id: Id<GuildMarker>,
    pub members: Vec<ThreadMember>,
    pub threads: Vec<Channel>,
}
