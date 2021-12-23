mod external;
mod stage_instance;
mod voice;

pub use self::{
    external::CreateGuildExternalScheduledEvent,
    stage_instance::CreateGuildStageInstanceScheduledEvent, voice::CreateGuildVoiceScheduledEvent,
};

use super::EntityMetadataFields;
use crate::{
    client::Client,
    error::Error,
    request::{AuditLogReason, AuditLogReasonError, Request, RequestBuilder, TryIntoRequest},
    response::ResponseFuture,
    routing::Route,
};
use serde::Serialize;
use twilight_model::{
    datetime::Timestamp,
    id::{
        marker::{ChannelMarker, GuildMarker},
        Id,
    },
    scheduled_event::{EntityType, GuildScheduledEvent, PrivacyLevel},
};

#[derive(Serialize)]
struct CreateGuildScheduledEventFields<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    channel_id: Option<Id<ChannelMarker>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    entity_metadata: Option<EntityMetadataFields<'a>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    entity_type: Option<EntityType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    privacy_level: Option<PrivacyLevel>,
    #[serde(skip_serializing_if = "Option::is_none")]
    scheduled_end_time: Option<&'a Timestamp>,
    #[serde(skip_serializing_if = "Option::is_none")]
    scheduled_start_time: Option<&'a Timestamp>,
}

/// Create a scheduled event in a guild.
///
/// Once a guild is selected, you must choose one of three event types to
/// create. The request builders will ensure you provide the correct data to
/// Discord. See [the Discord docs] for more information on which events require
/// which fields.
///
/// The name must be between 1 and 100 characters in length. For external
/// events, the location must be between 1 and 100 characters in length.
///
/// # Examples
///
/// Create an event in a stage instance:
///
/// ```no_run
/// # use twilight_http::Client;
/// use twilight_model::{datetime::Timestamp, id::Id};
/// # #[tokio::main]
/// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
/// # let client = Client::new("token".to_owned());
/// let guild_id = Id::new(1).expect("non zero");
/// let channel_id = Id::new(2).expect("non zero");
/// let garfield_start_time = Timestamp::parse("2022-01-01T14:00:00Z")?;
///
/// client
///     .create_guild_scheduled_event(guild_id)
///     .stage_instance(
///         channel_id,
///         "Garfield Appreciation Hour",
///         &garfield_start_time
///     )
///     .description("Discuss: How important is Garfield to You?")
///     .exec()
///     .await?;
/// # Ok(()) }
/// ```
///
/// Create an external event:
///
/// ```no_run
/// # use twilight_http::Client;
/// use twilight_model::{datetime::Timestamp, id::Id};
/// # #[tokio::main]
/// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
/// # let client = Client::new("token".to_owned());
/// let guild_id = Id::new(1).expect("non zero");
/// let garfield_con_start_time = Timestamp::parse("2022-01-04T08:00:00Z")?;
/// let garfield_con_end_time = Timestamp::parse("2022-01-06T17:00:00Z")?;
///
/// client
///     .create_guild_scheduled_event(guild_id)
///     .external(
///         "Garfield Con 2022",
///         "Baltimore Convention Center",
///         &garfield_con_start_time,
///         &garfield_con_end_time
///     )
///     .description("In a spiritual successor to BronyCon, Garfield fans from \
/// around the globe celebrate all things related to the loveable cat.")
///     .exec()
///     .await?;
/// # Ok(()) }
/// ```
///
/// [the Discord docs]: https://discord.com/developers/docs/resources/guild-scheduled-event#create-guild-scheduled-event
pub struct CreateGuildScheduledEvent<'a> {
    guild_id: Id<GuildMarker>,
    http: &'a Client,
    fields: CreateGuildScheduledEventFields<'a>,
    reason: Option<&'a str>,
}

impl<'a> CreateGuildScheduledEvent<'a> {
    pub(crate) const fn new(http: &'a Client, guild_id: Id<GuildMarker>) -> Self {
        Self {
            guild_id,
            http,
            fields: CreateGuildScheduledEventFields {
                channel_id: None,
                description: None,
                entity_metadata: None,
                entity_type: None,
                name: None,
                privacy_level: None,
                scheduled_end_time: None,
                scheduled_start_time: None,
            },
            reason: None,
        }
    }

    /// Create an external scheduled event in a guild.
    pub const fn external(
        self,
        name: &'a str,
        location: &'a str,
        scheduled_start_time: &'a Timestamp,
        scheduled_end_time: &'a Timestamp,
    ) -> CreateGuildExternalScheduledEvent<'a> {
        CreateGuildExternalScheduledEvent::new(
            self,
            name,
            location,
            scheduled_start_time,
            scheduled_end_time,
        )
    }

    /// Create a stage instance scheduled event in a guild.
    pub const fn stage_instance(
        self,
        channel_id: Id<ChannelMarker>,
        name: &'a str,
        scheduled_start_time: &'a Timestamp,
    ) -> CreateGuildStageInstanceScheduledEvent<'a> {
        CreateGuildStageInstanceScheduledEvent::new(self, channel_id, name, scheduled_start_time)
    }

    /// Create a voice channel scheduled event in a guild.
    pub const fn voice(
        self,
        channel_id: Id<ChannelMarker>,
        name: &'a str,
        scheduled_start_time: &'a Timestamp,
    ) -> CreateGuildVoiceScheduledEvent<'a> {
        CreateGuildVoiceScheduledEvent::new(self, channel_id, name, scheduled_start_time)
    }

    fn exec(self) -> ResponseFuture<GuildScheduledEvent> {
        let http = self.http;

        match self.try_into_request() {
            Ok(request) => http.request(request),
            Err(source) => ResponseFuture::error(source),
        }
    }
}

impl<'a> AuditLogReason<'a> for CreateGuildScheduledEvent<'a> {
    fn reason(mut self, reason: &'a str) -> Result<Self, AuditLogReasonError> {
        self.reason.replace(AuditLogReasonError::validate(reason)?);

        Ok(self)
    }
}

impl TryIntoRequest for CreateGuildScheduledEvent<'_> {
    fn try_into_request(self) -> Result<Request, Error> {
        Request::builder(&Route::CreateGuildScheduledEvent {
            guild_id: self.guild_id.get(),
        })
        .json(&self.fields)
        .map(RequestBuilder::build)
    }
}
