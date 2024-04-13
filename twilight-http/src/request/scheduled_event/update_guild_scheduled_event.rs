use super::EntityMetadataFields;
use crate::{
    client::Client,
    error::Error,
    request::{AuditLogReason, Nullable, Request, TryIntoRequest},
    response::{Response, ResponseFuture},
    routing::Route,
};
use serde::Serialize;
use std::future::IntoFuture;
use twilight_model::{
    guild::scheduled_event::{EntityType, GuildScheduledEvent, PrivacyLevel, Status},
    id::{
        marker::{ChannelMarker, GuildMarker, ScheduledEventMarker},
        Id,
    },
    util::Timestamp,
};
use twilight_validate::request::{
    audit_reason as validate_audit_reason,
    scheduled_event_description as validate_scheduled_event_description,
    scheduled_event_name as validate_scheduled_event_name, ValidationError,
};

#[derive(Serialize)]
struct UpdateGuildScheduledEventFields<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    channel_id: Option<Nullable<Id<ChannelMarker>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<Nullable<&'a str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    entity_metadata: Option<EntityMetadataFields<'a>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    entity_type: Option<EntityType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    image: Option<Nullable<&'a str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    privacy_level: Option<PrivacyLevel>,
    #[serde(skip_serializing_if = "Option::is_none")]
    scheduled_end_time: Option<Nullable<&'a Timestamp>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    scheduled_start_time: Option<&'a Timestamp>,
    #[serde(skip_serializing_if = "Option::is_none")]
    status: Option<Status>,
}

/// Update a scheduled event in a guild.
///
/// This endpoint supports changing the type of event. When changing the entity
/// type to either [`EntityType::StageInstance`] or [`EntityType::Voice`], an
/// [`Id<ChannelMarker>`] must be provided if it does not already exist.
///
/// When changing the entity type to [`EntityType::External`], the `channel_id`
/// field is cleared and the [`channel_id`] method has no effect.  Additionally,
/// you must set a location with [`location`].
///
/// [`channel_id`]: UpdateGuildScheduledEvent::channel_id
/// [`location`]: UpdateGuildScheduledEvent::location
/// [`channel_id`]: UpdateGuildScheduledEvent::channel_id
/// [`location`]: UpdateGuildScheduledEvent::location
#[must_use = "requests must be configured and executed"]
pub struct UpdateGuildScheduledEvent<'a> {
    guild_id: Id<GuildMarker>,
    http: &'a Client,
    fields: Result<UpdateGuildScheduledEventFields<'a>, ValidationError>,
    reason: Result<Option<&'a str>, ValidationError>,
    scheduled_event_id: Id<ScheduledEventMarker>,
}

impl<'a> UpdateGuildScheduledEvent<'a> {
    pub(crate) const fn new(
        http: &'a Client,
        guild_id: Id<GuildMarker>,
        scheduled_event_id: Id<ScheduledEventMarker>,
    ) -> Self {
        Self {
            guild_id,
            http,
            fields: Ok(UpdateGuildScheduledEventFields {
                channel_id: None,
                description: None,
                entity_metadata: None,
                entity_type: None,
                image: None,
                name: None,
                privacy_level: None,
                scheduled_end_time: None,
                scheduled_start_time: None,
                status: None,
            }),
            reason: Ok(None),
            scheduled_event_id,
        }
    }

    /// Set the channel ID.
    ///
    /// If `entity_type` is already [`EntityType::External`], this has no
    /// effect.
    pub fn channel_id(mut self, channel_id: Id<ChannelMarker>) -> Self {
        if let Ok(fields) = self.fields.as_mut() {
            if fields.entity_type != Some(EntityType::External) {
                fields.channel_id = Some(Nullable(Some(channel_id)));
            }
        }

        self
    }

    /// Set the description of the event.
    ///
    /// Must be between 1 and 1000 characters in length.
    ///
    /// # Errors
    ///
    /// Returns an error of type [`ScheduledEventDescription`] if the
    /// description is invalid.
    ///
    /// [`ScheduledEventDescription`]: twilight_validate::request::ValidationErrorType::ScheduledEventDescription
    pub fn description(mut self, description: Option<&'a str>) -> Self {
        self.fields = self.fields.and_then(|mut fields| {
            if let Some(description) = description {
                validate_scheduled_event_description(description)?;
            }

            fields.description = Some(Nullable(description));

            Ok(fields)
        });

        self
    }

    /// Set the [`EntityType`] of the scheduled event.
    ///
    /// See the struct-level documentation for information about required fields
    /// for each type.
    pub fn entity_type(mut self, entity_type: EntityType) -> Self {
        self.fields = self.fields.map(|mut fields| {
            if entity_type == EntityType::External {
                fields.channel_id = None;
            }

            fields.entity_type = Some(entity_type);

            fields
        });

        self
    }

    /// Set the cover image of the event.
    ///
    /// Pass [`None`] to clear the image.
    ///
    /// This must be a Data URI, in the form of
    /// `data:image/{type};base64,{data}` where `{type}` is the image MIME type
    /// and `{data}` is the base64-encoded image. See [Discord Docs/Image Data].
    ///
    /// [Discord Docs/Image Data]: https://discord.com/developers/docs/reference#image-data
    pub fn image(mut self, image: Option<&'a str>) -> Self {
        self.fields = self.fields.map(|mut fields| {
            fields.image = Some(Nullable(image));

            fields
        });

        self
    }

    /// Set the location of the external scheduled event.
    ///
    /// This only functions if the event's [`EntityType`] is [`External`].
    ///
    /// [`External`]: EntityType::External
    pub fn location(mut self, location: Option<&'a str>) -> Self {
        self.fields = self.fields.map(|mut fields| {
            fields.entity_metadata = Some(EntityMetadataFields { location });

            fields
        });

        self
    }

    /// Set the name of the event.
    ///
    /// Must be between 1 and 100 characters in length.
    ///
    /// # Errors
    ///
    /// Returns an error of type [`ScheduledEventName`] if the name is invalid.
    ///
    /// [`ScheduledEventName`]: twilight_validate::request::ValidationErrorType::ScheduledEventName
    pub fn name(mut self, name: &'a str) -> Self {
        self.fields = self.fields.and_then(|mut fields| {
            validate_scheduled_event_name(name)?;

            fields.name = Some(name);

            Ok(fields)
        });

        self
    }

    /// Set the scheduled end time of the event.
    ///
    /// Required for external events.
    pub fn scheduled_end_time(mut self, scheduled_end_time: Option<&'a Timestamp>) -> Self {
        self.fields = self.fields.map(|mut fields| {
            fields.scheduled_end_time = Some(Nullable(scheduled_end_time));

            fields
        });

        self
    }

    /// Set the scheduled start time of the event.
    pub fn scheduled_start_time(mut self, scheduled_start_time: &'a Timestamp) -> Self {
        self.fields = self.fields.map(|mut fields| {
            fields.scheduled_start_time = Some(scheduled_start_time);

            fields
        });

        self
    }

    /// Set the status of the event.
    ///
    /// If an event is currently [`Scheduled`], it can only be set to [`Active`]
    /// or [`Cancelled`]. If it is currently [`Active`], it can only be set to
    /// [`Completed`]. Otherwise, the status can not be updated.
    ///
    /// [`Active`]: Status::Active
    /// [`Cancelled`]: Status::Cancelled
    /// [`Completed`]: Status::Completed
    /// [`Scheduled`]: Status::Scheduled
    pub fn status(mut self, status: Status) -> Self {
        self.fields = self.fields.map(|mut fields| {
            fields.status = Some(status);

            fields
        });

        self
    }
}

impl<'a> AuditLogReason<'a> for UpdateGuildScheduledEvent<'a> {
    fn reason(mut self, reason: &'a str) -> Self {
        self.reason = validate_audit_reason(reason).and(Ok(Some(reason)));

        self
    }
}

impl IntoFuture for UpdateGuildScheduledEvent<'_> {
    type Output = Result<Response<GuildScheduledEvent>, Error>;

    type IntoFuture = ResponseFuture<GuildScheduledEvent>;

    fn into_future(self) -> Self::IntoFuture {
        let http = self.http;

        match self.try_into_request() {
            Ok(request) => http.request(request),
            Err(source) => ResponseFuture::error(source),
        }
    }
}

impl TryIntoRequest for UpdateGuildScheduledEvent<'_> {
    fn try_into_request(self) -> Result<Request, Error> {
        let fields = self.fields.map_err(Error::validation)?;

        Request::builder(&Route::UpdateGuildScheduledEvent {
            guild_id: self.guild_id.get(),
            scheduled_event_id: self.scheduled_event_id.get(),
        })
        .json(&fields)
        .build()
    }
}
