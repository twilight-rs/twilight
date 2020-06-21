use crate::id::{ChannelId, EmojiId, GuildId, MessageId};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct ReactionRemoveEmoji {
    pub channel_id: ChannelId,
    pub message_id: MessageId,
    pub guild_id: GuildId,
    pub emoji: PartialEmoji,
}

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct PartialEmoji {
    id: Option<EmojiId>,
    name: String,
}
