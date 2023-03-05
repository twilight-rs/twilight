use crate::{
    id::{
        marker::{ChannelMarker, GuildMarker},
        Id,
    },
    util::Timestamp,
};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
#[cfg_attr(
    feature = "rkyv",
    derive(rkyv::Archive, rkyv::Deserialize, rkyv::Serialize)
)]
pub struct ChannelPinsUpdate {
    pub channel_id: Id<ChannelMarker>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "rkyv", with(crate::id::IdNiche))]
    pub guild_id: Option<Id<GuildMarker>>,
    pub last_pin_timestamp: Option<Timestamp>,
}
