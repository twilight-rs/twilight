use crate::{
    guild::Emoji,
    id::{EmojiId, GuildId},
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct GuildEmojisUpdate {
    #[serde(with = "serde_mappable_seq")]
    pub emojis: HashMap<EmojiId, Emoji>,
    pub guild_id: GuildId,
}
