use crate::{
    guild::Emoji,
    id::{Id, marker::GuildMarker},
};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct GuildEmojisUpdate {
    pub emojis: Vec<Emoji>,
    pub guild_id: Id<GuildMarker>,
}
