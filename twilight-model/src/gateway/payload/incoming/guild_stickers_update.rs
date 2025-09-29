use crate::{
    channel::message::Sticker,
    id::{Id, marker::GuildMarker},
};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct GuildStickersUpdate {
    pub guild_id: Id<GuildMarker>,
    pub stickers: Vec<Sticker>,
}
