use crate::{
    channel::{embed::Embed, message::MessageType, Attachment},
    id::{ChannelId, MessageId, RoleId},
    user::User,
};

#[cfg_attr(
    feature = "serde-support",
    derive(serde::Deserialize, serde::Serialize)
)]
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct MessageUpdate {
    pub id: MessageId,
    pub attachments: Option<Vec<Attachment>>,
    pub author: Option<User>,
    pub channel_id: ChannelId,
    pub content: Option<String>,
    pub edited_timestamp: Option<String>,
    pub embeds: Option<Vec<Embed>>,
    pub kind: Option<MessageType>,
    pub mention_everyone: Option<bool>,
    pub mention_roles: Option<Vec<RoleId>>,
    pub mentions: Option<Vec<User>>,
    pub pinned: Option<bool>,
    pub timestamp: Option<String>,
    pub tts: Option<bool>,
}
