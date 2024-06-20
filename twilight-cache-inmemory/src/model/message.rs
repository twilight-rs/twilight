//! Cached message-related models.

use serde::Serialize;
use twilight_model::{
    application::interaction::InteractionType,
    channel::{
        message::{
            sticker::MessageSticker, Component, Embed, Message, MessageActivity,
            MessageApplication, MessageFlags, MessageInteraction, MessageReference, MessageType,
            Reaction, RoleSubscriptionData,
        },
        Attachment, ChannelMention,
    },
    gateway::payload::incoming::MessageUpdate,
    guild::PartialMember,
    id::{
        marker::{
            ApplicationMarker, ChannelMarker, GuildMarker, InteractionMarker, MessageMarker,
            RoleMarker, UserMarker, WebhookMarker,
        },
        Id,
    },
    util::Timestamp,
};

use crate::CacheableMessage;

/// Information about the message interaction.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct CachedMessageInteraction {
    id: Id<InteractionMarker>,
    #[serde(rename = "type")]
    kind: InteractionType,
    name: String,
    user_id: Id<UserMarker>,
}

impl CachedMessageInteraction {
    /// ID of the interaction.
    pub const fn id(&self) -> Id<InteractionMarker> {
        self.id
    }

    /// Type of the interaction.
    pub const fn kind(&self) -> InteractionType {
        self.kind
    }

    /// Name of the interaction used.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// ID of the user who invoked the interaction.
    pub const fn user_id(&self) -> Id<UserMarker> {
        self.user_id
    }

    /// Construct a cached message interaction from its [`twilight_model`] form.
    #[allow(clippy::missing_const_for_fn)]
    pub(crate) fn from_model(message_interaction: MessageInteraction) -> Self {
        // Reasons for dropping fields:
        //
        // - `member`: we have the user's ID from the `user_id` field
        let MessageInteraction {
            id,
            kind,
            member: _,
            name,
            user,
        } = message_interaction;

        Self {
            id,
            kind,
            name,
            user_id: user.id,
        }
    }
}

impl PartialEq<MessageInteraction> for CachedMessageInteraction {
    fn eq(&self, other: &MessageInteraction) -> bool {
        self.id == other.id
            && self.kind == other.kind
            && self.name == other.name
            && self.user_id == other.user.id
    }
}

/// Represents a cached [`Message`].
///
/// [`Message`]: twilight_model::channel::Message
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct CachedMessage {
    activity: Option<MessageActivity>,
    application: Option<MessageApplication>,
    application_id: Option<Id<ApplicationMarker>>,
    pub(crate) attachments: Vec<Attachment>,
    author: Id<UserMarker>,
    channel_id: Id<ChannelMarker>,
    components: Vec<Component>,
    pub(crate) content: String,
    pub(crate) edited_timestamp: Option<Timestamp>,
    pub(crate) embeds: Vec<Embed>,
    flags: Option<MessageFlags>,
    guild_id: Option<Id<GuildMarker>>,
    id: Id<MessageMarker>,
    interaction: Option<CachedMessageInteraction>,
    kind: MessageType,
    member: Option<PartialMember>,
    mention_channels: Vec<ChannelMention>,
    pub(crate) mention_everyone: bool,
    pub(crate) mention_roles: Vec<Id<RoleMarker>>,
    pub(crate) mentions: Vec<Id<UserMarker>>,
    pub(crate) pinned: bool,
    pub(crate) reactions: Vec<Reaction>,
    reference: Option<MessageReference>,
    role_subscription_data: Option<RoleSubscriptionData>,
    sticker_items: Vec<MessageSticker>,
    thread_id: Option<Id<ChannelMarker>>,
    pub(crate) timestamp: Timestamp,
    pub(crate) tts: bool,
    webhook_id: Option<Id<WebhookMarker>>,
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

    /// Associated application's ID.
    ///
    /// Sent if the message is a response to an Interaction.
    pub const fn application_id(&self) -> Option<Id<ApplicationMarker>> {
        self.application_id
    }

    /// List of attached files.
    ///
    /// Refer to the documentation for [`Message::attachments`] for caveats with
    /// receiving the attachments of messages.
    ///
    /// [`Message::attachments`]: twilight_model::channel::Message::attachments
    pub fn attachments(&self) -> &[Attachment] {
        &self.attachments
    }

    /// ID of the message author.
    ///
    /// If the author is a webhook, this is its ID.
    pub const fn author(&self) -> Id<UserMarker> {
        self.author
    }

    /// ID of the channel the message was sent in.
    pub const fn channel_id(&self) -> Id<ChannelMarker> {
        self.channel_id
    }

    /// List of provided components, such as buttons.
    ///
    /// Refer to the documentation for [`Message::components`] for caveats with
    /// receiving the components of messages.
    ///
    /// [`Message::components`]: twilight_model::channel::Message::components
    pub fn components(&self) -> &[Component] {
        &self.components
    }

    /// Content of a message.
    ///
    /// Refer to the documentation for [`Message::content`] for caveats with
    /// receiving the content of messages.
    ///
    /// [`Message::content`]: twilight_model::channel::Message::content
    pub fn content(&self) -> &str {
        &self.content
    }

    /// [`Timestamp`] of the date the message was last edited.
    pub const fn edited_timestamp(&self) -> Option<Timestamp> {
        self.edited_timestamp
    }

    /// List of embeds.
    ///
    /// Refer to the documentation for [`Message::embeds`] for caveats with
    /// receiving the embeds of messages.
    ///
    /// [`Message::embeds`]: twilight_model::channel::Message::embeds
    pub fn embeds(&self) -> &[Embed] {
        &self.embeds
    }

    /// Message flags.
    pub const fn flags(&self) -> Option<MessageFlags> {
        self.flags
    }

    /// ID of the guild the message was sent in, if there is one.
    pub const fn guild_id(&self) -> Option<Id<GuildMarker>> {
        self.guild_id
    }

    /// ID of the message.
    pub const fn id(&self) -> Id<MessageMarker> {
        self.id
    }

    /// Information about the message interaction.
    pub const fn interaction(&self) -> Option<&CachedMessageInteraction> {
        self.interaction.as_ref()
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
    pub fn mention_roles(&self) -> &[Id<RoleMarker>] {
        &self.mention_roles
    }

    /// Users mentioned in the content.
    pub fn mentions(&self) -> &[Id<UserMarker>] {
        &self.mentions
    }

    /// Whether or not the message is pinned.
    pub const fn pinned(&self) -> bool {
        self.pinned
    }

    /// Reactions to the message.
    pub fn reactions(&self) -> &[Reaction] {
        &self.reactions
    }

    /// Message reference.
    pub const fn reference(&self) -> Option<&MessageReference> {
        self.reference.as_ref()
    }

    /// Information about the role subscription purchase or renewal that
    /// prompted this message.
    pub const fn role_subscription_data(&self) -> Option<&RoleSubscriptionData> {
        self.role_subscription_data.as_ref()
    }

    /// Stickers within the message.
    pub fn sticker_items(&self) -> &[MessageSticker] {
        &self.sticker_items
    }

    /// ID of the thread the message was sent in.
    pub const fn thread_id(&self) -> Option<Id<ChannelMarker>> {
        self.thread_id
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
    pub const fn webhook_id(&self) -> Option<Id<WebhookMarker>> {
        self.webhook_id
    }
}

impl From<Message> for CachedMessage {
    #[allow(deprecated)]
    fn from(message: Message) -> Self {
        let Message {
            activity,
            application,
            application_id,
            attachments,
            author,
            channel_id,
            components,
            content,
            edited_timestamp,
            embeds,
            flags,
            guild_id,
            id,
            interaction,
            interaction_metadata: _,
            kind,
            member,
            mention_channels,
            mention_everyone,
            mention_roles,
            mentions,
            pinned,
            reactions,
            reference,
            referenced_message: _,
            role_subscription_data,
            sticker_items,
            timestamp,
            thread,
            tts,
            webhook_id,
        } = message;

        Self {
            id,
            activity,
            application,
            application_id,
            attachments,
            author: author.id,
            channel_id,
            components,
            content,
            edited_timestamp,
            embeds,
            flags,
            guild_id,
            interaction: interaction.map(CachedMessageInteraction::from_model),
            kind,
            member,
            mention_channels,
            mention_everyone,
            mention_roles,
            mentions: mentions.into_iter().map(|mention| mention.id).collect(),
            pinned,
            reactions,
            reference,
            role_subscription_data,
            sticker_items,
            thread_id: thread.map(|thread| thread.id),
            timestamp,
            tts,
            webhook_id,
        }
    }
}

impl PartialEq<Message> for CachedMessage {
    #[allow(deprecated)]
    fn eq(&self, other: &Message) -> bool {
        self.id == other.id
            && self.activity == other.activity
            && self.application == other.application
            && self.application_id == other.application_id
            && self.attachments == other.attachments
            && self.author == other.author.id
            && self.channel_id == other.channel_id
            && self.components == other.components
            && self.content == other.content
            && self.edited_timestamp == other.edited_timestamp
            && self.embeds == other.embeds
            && self.flags == other.flags
            && self.guild_id == other.guild_id
            && self
                .interaction
                .as_ref()
                .map_or(other.interaction.is_none(), |interaction| {
                    other
                        .interaction
                        .as_ref()
                        .map_or(false, |other_interaction| interaction == other_interaction)
                })
            && self.kind == other.kind
            && self.member == other.member
            && self.mention_channels == other.mention_channels
            && self.mention_everyone == other.mention_everyone
            && self.mention_roles == other.mention_roles
            && self.mentions.len() == other.mentions.len()
            && self
                .mentions
                .iter()
                .zip(other.mentions.iter())
                .all(|(user_id, mention)| user_id == &mention.id)
            && self.pinned == other.pinned
            && self.reactions == other.reactions
            && self.reference == other.reference
            && self.role_subscription_data == other.role_subscription_data
            && self.sticker_items == other.sticker_items
            && self.thread_id == other.thread.as_ref().map(|thread| thread.id)
            && self.timestamp == other.timestamp
            && self.tts == other.tts
            && self.webhook_id == other.webhook_id
    }
}

impl CacheableMessage for CachedMessage {
    fn update_with_message_update(&mut self, message_update: &MessageUpdate) {
        if let Some(attachments) = &message_update.attachments {
            self.attachments.clone_from(attachments);
        }

        if let Some(content) = &message_update.content {
            self.content.clone_from(content);
        }

        if let Some(edited_timestamp) = message_update.edited_timestamp {
            self.edited_timestamp.replace(edited_timestamp);
        }

        if let Some(embeds) = &message_update.embeds {
            self.embeds.clone_from(embeds);
        }

        if let Some(mention_everyone) = message_update.mention_everyone {
            self.mention_everyone = mention_everyone;
        }

        if let Some(mention_roles) = &message_update.mention_roles {
            self.mention_roles.clone_from(mention_roles);
        }

        if let Some(mentions) = &message_update.mentions {
            self.mentions = mentions.iter().map(|x| x.id).collect::<Vec<_>>();
        }

        if let Some(pinned) = message_update.pinned {
            self.pinned = pinned;
        }

        if let Some(timestamp) = message_update.timestamp {
            self.timestamp = timestamp;
        }

        if let Some(tts) = message_update.tts {
            self.tts = tts;
        }
    }

    fn reactions(&self) -> &[Reaction] {
        &self.reactions
    }

    fn reactions_mut(&mut self) -> &mut [Reaction] {
        &mut self.reactions
    }

    fn retain_reactions(&mut self, f: impl FnMut(&Reaction) -> bool) {
        self.reactions.retain(f);
    }

    fn clear_reactions(&mut self) {
        self.reactions.clear();
    }

    fn add_reaction(&mut self, reaction: Reaction) {
        self.reactions.push(reaction);
    }

    fn remove_reaction(&mut self, idx: usize) {
        self.reactions.remove(idx);
    }
}

#[cfg(test)]
mod tests {
    use super::{CachedMessage, CachedMessageInteraction};
    use serde::Serialize;
    use static_assertions::{assert_fields, assert_impl_all};
    use std::fmt::Debug;
    use twilight_model::channel::message::Message;

    assert_fields!(
        CachedMessage: activity,
        application,
        application_id,
        attachments,
        author,
        channel_id,
        components,
        content,
        edited_timestamp,
        embeds,
        flags,
        guild_id,
        id,
        interaction,
        kind,
        member,
        mention_channels,
        mention_everyone,
        mention_roles,
        mentions,
        pinned,
        reactions,
        reference,
        sticker_items,
        thread_id,
        timestamp,
        tts,
        webhook_id
    );
    assert_impl_all!(
        CachedMessage: Clone,
        Debug,
        From<Message>,
        PartialEq,
        Send,
        Serialize,
        Sync,
    );
    assert_fields!(CachedMessageInteraction: id, kind, name, user_id);
    assert_impl_all!(
        CachedMessageInteraction: Clone,
        Debug,
        Eq,
        PartialEq,
        Send,
        Serialize,
        Sync,
    );
}
