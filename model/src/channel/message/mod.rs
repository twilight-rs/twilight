mod kind;
mod activity;
mod activity_type;
mod application;
mod flags;
mod reference;
mod reaction;

pub use self::{
    kind::MessageType,
    activity::MessageActivity,
    activity_type::MessageActivityType,
    application::MessageApplication,
    flags::MessageFlags,
    reference::MessageReference,
    reaction::MessageReaction,
};

use crate::{
    channel::{
        embed::Embed,
        Attachment,
        ChannelMention,
    },
    guild::PartialMember,
    id::{ChannelId, GuildId, MessageId, RoleId, WebhookId},
    user::User,
};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Message {
    pub id: MessageId,
    pub activity: Option<MessageActivity>,
    pub application: Option<MessageApplication>,
    pub attachments: Vec<Attachment>,
    pub author: User,
    pub channel_id: ChannelId,
    pub content: String,
    #[cfg(feature = "chrono")]
    pub edited_timestamp: Option<chrono::DateTime<chrono::FixedOffset>>,
    #[cfg(not(feature = "chrono"))]
    pub edited_timestamp: Option<String>,
    pub embeds: Vec<Embed>,
    pub flags: Option<MessageFlags>,
    pub guild_id: Option<GuildId>,
    #[serde(rename = "type")]
    pub kind: MessageType,
    pub member: Option<PartialMember>,
    #[serde(default)]
    pub mention_channels: Vec<ChannelMention>,
    pub mention_everyone: bool,
    pub mention_roles: Vec<RoleId>,
    pub mentions: Vec<User>,
    pub pinned: bool,
    #[serde(default)]
    pub reactions: Vec<MessageReaction>,
    #[serde(rename = "message_reference")]
    pub reference: Option<MessageReference>,
    #[cfg(feature = "chrono")]
    pub timestamp: chrono::DateTime<chrono::FixedOffset>,
    #[cfg(not(feature = "chrono"))]
    pub timestamp: String,
    pub tts: bool,
    pub webhook_id: Option<WebhookId>,
}
