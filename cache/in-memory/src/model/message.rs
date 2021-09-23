use serde::Serialize;
use twilight_model::{
    channel::{
        embed::Embed,
        message::{
            sticker::MessageSticker, Message, MessageActivity, MessageApplication, MessageFlags,
            MessageReaction, MessageReference, MessageType,
        },
        Attachment, ChannelMention,
    },
    datetime::Timestamp,
    guild::PartialMember,
    id::{ChannelId, GuildId, MessageId, RoleId, UserId, WebhookId},
};

/// Represents a cached [`Message`].
///
/// [`Message`]: twilight_model::channel::Message
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct CachedMessage {
    activity: Option<MessageActivity>,
    application: Option<MessageApplication>,
    pub(crate) attachments: Vec<Attachment>,
    author: UserId,
    channel_id: ChannelId,
    pub(crate) content: String,
    pub(crate) edited_timestamp: Option<Timestamp>,
    pub(crate) embeds: Vec<Embed>,
    flags: Option<MessageFlags>,
    guild_id: Option<GuildId>,
    id: MessageId,
    kind: MessageType,
    member: Option<PartialMember>,
    mention_channels: Vec<ChannelMention>,
    pub(crate) mention_everyone: bool,
    pub(crate) mention_roles: Vec<RoleId>,
    pub(crate) mentions: Vec<UserId>,
    pub(crate) pinned: bool,
    pub(crate) reactions: Vec<MessageReaction>,
    reference: Option<MessageReference>,
    sticker_items: Vec<MessageSticker>,
    pub(crate) timestamp: Timestamp,
    pub(crate) tts: bool,
    webhook_id: Option<WebhookId>,
}

impl CachedMessage {
    /// For rich presence chat embeds, the activity object.
    pub const fn activity(&self) -> Option<&MessageActivity> {
        self.activity.as_ref()
    }

    /// For interaction responses, the ID of the interaction's application.
    pub const fn application(&self) -> Option<&MessageApplication> {
        self.application.as_ref()
    }

    /// Attached files.
    pub fn attachments(&self) -> &[Attachment] {
        &self.attachments
    }

    /// ID of the message author.
    ///
    /// If the author is a webhook, this is its ID.
    pub const fn author(&self) -> UserId {
        self.author
    }

    /// ID of the channel the message was sent in.
    pub const fn channel_id(&self) -> ChannelId {
        self.channel_id
    }

    /// Content of the message.
    pub fn content(&self) -> &str {
        &self.content
    }

    /// [`Timestamp`] of the date the message was last edited.
    pub const fn edited_timestamp(&self) -> Option<Timestamp> {
        self.edited_timestamp
    }

    /// Embeds attached to the message.
    pub fn embeds(&self) -> &[Embed] {
        &self.embeds
    }

    /// Message flags.
    pub const fn flags(&self) -> Option<MessageFlags> {
        self.flags
    }

    /// ID of the guild the message was sent in, if there is one.
    pub const fn guild_id(&self) -> Option<GuildId> {
        self.guild_id
    }

    /// ID of the message.
    pub const fn id(&self) -> MessageId {
        self.id
    }

    /// Type of the message.
    pub const fn kind(&self) -> MessageType {
        self.kind
    }

    /// Member data for the author, if there is any.
    pub const fn member(&self) -> Option<&PartialMember> {
        self.member.as_ref()
    }

    /// Channels mentioned in the content.
    pub fn mention_channels(&self) -> &[ChannelMention] {
        &self.mention_channels
    }

    /// Whether or not '@everyone' or '@here' is mentioned in the content.
    pub const fn mention_everyone(&self) -> bool {
        self.mention_everyone
    }

    /// Roles mentioned in the content.
    pub fn mention_roles(&self) -> &[RoleId] {
        &self.mention_roles
    }

    /// Users mentioned in the content.
    pub fn mentions(&self) -> &[UserId] {
        &self.mentions
    }

    /// Whether or not the message is pinned.
    pub const fn pinned(&self) -> bool {
        self.pinned
    }

    /// Reactions to the message.
    pub fn reactions(&self) -> &[MessageReaction] {
        &self.reactions
    }

    /// Message reference.
    pub const fn reference(&self) -> Option<&MessageReference> {
        self.reference.as_ref()
    }

    /// Stickers within the message.
    pub fn sticker_items(&self) -> &[MessageSticker] {
        &self.sticker_items
    }

    /// [`Timestamp`] of the date the message was sent.
    pub const fn timestamp(&self) -> Timestamp {
        self.timestamp
    }

    /// Whether the message is text-to-speech.
    pub const fn tts(&self) -> bool {
        self.tts
    }

    /// For messages sent by webhooks, the webhook ID.
    pub const fn webhook_id(&self) -> Option<WebhookId> {
        self.webhook_id
    }
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
            sticker_items: msg.sticker_items,
            timestamp: msg.timestamp,
            tts: msg.tts,
            webhook_id: msg.webhook_id,
        }
    }
}
