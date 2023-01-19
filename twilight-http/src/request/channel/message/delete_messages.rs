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
use twilight_validate::request::{audit_reason as validate_audit_reason, ValidationError};

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
    fields: DeleteMessagesFields<'a>,
    http: &'a Client,
    reason: Option<&'a str>,
}

impl<'a> DeleteMessages<'a> {
    pub(crate) const fn new(
        http: &'a Client,
        channel_id: Id<ChannelMarker>,
        messages: &'a [Id<MessageMarker>],
    ) -> Self {
        Self {
            channel_id,
            fields: DeleteMessagesFields { messages },
            http,
            reason: None,
        }
    }

    /// Execute the request, returning a future resolving to a [`Response`].
    #[deprecated(since = "0.14.0", note = "use `.await` or `into_future` instead")]
    pub fn exec(self) -> ResponseFuture<EmptyBody> {
        self.into_future()
    }
}

impl<'a> AuditLogReason<'a> for DeleteMessages<'a> {
    fn reason(mut self, reason: &'a str) -> Result<Self, ValidationError> {
        validate_audit_reason(reason)?;

        self.reason.replace(reason);

        Ok(self)
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
        let mut request = Request::builder(&Route::DeleteMessages {
            channel_id: self.channel_id.get(),
        });

        request = request.json(&self.fields)?;

        if let Some(reason) = &self.reason {
            let header = request::audit_header(reason)?;

            request = request.headers(header);
        }

        Ok(request.build())
    }
}
