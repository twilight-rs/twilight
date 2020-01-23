use crate::id::{ChannelId, GuildId};

#[cfg_attr(
    feature = "serde-support",
    derive(serde::Deserialize, serde::Serialize)
)]
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct InviteDelete {
    pub channel_id: ChannelId,
    pub code: String,
    pub guild_id: GuildId,
}
