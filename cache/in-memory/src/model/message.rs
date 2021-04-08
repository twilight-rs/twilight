use serde::Serialize;
use twilight_model::{
    channel::{
        embed::Embed,
        message::{
            Message, MessageActivity, MessageApplication, MessageFlags, MessageReaction,
            MessageReference, MessageType, Sticker,
        },
        Attachment, ChannelMention,
    },
    guild::PartialMember,
    id::{ChannelId, GuildId, MessageId, RoleId, UserId, WebhookId},
};

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct CachedMessage {
    pub id: MessageId,
    pub activity: Option<MessageActivity>,
    pub application: Option<MessageApplication>,
    pub attachments: Vec<Attachment>,
    pub author: UserId,
    pub channel_id: ChannelId,
    pub content: String,
    pub edited_timestamp: Option<String>,
    pub embeds: Vec<Embed>,
    pub flags: Option<MessageFlags>,
    pub guild_id: Option<GuildId>,
    pub kind: MessageType,
    pub member: Option<PartialMember>,
    pub mention_channels: Vec<ChannelMention>,
    pub mention_everyone: bool,
    pub mention_roles: Vec<RoleId>,
    pub mentions: Vec<UserId>,
    pub pinned: bool,
    pub reactions: Vec<MessageReaction>,
    pub reference: Option<MessageReference>,
    pub stickers: Vec<Sticker>,
    pub timestamp: String,
    pub tts: bool,
    pub webhook_id: Option<WebhookId>,
}

impl From<Message> for CachedMessage {
    fn from(msg: Message) -> Self {
        Self {
            id: msg.id,
            activity: msg.activity,
            application: msg.application,
            attachments: msg.attachments,
            author: msg.author.id,
            channel_id: msg.channel_id,
            content: msg.content,
            edited_timestamp: msg.edited_timestamp,
            embeds: msg.embeds,
            flags: msg.flags,
            guild_id: msg.guild_id,
            kind: msg.kind,
            member: msg.member,
            mention_channels: msg.mention_channels,
            mention_everyone: msg.mention_everyone,
            mention_roles: msg.mention_roles,
            mentions: msg.mentions.iter().map(|mention| mention.id).collect(),
            pinned: msg.pinned,
            reactions: msg.reactions,
            reference: msg.reference,
            stickers: msg.stickers,
            timestamp: msg.timestamp,
            tts: msg.tts,
            webhook_id: msg.webhook_id,
        }
    }
}
