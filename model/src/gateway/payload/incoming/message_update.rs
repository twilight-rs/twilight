use crate::{
    channel::{
        embed::Embed,
        message::{Mention, MessageType},
        Attachment,
    },
    datetime::Timestamp,
    id::{marker, Id},
    user::User,
};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct MessageUpdate {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attachments: Option<Vec<Attachment>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub author: Option<User>,
    pub channel_id: Id<marker::Channel>,
    pub content: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub edited_timestamp: Option<Timestamp>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub embeds: Option<Vec<Embed>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub guild_id: Option<Id<marker::Guild>>,
    pub id: Id<marker::Message>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub kind: Option<MessageType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mention_everyone: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mention_roles: Option<Vec<Id<marker::Role>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mentions: Option<Vec<Mention>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pinned: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<Timestamp>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tts: Option<bool>,
}
