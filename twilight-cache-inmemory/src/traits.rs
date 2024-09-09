//! Traits for implementing a [`InMemoryCache`] with custom structs.
//!
//! By default, the cache uses widely compatible default types that contain almost all
//! fields that are present in the Discord API. Fields that are never used by the user
//! will result in excess memory usage that will especially matter to big bots with a
//! lot of cached data.
//!
//! The traits in this module allow creating custom cached representations of Discord
//! API models compatible with the [`InMemoryCache`]. They may be mixed with the default
//! types provided by twilight, which also implement these traits.
//!
//! However, as Discord extends its API models with new fields or changes the types,
//! the trait definitions may change in minor crate releases to allow twilight to keep
//! up with upstream API changes. Since not all fields are required for caching logic,
//! this is not very likely to happen on a regular basis, but should be considered when
//! deciding to opt for writing custom types.
//!
//! Many traits require getters for certain types, which means they are used for caching
//! logic. However, users generally won't have to store all the fields. It is possible
//! to return `None` or empty arrays on most of the methods if the data that is accessed
//! is not stored in the custom implementation.
//!
//! [`InMemoryCache`]: crate::InMemoryCache

use crate::model::member::ComputedInteractionMember;
use std::fmt::Debug;
use twilight_model::{
    application::interaction::InteractionMember,
    channel::{
        message::{Reaction, Sticker},
        Channel, ChannelType, Message, StageInstance,
    },
    gateway::{
        payload::incoming::{GuildUpdate, MemberUpdate, MessageUpdate},
        presence::Presence,
    },
    guild::{
        scheduled_event::GuildScheduledEvent, Emoji, Guild, GuildIntegration, Member,
        PartialMember, Role,
    },
    id::{
        marker::{
            ChannelMarker, GuildMarker, RoleMarker, ScheduledEventMarker, StickerMarker, UserMarker,
        },
        Id,
    },
    user::{CurrentUser, User},
    util::{ImageHash, Timestamp},
    voice::VoiceState,
};
#[cfg(feature = "permission-calculator")]
use twilight_model::{channel::permission_overwrite::PermissionOverwrite, guild::Permissions};

/// Super-trait for the generic cached representations of Discord API models.
pub trait CacheableModels: Clone + Debug {
    /// The cached [`Channel`] model representation.
    type Channel: CacheableChannel;
    /// The cached [`CurrentUser`] model representation.
    type CurrentUser: CacheableCurrentUser;
    /// The cached [`Emoji`] model representation.
    type Emoji: CacheableEmoji;
    /// The cached [`Guild`] model representation.
    type Guild: CacheableGuild;
    /// The cached [`GuildIntegration`] model representation.
    type GuildIntegration: CacheableGuildIntegration;
    /// The cached [`GuildScheduledEvent` model representation.
    type GuildScheduledEvent: CacheableGuildScheduledEvent;
    /// The cached [`Member`] model representation.
    type Member: CacheableMember;
    /// The cached [`Message`] model representation.
    type Message: CacheableMessage;
    /// The cached [`Presence`] model representation.
    type Presence: CacheablePresence;
    /// The cached [`Role`] model representation.
    type Role: CacheableRole;
    /// The cached [`StageInstance`] model representation.
    type StageInstance: CacheableStageInstance;
    /// The cached [`Sticker`] model representation.
    type Sticker: CacheableSticker;
    /// The cached [`User`] model representation.
    type User: CacheableUser;
    /// The cached [`VoiceState`] model representation.
    type VoiceState: CacheableVoiceState;
}

/// Trait for a generic cached representation of a [`Member`].
pub trait CacheableMember:
    From<Member>
    + From<ComputedInteractionMember>
    + From<(Id<UserMarker>, PartialMember)>
    + PartialEq<Member>
    + PartialEq<PartialMember>
    + PartialEq<InteractionMember>
    + PartialEq<Self>
    + Clone
    + Debug
{
    /// Roles of this member.
    fn roles(&self) -> &[Id<RoleMarker>];

    /// Timestamp until which this member's communication is disabled.
    #[cfg(feature = "permission-calculator")]
    fn communication_disabled_until(&self) -> Option<Timestamp>;

    /// Avatar of this member.
    fn avatar(&self) -> Option<ImageHash>;

    /// Whether this member is deafened.
    fn deaf(&self) -> Option<bool>;

    /// Whether this member is muted.
    fn mute(&self) -> Option<bool>;

    /// Update the cached data with a [`MemberUpdate`] event.
    fn update_with_member_update(&mut self, member_update: &MemberUpdate);
}

/// Trait for a generic cached representation of a [`Role`].
pub trait CacheableRole: From<Role> + PartialEq<Role> + PartialEq<Self> + Clone + Debug {
    /// Role's position in the guild roles.
    fn position(&self) -> i64;

    /// ID of the role.
    fn id(&self) -> Id<RoleMarker>;

    /// Permissions granted to members with the role.
    #[cfg(feature = "permission-calculator")]
    fn permissions(&self) -> Permissions;
}

impl CacheableRole for Role {
    fn position(&self) -> i64 {
        self.position
    }

    fn id(&self) -> Id<RoleMarker> {
        self.id
    }

    #[cfg(feature = "permission-calculator")]
    fn permissions(&self) -> Permissions {
        self.permissions
    }
}

/// Trait for a generic cached representation of a [`Channel`].
pub trait CacheableChannel:
    From<Channel> + PartialEq<Channel> + PartialEq<Self> + Clone + Debug
{
    /// ID of the guild this channel belongs to.
    fn guild_id(&self) -> Option<Id<GuildMarker>>;

    /// Type of the channel.
    fn kind(&self) -> ChannelType;

    /// ID of the parent channel if this is a thread.
    #[cfg(feature = "permission-calculator")]
    fn parent_id(&self) -> Option<Id<ChannelMarker>>;

    /// ID of the channel.
    fn id(&self) -> Id<ChannelMarker>;

    /// Permission overwrites for the channel.
    #[cfg(feature = "permission-calculator")]
    fn permission_overwrites(&self) -> Option<&[PermissionOverwrite]>;

    /// Set the last pin timestamp to a new timestamp.
    fn set_last_pin_timestamp(&mut self, timestamp: Option<Timestamp>);
}

impl CacheableChannel for Channel {
    fn guild_id(&self) -> Option<Id<GuildMarker>> {
        self.guild_id
    }

    fn kind(&self) -> ChannelType {
        self.kind
    }

    #[cfg(feature = "permission-calculator")]
    fn parent_id(&self) -> Option<Id<ChannelMarker>> {
        self.parent_id
    }

    fn id(&self) -> Id<ChannelMarker> {
        self.id
    }

    #[cfg(feature = "permission-calculator")]
    fn permission_overwrites(&self) -> Option<&[PermissionOverwrite]> {
        self.permission_overwrites.as_deref()
    }

    fn set_last_pin_timestamp(&mut self, timestamp: Option<Timestamp>) {
        self.last_pin_timestamp = timestamp;
    }
}

/// Trait for a generic cached representation of a [`Guild`].
pub trait CacheableGuild: From<Guild> + PartialEq<Guild> + PartialEq<Self> + Clone + Debug {
    /// ID of the guild.
    fn id(&self) -> Id<GuildMarker>;

    /// ID of the guild's owner.
    #[cfg(feature = "permission-calculator")]
    fn owner_id(&self) -> Id<UserMarker>;

    /// Set the guild's unavailable flag.
    fn set_unavailable(&mut self, unavailable: bool);

    /// Update the cached data with a [`GuildUpdate`] event. Fields containing other
    /// cached structures such as channels are cleared prior.
    fn update_with_guild_update(&mut self, guild_update: &GuildUpdate);

    /// Increase the guild member count.
    fn increase_member_count(&mut self, amount: u64);

    /// Decrease the guild member count.
    fn decrease_member_count(&mut self, amount: u64);
}

/// Trait for a generic cached representation of a [`VoiceState`].
pub trait CacheableVoiceState:
    From<(Id<ChannelMarker>, Id<GuildMarker>, VoiceState)>
    + PartialEq<VoiceState>
    + PartialEq<Self>
    + Clone
    + Debug
{
    /// ID of the channel this voice state belongs to.
    fn channel_id(&self) -> Id<ChannelMarker>;
}

/// Trait for a generic cached representation of a [`Message`].
pub trait CacheableMessage:
    From<Message> + PartialEq<Message> + PartialEq<Self> + Clone + Debug
{
    /// Update the cached data with a [`MessageUpdate`] event.
    fn update_with_message_update(&mut self, message_update: &MessageUpdate);

    /// Reactions added to this message.
    fn reactions(&self) -> &[Reaction];

    /// Mutable getter for reactions added to this message.
    fn reactions_mut(&mut self) -> &mut [Reaction];

    /// Retain all reactions to this message matching a predicate, removing non-matching ones.
    fn retain_reactions(&mut self, f: impl FnMut(&Reaction) -> bool);

    /// Clear all reactions to this message.
    fn clear_reactions(&mut self);

    /// Add a reaction to this message.
    fn add_reaction(&mut self, reaction: Reaction);

    /// Remove a reaction from this message.
    fn remove_reaction(&mut self, idx: usize);
}

/// Trait for a generic cached representation of a [`CurrentUser`].
pub trait CacheableCurrentUser:
    From<CurrentUser> + PartialEq<CurrentUser> + PartialEq<Self> + Clone + Debug
{
    /// ID of the user.
    fn id(&self) -> Id<UserMarker>;
}

impl CacheableCurrentUser for CurrentUser {
    fn id(&self) -> Id<UserMarker> {
        self.id
    }
}

/// Trait for a generic cached representation of a [`Sticker`].
pub trait CacheableSticker:
    From<Sticker> + PartialEq<Sticker> + PartialEq<Self> + Clone + Debug
{
    /// ID of the sticker.
    fn id(&self) -> Id<StickerMarker>;
}

/// Trait for a generic cached representation of a [`Emoji`].
pub trait CacheableEmoji: From<Emoji> + PartialEq<Emoji> + PartialEq<Self> + Clone + Debug {}

/// Trait for a generic cached representation of a [`GuildIntegration`].
pub trait CacheableGuildIntegration:
    From<GuildIntegration> + PartialEq<GuildIntegration> + PartialEq<Self> + Clone + Debug
{
}

impl CacheableGuildIntegration for GuildIntegration {}

/// Trait for a generic cached representation of a [`Presence`].
pub trait CacheablePresence:
    From<Presence> + PartialEq<Presence> + PartialEq<Self> + Clone + Debug
{
}

/// Trait for a generic cached representation of a [`StageInstance`].
pub trait CacheableStageInstance:
    From<StageInstance> + PartialEq<StageInstance> + PartialEq<Self> + Clone + Debug
{
}

impl CacheableStageInstance for StageInstance {}

/// Trait for a generic cached representation of a [`User`].
pub trait CacheableUser: From<User> + PartialEq<User> + PartialEq<Self> + Clone + Debug {}

impl CacheableUser for User {}

/// Trait for a generic cached representation of a [`GuildScheduledEvent`].
pub trait CacheableGuildScheduledEvent:
    From<GuildScheduledEvent> + PartialEq<GuildScheduledEvent> + PartialEq<Self> + Clone + Debug
{
    /// Add a user to an event.
    fn add_user(
        &mut self,
        guild_id: Id<GuildMarker>,
        event_id: Id<ScheduledEventMarker>,
        user_id: Id<UserMarker>,
    );

    /// Remove a user from an event.
    fn remove_user(
        &mut self,
        guild_id: Id<GuildMarker>,
        event_id: Id<ScheduledEventMarker>,
        user_id: Id<UserMarker>,
    );
}

impl CacheableGuildScheduledEvent for GuildScheduledEvent {
    fn add_user(
        &mut self,
        _guild_id: Id<GuildMarker>,
        _event_id: Id<ScheduledEventMarker>,
        _user_id: Id<UserMarker>,
    ) {
        self.user_count = self.user_count.map(|count| count.saturating_add(1));
    }

    fn remove_user(
        &mut self,
        _guild_id: Id<GuildMarker>,
        _event_id: Id<ScheduledEventMarker>,
        _user_id: Id<UserMarker>,
    ) {
        self.user_count = self.user_count.map(|count| count.saturating_sub(1));
    }
}
