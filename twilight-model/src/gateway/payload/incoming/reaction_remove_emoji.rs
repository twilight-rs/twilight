use crate::{
    channel::message::EmojiReactionType,
    id::{
        marker::{ChannelMarker, GuildMarker, MessageMarker},
        Id,
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
