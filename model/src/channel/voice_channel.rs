use crate::{
    channel::{permission_overwrite::PermissionOverwrite, ChannelType},
    id::{ChannelId, GuildId},
};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct VoiceChannel {
    pub bitrate: u64,
    pub guild_id: Option<GuildId>,
    pub id: ChannelId,
    #[serde(rename = "type")]
    pub kind: ChannelType,
    pub name: String,
    pub permission_overwrites: Vec<PermissionOverwrite>,
    pub parent_id: Option<ChannelId>,
    pub position: i64,
    pub user_limit: Option<u64>,
}
