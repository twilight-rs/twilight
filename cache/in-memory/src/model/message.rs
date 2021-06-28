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

/// Represents a cached [`Message`].
///
/// [`Message`]: twilight_model::channel::Message
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct CachedMessage {
    /// For rich presence chat embeds, the activity object.
    pub activity: Option<MessageActivity>,
    /// For interaction responses, the ID of the interaction's application.
    pub application: Option<MessageApplication>,
    /// Attached files.
    pub attachments: Vec<Attachment>,
    /// ID of the message author.
    ///
    /// If the author is a webhook, this is its ID.
    pub author: UserId,
    /// ID of the channel the message was sent in.
    pub channel_id: ChannelId,
    /// Content of the message.
    pub content: String,
    /// ISO8601 timestamp of the date the message was last edited.
    pub edited_timestamp: Option<String>,
    /// Embeds attached to the message.
    pub embeds: Vec<Embed>,
    /// Message flags.
    pub flags: Option<MessageFlags>,
    /// ID of the guild the message was sent in, if there is one.
    pub guild_id: Option<GuildId>,
    /// ID of the message.
    pub id: MessageId,
    /// Type of the message.
    pub kind: MessageType,
    /// Member data for the author, if there is any.
    pub member: Option<PartialMember>,
    /// Channels mentioned in the content.
    pub mention_channels: Vec<ChannelMention>,
    /// Whether or not '@everyone' or '@here' is mentioned in the content.
    pub mention_everyone: bool,
    /// Roles mentioned in the content.
    pub mention_roles: Vec<RoleId>,
    /// Users mentioned in the content.
    pub mentions: Vec<UserId>,
    /// Whether or not the message is pinned.
    pub pinned: bool,
    /// Reactions to the message.
    pub reactions: Vec<MessageReaction>,
    /// Message reference.
    pub reference: Option<MessageReference>,
    #[allow(missing_docs)]
    pub stickers: Vec<Sticker>,
    /// ISO8601 timestamp of the date the message was sent.
    pub timestamp: String,
    /// Whether the message is text-to-speech.
    pub tts: bool,
    /// For messages sent by webhooks, the webhook ID.
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
