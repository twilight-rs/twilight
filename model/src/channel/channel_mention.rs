use crate::{
    channel::ChannelType,
    id::{ChannelId, GuildId},
};

#[cfg_attr(
    feature = "serde-support",
    derive(serde::Deserialize, serde::Serialize)
)]
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct ChannelMention {
    pub guild_id: GuildId,
    pub id: ChannelId,
    #[cfg_attr(feature = "serde-support", serde(rename = "type"))]
    pub kind: ChannelType,
    pub name: String,
}
