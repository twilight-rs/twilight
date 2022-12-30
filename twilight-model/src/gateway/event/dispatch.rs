#![allow(clippy::wildcard_imports)]

use super::{Event, EventConversionError};
use crate::{
    gateway::payload::incoming::*,
    id::{marker::GuildMarker, Id},
};
use serde::{
    de::{value, DeserializeSeed, Deserializer, IgnoredAny, IntoDeserializer},
    Deserialize, Serialize,
};
use std::{
    fmt::{Display, Formatter, Result as FmtResult},
    str::FromStr,
};

/// Dispatch event payload.
///
/// Corresponds to an event's `d` field if it is a dispatch event.
#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(untagged)]
pub enum DispatchEvent {
    /// Message was blocked by AutoMod according to a rule.
    AutoModerationActionExecution(AutoModerationActionExecution),
    /// Sent when an auto moderation rule is created.
    AutoModerationRuleCreate(Box<AutoModerationRuleCreate>),
    /// Sent when an auto moderation rule is deleted.
    AutoModerationRuleDelete(Box<AutoModerationRuleDelete>),
    /// Sent when an auto moderation rule is updated.
    AutoModerationRuleUpdate(Box<AutoModerationRuleUpdate>),
    /// A user was banned from a guild.
    BanAdd(BanAdd),
    /// A user's ban from a guild was removed.
    BanRemove(BanRemove),
    /// A channel was created.
    ChannelCreate(Box<ChannelCreate>),
    /// A channel was deleted.
    ChannelDelete(Box<ChannelDelete>),
    /// A channel's pins were updated.
    ChannelPinsUpdate(ChannelPinsUpdate),
    /// A channel was updated.
    ChannelUpdate(Box<ChannelUpdate>),
    /// A command's permissions were updated.
    CommandPermissionsUpdate(CommandPermissionsUpdate),
    /// Undocumented event, should be ignored.
    GiftCodeUpdate,
    /// A audit log entry was created.
    GuildAuditLogEntryCreate(Box<GuildAuditLogEntryCreate>),
    /// A guild was created.
    GuildCreate(Box<GuildCreate>),
    /// A guild was deleted or the current user was removed from a guild.
    GuildDelete(GuildDelete),
    /// A guild's emojis were updated.
    GuildEmojisUpdate(GuildEmojisUpdate),
    /// A guild's integrations were updated.
    GuildIntegrationsUpdate(GuildIntegrationsUpdate),
    /// A guild scheduled event was created.
    GuildScheduledEventCreate(Box<GuildScheduledEventCreate>),
    /// A guild scheduled event was deleted.
    GuildScheduledEventDelete(Box<GuildScheduledEventDelete>),
    /// A guild scheduled event was updated.
    GuildScheduledEventUpdate(Box<GuildScheduledEventUpdate>),
    /// A user was added to a guild scheduled event.
    GuildScheduledEventUserAdd(GuildScheduledEventUserAdd),
    /// A user was removed from a guild scheduled event.
    GuildScheduledEventUserRemove(GuildScheduledEventUserRemove),
    /// A guild's stickers were updated.
    GuildStickersUpdate(GuildStickersUpdate),
    /// A guild was updated.
    GuildUpdate(Box<GuildUpdate>),
    /// A guild integration was created.
    IntegrationCreate(Box<IntegrationCreate>),
    /// A guild integration was updated.
    IntegrationDelete(IntegrationDelete),
    /// A guild integration was deleted.
    IntegrationUpdate(Box<IntegrationUpdate>),
    /// An interaction was invoked by a user.
    InteractionCreate(Box<InteractionCreate>),
    /// A invite was made.
    InviteCreate(Box<InviteCreate>),
    /// A invite was deleted.
    InviteDelete(InviteDelete),
    /// A user was added to a guild.
    MemberAdd(Box<MemberAdd>),
    /// A user was removed from a guild.
    MemberRemove(MemberRemove),
    /// A user's member object in a guild was updated.
    MemberUpdate(Box<MemberUpdate>),
    /// A chunk of members were received from the gateway.
    MemberChunk(MemberChunk),
    /// A message was created in a channel.
    MessageCreate(Box<MessageCreate>),
    /// A message was deleted in a channel.
    MessageDelete(MessageDelete),
    /// Multiple messages were deleted in a channel.
    MessageDeleteBulk(MessageDeleteBulk),
    /// A message was updated in a channel.
    MessageUpdate(Box<MessageUpdate>),
    /// A user's active presence (such as game or online status) was updated.
    PresenceUpdate(Box<PresenceUpdate>),
    /// Multiple presences outside of a guild were updated.
    ///
    /// For bots this is always empty and useless.
    PresencesReplace,
    /// A reaction was added to a message.
    ReactionAdd(Box<ReactionAdd>),
    /// A reaction was removed from a message.
    ReactionRemove(Box<ReactionRemove>),
    /// All reactions were removed from a message.
    ReactionRemoveAll(ReactionRemoveAll),
    /// All instances of a given emoji from the reactions of a message were
    /// removed.
    ReactionRemoveEmoji(ReactionRemoveEmoji),
    /// A shard is now "ready" and fully connected.
    Ready(Box<Ready>),
    /// A shard has successfully resumed.
    Resumed,
    /// A role was created in a guild.
    RoleCreate(RoleCreate),
    /// A role was deleted in a guild.
    RoleDelete(RoleDelete),
    /// A role was updated in a guild.
    RoleUpdate(RoleUpdate),
    /// A stage instance was created in a stage channel.
    StageInstanceCreate(StageInstanceCreate),
    /// A stage instance was deleted in a stage channel.
    StageInstanceDelete(StageInstanceDelete),
    /// A stage instance was updated in a stage channel.
    StageInstanceUpdate(StageInstanceUpdate),
    /// A thread has been created, relevant to the current user,
    /// or the current user has been added to a thread.
    ThreadCreate(Box<ThreadCreate>),
    /// A thread, relevant to the current user, has been deleted.
    ThreadDelete(ThreadDelete),
    /// The current user has gained access to a thread.
    ThreadListSync(ThreadListSync),
    /// The thread member object for the current user has been updated.
    ThreadMemberUpdate(Box<ThreadMemberUpdate>),
    /// A user has been added to or removed from a thread.
    ThreadMembersUpdate(ThreadMembersUpdate),
    /// A thread has been updated.
    ThreadUpdate(Box<ThreadUpdate>),
    /// A user started typing in a channel.
    TypingStart(Box<TypingStart>),
    /// A guild is now unavailable.
    UnavailableGuild(UnavailableGuild),
    /// The current user was updated.
    UserUpdate(UserUpdate),
    /// A voice server update was sent.
    VoiceServerUpdate(VoiceServerUpdate),
    /// A voice state in a voice channel was updated.
    VoiceStateUpdate(Box<VoiceStateUpdate>),
    /// A webhook was updated.
    WebhooksUpdate(WebhooksUpdate),
}

impl DispatchEvent {
    /// ID of the guild from which the event originated, if known.
    ///
    /// Some events, such as [`MessageDelete`], will never include a guild ID,
    /// others, such as [`BanAdd`], will always include one and some events,
    /// such as [`ChannelCreate`] might include one.
    pub const fn guild_id(&self) -> Option<Id<GuildMarker>> {
        match self {
            Self::AutoModerationActionExecution(e) => Some(e.guild_id),
            Self::AutoModerationRuleCreate(e) => Some(e.0.guild_id),
            Self::AutoModerationRuleDelete(e) => Some(e.0.guild_id),
            Self::AutoModerationRuleUpdate(e) => Some(e.0.guild_id),
            Self::BanAdd(e) => Some(e.guild_id),
            Self::BanRemove(e) => Some(e.guild_id),
            Self::ChannelCreate(e) => e.0.guild_id,
            Self::ChannelDelete(e) => e.0.guild_id,
            Self::ChannelUpdate(e) => e.0.guild_id,
            Self::CommandPermissionsUpdate(e) => Some(e.0.guild_id),
            Self::GuildAuditLogEntryCreate(e) => e.0.guild_id,
            Self::GuildCreate(e) => Some(e.0.id),
            Self::GuildDelete(e) => Some(e.id),
            Self::GuildEmojisUpdate(e) => Some(e.guild_id),
            Self::GuildIntegrationsUpdate(e) => Some(e.guild_id),
            Self::GuildScheduledEventCreate(e) => Some(e.0.guild_id),
            Self::GuildScheduledEventDelete(e) => Some(e.0.guild_id),
            Self::GuildScheduledEventUpdate(e) => Some(e.0.guild_id),
            Self::GuildScheduledEventUserAdd(e) => Some(e.guild_id),
            Self::GuildScheduledEventUserRemove(e) => Some(e.guild_id),
            Self::GuildStickersUpdate(e) => Some(e.guild_id),
            Self::GuildUpdate(e) => Some(e.0.id),
            Self::IntegrationCreate(e) => e.0.guild_id,
            Self::IntegrationDelete(e) => Some(e.guild_id),
            Self::IntegrationUpdate(e) => e.0.guild_id,
            Self::InteractionCreate(e) => e.0.guild_id,
            Self::InviteCreate(e) => Some(e.guild_id),
            Self::InviteDelete(e) => Some(e.guild_id),
            Self::MemberAdd(e) => Some(e.guild_id),
            Self::MemberChunk(e) => Some(e.guild_id),
            Self::MemberRemove(e) => Some(e.guild_id),
            Self::MemberUpdate(e) => Some(e.guild_id),
            Self::MessageCreate(e) => e.0.guild_id,
            Self::PresenceUpdate(e) => Some(e.0.guild_id),
            Self::ReactionAdd(e) => e.0.guild_id,
            Self::ReactionRemove(e) => e.0.guild_id,
            Self::ReactionRemoveAll(e) => e.guild_id,
            Self::ReactionRemoveEmoji(e) => Some(e.guild_id),
            Self::RoleCreate(e) => Some(e.guild_id),
            Self::RoleDelete(e) => Some(e.guild_id),
            Self::RoleUpdate(e) => Some(e.guild_id),
            Self::StageInstanceCreate(e) => Some(e.0.guild_id),
            Self::StageInstanceDelete(e) => Some(e.0.guild_id),
            Self::StageInstanceUpdate(e) => Some(e.0.guild_id),
            Self::ThreadCreate(e) => e.0.guild_id,
            Self::ThreadDelete(e) => Some(e.guild_id),
            Self::ThreadListSync(e) => Some(e.guild_id),
            Self::ThreadMembersUpdate(e) => Some(e.guild_id),
            Self::ThreadUpdate(e) => e.0.guild_id,
            Self::TypingStart(e) => e.guild_id,
            Self::UnavailableGuild(e) => Some(e.id),
            Self::VoiceServerUpdate(e) => Some(e.guild_id),
            Self::VoiceStateUpdate(e) => e.0.guild_id,
            Self::WebhooksUpdate(e) => Some(e.guild_id),
            Self::ChannelPinsUpdate(_)
            | Self::GiftCodeUpdate
            | Self::MessageDelete(_)
            | Self::MessageDeleteBulk(_)
            | Self::MessageUpdate(_)
            | Self::PresencesReplace
            | Self::Ready(_)
            | Self::Resumed
            | Self::ThreadMemberUpdate(_)
            | Self::UserUpdate(_) => None,
        }
    }

    /// Type of dispatch event.
    pub const fn kind(&self) -> DispatchEventType {
        match self {
            Self::AutoModerationActionExecution(_) => {
                DispatchEventType::AutoModerationActionExecution
            }
            Self::AutoModerationRuleCreate(_) => DispatchEventType::AutoModerationRuleCreate,
            Self::AutoModerationRuleDelete(_) => DispatchEventType::AutoModerationRuleDelete,
            Self::AutoModerationRuleUpdate(_) => DispatchEventType::AutoModerationRuleUpdate,
            Self::BanAdd(_) => DispatchEventType::BanAdd,
            Self::BanRemove(_) => DispatchEventType::BanRemove,
            Self::ChannelCreate(_) => DispatchEventType::ChannelCreate,
            Self::ChannelDelete(_) => DispatchEventType::ChannelDelete,
            Self::ChannelPinsUpdate(_) => DispatchEventType::ChannelPinsUpdate,
            Self::ChannelUpdate(_) => DispatchEventType::ChannelUpdate,
            Self::CommandPermissionsUpdate(_) => DispatchEventType::CommandPermissionsUpdate,
            Self::GiftCodeUpdate => DispatchEventType::GiftCodeUpdate,
            Self::GuildAuditLogEntryCreate(_) => DispatchEventType::GuildAuditLogEntryCreate,
            Self::GuildCreate(_) => DispatchEventType::GuildCreate,
            Self::GuildDelete(_) => DispatchEventType::GuildDelete,
            Self::GuildEmojisUpdate(_) => DispatchEventType::GuildEmojisUpdate,
            Self::GuildIntegrationsUpdate(_) => DispatchEventType::GuildIntegrationsUpdate,
            Self::GuildScheduledEventCreate(_) => DispatchEventType::GuildScheduledEventCreate,
            Self::GuildScheduledEventDelete(_) => DispatchEventType::GuildScheduledEventDelete,
            Self::GuildScheduledEventUpdate(_) => DispatchEventType::GuildScheduledEventUpdate,
            Self::GuildScheduledEventUserAdd(_) => DispatchEventType::GuildScheduledEventUserAdd,
            Self::GuildScheduledEventUserRemove(_) => {
                DispatchEventType::GuildScheduledEventUserRemove
            }
            Self::GuildStickersUpdate(_) => DispatchEventType::GuildStickersUpdate,
            Self::GuildUpdate(_) => DispatchEventType::GuildUpdate,
            Self::IntegrationCreate(_) => DispatchEventType::IntegrationCreate,
            Self::IntegrationDelete(_) => DispatchEventType::IntegrationDelete,
            Self::IntegrationUpdate(_) => DispatchEventType::IntegrationUpdate,
            Self::InteractionCreate(_) => DispatchEventType::InteractionCreate,
            Self::InviteCreate(_) => DispatchEventType::InviteCreate,
            Self::InviteDelete(_) => DispatchEventType::InviteDelete,
            Self::MemberAdd(_) => DispatchEventType::MemberAdd,
            Self::MemberRemove(_) => DispatchEventType::MemberRemove,
            Self::MemberUpdate(_) => DispatchEventType::MemberUpdate,
            Self::MemberChunk(_) => DispatchEventType::MemberChunk,
            Self::MessageCreate(_) => DispatchEventType::MessageCreate,
            Self::MessageDelete(_) => DispatchEventType::MessageDelete,
            Self::MessageDeleteBulk(_) => DispatchEventType::MessageDeleteBulk,
            Self::MessageUpdate(_) => DispatchEventType::MessageUpdate,
            Self::PresenceUpdate(_) => DispatchEventType::PresenceUpdate,
            Self::PresencesReplace => DispatchEventType::PresencesReplace,
            Self::ReactionAdd(_) => DispatchEventType::ReactionAdd,
            Self::ReactionRemove(_) => DispatchEventType::ReactionRemove,
            Self::ReactionRemoveAll(_) => DispatchEventType::ReactionRemoveAll,
            Self::ReactionRemoveEmoji(_) => DispatchEventType::ReactionRemoveEmoji,
            Self::Ready(_) => DispatchEventType::Ready,
            Self::Resumed => DispatchEventType::Resumed,
            Self::RoleCreate(_) => DispatchEventType::RoleCreate,
            Self::RoleDelete(_) => DispatchEventType::RoleDelete,
            Self::RoleUpdate(_) => DispatchEventType::RoleUpdate,
            Self::StageInstanceCreate(_) => DispatchEventType::StageInstanceCreate,
            Self::StageInstanceDelete(_) => DispatchEventType::StageInstanceDelete,
            Self::StageInstanceUpdate(_) => DispatchEventType::StageInstanceUpdate,
            Self::ThreadCreate(_) => DispatchEventType::ThreadCreate,
            Self::ThreadDelete(_) => DispatchEventType::ThreadDelete,
            Self::ThreadListSync(_) => DispatchEventType::ThreadListSync,
            Self::ThreadMemberUpdate(_) => DispatchEventType::ThreadMemberUpdate,
            Self::ThreadMembersUpdate(_) => DispatchEventType::ThreadMembersUpdate,
            Self::ThreadUpdate(_) => DispatchEventType::ThreadUpdate,
            Self::TypingStart(_) => DispatchEventType::TypingStart,
            Self::UnavailableGuild(_) => DispatchEventType::UnavailableGuild,
            Self::UserUpdate(_) => DispatchEventType::UserUpdate,
            Self::VoiceServerUpdate(_) => DispatchEventType::VoiceServerUpdate,
            Self::VoiceStateUpdate(_) => DispatchEventType::VoiceStateUpdate,
            Self::WebhooksUpdate(_) => DispatchEventType::WebhooksUpdate,
        }
    }
}

impl TryFrom<Event> for DispatchEvent {
    type Error = EventConversionError;

    fn try_from(event: Event) -> Result<Self, Self::Error> {
        Ok(match event {
            Event::AutoModerationActionExecution(v) => Self::AutoModerationActionExecution(v),
            Event::AutoModerationRuleCreate(v) => Self::AutoModerationRuleCreate(v),
            Event::AutoModerationRuleDelete(v) => Self::AutoModerationRuleDelete(v),
            Event::AutoModerationRuleUpdate(v) => Self::AutoModerationRuleUpdate(v),
            Event::BanAdd(v) => Self::BanAdd(v),
            Event::BanRemove(v) => Self::BanRemove(v),
            Event::ChannelCreate(v) => Self::ChannelCreate(v),
            Event::ChannelDelete(v) => Self::ChannelDelete(v),
            Event::ChannelPinsUpdate(v) => Self::ChannelPinsUpdate(v),
            Event::ChannelUpdate(v) => Self::ChannelUpdate(v),
            Event::CommandPermissionsUpdate(v) => Self::CommandPermissionsUpdate(v),
            Event::GiftCodeUpdate => Self::GiftCodeUpdate,
            Event::GuildAuditLogEntryCreate(v) => Self::GuildAuditLogEntryCreate(v),
            Event::GuildCreate(v) => Self::GuildCreate(v),
            Event::GuildDelete(v) => Self::GuildDelete(v),
            Event::GuildEmojisUpdate(v) => Self::GuildEmojisUpdate(v),
            Event::GuildIntegrationsUpdate(v) => Self::GuildIntegrationsUpdate(v),
            Event::GuildScheduledEventCreate(v) => Self::GuildScheduledEventCreate(v),
            Event::GuildScheduledEventDelete(v) => Self::GuildScheduledEventDelete(v),
            Event::GuildScheduledEventUpdate(v) => Self::GuildScheduledEventUpdate(v),
            Event::GuildScheduledEventUserAdd(v) => Self::GuildScheduledEventUserAdd(v),
            Event::GuildScheduledEventUserRemove(v) => Self::GuildScheduledEventUserRemove(v),
            Event::GuildUpdate(v) => Self::GuildUpdate(v),
            Event::IntegrationCreate(v) => Self::IntegrationCreate(v),
            Event::IntegrationDelete(v) => Self::IntegrationDelete(v),
            Event::IntegrationUpdate(v) => Self::IntegrationUpdate(v),
            Event::InteractionCreate(v) => Self::InteractionCreate(v),
            Event::InviteCreate(v) => Self::InviteCreate(v),
            Event::InviteDelete(v) => Self::InviteDelete(v),
            Event::MemberAdd(v) => Self::MemberAdd(v),
            Event::MemberRemove(v) => Self::MemberRemove(v),
            Event::MemberUpdate(v) => Self::MemberUpdate(v),
            Event::MemberChunk(v) => Self::MemberChunk(v),
            Event::MessageCreate(v) => Self::MessageCreate(v),
            Event::MessageDelete(v) => Self::MessageDelete(v),
            Event::MessageDeleteBulk(v) => Self::MessageDeleteBulk(v),
            Event::MessageUpdate(v) => Self::MessageUpdate(v),
            Event::PresenceUpdate(v) => Self::PresenceUpdate(v),
            Event::PresencesReplace => Self::PresencesReplace,
            Event::ReactionAdd(v) => Self::ReactionAdd(v),
            Event::ReactionRemove(v) => Self::ReactionRemove(v),
            Event::ReactionRemoveAll(v) => Self::ReactionRemoveAll(v),
            Event::ReactionRemoveEmoji(v) => Self::ReactionRemoveEmoji(v),
            Event::Ready(v) => Self::Ready(v),
            Event::Resumed => Self::Resumed,
            Event::RoleCreate(v) => Self::RoleCreate(v),
            Event::RoleDelete(v) => Self::RoleDelete(v),
            Event::RoleUpdate(v) => Self::RoleUpdate(v),
            Event::StageInstanceCreate(v) => Self::StageInstanceCreate(v),
            Event::StageInstanceDelete(v) => Self::StageInstanceDelete(v),
            Event::StageInstanceUpdate(v) => Self::StageInstanceUpdate(v),
            Event::ThreadCreate(v) => Self::ThreadCreate(v),
            Event::ThreadDelete(v) => Self::ThreadDelete(v),
            Event::ThreadListSync(v) => Self::ThreadListSync(v),
            Event::ThreadMemberUpdate(v) => Self::ThreadMemberUpdate(v),
            Event::ThreadMembersUpdate(v) => Self::ThreadMembersUpdate(v),
            Event::ThreadUpdate(v) => Self::ThreadUpdate(v),
            Event::TypingStart(v) => Self::TypingStart(v),
            Event::UnavailableGuild(v) => Self::UnavailableGuild(v),
            Event::UserUpdate(v) => Self::UserUpdate(v),
            Event::VoiceServerUpdate(v) => Self::VoiceServerUpdate(v),
            Event::VoiceStateUpdate(v) => Self::VoiceStateUpdate(v),
            Event::WebhooksUpdate(v) => Self::WebhooksUpdate(v),

            _ => return Err(EventConversionError::new(event)),
        })
    }
}

/// Deserialize into a [`DispatchEvent`] by knowing its type.
#[derive(Clone, Copy, Debug)]
pub struct DispatchEventDeserializer(pub DispatchEventType);

impl<'de> DeserializeSeed<'de> for DispatchEventDeserializer {
    type Value = DispatchEvent;

    #[allow(clippy::too_many_lines)]
    fn deserialize<D: Deserializer<'de>>(self, deserializer: D) -> Result<Self::Value, D::Error> {
        Ok(match self.0 {
            DispatchEventType::AutoModerationActionExecution => {
                DispatchEvent::AutoModerationActionExecution(
                    AutoModerationActionExecution::deserialize(deserializer)?,
                )
            }
            DispatchEventType::AutoModerationRuleCreate => DispatchEvent::AutoModerationRuleCreate(
                Box::new(AutoModerationRuleCreate::deserialize(deserializer)?),
            ),
            DispatchEventType::AutoModerationRuleDelete => DispatchEvent::AutoModerationRuleDelete(
                Box::new(AutoModerationRuleDelete::deserialize(deserializer)?),
            ),
            DispatchEventType::AutoModerationRuleUpdate => DispatchEvent::AutoModerationRuleUpdate(
                Box::new(AutoModerationRuleUpdate::deserialize(deserializer)?),
            ),
            DispatchEventType::ChannelCreate => {
                DispatchEvent::ChannelCreate(Box::new(ChannelCreate::deserialize(deserializer)?))
            }
            DispatchEventType::ChannelDelete => {
                DispatchEvent::ChannelDelete(Box::new(ChannelDelete::deserialize(deserializer)?))
            }
            DispatchEventType::ChannelPinsUpdate => {
                DispatchEvent::ChannelPinsUpdate(ChannelPinsUpdate::deserialize(deserializer)?)
            }
            DispatchEventType::ChannelUpdate => {
                DispatchEvent::ChannelUpdate(Box::new(ChannelUpdate::deserialize(deserializer)?))
            }
            DispatchEventType::CommandPermissionsUpdate => DispatchEvent::CommandPermissionsUpdate(
                CommandPermissionsUpdate::deserialize(deserializer)?,
            ),
            DispatchEventType::GiftCodeUpdate => {
                deserializer.deserialize_ignored_any(IgnoredAny)?;

                DispatchEvent::GiftCodeUpdate
            }
            DispatchEventType::BanAdd => DispatchEvent::BanAdd(BanAdd::deserialize(deserializer)?),
            DispatchEventType::BanRemove => {
                DispatchEvent::BanRemove(BanRemove::deserialize(deserializer)?)
            }
            DispatchEventType::GuildAuditLogEntryCreate => DispatchEvent::GuildAuditLogEntryCreate(
                Box::new(GuildAuditLogEntryCreate::deserialize(deserializer)?),
            ),
            DispatchEventType::GuildCreate => {
                DispatchEvent::GuildCreate(Box::new(GuildCreate::deserialize(deserializer)?))
            }
            DispatchEventType::GuildDelete => {
                DispatchEvent::GuildDelete(GuildDelete::deserialize(deserializer)?)
            }
            DispatchEventType::GuildEmojisUpdate => {
                DispatchEvent::GuildEmojisUpdate(GuildEmojisUpdate::deserialize(deserializer)?)
            }
            DispatchEventType::GuildIntegrationsUpdate => DispatchEvent::GuildIntegrationsUpdate(
                GuildIntegrationsUpdate::deserialize(deserializer)?,
            ),
            DispatchEventType::GuildScheduledEventCreate => {
                DispatchEvent::GuildScheduledEventCreate(Box::new(
                    GuildScheduledEventCreate::deserialize(deserializer)?,
                ))
            }
            DispatchEventType::GuildScheduledEventDelete => {
                DispatchEvent::GuildScheduledEventDelete(Box::new(
                    GuildScheduledEventDelete::deserialize(deserializer)?,
                ))
            }
            DispatchEventType::GuildScheduledEventUpdate => {
                DispatchEvent::GuildScheduledEventUpdate(Box::new(
                    GuildScheduledEventUpdate::deserialize(deserializer)?,
                ))
            }
            DispatchEventType::GuildScheduledEventUserAdd => {
                DispatchEvent::GuildScheduledEventUserAdd(GuildScheduledEventUserAdd::deserialize(
                    deserializer,
                )?)
            }
            DispatchEventType::GuildScheduledEventUserRemove => {
                DispatchEvent::GuildScheduledEventUserRemove(
                    GuildScheduledEventUserRemove::deserialize(deserializer)?,
                )
            }
            DispatchEventType::MemberChunk => {
                DispatchEvent::MemberChunk(MemberChunk::deserialize(deserializer)?)
            }
            DispatchEventType::MemberAdd => {
                DispatchEvent::MemberAdd(Box::new(MemberAdd::deserialize(deserializer)?))
            }
            DispatchEventType::MemberRemove => {
                DispatchEvent::MemberRemove(MemberRemove::deserialize(deserializer)?)
            }
            DispatchEventType::MemberUpdate => {
                DispatchEvent::MemberUpdate(Box::new(MemberUpdate::deserialize(deserializer)?))
            }
            DispatchEventType::RoleCreate => {
                DispatchEvent::RoleCreate(RoleCreate::deserialize(deserializer)?)
            }
            DispatchEventType::RoleDelete => {
                DispatchEvent::RoleDelete(RoleDelete::deserialize(deserializer)?)
            }
            DispatchEventType::RoleUpdate => {
                DispatchEvent::RoleUpdate(RoleUpdate::deserialize(deserializer)?)
            }
            DispatchEventType::GuildStickersUpdate => {
                DispatchEvent::GuildStickersUpdate(GuildStickersUpdate::deserialize(deserializer)?)
            }
            DispatchEventType::GuildUpdate => {
                DispatchEvent::GuildUpdate(Box::new(GuildUpdate::deserialize(deserializer)?))
            }
            DispatchEventType::IntegrationCreate => DispatchEvent::IntegrationCreate(Box::new(
                IntegrationCreate::deserialize(deserializer)?,
            )),
            DispatchEventType::IntegrationDelete => {
                DispatchEvent::IntegrationDelete(IntegrationDelete::deserialize(deserializer)?)
            }
            DispatchEventType::IntegrationUpdate => DispatchEvent::IntegrationUpdate(Box::new(
                IntegrationUpdate::deserialize(deserializer)?,
            )),
            DispatchEventType::InteractionCreate => DispatchEvent::InteractionCreate(Box::new(
                InteractionCreate::deserialize(deserializer)?,
            )),
            DispatchEventType::InviteCreate => {
                DispatchEvent::InviteCreate(Box::new(InviteCreate::deserialize(deserializer)?))
            }
            DispatchEventType::InviteDelete => {
                DispatchEvent::InviteDelete(InviteDelete::deserialize(deserializer)?)
            }
            DispatchEventType::MessageCreate => {
                DispatchEvent::MessageCreate(Box::new(MessageCreate::deserialize(deserializer)?))
            }
            DispatchEventType::MessageDelete => {
                DispatchEvent::MessageDelete(MessageDelete::deserialize(deserializer)?)
            }
            DispatchEventType::MessageDeleteBulk => {
                DispatchEvent::MessageDeleteBulk(MessageDeleteBulk::deserialize(deserializer)?)
            }
            DispatchEventType::ReactionAdd => {
                DispatchEvent::ReactionAdd(Box::new(ReactionAdd::deserialize(deserializer)?))
            }
            DispatchEventType::ReactionRemove => {
                DispatchEvent::ReactionRemove(Box::new(ReactionRemove::deserialize(deserializer)?))
            }
            DispatchEventType::ReactionRemoveEmoji => {
                DispatchEvent::ReactionRemoveEmoji(ReactionRemoveEmoji::deserialize(deserializer)?)
            }
            DispatchEventType::ReactionRemoveAll => {
                DispatchEvent::ReactionRemoveAll(ReactionRemoveAll::deserialize(deserializer)?)
            }
            DispatchEventType::MessageUpdate => {
                DispatchEvent::MessageUpdate(Box::new(MessageUpdate::deserialize(deserializer)?))
            }
            DispatchEventType::PresenceUpdate => {
                DispatchEvent::PresenceUpdate(Box::new(PresenceUpdate::deserialize(deserializer)?))
            }
            DispatchEventType::PresencesReplace => {
                deserializer.deserialize_ignored_any(IgnoredAny)?;

                DispatchEvent::PresencesReplace
            }
            DispatchEventType::Ready => {
                DispatchEvent::Ready(Box::new(Ready::deserialize(deserializer)?))
            }
            DispatchEventType::Resumed => {
                deserializer.deserialize_ignored_any(IgnoredAny)?;

                DispatchEvent::Resumed
            }
            DispatchEventType::StageInstanceCreate => {
                DispatchEvent::StageInstanceCreate(StageInstanceCreate::deserialize(deserializer)?)
            }
            DispatchEventType::StageInstanceDelete => {
                DispatchEvent::StageInstanceDelete(StageInstanceDelete::deserialize(deserializer)?)
            }
            DispatchEventType::StageInstanceUpdate => {
                DispatchEvent::StageInstanceUpdate(StageInstanceUpdate::deserialize(deserializer)?)
            }
            DispatchEventType::ThreadCreate => {
                DispatchEvent::ThreadCreate(Box::new(ThreadCreate::deserialize(deserializer)?))
            }
            DispatchEventType::ThreadDelete => {
                DispatchEvent::ThreadDelete(ThreadDelete::deserialize(deserializer)?)
            }
            DispatchEventType::ThreadListSync => {
                DispatchEvent::ThreadListSync(ThreadListSync::deserialize(deserializer)?)
            }
            DispatchEventType::ThreadMemberUpdate => DispatchEvent::ThreadMemberUpdate(Box::new(
                ThreadMemberUpdate::deserialize(deserializer)?,
            )),
            DispatchEventType::ThreadMembersUpdate => {
                DispatchEvent::ThreadMembersUpdate(ThreadMembersUpdate::deserialize(deserializer)?)
            }
            DispatchEventType::ThreadUpdate => {
                DispatchEvent::ThreadUpdate(Box::new(ThreadUpdate::deserialize(deserializer)?))
            }
            DispatchEventType::TypingStart => {
                DispatchEvent::TypingStart(Box::new(TypingStart::deserialize(deserializer)?))
            }
            DispatchEventType::UnavailableGuild => {
                DispatchEvent::UnavailableGuild(UnavailableGuild::deserialize(deserializer)?)
            }
            DispatchEventType::UserUpdate => {
                DispatchEvent::UserUpdate(UserUpdate::deserialize(deserializer)?)
            }
            DispatchEventType::VoiceServerUpdate => {
                DispatchEvent::VoiceServerUpdate(VoiceServerUpdate::deserialize(deserializer)?)
            }
            DispatchEventType::VoiceStateUpdate => DispatchEvent::VoiceStateUpdate(Box::new(
                VoiceStateUpdate::deserialize(deserializer)?,
            )),
            DispatchEventType::WebhooksUpdate => {
                DispatchEvent::WebhooksUpdate(WebhooksUpdate::deserialize(deserializer)?)
            }
        })
    }
}

/// Type of a [`DispatchEvent`].
///
/// Corresponds to an event's `t` field.
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum DispatchEventType {
    AutoModerationActionExecution,
    AutoModerationRuleCreate,
    AutoModerationRuleDelete,
    AutoModerationRuleUpdate,
    #[serde(rename = "GUILD_BAN_ADD")]
    BanAdd,
    #[serde(rename = "GUILD_BAN_REMOVE")]
    BanRemove,
    ChannelCreate,
    ChannelDelete,
    ChannelPinsUpdate,
    ChannelUpdate,
    #[serde(rename = "APPLICATION_COMMAND_PERMISSIONS_UPDATE")]
    CommandPermissionsUpdate,
    GiftCodeUpdate,
    GuildAuditLogEntryCreate,
    GuildCreate,
    GuildDelete,
    GuildEmojisUpdate,
    GuildIntegrationsUpdate,
    GuildScheduledEventCreate,
    GuildScheduledEventDelete,
    GuildScheduledEventUpdate,
    GuildScheduledEventUserAdd,
    GuildScheduledEventUserRemove,
    GuildStickersUpdate,
    GuildUpdate,
    IntegrationCreate,
    IntegrationDelete,
    IntegrationUpdate,
    InteractionCreate,
    InviteCreate,
    InviteDelete,
    #[serde(rename = "GUILD_MEMBER_ADD")]
    MemberAdd,
    #[serde(rename = "GUILD_MEMBERS_CHUNK")]
    MemberChunk,
    #[serde(rename = "GUILD_MEMBER_REMOVE")]
    MemberRemove,
    #[serde(rename = "GUILD_MEMBER_UPDATE")]
    MemberUpdate,
    MessageCreate,
    MessageDelete,
    MessageDeleteBulk,
    MessageUpdate,
    PresenceUpdate,
    PresencesReplace,
    #[serde(rename = "MESSAGE_REACTION_ADD")]
    ReactionAdd,
    #[serde(rename = "MESSAGE_REACTION_REMOVE")]
    ReactionRemove,
    #[serde(rename = "MESSAGE_REACTION_REMOVE_ALL")]
    ReactionRemoveAll,
    #[serde(rename = "MESSAGE_REACTION_REMOVE_EMOJI")]
    ReactionRemoveEmoji,
    Ready,
    Resumed,
    #[serde(rename = "GUILD_ROLE_CREATE")]
    RoleCreate,
    #[serde(rename = "GUILD_ROLE_DELETE")]
    RoleDelete,
    #[serde(rename = "GUILD_ROLE_UPDATE")]
    RoleUpdate,
    StageInstanceCreate,
    StageInstanceDelete,
    StageInstanceUpdate,
    ThreadCreate,
    ThreadDelete,
    ThreadListSync,
    ThreadMemberUpdate,
    ThreadMembersUpdate,
    ThreadUpdate,
    TypingStart,
    UnavailableGuild,
    UserUpdate,
    VoiceServerUpdate,
    VoiceStateUpdate,
    WebhooksUpdate,
}

impl DispatchEventType {
    /// Dispatch event type name.
    pub const fn name(self) -> &'static str {
        match self {
            Self::AutoModerationActionExecution => "AUTO_MODERATION_ACTION_EXECUTION",
            Self::AutoModerationRuleCreate => "AUTO_MODERATION_RULE_CREATE",
            Self::AutoModerationRuleDelete => "AUTO_MODERATION_RULE_DELETE",
            Self::AutoModerationRuleUpdate => "AUTO_MODERATION_RULE_UPDATE",
            Self::BanAdd => "GUILD_BAN_ADD",
            Self::BanRemove => "GUILD_BAN_REMOVE",
            Self::ChannelCreate => "CHANNEL_CREATE",
            Self::ChannelDelete => "CHANNEL_DELETE",
            Self::ChannelPinsUpdate => "CHANNEL_PINS_UPDATE",
            Self::ChannelUpdate => "CHANNEL_UPDATE",
            Self::CommandPermissionsUpdate => "APPLICATION_COMMAND_PERMISSIONS_UPDATE",
            Self::GiftCodeUpdate => "GIFT_CODE_UPDATE",
            Self::GuildAuditLogEntryCreate => "GUILD_AUDIT_LOG_ENTRY_CREATE",
            Self::GuildCreate => "GUILD_CREATE",
            Self::GuildDelete => "GUILD_DELETE",
            Self::GuildEmojisUpdate => "GUILD_EMOJIS_UPDATE",
            Self::GuildIntegrationsUpdate => "GUILD_INTEGRATIONS_UPDATE",
            Self::GuildScheduledEventCreate => "GUILD_SCHEDULED_EVENT_CREATE",
            Self::GuildScheduledEventDelete => "GUILD_SCHEDULED_EVENT_DELETE",
            Self::GuildScheduledEventUpdate => "GUILD_SCHEDULED_EVENT_UPDATE",
            Self::GuildScheduledEventUserAdd => "GUILD_SCHEDULED_EVENT_USER_ADD",
            Self::GuildScheduledEventUserRemove => "GUILD_SCHEDULED_EVENT_USER_REMOVE",
            Self::GuildStickersUpdate => "GUILD_STICKERS_UPDATE",
            Self::GuildUpdate => "GUILD_UPDATE",
            Self::IntegrationCreate => "INTEGRATION_CREATE",
            Self::IntegrationDelete => "INTEGRATION_DELETE",
            Self::IntegrationUpdate => "INTEGRATION_UPDATE",
            Self::InteractionCreate => "INTERACTION_CREATE",
            Self::InviteCreate => "INVITE_CREATE",
            Self::InviteDelete => "INVITE_DELETE",
            Self::MemberAdd => "GUILD_MEMBER_ADD",
            Self::MemberChunk => "GUILD_MEMBERS_CHUNK",
            Self::MemberRemove => "GUILD_MEMBER_REMOVE",
            Self::MemberUpdate => "GUILD_MEMBER_UPDATE",
            Self::MessageCreate => "MESSAGE_CREATE",
            Self::MessageDelete => "MESSAGE_DELETE",
            Self::MessageDeleteBulk => "MESSAGE_DELETE_BULK",
            Self::MessageUpdate => "MESSAGE_UPDATE",
            Self::PresencesReplace => "PRESENCES_REPLACE",
            Self::PresenceUpdate => "PRESENCE_UPDATE",
            Self::ReactionAdd => "MESSAGE_REACTION_ADD",
            Self::ReactionRemove => "MESSAGE_REACTION_REMOVE",
            Self::ReactionRemoveAll => "MESSAGE_REACTION_REMOVE_ALL",
            Self::ReactionRemoveEmoji => "MESSAGE_REACTION_REMOVE_EMOJI",
            Self::Ready => "READY",
            Self::Resumed => "RESUMED",
            Self::RoleCreate => "GUILD_ROLE_CREATE",
            Self::RoleDelete => "GUILD_ROLE_DELETE",
            Self::RoleUpdate => "GUILD_ROLE_UPDATE",
            Self::StageInstanceCreate => "STAGE_INSTANCE_CREATE",
            Self::StageInstanceDelete => "STAGE_INSTANCE_DELETE",
            Self::StageInstanceUpdate => "STAGE_INSTANCE_UPDATE",
            Self::ThreadCreate => "THREAD_CREATE",
            Self::ThreadDelete => "THREAD_DELETE",
            Self::ThreadListSync => "THREAD_LIST_SYNC",
            Self::ThreadMembersUpdate => "THREAD_MEMBERS_UPDATE",
            Self::ThreadMemberUpdate => "THREAD_MEMBER_UPDATE",
            Self::ThreadUpdate => "THREAD_UPDATE",
            Self::TypingStart => "TYPING_START",
            Self::UnavailableGuild => "UNAVAILABLE_GUILD",
            Self::UserUpdate => "USER_UPDATE",
            Self::VoiceServerUpdate => "VOICE_SERVER_UPDATE",
            Self::VoiceStateUpdate => "VOICE_STATE_UPDATE",
            Self::WebhooksUpdate => "WEBHOOKS_UPDATE",
        }
    }
}

impl Display for DispatchEventType {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        f.write_str(self.name())
    }
}

impl FromStr for DispatchEventType {
    type Err = value::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}

#[cfg(test)]
mod tests {
    //! [`EVENT_THRESHOLD`] is equivalent to 184 bytes. This was decided based
    //! on the size of [`DispatchEvent`] at the time of writing. The assertions
    //! here are to ensure that in the case the events themselves grow or shrink
    //! past the threshold, they are properly boxed or unboxed respectively.
    //!
    //! If a field has been added to an event in the "unboxed" section and its
    //! assertion now fails, then you will need to wrap the event in a box in
    //! the `Event` type and move the assertion to the "boxed" section.
    //!
    //! Likewise, if a field has been removed from an event in the "boxed"
    //! section and the assertion now fails, you will need to remove the box
    //! wrapping the event in the `Event` type and move the assertion to the
    //! "unboxed" section.

    use super::{DispatchEvent, DispatchEventDeserializer, DispatchEventType};
    use crate::gateway::{event::Event, payload::incoming::*};
    use serde::{
        de::{DeserializeOwned, DeserializeSeed},
        Serialize,
    };
    use serde_test::Token;
    use static_assertions::{assert_impl_all, const_assert};
    use std::{
        fmt::{Debug, Display},
        hash::Hash,
        mem,
        str::FromStr,
    };

    assert_impl_all!(
        DispatchEvent: Clone,
        Debug,
        PartialEq,
        Send,
        Serialize,
        Sync,
        TryFrom<Event>
    );

    assert_impl_all!(
        DispatchEventDeserializer: Clone,
        Copy,
        Debug,
        DeserializeSeed<'static>
    );

    assert_impl_all!(
        DispatchEventType: Clone,
        Copy,
        Debug,
        DeserializeOwned,
        Display,
        Eq,
        FromStr,
        Hash,
        PartialEq,
        Send,
        Serialize,
        Sync
    );

    // `dead_code`: `const_assert` operates at the compiler level, and the lint
    // requires a variable to be used in a function, so this is a false
    // positive.
    #[allow(dead_code)]
    const EVENT_THRESHOLD: usize = 184;

    const_assert!(mem::size_of::<DispatchEvent>() == EVENT_THRESHOLD);

    // Boxed events.
    const_assert!(mem::size_of::<AutoModerationRuleCreate>() > EVENT_THRESHOLD);
    const_assert!(mem::size_of::<AutoModerationRuleDelete>() > EVENT_THRESHOLD);
    const_assert!(mem::size_of::<AutoModerationRuleUpdate>() > EVENT_THRESHOLD);
    const_assert!(mem::size_of::<ChannelCreate>() > EVENT_THRESHOLD);
    const_assert!(mem::size_of::<ChannelDelete>() > EVENT_THRESHOLD);
    const_assert!(mem::size_of::<ChannelUpdate>() > EVENT_THRESHOLD);
    const_assert!(mem::size_of::<GuildScheduledEventCreate>() > EVENT_THRESHOLD);
    const_assert!(mem::size_of::<GuildScheduledEventDelete>() > EVENT_THRESHOLD);
    const_assert!(mem::size_of::<GuildScheduledEventUpdate>() > EVENT_THRESHOLD);
    const_assert!(mem::size_of::<GuildUpdate>() > EVENT_THRESHOLD);
    const_assert!(mem::size_of::<IntegrationCreate>() > EVENT_THRESHOLD);
    const_assert!(mem::size_of::<IntegrationUpdate>() > EVENT_THRESHOLD);
    const_assert!(mem::size_of::<InviteCreate>() > EVENT_THRESHOLD);
    const_assert!(mem::size_of::<InteractionCreate>() > EVENT_THRESHOLD);
    const_assert!(mem::size_of::<MemberAdd>() > EVENT_THRESHOLD);
    const_assert!(mem::size_of::<MemberUpdate>() > EVENT_THRESHOLD);
    const_assert!(mem::size_of::<MessageCreate>() > EVENT_THRESHOLD);
    const_assert!(mem::size_of::<MessageUpdate>() > EVENT_THRESHOLD);
    const_assert!(mem::size_of::<PresenceUpdate>() > EVENT_THRESHOLD);
    const_assert!(mem::size_of::<ReactionAdd>() > EVENT_THRESHOLD);
    const_assert!(mem::size_of::<ReactionRemove>() > EVENT_THRESHOLD);
    const_assert!(mem::size_of::<Ready>() > EVENT_THRESHOLD);
    const_assert!(mem::size_of::<ThreadCreate>() > EVENT_THRESHOLD);
    const_assert!(mem::size_of::<ThreadMemberUpdate>() > EVENT_THRESHOLD);
    const_assert!(mem::size_of::<ThreadUpdate>() > EVENT_THRESHOLD);
    const_assert!(mem::size_of::<TypingStart>() > EVENT_THRESHOLD);
    const_assert!(mem::size_of::<VoiceStateUpdate>() > EVENT_THRESHOLD);

    // Unboxed.
    const_assert!(mem::size_of::<AutoModerationActionExecution>() <= EVENT_THRESHOLD);
    const_assert!(mem::size_of::<BanAdd>() <= EVENT_THRESHOLD);
    const_assert!(mem::size_of::<BanRemove>() <= EVENT_THRESHOLD);
    const_assert!(mem::size_of::<ChannelPinsUpdate>() <= EVENT_THRESHOLD);
    const_assert!(mem::size_of::<CommandPermissionsUpdate>() <= EVENT_THRESHOLD);
    const_assert!(mem::size_of::<GuildDelete>() <= EVENT_THRESHOLD);
    const_assert!(mem::size_of::<GuildEmojisUpdate>() <= EVENT_THRESHOLD);
    const_assert!(mem::size_of::<GuildIntegrationsUpdate>() <= EVENT_THRESHOLD);
    const_assert!(mem::size_of::<GuildScheduledEventUserAdd>() <= EVENT_THRESHOLD);
    const_assert!(mem::size_of::<GuildScheduledEventUserRemove>() <= EVENT_THRESHOLD);
    const_assert!(mem::size_of::<IntegrationDelete>() <= EVENT_THRESHOLD);
    const_assert!(mem::size_of::<InviteDelete>() <= EVENT_THRESHOLD);
    const_assert!(mem::size_of::<MemberChunk>() <= EVENT_THRESHOLD);
    const_assert!(mem::size_of::<MemberRemove>() <= EVENT_THRESHOLD);
    const_assert!(mem::size_of::<MessageDelete>() <= EVENT_THRESHOLD);
    const_assert!(mem::size_of::<MessageDeleteBulk>() <= EVENT_THRESHOLD);
    const_assert!(mem::size_of::<ReactionRemoveAll>() <= EVENT_THRESHOLD);
    const_assert!(mem::size_of::<RoleCreate>() <= EVENT_THRESHOLD);
    const_assert!(mem::size_of::<RoleDelete>() <= EVENT_THRESHOLD);
    const_assert!(mem::size_of::<RoleUpdate>() <= EVENT_THRESHOLD);
    const_assert!(mem::size_of::<StageInstanceCreate>() <= EVENT_THRESHOLD);
    const_assert!(mem::size_of::<StageInstanceDelete>() <= EVENT_THRESHOLD);
    const_assert!(mem::size_of::<StageInstanceUpdate>() <= EVENT_THRESHOLD);
    const_assert!(mem::size_of::<ThreadDelete>() <= EVENT_THRESHOLD);
    const_assert!(mem::size_of::<ThreadListSync>() <= EVENT_THRESHOLD);
    const_assert!(mem::size_of::<ThreadMembersUpdate>() <= EVENT_THRESHOLD);
    const_assert!(mem::size_of::<UnavailableGuild>() <= EVENT_THRESHOLD);
    const_assert!(mem::size_of::<UserUpdate>() <= EVENT_THRESHOLD);
    const_assert!(mem::size_of::<VoiceServerUpdate>() <= EVENT_THRESHOLD);
    const_assert!(mem::size_of::<WebhooksUpdate>() <= EVENT_THRESHOLD);

    const MAP: &[(DispatchEventType, &str)] = &[
        (
            DispatchEventType::AutoModerationActionExecution,
            "AUTO_MODERATION_ACTION_EXECUTION",
        ),
        (
            DispatchEventType::AutoModerationRuleCreate,
            "AUTO_MODERATION_RULE_CREATE",
        ),
        (
            DispatchEventType::AutoModerationRuleDelete,
            "AUTO_MODERATION_RULE_DELETE",
        ),
        (
            DispatchEventType::AutoModerationRuleUpdate,
            "AUTO_MODERATION_RULE_UPDATE",
        ),
        (DispatchEventType::BanAdd, "GUILD_BAN_ADD"),
        (DispatchEventType::BanRemove, "GUILD_BAN_REMOVE"),
        (DispatchEventType::ChannelCreate, "CHANNEL_CREATE"),
        (DispatchEventType::ChannelDelete, "CHANNEL_DELETE"),
        (DispatchEventType::ChannelPinsUpdate, "CHANNEL_PINS_UPDATE"),
        (DispatchEventType::ChannelUpdate, "CHANNEL_UPDATE"),
        (
            DispatchEventType::CommandPermissionsUpdate,
            "APPLICATION_COMMAND_PERMISSIONS_UPDATE",
        ),
        (DispatchEventType::GiftCodeUpdate, "GIFT_CODE_UPDATE"),
        (
            DispatchEventType::GuildAuditLogEntryCreate,
            "GUILD_AUDIT_LOG_ENTRY_CREATE",
        ),
        (DispatchEventType::GuildCreate, "GUILD_CREATE"),
        (DispatchEventType::GuildDelete, "GUILD_DELETE"),
        (DispatchEventType::GuildEmojisUpdate, "GUILD_EMOJIS_UPDATE"),
        (
            DispatchEventType::GuildIntegrationsUpdate,
            "GUILD_INTEGRATIONS_UPDATE",
        ),
        (
            DispatchEventType::GuildScheduledEventCreate,
            "GUILD_SCHEDULED_EVENT_CREATE",
        ),
        (
            DispatchEventType::GuildScheduledEventDelete,
            "GUILD_SCHEDULED_EVENT_DELETE",
        ),
        (
            DispatchEventType::GuildScheduledEventUpdate,
            "GUILD_SCHEDULED_EVENT_UPDATE",
        ),
        (
            DispatchEventType::GuildScheduledEventUserAdd,
            "GUILD_SCHEDULED_EVENT_USER_ADD",
        ),
        (
            DispatchEventType::GuildScheduledEventUserRemove,
            "GUILD_SCHEDULED_EVENT_USER_REMOVE",
        ),
        (DispatchEventType::GuildUpdate, "GUILD_UPDATE"),
        (DispatchEventType::IntegrationCreate, "INTEGRATION_CREATE"),
        (DispatchEventType::IntegrationDelete, "INTEGRATION_DELETE"),
        (DispatchEventType::IntegrationUpdate, "INTEGRATION_UPDATE"),
        (DispatchEventType::InteractionCreate, "INTERACTION_CREATE"),
        (DispatchEventType::InviteCreate, "INVITE_CREATE"),
        (DispatchEventType::InviteDelete, "INVITE_DELETE"),
        (DispatchEventType::MemberAdd, "GUILD_MEMBER_ADD"),
        (DispatchEventType::MemberChunk, "GUILD_MEMBERS_CHUNK"),
        (DispatchEventType::MemberRemove, "GUILD_MEMBER_REMOVE"),
        (DispatchEventType::MemberUpdate, "GUILD_MEMBER_UPDATE"),
        (DispatchEventType::MessageCreate, "MESSAGE_CREATE"),
        (DispatchEventType::MessageDelete, "MESSAGE_DELETE"),
        (DispatchEventType::MessageDeleteBulk, "MESSAGE_DELETE_BULK"),
        (DispatchEventType::MessageUpdate, "MESSAGE_UPDATE"),
        (DispatchEventType::PresenceUpdate, "PRESENCE_UPDATE"),
        (DispatchEventType::PresencesReplace, "PRESENCES_REPLACE"),
        (DispatchEventType::ReactionAdd, "MESSAGE_REACTION_ADD"),
        (DispatchEventType::ReactionRemove, "MESSAGE_REACTION_REMOVE"),
        (
            DispatchEventType::ReactionRemoveAll,
            "MESSAGE_REACTION_REMOVE_ALL",
        ),
        (
            DispatchEventType::ReactionRemoveEmoji,
            "MESSAGE_REACTION_REMOVE_EMOJI",
        ),
        (DispatchEventType::Ready, "READY"),
        (DispatchEventType::Resumed, "RESUMED"),
        (DispatchEventType::RoleCreate, "GUILD_ROLE_CREATE"),
        (DispatchEventType::RoleDelete, "GUILD_ROLE_DELETE"),
        (DispatchEventType::RoleUpdate, "GUILD_ROLE_UPDATE"),
        (
            DispatchEventType::StageInstanceCreate,
            "STAGE_INSTANCE_CREATE",
        ),
        (
            DispatchEventType::StageInstanceDelete,
            "STAGE_INSTANCE_DELETE",
        ),
        (
            DispatchEventType::StageInstanceUpdate,
            "STAGE_INSTANCE_UPDATE",
        ),
        (DispatchEventType::ThreadCreate, "THREAD_CREATE"),
        (DispatchEventType::ThreadDelete, "THREAD_DELETE"),
        (DispatchEventType::ThreadListSync, "THREAD_LIST_SYNC"),
        (
            DispatchEventType::ThreadMemberUpdate,
            "THREAD_MEMBER_UPDATE",
        ),
        (
            DispatchEventType::ThreadMembersUpdate,
            "THREAD_MEMBERS_UPDATE",
        ),
        (DispatchEventType::ThreadUpdate, "THREAD_UPDATE"),
        (DispatchEventType::TypingStart, "TYPING_START"),
        (DispatchEventType::UnavailableGuild, "UNAVAILABLE_GUILD"),
        (DispatchEventType::UserUpdate, "USER_UPDATE"),
        (DispatchEventType::VoiceServerUpdate, "VOICE_SERVER_UPDATE"),
        (DispatchEventType::VoiceStateUpdate, "VOICE_STATE_UPDATE"),
        (DispatchEventType::WebhooksUpdate, "WEBHOOKS_UPDATE"),
    ];

    #[test]
    fn event_type_serde() {
        for (value, name) in MAP {
            serde_test::assert_tokens(
                value,
                &[Token::UnitVariant {
                    name: "DispatchEventType",
                    variant: name,
                }],
            );
            assert_eq!(value.name(), *name);
        }
    }
}
