use crate::id::{ChannelId, EmojiId, GuildId, MessageId};

#[cfg_attr(
    feature = "serde-support",
    derive(serde::Deserialize, serde::Serialize)
)]
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct ReactionRemoveEmoji {
    pub channel_id: ChannelId,
    pub message_id: MessageId,
    pub guild_id: GuildId,
    pub emoji: ParitalEmoji,
}

#[cfg_attr(
    feature = "serde-support",
    derive(serde::Deserialize, serde::Serialize)
)]
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct ParitalEmoji {
    id: Option<EmojiId>,
    name: String,
}
