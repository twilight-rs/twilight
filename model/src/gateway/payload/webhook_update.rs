use crate::id::{ChannelId, GuildId};

#[cfg_attr(
    feature = "serde-support",
    derive(serde::Deserialize, serde::Serialize)
)]
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct WebhookUpdate {
    pub channel_id: ChannelId,
    pub guild_id: GuildId,
}
