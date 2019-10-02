use crate::{
    guild::Emoji,
    id::{EmojiId, GuildId},
};
use std::collections::HashMap;

#[cfg_attr(
    feature = "serde-support",
    derive(serde::Deserialize, serde::Serialize)
)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GuildEmojisUpdate {
    #[cfg_attr(feature = "serde-support", serde(with = "serde_mappable_seq"))]
    pub emojis: HashMap<EmojiId, Emoji>,
    pub guild_id: GuildId,
}
