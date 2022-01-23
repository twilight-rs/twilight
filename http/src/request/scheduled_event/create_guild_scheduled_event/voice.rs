use super::{CreateGuildScheduledEvent, CreateGuildScheduledEventFields};
use crate::{
    error::Error,
    request::{AuditLogReason, AuditLogReasonError, Request, TryIntoRequest},
    response::ResponseFuture,
};
use twilight_model::{
    datetime::Timestamp,
    id::{marker::ChannelMarker, Id},
    scheduled_event::{EntityType, GuildScheduledEvent},
};
use twilight_validate::request::{
    scheduled_event_description as validate_scheduled_event_description, ValidationError,
};

/// Create a voice channel scheduled event in a guild.
#[must_use = "requests must be configured and executed"]
pub struct CreateGuildVoiceScheduledEvent<'a>(CreateGuildScheduledEvent<'a>);

impl<'a> CreateGuildVoiceScheduledEvent<'a> {
    pub(crate) const fn new(
        inner: CreateGuildScheduledEvent<'a>,
        channel_id: Id<ChannelMarker>,
        name: &'a str,
        scheduled_start_time: &'a Timestamp,
    ) -> Self {
        Self(CreateGuildScheduledEvent {
            fields: CreateGuildScheduledEventFields {
                channel_id: Some(channel_id),
                entity_type: Some(EntityType::Voice),
                name: Some(name),
                scheduled_start_time: Some(scheduled_start_time),
                ..inner.fields
            },
            ..inner
        })
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
    pub fn description(mut self, description: &'a str) -> Result<Self, ValidationError> {
        validate_scheduled_event_description(description)?;

        self.0.fields.description = Some(description);

        Ok(self)
    }

    /// Set the scheduled end time of the event.
    ///
    /// This is not a required field for voice channel events.
    pub const fn scheduled_end_time(mut self, scheduled_end_time: &'a Timestamp) -> Self {
        self.0.fields.scheduled_end_time = Some(scheduled_end_time);

        self
    }

    /// Execute the request, returning a future resolving to a [`Response`].
    ///
    /// [`Response`]: crate::response::Response
    pub fn exec(self) -> ResponseFuture<GuildScheduledEvent> {
        self.0.exec()
    }
}

impl<'a> AuditLogReason<'a> for CreateGuildVoiceScheduledEvent<'a> {
    fn reason(mut self, reason: &'a str) -> Result<Self, AuditLogReasonError> {
        self.0
            .reason
            .replace(AuditLogReasonError::validate(reason)?);

        Ok(self)
    }
}

impl TryIntoRequest for CreateGuildVoiceScheduledEvent<'_> {
    fn try_into_request(self) -> Result<Request, Error> {
        self.0.try_into_request()
    }
}
