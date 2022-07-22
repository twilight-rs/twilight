use super::EntityMetadataFields;
use crate::{
    client::Client,
    error::Error,
    request::{AuditLogReason, Nullable, Request, RequestBuilder, TryIntoRequest},
    response::ResponseFuture,
    routing::Route,
};
use serde::Serialize;
use twilight_model::{
    id::{
        marker::{ChannelMarker, GuildMarker, ScheduledEventMarker},
        Id,
    },
    scheduled_event::{EntityType, GuildScheduledEvent, PrivacyLevel, Status},
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
    fields: UpdateGuildScheduledEventFields<'a>,
    reason: Option<&'a str>,
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
            fields: UpdateGuildScheduledEventFields {
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
            },
            reason: None,
            scheduled_event_id,
        }
    }

    /// Set the channel ID.
    ///
    /// If `entity_type` is already [`EntityType::External`], this has no
    /// effect.
    pub fn channel_id(mut self, channel_id: Id<ChannelMarker>) -> Self {
        if let Some(entity_type) = self.fields.entity_type {
            if entity_type == EntityType::External {
                return self;
            }
        }

        self.fields.channel_id = Some(Nullable(Some(channel_id)));

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
    pub fn description(mut self, description: Option<&'a str>) -> Result<Self, ValidationError> {
        if let Some(description) = description {
            validate_scheduled_event_description(description)?;
        }

        self.fields.description = Some(Nullable(description));

        Ok(self)
    }

    /// Set the [`EntityType`] of the scheduled event.
    ///
    /// See the struct-level documentation for information about required fields
    /// for each type.
    pub fn entity_type(mut self, entity_type: EntityType) -> Self {
        if entity_type == EntityType::External {
            self.fields.channel_id = None;
        }

        self.fields.entity_type = Some(entity_type);

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
    pub const fn image(mut self, image: Option<&'a str>) -> Self {
        self.fields.image = Some(Nullable(image));

        self
    }

    /// Set the location of the external scheduled event.
    ///
    /// This only functions if the event's [`EntityType`] is [`External`].
    ///
    /// [`External`]: EntityType::External
    pub const fn location(mut self, location: Option<&'a str>) -> Self {
        self.fields.entity_metadata = Some(EntityMetadataFields { location });

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
    pub fn name(mut self, name: &'a str) -> Result<Self, ValidationError> {
        validate_scheduled_event_name(name)?;

        self.fields.name = Some(name);

        Ok(self)
    }

    /// Set the scheduled end time of the event.
    ///
    /// Required for external events.
    pub const fn scheduled_end_time(mut self, scheduled_end_time: Option<&'a Timestamp>) -> Self {
        self.fields.scheduled_end_time = Some(Nullable(scheduled_end_time));

        self
    }

    /// Set the scheduled start time of the event.
    pub const fn scheduled_start_time(mut self, scheduled_start_time: &'a Timestamp) -> Self {
        self.fields.scheduled_start_time = Some(scheduled_start_time);

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
    pub const fn status(mut self, status: Status) -> Self {
        self.fields.status = Some(status);

        self
    }

    /// Execute the request, returning a future resolving to a [`Response`].
    ///
    /// [`Response`]: crate::response::Response
    pub fn exec(self) -> ResponseFuture<GuildScheduledEvent> {
        let http = self.http;

        match self.try_into_request() {
            Ok(request) => http.request(request),
            Err(source) => ResponseFuture::error(source),
        }
    }
}

impl<'a> AuditLogReason<'a> for UpdateGuildScheduledEvent<'a> {
    fn reason(mut self, reason: &'a str) -> Result<Self, ValidationError> {
        validate_audit_reason(reason)?;

        self.reason.replace(reason);

        Ok(self)
    }
}

impl TryIntoRequest for UpdateGuildScheduledEvent<'_> {
    fn try_into_request(self) -> Result<Request, Error> {
        Request::builder(&Route::UpdateGuildScheduledEvent {
            guild_id: self.guild_id.get(),
            scheduled_event_id: self.scheduled_event_id.get(),
        })
        .json(&self.fields)
        .map(RequestBuilder::build)
    }
}
