use crate::id::{ChannelId, EmojiId, GuildId, MessageId};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct ReactionRemoveEmoji {
    pub channel_id: ChannelId,
    pub emoji: PartialEmoji,
    pub guild_id: GuildId,
    pub message_id: MessageId,
}

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct PartialEmoji {
    pub id: Option<EmojiId>,
    pub name: String,
}
