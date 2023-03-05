use crate::{
    guild::Emoji,
    id::{marker::GuildMarker, Id},
};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[cfg_attr(
    feature = "rkyv",
    derive(rkyv::Archive, rkyv::Deserialize, rkyv::Serialize)
)]
pub struct GuildEmojisUpdate {
    pub emojis: Vec<Emoji>,
    pub guild_id: Id<GuildMarker>,
}
