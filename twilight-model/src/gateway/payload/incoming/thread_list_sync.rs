use crate::{
    channel::{thread::ThreadMember, Channel},
    id::{
        marker::{ChannelMarker, GuildMarker},
        Id,
    },
};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
#[cfg_attr(
    feature = "rkyv",
    derive(rkyv::Archive, rkyv::Deserialize, rkyv::Serialize)
)]
pub struct ThreadListSync {
    #[serde(default)]
    #[cfg_attr(feature = "rkyv", with(rkyv::with::CopyOptimize))]
    pub channel_ids: Vec<Id<ChannelMarker>>,
    pub guild_id: Id<GuildMarker>,
    pub members: Vec<ThreadMember>,
    pub threads: Vec<Channel>,
}
