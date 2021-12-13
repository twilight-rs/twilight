use crate::{
    channel::message::Sticker,
    id::{marker, Id},
};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct GuildStickersUpdate {
    pub guild_id: Id<marker::Guild>,
    pub stickers: Vec<Sticker>,
}
