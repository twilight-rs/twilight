mod activity;
mod activity_type;
mod application;
mod flags;
mod kind;
mod reaction;
mod reference;

pub use self::{
    activity::MessageActivity,
    activity_type::MessageActivityType,
    application::MessageApplication,
    flags::MessageFlags,
    kind::MessageType,
    reaction::MessageReaction,
    reference::MessageReference,
};

use crate::{
    channel::{embed::Embed, Attachment, ChannelMention},
    guild::PartialMember,
    id::{ChannelId, GuildId, MessageId, RoleId, UserId, WebhookId},
    user::User,
};
use std::collections::HashMap;

#[cfg_attr(
    feature = "serde-support",
    derive(serde::Deserialize, serde::Serialize)
)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Message {
    pub id: MessageId,
    pub activity: Option<MessageActivity>,
    pub application: Option<MessageApplication>,
    pub attachments: Vec<Attachment>,
    pub author: User,
    pub channel_id: ChannelId,
    pub content: String,
    pub edited_timestamp: Option<String>,
    pub embeds: Vec<Embed>,
    pub flags: Option<MessageFlags>,
    pub guild_id: Option<GuildId>,
    #[cfg_attr(feature = "serde-support", serde(rename = "type"))]
    pub kind: MessageType,
    pub member: Option<PartialMember>,
    #[cfg_attr(feature = "serde-support", serde(default))]
    pub mention_channels: Vec<ChannelMention>,
    pub mention_everyone: bool,
    pub mention_roles: Vec<RoleId>,
    #[cfg_attr(feature = "serde-support", serde(with = "serde_mappable_seq"))]
    pub mentions: HashMap<UserId, User>,
    pub pinned: bool,
    #[cfg_attr(feature = "serde-support", serde(default))]
    pub reactions: Vec<MessageReaction>,
    #[cfg_attr(feature = "serde-support", serde(rename = "message_reference"))]
    pub reference: Option<MessageReference>,
    pub timestamp: String,
    pub tts: bool,
    pub webhook_id: Option<WebhookId>,
}
