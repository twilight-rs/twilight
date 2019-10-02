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
pub struct CategoryChannel {
    pub id: ChannelId,
    pub guild_id: GuildId,
    #[cfg_attr(feature = "serde-support", serde(rename = "type"))]
    pub kind: ChannelType,
    pub name: String,
    #[cfg_attr(feature = "serde-support", serde(default))]
    pub nsfw: bool,
    pub parent_id: Option<ChannelId>,
    pub permission_overwrites: Vec<PermissionOverwrite>,
    pub position: i64,
}

impl Hash for CategoryChannel {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}
