use crate::{channel::message::Sticker, id::GuildId};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct GuildStickersUpdate {
    pub guild_id: GuildId,
    pub stickers: Vec<Sticker>,
}
