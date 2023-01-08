//! Types for interacting with scheduled events.

mod user;

pub use self::user::GuildScheduledEventUser;

use crate::{
    id::{
        marker::{
            ChannelMarker, GuildMarker, ScheduledEventEntityMarker, ScheduledEventMarker,
            UserMarker,
        },
        Id,
    },
    user::User,
    util::{ImageHash, Timestamp},
};
use serde::{Deserialize, Serialize};

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
    /// Present on events of [`EntityType::STAGE_INSTANCE`] and
    /// [`EntityType::VOICE`].
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel_id: Option<Id<ChannelMarker>>,
    /// User object of the event's creator.
    ///
    /// Only present on events created after October 25th, 2021.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creator: Option<User>,
    /// ID of the event's creator.
    ///
    /// Only present on events created after October 25th, 2021.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creator_id: Option<Id<UserMarker>>,
    /// Description of the event.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// ID of the event's entity.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entity_id: Option<Id<ScheduledEventEntityMarker>>,
    /// Metadata of an entity, if it exists.
    ///
    /// Currently, only present on events of [`EntityType::EXTERNAL`].
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entity_metadata: Option<EntityMetadata>,
    /// Type of entity associated with the event.
    pub entity_type: EntityType,
    /// ID of the guild the event takes place in.
    pub guild_id: Id<GuildMarker>,
    /// ID of the event.
    pub id: Id<ScheduledEventMarker>,
    /// Hash of the event's cover image, if it has one.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image: Option<ImageHash>,
    /// Name of the event.
    pub name: String,
    /// Privacy level of the event.
    pub privacy_level: PrivacyLevel,
    /// Scheduled end time of the event.
    ///
    /// Required on events of type [`EntityType::EXTERNAL`]. It also may be
    /// present in other event types.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scheduled_end_time: Option<Timestamp>,
    /// Scheduled start time of the event.
    pub scheduled_start_time: Timestamp,
    /// Status of the event.
    pub status: Status,
    /// Number of users subscribed to the event.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_count: Option<u64>,
}

/// Metadata associated with an event.
#[derive(Clone, Debug, Default, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct EntityMetadata {
    /// Physical location of an event with type [`EntityType::EXTERNAL`].
    pub location: Option<String>,
}

/// Type of event.
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct EntityType(u8);

impl EntityType {
    /// Event takes place in a stage instance.
    pub const STAGE_INSTANCE: Self = Self::new(1);

    /// Event takes place in a voice channel.
    pub const VOICE: Self = Self::new(2);

    /// Event takes place outside of Discord.
    pub const EXTERNAL: Self = Self::new(3);

    /// Create a new scheduled event entity type from a dynamic value.
    ///
    /// The provided value isn't validated. Known valid values are associated
    /// constants such as [`STAGE_INSTANCE`][`Self::STAGE_INSTANCE`].
    pub const fn new(entity_type: u8) -> Self {
        Self(entity_type)
    }

    /// Retrieve the value of the scheduled event entity type.
    ///
    /// # Examples
    ///
    /// ```
    /// use twilight_model::guild::scheduled_event::EntityType;
    ///
    /// assert_eq!(2, EntityType::VOICE.get());
    /// ```
    pub const fn get(&self) -> u8 {
        self.0
    }
}

impl From<u8> for EntityType {
    fn from(value: u8) -> Self {
        Self(value)
    }
}

impl From<EntityType> for u8 {
    fn from(value: EntityType) -> Self {
        value.get()
    }
}

/// Privacy level of an event.
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct PrivacyLevel(u8);

impl PrivacyLevel {
    /// Event is only accessible to guild members.
    pub const GUILD_ONLY: Self = Self::new(2);

    /// Create a new privacy level from a dynamic value.
    ///
    /// The provided value isn't validated. Known valid values are associated
    /// constants such as [`GUILD_ONLY`][`Self::GUILD_ONLY`].
    pub const fn new(privacy_level: u8) -> Self {
        Self(privacy_level)
    }

    /// Retrieve the value of the privacy level.
    ///
    /// # Examples
    ///
    /// ```
    /// use twilight_model::guild::scheduled_event::PrivacyLevel;
    ///
    /// assert_eq!(2, PrivacyLevel::GUILD_ONLY.get());
    /// ```
    pub const fn get(&self) -> u8 {
        self.0
    }
}

impl From<u8> for PrivacyLevel {
    fn from(value: u8) -> Self {
        Self(value)
    }
}

impl From<PrivacyLevel> for u8 {
    fn from(value: PrivacyLevel) -> Self {
        value.get()
    }
}

/// Status of an event.
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct Status(u8);

impl Status {
    /// Event is scheduled.
    ///
    /// With this status, the event can either be made active or cancelled.
    pub const SCHEDULED: Self = Self::new(1);

    /// Event is active.
    ///
    /// With this status, the event can only be made complete.
    pub const ACTIVE: Self = Self::new(2);

    /// Event is complete.
    pub const COMPLETED: Self = Self::new(3);

    /// Event is cancelled.
    pub const CANCELLED: Self = Self::new(4);

    /// Create a new scheduled event status from a dynamic value.
    ///
    /// The provided value isn't validated. Known valid values are associated
    /// constants such as [`SCHEDULED`][`Self::SCHEDULED`].
    pub const fn new(scheduled_event_status: u8) -> Self {
        Self(scheduled_event_status)
    }

    /// Retrieve the value of the scheduled event status.
    ///
    /// # Examples
    ///
    /// ```
    /// use twilight_model::guild::scheduled_event::Status;
    ///
    /// assert_eq!(2, Status::ACTIVE.get());
    /// ```
    pub const fn get(&self) -> u8 {
        self.0
    }
}

impl From<u8> for Status {
    fn from(value: u8) -> Self {
        Self(value)
    }
}

impl From<Status> for u8 {
    fn from(value: Status) -> Self {
        value.get()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test::image_hash::{COVER, COVER_INPUT};
    use serde_test::Token;
    use std::error::Error;

    #[test]
    fn scheduled_event() -> Result<(), Box<dyn Error>> {
        let scheduled_start_time = Timestamp::parse("2022-01-01T00:00:00.000000+00:00")?;

        let value = GuildScheduledEvent {
            channel_id: Some(Id::new(1)),
            creator: None,
            creator_id: None,
            description: Some("this is a dance party for garfield lovers".into()),
            entity_id: Some(Id::new(2)),
            entity_metadata: None,
            entity_type: EntityType::STAGE_INSTANCE,
            guild_id: Id::new(3),
            id: Id::new(4),
            image: Some(COVER),
            name: "garfield dance party".into(),
            privacy_level: PrivacyLevel::GUILD_ONLY,
            scheduled_end_time: None,
            scheduled_start_time,
            status: Status::COMPLETED,
            user_count: Some(1),
        };

        serde_test::assert_tokens(
            &value,
            &[
                Token::Struct {
                    name: "GuildScheduledEvent",
                    len: 12,
                },
                Token::Str("channel_id"),
                Token::Some,
                Token::NewtypeStruct { name: "Id" },
                Token::Str("1"),
                Token::Str("description"),
                Token::Some,
                Token::Str("this is a dance party for garfield lovers"),
                Token::Str("entity_id"),
                Token::Some,
                Token::NewtypeStruct { name: "Id" },
                Token::Str("2"),
                Token::Str("entity_type"),
                Token::NewtypeStruct { name: "EntityType" },
                Token::U8(1),
                Token::Str("guild_id"),
                Token::NewtypeStruct { name: "Id" },
                Token::Str("3"),
                Token::Str("id"),
                Token::NewtypeStruct { name: "Id" },
                Token::Str("4"),
                Token::Str("image"),
                Token::Some,
                Token::Str(COVER_INPUT),
                Token::Str("name"),
                Token::Str("garfield dance party"),
                Token::Str("privacy_level"),
                Token::NewtypeStruct {
                    name: "PrivacyLevel",
                },
                Token::U8(2),
                Token::Str("scheduled_start_time"),
                Token::Str("2022-01-01T00:00:00.000000+00:00"),
                Token::Str("status"),
                Token::NewtypeStruct { name: "Status" },
                Token::U8(3),
                Token::Str("user_count"),
                Token::Some,
                Token::U64(1),
                Token::StructEnd,
            ],
        );

        Ok(())
    }
}
