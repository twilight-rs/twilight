use crate::id::{
    marker::{ChannelMarker, GuildMarker, MessageMarker},
    Id,
};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
#[cfg_attr(
    feature = "rkyv",
    derive(rkyv::Archive, rkyv::Deserialize, rkyv::Serialize)
)]
pub struct MessageDeleteBulk {
    pub channel_id: Id<ChannelMarker>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "rkyv", with(crate::id::IdNiche))]
    pub guild_id: Option<Id<GuildMarker>>,
    #[cfg_attr(feature = "rkyv", with(rkyv::with::CopyOptimize))]
    pub ids: Vec<Id<MessageMarker>>,
}
