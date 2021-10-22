use crate::{
    channel::ReactionType,
    id::{ChannelId, GuildId, MessageId},
};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct ReactionRemoveEmoji {
    pub channel_id: ChannelId,
    pub emoji: ReactionType,
    pub guild_id: GuildId,
    pub message_id: MessageId,
}
