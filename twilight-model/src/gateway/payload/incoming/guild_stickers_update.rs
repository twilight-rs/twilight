use crate::{
    channel::message::Sticker,
    id::{marker::GuildMarker, Id},
};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[cfg_attr(
    feature = "rkyv",
    derive(rkyv::Archive, rkyv::Deserialize, rkyv::Serialize)
)]
pub struct GuildStickersUpdate {
    pub guild_id: Id<GuildMarker>,
    pub stickers: Vec<Sticker>,
}
