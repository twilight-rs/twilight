use crate::{
    channel::{
        permission_overwrite::PermissionOverwrite,
        ChannelType,
    },
    id::{ChannelId, GuildId},
};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct VoiceChannel {
    pub id: ChannelId,
    pub bitrate: u64,
    pub guild_id: GuildId,
    #[serde(rename = "type")]
    pub kind: ChannelType,
    pub name: String,
    pub permission_overwrites: Vec<PermissionOverwrite>,
    pub parent_id: Option<ChannelId>,
    pub position: i64,
    pub user_limit: Option<u64>,
}
