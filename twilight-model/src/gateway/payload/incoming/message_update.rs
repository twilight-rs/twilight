use crate::{
    channel::{
        embed::Embed,
        message::{Mention, MessageType},
        Attachment,
    },
    id::{
        marker::{ChannelMarker, GuildMarker, MessageMarker, RoleMarker},
        Id,
    },
    user::User,
    util::Timestamp,
};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct MessageUpdate {
    /// List of attachments.
    ///
    /// Refer to the documentation for [`Message::attachments`] for caveats with
    /// receiving the attachments of messages.
    ///
    /// [`Message::attachments`]: crate::channel::Message::attachments
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attachments: Option<Vec<Attachment>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub author: Option<User>,
    pub channel_id: Id<ChannelMarker>,
    /// Content of the message.
    ///
    /// Refer to the documentation for [`Message::content`] for caveats with
    /// receiving the content of messages.
    ///
    /// [`Message::content`]: crate::channel::Message::content
    pub content: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub edited_timestamp: Option<Timestamp>,
    /// List of embeds.
    ///
    /// Refer to the documentation for [`Message::embeds`] for caveats with
    /// receiving the embeds of messages.
    ///
    /// [`Message::embeds`]: crate::channel::Message::embeds
    #[serde(skip_serializing_if = "Option::is_none")]
    pub embeds: Option<Vec<Embed>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub guild_id: Option<Id<GuildMarker>>,
    pub id: Id<MessageMarker>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub kind: Option<MessageType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mention_everyone: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mention_roles: Option<Vec<Id<RoleMarker>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mentions: Option<Vec<Mention>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pinned: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<Timestamp>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tts: Option<bool>,
}
