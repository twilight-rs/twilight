use crate::id::{
    marker::{ChannelMarker, GuildMarker},
    Id,
};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
#[cfg_attr(
    feature = "rkyv",
    derive(rkyv::Archive, rkyv::Deserialize, rkyv::Serialize)
)]
pub struct InviteDelete {
    pub channel_id: Id<ChannelMarker>,
    pub code: String,
    pub guild_id: Id<GuildMarker>,
}
