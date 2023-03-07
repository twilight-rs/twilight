use super::{CreateGuildScheduledEvent, CreateGuildScheduledEventFields};
use crate::{
    error::Error,
    request::{AuditLogReason, Request, TryIntoRequest},
    response::{Response, ResponseFuture},
};
use std::future::IntoFuture;
use twilight_model::{
    guild::scheduled_event::{EntityType, GuildScheduledEvent},
    id::{marker::ChannelMarker, Id},
    util::Timestamp,
};
use twilight_validate::request::{
    audit_reason as validate_audit_reason,
    scheduled_event_description as validate_scheduled_event_description,
};

/// Create a voice channel scheduled event in a guild.
#[must_use = "requests must be configured and executed"]
pub struct CreateGuildVoiceScheduledEvent<'a>(CreateGuildScheduledEvent<'a>);

impl<'a> CreateGuildVoiceScheduledEvent<'a> {
    pub(crate) fn new(
        mut inner: CreateGuildScheduledEvent<'a>,
        channel_id: Id<ChannelMarker>,
        name: &'a str,
        scheduled_start_time: &'a Timestamp,
    ) -> Self {
        inner.fields = inner.fields.map(|fields| CreateGuildScheduledEventFields {
            channel_id: Some(channel_id),
            entity_type: Some(EntityType::Voice),
            name: Some(name),
            scheduled_start_time: Some(scheduled_start_time),
            ..fields
        });

        Self(inner)
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
    pub fn description(mut self, description: &'a str) -> Self {
        self.0.fields = self.0.fields.and_then(|mut fields| {
            validate_scheduled_event_description(description)?;
            fields.description.replace(description);

            Ok(fields)
        });

        self
    }

    /// Set the cover image of the event.
    ///
    /// This must be a Data URI, in the form of
    /// `data:image/{type};base64,{data}` where `{type}` is the image MIME type
    /// and `{data}` is the base64-encoded image. See [Discord Docs/Image Data].
    ///
    /// [Discord Docs/Image Data]: https://discord.com/developers/docs/reference#image-data
    pub fn image(mut self, image: &'a str) -> Self {
        self.0.fields = self.0.fields.map(|mut fields| {
            fields.image = Some(image);

            fields
        });

        self
    }

    /// Set the scheduled end time of the event.
    ///
    /// This is not a required field for voice channel events.
    pub fn scheduled_end_time(mut self, scheduled_end_time: &'a Timestamp) -> Self {
        self.0.fields = self.0.fields.map(|mut fields| {
            fields.scheduled_end_time = Some(scheduled_end_time);

            fields
        });

        self
    }
}

impl<'a> AuditLogReason<'a> for CreateGuildVoiceScheduledEvent<'a> {
    fn reason(mut self, reason: &'a str) -> Self {
        self.0.reason = validate_audit_reason(reason).and(Ok(Some(reason)));

        self
    }
}

impl IntoFuture for CreateGuildVoiceScheduledEvent<'_> {
    type Output = Result<Response<GuildScheduledEvent>, Error>;

    type IntoFuture = ResponseFuture<GuildScheduledEvent>;

    fn into_future(self) -> Self::IntoFuture {
        self.0.exec()
    }
}

impl TryIntoRequest for CreateGuildVoiceScheduledEvent<'_> {
    fn try_into_request(self) -> Result<Request, Error> {
        self.0.try_into_request()
    }
}
