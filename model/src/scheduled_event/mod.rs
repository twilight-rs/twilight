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
    /// Present on events of [`EntityType::StageInstance`] and
    /// [`EntityType::Voice`].
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
    /// Currently, only present on events of [`EntityType::External`].
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
    /// Required on events of type [`EntityType::External`]. It also may be
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
    /// Physical location of an event with type [`EntityType::External`].
    pub location: Option<String>,
}

/// Type of event.
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
#[serde(from = "u8", into = "u8")]
pub enum EntityType {
    /// Event takes place in a stage instance.
    StageInstance,
    /// Event takes place in a voice channel.
    Voice,
    /// Event takes place outside of Discord.
    External,
    /// Variant value is unknown to the library.
    Unknown(u8),
}

impl From<u8> for EntityType {
    fn from(value: u8) -> Self {
        match value {
            1 => EntityType::StageInstance,
            2 => EntityType::Voice,
            3 => EntityType::External,
            unknown => EntityType::Unknown(unknown),
        }
    }
}

impl From<EntityType> for u8 {
    fn from(value: EntityType) -> Self {
        match value {
            EntityType::StageInstance => 1,
            EntityType::Voice => 2,
            EntityType::External => 3,
            EntityType::Unknown(unknown) => unknown,
        }
    }
}

/// Privacy level of an event.
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
#[serde(from = "u8", into = "u8")]
pub enum PrivacyLevel {
    /// Event is only accessible to guild members.
    GuildOnly,
    /// Variant value is unknown to the library.
    Unknown(u8),
}

impl From<u8> for PrivacyLevel {
    fn from(value: u8) -> Self {
        match value {
            2 => PrivacyLevel::GuildOnly,
            unknown => PrivacyLevel::Unknown(unknown),
        }
    }
}

impl From<PrivacyLevel> for u8 {
    fn from(value: PrivacyLevel) -> Self {
        match value {
            PrivacyLevel::GuildOnly => 2,
            PrivacyLevel::Unknown(unknown) => unknown,
        }
    }
}

/// Status of an event.
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
#[serde(from = "u8", into = "u8")]
pub enum Status {
    /// Event is scheduled.
    ///
    /// With this status, the event can either be made active or cancelled.
    Scheduled,
    /// Event is active.
    ///
    /// With this status, the event can only be made complete.
    Active,
    /// Event is complete.
    Completed,
    /// Event is cancelled.
    Cancelled,
    /// Variant value is unknown to the library.
    Unknown(u8),
}

impl From<u8> for Status {
    fn from(value: u8) -> Self {
        match value {
            1 => Status::Scheduled,
            2 => Status::Active,
            3 => Status::Completed,
            4 => Status::Cancelled,
            unknown => Status::Unknown(unknown),
        }
    }
}

impl From<Status> for u8 {
    fn from(value: Status) -> Self {
        match value {
            Status::Scheduled => 1,
            Status::Active => 2,
            Status::Completed => 3,
            Status::Cancelled => 4,
            Status::Unknown(unknown) => unknown,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test::image_hash::{COVER, COVER_INPUT};
    use serde_test::Token;
    use std::error::Error;

    #[test]
    fn test_scheduled_event() -> Result<(), Box<dyn Error>> {
        let scheduled_start_time = Timestamp::parse("2022-01-01T00:00:00.000000+00:00")?;

        let value = GuildScheduledEvent {
            channel_id: Some(Id::new(1)),
            creator: None,
            creator_id: None,
            description: Some("this is a dance party for garfield lovers".into()),
            entity_id: Some(Id::new(2)),
            entity_metadata: None,
            entity_type: EntityType::StageInstance,
            guild_id: Id::new(3),
            id: Id::new(4),
            image: Some(COVER),
            name: "garfield dance party".into(),
            privacy_level: PrivacyLevel::GuildOnly,
            scheduled_end_time: None,
            scheduled_start_time,
            status: Status::Completed,
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
                Token::U8(2),
                Token::Str("scheduled_start_time"),
                Token::Str("2022-01-01T00:00:00.000000+00:00"),
                Token::Str("status"),
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
