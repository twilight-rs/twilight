use crate::{
    channel::{permission_overwrite::PermissionOverwrite, ChannelType},
    id::{ChannelId, GuildId},
};
use std::hash::{Hash, Hasher};

#[cfg_attr(
    feature = "serde-support",
    derive(serde::Deserialize, serde::Serialize)
)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct VoiceChannel {
    pub id: ChannelId,
    pub bitrate: u64,
    pub guild_id: GuildId,
    #[cfg_attr(feature = "serde-support", serde(rename = "type"))]
    pub kind: ChannelType,
    pub name: String,
    pub permission_overwrites: Vec<PermissionOverwrite>,
    pub parent_id: Option<ChannelId>,
    pub position: i64,
    pub user_limit: Option<u64>,
}

impl Hash for VoiceChannel {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}
