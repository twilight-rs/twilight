use crate::id::{ChannelId, GuildId, MessageId};

#[cfg_attr(
    feature = "serde-support",
    derive(serde::Deserialize, serde::Serialize)
)]
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct MessageReference {
    pub channel_id: ChannelId,
    pub guild_id: Option<GuildId>,
    pub message_id: Option<MessageId>,
}
