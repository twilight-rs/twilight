#![allow(clippy::wildcard_imports)]

pub mod gateway;

mod dispatch;
mod kind;

pub use self::{
    dispatch::{DispatchEvent, DispatchEventWithTypeDeserializer},
    gateway::{GatewayEvent, GatewayEventDeserializer},
    kind::EventType,
};

use super::{payload::incoming::*, CloseFrame};
use crate::id::{marker::GuildMarker, Id};
use std::error::Error;
use std::fmt::{Debug, Display, Formatter, Result as FmtResult};

/// Any type of event that a shard emits.
///
/// Flattened type containing all the variants of [`DispatchEvent`] and
/// [`GatewayEvent`], along with a [gateway close event].
///
/// [gateway close event]: Self::GatewayClose
#[derive(Clone, Debug, PartialEq)]
pub enum Event {
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
    /// A user subscribes to a SKU.
    EntitlementCreate(EntitlementCreate),
    /// A user's entitlement is removed.
    EntitlementDelete(EntitlementDelete),
    /// A user's subscription renews for the
    /// next billing period.
    ///
    /// The `ends_at` field will have an updated value with
    /// the new expiration date.
    EntitlementUpdate(EntitlementUpdate),
    /// Close message with an optional frame including information about the
    /// reason for the close.
    GatewayClose(Option<CloseFrame<'static>>),
    /// A heartbeat was sent to or received from the gateway.
    GatewayHeartbeat(u64),
    /// A heartbeat acknowledgement was received from the gateway.
    GatewayHeartbeatAck,
    /// A "hello" packet was received from the gateway.
    GatewayHello(Hello),
    /// A shard's session was invalidated.
    ///
    /// `true` if resumable. If not, then the shard must do a full reconnect.
    GatewayInvalidateSession(bool),
    /// The gateway is indicating to perform a reconnect.
    GatewayReconnect,
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
    /// A guild integration was deleted.
    IntegrationDelete(IntegrationDelete),
    /// A guild integration was updated.
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
    /// A vote was added to a poll.
    MessagePollVoteAdd(MessagePollVoteAdd),
    /// A vote was removed from a poll.
    MessagePollVoteRemove(MessagePollVoteRemove),
    /// A message was updated in a channel.
    MessageUpdate(Box<MessageUpdate>),
    /// A user's active presence (such as game or online status) was updated.
    PresenceUpdate(Box<PresenceUpdate>),
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

impl Event {
    /// Guild ID of the event, if available.
    pub const fn guild_id(&self) -> Option<Id<GuildMarker>> {
        match self {
            Event::AutoModerationActionExecution(e) => Some(e.guild_id),
            Event::AutoModerationRuleCreate(e) => Some(e.0.guild_id),
            Event::AutoModerationRuleDelete(e) => Some(e.0.guild_id),
            Event::AutoModerationRuleUpdate(e) => Some(e.0.guild_id),
            Event::BanAdd(e) => Some(e.guild_id),
            Event::BanRemove(e) => Some(e.guild_id),
            Event::ChannelCreate(e) => e.0.guild_id,
            Event::ChannelDelete(e) => e.0.guild_id,
            Event::ChannelPinsUpdate(e) => e.guild_id,
            Event::ChannelUpdate(e) => e.0.guild_id,
            Event::CommandPermissionsUpdate(e) => Some(e.0.guild_id),
            Event::GuildAuditLogEntryCreate(e) => e.0.guild_id,
            Event::GuildCreate(e) => Some(e.id()),
            Event::GuildDelete(e) => Some(e.id),
            Event::GuildEmojisUpdate(e) => Some(e.guild_id),
            Event::GuildIntegrationsUpdate(e) => Some(e.guild_id),
            Event::GuildScheduledEventCreate(e) => Some(e.0.guild_id),
            Event::GuildScheduledEventDelete(e) => Some(e.0.guild_id),
            Event::GuildScheduledEventUpdate(e) => Some(e.0.guild_id),
            Event::GuildScheduledEventUserAdd(e) => Some(e.guild_id),
            Event::GuildScheduledEventUserRemove(e) => Some(e.guild_id),
            Event::GuildStickersUpdate(e) => Some(e.guild_id),
            Event::GuildUpdate(e) => Some(e.0.id),
            Event::IntegrationCreate(e) => e.0.guild_id,
            Event::IntegrationDelete(e) => Some(e.guild_id),
            Event::IntegrationUpdate(e) => e.0.guild_id,
            Event::InteractionCreate(e) => e.0.guild_id,
            Event::InviteCreate(e) => Some(e.guild_id),
            Event::InviteDelete(e) => Some(e.guild_id),
            Event::MemberAdd(e) => Some(e.guild_id),
            Event::MemberChunk(e) => Some(e.guild_id),
            Event::MemberRemove(e) => Some(e.guild_id),
            Event::MemberUpdate(e) => Some(e.guild_id),
            Event::MessageCreate(e) => e.0.guild_id,
            Event::MessageDelete(e) => e.guild_id,
            Event::MessageDeleteBulk(e) => e.guild_id,
            Event::MessageUpdate(e) => e.guild_id,
            Event::MessagePollVoteAdd(e) => e.guild_id,
            Event::MessagePollVoteRemove(e) => e.guild_id,
            Event::PresenceUpdate(e) => Some(e.0.guild_id),
            Event::ReactionAdd(e) => e.0.guild_id,
            Event::ReactionRemove(e) => e.0.guild_id,
            Event::ReactionRemoveAll(e) => e.guild_id,
            Event::ReactionRemoveEmoji(e) => Some(e.guild_id),
            Event::RoleCreate(e) => Some(e.guild_id),
            Event::RoleDelete(e) => Some(e.guild_id),
            Event::RoleUpdate(e) => Some(e.guild_id),
            Event::StageInstanceCreate(e) => Some(e.0.guild_id),
            Event::StageInstanceDelete(e) => Some(e.0.guild_id),
            Event::StageInstanceUpdate(e) => Some(e.0.guild_id),
            Event::ThreadCreate(e) => e.0.guild_id,
            Event::ThreadDelete(e) => Some(e.guild_id),
            Event::ThreadListSync(e) => Some(e.guild_id),
            Event::ThreadMemberUpdate(e) => Some(e.guild_id),
            Event::ThreadMembersUpdate(e) => Some(e.guild_id),
            Event::ThreadUpdate(e) => e.0.guild_id,
            Event::TypingStart(e) => e.guild_id,
            Event::UnavailableGuild(e) => Some(e.id),
            Event::VoiceServerUpdate(e) => Some(e.guild_id),
            Event::VoiceStateUpdate(e) => e.0.guild_id,
            Event::WebhooksUpdate(e) => Some(e.guild_id),
            Event::GatewayClose(_)
            | Event::EntitlementCreate(_)
            | Event::EntitlementDelete(_)
            | Event::EntitlementUpdate(_)
            | Event::GatewayHeartbeat(_)
            | Event::GatewayHeartbeatAck
            | Event::GatewayHello(_)
            | Event::GatewayInvalidateSession(_)
            | Event::GatewayReconnect
            | Event::Ready(_)
            | Event::Resumed
            | Event::UserUpdate(_) => None,
        }
    }

    pub const fn kind(&self) -> EventType {
        match self {
            Self::AutoModerationActionExecution(_) => EventType::AutoModerationActionExecution,
            Self::AutoModerationRuleCreate(_) => EventType::AutoModerationRuleCreate,
            Self::AutoModerationRuleDelete(_) => EventType::AutoModerationRuleDelete,
            Self::AutoModerationRuleUpdate(_) => EventType::AutoModerationRuleUpdate,
            Self::BanAdd(_) => EventType::BanAdd,
            Self::BanRemove(_) => EventType::BanRemove,
            Self::ChannelCreate(_) => EventType::ChannelCreate,
            Self::ChannelDelete(_) => EventType::ChannelDelete,
            Self::ChannelPinsUpdate(_) => EventType::ChannelPinsUpdate,
            Self::ChannelUpdate(_) => EventType::ChannelUpdate,
            Self::CommandPermissionsUpdate(_) => EventType::CommandPermissionsUpdate,
            Self::EntitlementCreate(_) => EventType::EntitlementCreate,
            Self::EntitlementDelete(_) => EventType::EntitlementDelete,
            Self::EntitlementUpdate(_) => EventType::EntitlementUpdate,
            Self::GatewayClose(_) => EventType::GatewayClose,
            Self::GatewayHeartbeat(_) => EventType::GatewayHeartbeat,
            Self::GatewayHeartbeatAck => EventType::GatewayHeartbeatAck,
            Self::GatewayHello(_) => EventType::GatewayHello,
            Self::GatewayInvalidateSession(_) => EventType::GatewayInvalidateSession,
            Self::GatewayReconnect => EventType::GatewayReconnect,
            Self::GuildAuditLogEntryCreate(_) => EventType::GuildAuditLogEntryCreate,
            Self::GuildCreate(_) => EventType::GuildCreate,
            Self::GuildDelete(_) => EventType::GuildDelete,
            Self::GuildEmojisUpdate(_) => EventType::GuildEmojisUpdate,
            Self::GuildIntegrationsUpdate(_) => EventType::GuildIntegrationsUpdate,
            Self::GuildScheduledEventCreate(_) => EventType::GuildScheduledEventCreate,
            Self::GuildScheduledEventDelete(_) => EventType::GuildScheduledEventDelete,
            Self::GuildScheduledEventUpdate(_) => EventType::GuildScheduledEventUpdate,
            Self::GuildScheduledEventUserAdd(_) => EventType::GuildScheduledEventUserAdd,
            Self::GuildScheduledEventUserRemove(_) => EventType::GuildScheduledEventUserRemove,
            Self::GuildStickersUpdate(_) => EventType::GuildStickersUpdate,
            Self::GuildUpdate(_) => EventType::GuildUpdate,
            Self::IntegrationCreate(_) => EventType::IntegrationCreate,
            Self::IntegrationDelete(_) => EventType::IntegrationDelete,
            Self::IntegrationUpdate(_) => EventType::IntegrationUpdate,
            Self::InteractionCreate(_) => EventType::InteractionCreate,
            Self::InviteCreate(_) => EventType::InviteCreate,
            Self::InviteDelete(_) => EventType::InviteDelete,
            Self::MemberAdd(_) => EventType::MemberAdd,
            Self::MemberRemove(_) => EventType::MemberRemove,
            Self::MemberUpdate(_) => EventType::MemberUpdate,
            Self::MemberChunk(_) => EventType::MemberChunk,
            Self::MessageCreate(_) => EventType::MessageCreate,
            Self::MessageDelete(_) => EventType::MessageDelete,
            Self::MessageDeleteBulk(_) => EventType::MessageDeleteBulk,
            Self::MessagePollVoteAdd(_) => EventType::MessagePollVoteAdd,
            Self::MessagePollVoteRemove(_) => EventType::MessagePollVoteRemove,
            Self::MessageUpdate(_) => EventType::MessageUpdate,
            Self::PresenceUpdate(_) => EventType::PresenceUpdate,
            Self::ReactionAdd(_) => EventType::ReactionAdd,
            Self::ReactionRemove(_) => EventType::ReactionRemove,
            Self::ReactionRemoveAll(_) => EventType::ReactionRemoveAll,
            Self::ReactionRemoveEmoji(_) => EventType::ReactionRemoveEmoji,
            Self::Ready(_) => EventType::Ready,
            Self::Resumed => EventType::Resumed,
            Self::RoleCreate(_) => EventType::RoleCreate,
            Self::RoleDelete(_) => EventType::RoleDelete,
            Self::RoleUpdate(_) => EventType::RoleUpdate,
            Self::StageInstanceCreate(_) => EventType::StageInstanceCreate,
            Self::StageInstanceDelete(_) => EventType::StageInstanceDelete,
            Self::StageInstanceUpdate(_) => EventType::StageInstanceUpdate,
            Self::ThreadCreate(_) => EventType::ThreadCreate,
            Self::ThreadDelete(_) => EventType::ThreadDelete,
            Self::ThreadListSync(_) => EventType::ThreadListSync,
            Self::ThreadMemberUpdate(_) => EventType::ThreadMemberUpdate,
            Self::ThreadMembersUpdate(_) => EventType::ThreadMembersUpdate,
            Self::ThreadUpdate(_) => EventType::ThreadUpdate,
            Self::TypingStart(_) => EventType::TypingStart,
            Self::UnavailableGuild(_) => EventType::UnavailableGuild,
            Self::UserUpdate(_) => EventType::UserUpdate,
            Self::VoiceServerUpdate(_) => EventType::VoiceServerUpdate,
            Self::VoiceStateUpdate(_) => EventType::VoiceStateUpdate,
            Self::WebhooksUpdate(_) => EventType::WebhooksUpdate,
        }
    }
}

impl From<DispatchEvent> for Event {
    fn from(event: DispatchEvent) -> Self {
        match event {
            DispatchEvent::AutoModerationActionExecution(v) => {
                Self::AutoModerationActionExecution(v)
            }
            DispatchEvent::AutoModerationRuleCreate(v) => Self::AutoModerationRuleCreate(v),
            DispatchEvent::AutoModerationRuleDelete(v) => Self::AutoModerationRuleDelete(v),
            DispatchEvent::AutoModerationRuleUpdate(v) => Self::AutoModerationRuleUpdate(v),
            DispatchEvent::BanAdd(v) => Self::BanAdd(v),
            DispatchEvent::BanRemove(v) => Self::BanRemove(v),
            DispatchEvent::ChannelCreate(v) => Self::ChannelCreate(v),
            DispatchEvent::ChannelDelete(v) => Self::ChannelDelete(v),
            DispatchEvent::ChannelPinsUpdate(v) => Self::ChannelPinsUpdate(v),
            DispatchEvent::ChannelUpdate(v) => Self::ChannelUpdate(v),
            DispatchEvent::CommandPermissionsUpdate(v) => Self::CommandPermissionsUpdate(v),
            DispatchEvent::EntitlementCreate(v) => Self::EntitlementCreate(v),
            DispatchEvent::EntitlementDelete(v) => Self::EntitlementDelete(v),
            DispatchEvent::EntitlementUpdate(v) => Self::EntitlementUpdate(v),
            DispatchEvent::GuildAuditLogEntryCreate(v) => Self::GuildAuditLogEntryCreate(v),
            DispatchEvent::GuildCreate(v) => Self::GuildCreate(v),
            DispatchEvent::GuildDelete(v) => Self::GuildDelete(v),
            DispatchEvent::GuildEmojisUpdate(v) => Self::GuildEmojisUpdate(v),
            DispatchEvent::GuildIntegrationsUpdate(v) => Self::GuildIntegrationsUpdate(v),
            DispatchEvent::GuildScheduledEventCreate(v) => Self::GuildScheduledEventCreate(v),
            DispatchEvent::GuildScheduledEventDelete(v) => Self::GuildScheduledEventDelete(v),
            DispatchEvent::GuildScheduledEventUpdate(v) => Self::GuildScheduledEventUpdate(v),
            DispatchEvent::GuildScheduledEventUserAdd(v) => Self::GuildScheduledEventUserAdd(v),
            DispatchEvent::GuildScheduledEventUserRemove(v) => {
                Self::GuildScheduledEventUserRemove(v)
            }
            DispatchEvent::GuildStickersUpdate(v) => Self::GuildStickersUpdate(v),
            DispatchEvent::GuildUpdate(v) => Self::GuildUpdate(v),
            DispatchEvent::IntegrationCreate(v) => Self::IntegrationCreate(v),
            DispatchEvent::IntegrationDelete(v) => Self::IntegrationDelete(v),
            DispatchEvent::IntegrationUpdate(v) => Self::IntegrationUpdate(v),
            DispatchEvent::InteractionCreate(v) => Self::InteractionCreate(v),
            DispatchEvent::InviteCreate(v) => Self::InviteCreate(v),
            DispatchEvent::InviteDelete(v) => Self::InviteDelete(v),
            DispatchEvent::MemberAdd(v) => Self::MemberAdd(v),
            DispatchEvent::MemberRemove(v) => Self::MemberRemove(v),
            DispatchEvent::MemberUpdate(v) => Self::MemberUpdate(v),
            DispatchEvent::MemberChunk(v) => Self::MemberChunk(v),
            DispatchEvent::RoleCreate(v) => Self::RoleCreate(v),
            DispatchEvent::RoleDelete(v) => Self::RoleDelete(v),
            DispatchEvent::RoleUpdate(v) => Self::RoleUpdate(v),
            DispatchEvent::MessageCreate(v) => Self::MessageCreate(v),
            DispatchEvent::MessageDelete(v) => Self::MessageDelete(v),
            DispatchEvent::MessageDeleteBulk(v) => Self::MessageDeleteBulk(v),
            DispatchEvent::MessagePollVoteAdd(v) => Self::MessagePollVoteAdd(v),
            DispatchEvent::MessagePollVoteRemove(v) => Self::MessagePollVoteRemove(v),
            DispatchEvent::MessageUpdate(v) => Self::MessageUpdate(v),
            DispatchEvent::PresenceUpdate(v) => Self::PresenceUpdate(v),
            DispatchEvent::ReactionAdd(v) => Self::ReactionAdd(v),
            DispatchEvent::ReactionRemove(v) => Self::ReactionRemove(v),
            DispatchEvent::ReactionRemoveAll(v) => Self::ReactionRemoveAll(v),
            DispatchEvent::ReactionRemoveEmoji(v) => Self::ReactionRemoveEmoji(v),
            DispatchEvent::Ready(v) => Self::Ready(v),
            DispatchEvent::Resumed => Self::Resumed,
            DispatchEvent::StageInstanceCreate(v) => Self::StageInstanceCreate(v),
            DispatchEvent::StageInstanceDelete(v) => Self::StageInstanceDelete(v),
            DispatchEvent::StageInstanceUpdate(v) => Self::StageInstanceUpdate(v),
            DispatchEvent::ThreadCreate(v) => Self::ThreadCreate(v),
            DispatchEvent::ThreadDelete(v) => Self::ThreadDelete(v),
            DispatchEvent::ThreadListSync(v) => Self::ThreadListSync(v),
            DispatchEvent::ThreadMemberUpdate(v) => Self::ThreadMemberUpdate(v),
            DispatchEvent::ThreadMembersUpdate(v) => Self::ThreadMembersUpdate(v),
            DispatchEvent::ThreadUpdate(v) => Self::ThreadUpdate(v),
            DispatchEvent::TypingStart(v) => Self::TypingStart(v),
            DispatchEvent::UnavailableGuild(v) => Self::UnavailableGuild(v),
            DispatchEvent::UserUpdate(v) => Self::UserUpdate(v),
            DispatchEvent::VoiceServerUpdate(v) => Self::VoiceServerUpdate(v),
            DispatchEvent::VoiceStateUpdate(v) => Self::VoiceStateUpdate(v),
            DispatchEvent::WebhooksUpdate(v) => Self::WebhooksUpdate(v),
        }
    }
}

impl From<GatewayEvent> for Event {
    fn from(event: GatewayEvent) -> Self {
        match event {
            GatewayEvent::Dispatch(_, e) => Self::from(e),
            GatewayEvent::Heartbeat(interval) => Self::GatewayHeartbeat(interval),
            GatewayEvent::HeartbeatAck => Self::GatewayHeartbeatAck,
            GatewayEvent::Hello(interval) => Self::GatewayHello(interval),
            GatewayEvent::InvalidateSession(r) => Self::GatewayInvalidateSession(r),
            GatewayEvent::Reconnect => Self::GatewayReconnect,
        }
    }
}

/// An error that describes a failure to convert from one event type to another.
#[derive(Debug)]
pub struct EventConversionError {
    event: Event,
}

impl EventConversionError {
    pub const fn new(event: Event) -> EventConversionError {
        Self { event }
    }

    /// Return an immutable reference to the original event.
    pub const fn event_ref(&self) -> &Event {
        &self.event
    }

    /// Consume the error, returning the original event.
    #[allow(clippy::missing_const_for_fn)]
    pub fn into_event(self) -> Event {
        self.event
    }
}

impl Display for EventConversionError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        f.write_str("event variant failed to convert")
    }
}

impl Error for EventConversionError {}

#[cfg(test)]
mod tests {
    //! `EVENT_THRESHOLD` is equivalent to 192 bytes. This was decided based on
    //! the size of `Event` at the time of writing. The assertions here are to
    //! ensure that in the case the events themselves grow or shrink past the
    //! threshold, they are properly boxed or unboxed respectively.
    //!
    //! If a field has been added to an event in the "unboxed" section and its
    //! assertion now fails, then you will need to wrap the event in a box in
    //! the `Event` type and move the assertion to the "boxed" section.
    //!
    //! Likewise, if a field has been removed from an event in the "boxed"
    //! section and the assertion now fails, you will need to remove the box
    //! wrapping the event in the `Event` type and move the assertion to the
    //! "unboxed" section.

    use super::{super::payload::incoming::*, Event};
    use static_assertions::const_assert;
    use std::mem;

    // `dead_code`: `const_assert` operates at the compiler level, and the lint
    // requires a variable to be used in a function, so this is a false
    // positive.
    #[allow(dead_code)]
    const EVENT_THRESHOLD: usize = 224;

    const_assert!(mem::size_of::<Event>() == EVENT_THRESHOLD);

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
    const_assert!(mem::size_of::<MessagePollVoteAdd>() <= EVENT_THRESHOLD);
    const_assert!(mem::size_of::<MessagePollVoteRemove>() <= EVENT_THRESHOLD);
}
