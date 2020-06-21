use crate::{
    channel::{permission_overwrite::PermissionOverwrite, ChannelType},
    id::{ChannelId, GuildId},
};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct CategoryChannel {
    pub id: ChannelId,
    pub guild_id: Option<GuildId>,
    #[serde(rename = "type")]
    pub kind: ChannelType,
    pub name: String,
    #[serde(default)]
    pub nsfw: bool,
    pub parent_id: Option<ChannelId>,
    pub permission_overwrites: Vec<PermissionOverwrite>,
    pub position: i64,
}
