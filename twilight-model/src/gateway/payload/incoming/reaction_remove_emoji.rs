use crate::{
    channel::message::EmojiReactionType,
    id::{
        Id,
        marker::{ChannelMarker, GuildMarker, MessageMarker},
    },
};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct ReactionRemoveEmoji {
    pub channel_id: Id<ChannelMarker>,
    pub emoji: EmojiReactionType,
    pub guild_id: Id<GuildMarker>,
    pub message_id: Id<MessageMarker>,
}
