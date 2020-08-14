use crate::{
    channel::{embed::Embed, message::MessageType, Attachment},
    id::{ChannelId, GuildId, MessageId, RoleId},
    user::User,
};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct MessageUpdate {
    pub attachments: Option<Vec<Attachment>>,
    pub author: Option<User>,
    pub channel_id: ChannelId,
    pub content: Option<String>,
    pub edited_timestamp: Option<String>,
    pub embeds: Option<Vec<Embed>>,
    pub guild_id: Option<GuildId>,
    pub id: MessageId,
    #[serde(rename = "type")]
    pub kind: Option<MessageType>,
    pub mention_everyone: Option<bool>,
    pub mention_roles: Option<Vec<RoleId>>,
    pub mentions: Option<Vec<User>>,
    pub pinned: Option<bool>,
    pub timestamp: Option<String>,
    pub tts: Option<bool>,
}
