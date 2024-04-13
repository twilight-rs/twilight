use crate::{
    client::Client,
    error::Error,
    request::{self, AuditLogReason, Request, TryIntoRequest},
    response::{marker::EmptyBody, Response, ResponseFuture},
    routing::Route,
};
use serde::Serialize;
use std::future::IntoFuture;
use twilight_model::id::{
    marker::{ChannelMarker, MessageMarker},
    Id,
};
use twilight_validate::{
    channel::{bulk_delete_messages as validate_bulk_delete_messages, ChannelValidationError},
    request::{audit_reason as validate_audit_reason, ValidationError},
};

#[derive(Serialize)]
struct DeleteMessagesFields<'a> {
    messages: &'a [Id<MessageMarker>],
}

/// Delete messages by [`Id<ChannelMarker>`] and a list of [`Id<MessageMarker>`]s.
///
/// The number of message IDs must be between 2 and 100. If the supplied message
/// IDs are invalid, they still count towards the lower and upper limits. This
/// method will not delete messages older than two weeks. See
/// [Discord Docs/Bulk Delete Messages].
///
/// [Discord Docs/Bulk Delete Messages]: https://discord.com/developers/docs/resources/channel#bulk-delete-messages
#[must_use = "requests must be configured and executed"]
pub struct DeleteMessages<'a> {
    channel_id: Id<ChannelMarker>,
    fields: Result<DeleteMessagesFields<'a>, ChannelValidationError>,
    http: &'a Client,
    reason: Result<Option<&'a str>, ValidationError>,
}

impl<'a> DeleteMessages<'a> {
    pub(crate) fn new(
        http: &'a Client,
        channel_id: Id<ChannelMarker>,
        messages: &'a [Id<MessageMarker>],
    ) -> Self {
        let fields = Ok(DeleteMessagesFields { messages }).and_then(|fields| {
            validate_bulk_delete_messages(messages.len())?;

            Ok(fields)
        });

        Self {
            channel_id,
            fields,
            http,
            reason: Ok(None),
        }
    }
}

impl<'a> AuditLogReason<'a> for DeleteMessages<'a> {
    fn reason(mut self, reason: &'a str) -> Self {
        self.reason = validate_audit_reason(reason).and(Ok(Some(reason)));

        self
    }
}

impl IntoFuture for DeleteMessages<'_> {
    type Output = Result<Response<EmptyBody>, Error>;

    type IntoFuture = ResponseFuture<EmptyBody>;

    fn into_future(self) -> Self::IntoFuture {
        let http = self.http;

        match self.try_into_request() {
            Ok(request) => http.request(request),
            Err(source) => ResponseFuture::error(source),
        }
    }
}

impl TryIntoRequest for DeleteMessages<'_> {
    fn try_into_request(self) -> Result<Request, Error> {
        let fields = self.fields.map_err(Error::validation)?;
        let mut request = Request::builder(&Route::DeleteMessages {
            channel_id: self.channel_id.get(),
        })
        .json(&fields);

        if let Some(reason) = self.reason.map_err(Error::validation)? {
            request = request.headers(request::audit_header(reason)?);
        }

        request.build()
    }
}
