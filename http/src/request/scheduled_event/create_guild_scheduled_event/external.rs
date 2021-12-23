use super::{
    super::EntityMetadataFields, CreateGuildScheduledEvent, CreateGuildScheduledEventFields,
};
use crate::{
    error::Error,
    request::{AuditLogReason, AuditLogReasonError, Request, TryIntoRequest},
    response::ResponseFuture,
};
use twilight_model::{
    datetime::Timestamp,
    scheduled_event::{EntityType, GuildScheduledEvent},
};

/// Create an external scheduled event in a guild.
#[must_use = "requests must be configured and executed"]
pub struct CreateGuildExternalScheduledEvent<'a>(CreateGuildScheduledEvent<'a>);

#[allow(clippy::needless_pass_by_value)]
impl<'a> CreateGuildExternalScheduledEvent<'a> {
    pub(crate) const fn new(
        inner: CreateGuildScheduledEvent<'a>,
        name: &'a str,
        location: &'a str,
        scheduled_start_time: &'a Timestamp,
        scheduled_end_time: &'a Timestamp,
    ) -> Self {
        Self(CreateGuildScheduledEvent {
            fields: CreateGuildScheduledEventFields {
                entity_type: Some(EntityType::External),
                entity_metadata: Some(EntityMetadataFields {
                    location: Some(location),
                }),
                name: Some(name),
                scheduled_end_time: Some(scheduled_end_time),
                scheduled_start_time: Some(scheduled_start_time),
                ..inner.fields
            },
            ..inner
        })
    }

    /// Set the description of the event.
    ///
    /// Must be between 1 and 1000 characters in length.
    pub const fn description(mut self, description: &'a str) -> Self {
        self.0.fields.description = Some(description);

        self
    }

    /// Execute the request, returning a future resolving to a [`Response`].
    ///
    /// [`Response`]: crate::response::Response
    pub fn exec(self) -> ResponseFuture<GuildScheduledEvent> {
        self.0.exec()
    }
}

impl<'a> AuditLogReason<'a> for CreateGuildExternalScheduledEvent<'a> {
    fn reason(mut self, reason: &'a str) -> Result<Self, AuditLogReasonError> {
        self.0
            .reason
            .replace(AuditLogReasonError::validate(reason)?);

        Ok(self)
    }
}

impl TryIntoRequest for CreateGuildExternalScheduledEvent<'_> {
    fn try_into_request(self) -> Result<Request, Error> {
        self.0.try_into_request()
    }
}
