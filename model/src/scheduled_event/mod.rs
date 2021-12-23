//! Types for interacting with scheduled events.

mod user;

pub use self::user::GuildScheduledEventUser;

use crate::{
    datetime::Timestamp,
    id::{
        marker::{
            ChannelMarker, GuildMarker, ScheduledEventEntityMarker, ScheduledEventMarker,
            UserMarker,
        },
        Id,
    },
    user::User,
};
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

/// Representation of a scheduled event.
///
/// For events created before October 25th, 2021, [`creator`] and [`creator_id`]
/// will be [`None`].
///
/// [`creator`]: Self::creator
/// [`creator_id`]: Self::creator_id
#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct GuildScheduledEvent {
    /// ID of the stage or voice channel if there is one.
    ///
    /// Present on events of [`EntityType::StageInstance`] and
    /// [`EntityType::Voice`].
    pub channel_id: Option<Id<ChannelMarker>>,
    /// User object of the event's creator.
    ///
    /// Only present on events created after October 25th, 2021.
    pub creator: Option<User>,
    /// ID of the event's creator.
    ///
    /// Only present on events created after October 25th, 2021.
    pub creator_id: Option<Id<UserMarker>>,
    /// Description of the event.
    pub description: Option<String>,
    /// ID of the event's entity.
    pub entity_id: Option<Id<ScheduledEventEntityMarker>>,
    /// Metadata of an entity, if it exists.
    ///
    /// Currently, only present on events of [`EntityType::External`].
    pub entity_metadata: Option<EntityMetadata>,
    /// Type of entity associated with the event.
    pub entity_type: EntityType,
    /// ID of the guild the event takes place in.
    pub guild_id: Id<GuildMarker>,
    /// ID of the event.
    pub id: Id<ScheduledEventMarker>,
    /// Name of the event.
    pub name: String,
    /// Privacy level of the event.
    pub privacy_level: PrivacyLevel,
    /// Scheduled end time of the event.
    ///
    /// Required on events of type [`EntityType::External`]. It also may be
    /// present in other event types.
    pub scheduled_end_time: Option<Timestamp>,
    /// Scheduled start time of the event.
    pub scheduled_start_time: Timestamp,
    /// Status of the event.
    pub status: Status,
    /// Number of users subscribed to the event.
    pub user_count: Option<u64>,
}

/// Metadata associated with an event.
#[derive(Clone, Debug, Default, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct EntityMetadata {
    /// Physical location of an event with type [`EntityType::External`].
    pub location: Option<String>,
}

/// Type of event.
#[derive(Clone, Copy, Debug, Deserialize_repr, Eq, Hash, PartialEq, Serialize_repr)]
#[repr(u8)]
pub enum EntityType {
    /// Event takes place in a stage instance.
    StageInstance = 1,
    /// Event takes place in a voice channel.
    Voice = 2,
    /// Event takes place outside of Discord.
    External = 3,
}

/// Privacy level of an event.
#[derive(Clone, Copy, Debug, Deserialize_repr, Eq, Hash, PartialEq, Serialize_repr)]
#[repr(u8)]
pub enum PrivacyLevel {
    /// Event is only accessible to guild members.
    GuildOnly = 2,
}

/// Status of an event.
#[derive(Clone, Copy, Debug, Deserialize_repr, Eq, Hash, PartialEq, Serialize_repr)]
#[repr(u8)]
pub enum Status {
    /// Event is scheduled.
    ///
    /// With this status, the event can either be made active or cancelled.
    Scheduled = 1,
    /// Event is active.
    ///
    /// With this status, the event can only be made complete.
    Active = 2,
    /// Event is complete.
    Completed = 3,
    /// Event is cancelled.
    Cancelled = 4,
}
