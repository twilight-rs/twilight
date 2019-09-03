use crate::{
    channel::{
        permission_overwrite::PermissionOverwrite,
        ChannelType,
    },
    id::{ChannelId, GuildId, MessageId},
};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct TextChannel {
    pub id: ChannelId,
    pub guild_id: GuildId,
    #[serde(rename = "type")]
    pub kind: ChannelType,
    pub last_message_id: Option<MessageId>,
    #[cfg(feature = "chrono")]
    pub last_pin_timestamp: Option<chrono::DateTime<chrono::FixedOffset>>,
    #[cfg(not(feature = "chrono"))]
    pub last_pin_timestamp: Option<String>,
    pub name: String,
    #[serde(default)]
    pub nsfw: bool,
    pub permission_overwrites: Vec<PermissionOverwrite>,
    pub parent_id: Option<ChannelId>,
    pub position: i64,
    pub rate_limit_per_user: Option<u64>,
    pub topic: Option<String>,
}
