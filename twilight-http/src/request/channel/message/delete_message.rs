use crate::{
    client::Client,
    error::Error,
    request::{self, AuditLogReason, Method, Request, TryIntoRequest, path::Path},
    response::{Response, ResponseFuture, marker::EmptyBody},
};
use std::future::IntoFuture;
use twilight_model::id::{
    Id,
    marker::{ChannelMarker, MessageMarker},
};
use twilight_validate::request::{ValidationError, audit_reason as validate_audit_reason};

struct DeleteMessagePath {
    channel_id: Id<ChannelMarker>,
    message_id: Id<MessageMarker>,
}

/// Delete a message by [`Id<ChannelMarker>`] and [`Id<MessageMarker>`].
#[must_use = "requests must be configured and executed"]
pub struct DeleteMessage<'a> {
    http: &'a Client,
    path: DeleteMessagePath,
    reason: Result<Option<&'a str>, ValidationError>,
}

impl<'a> DeleteMessage<'a> {
    pub(crate) const fn new(
        http: &'a Client,
        channel_id: Id<ChannelMarker>,
        message_id: Id<MessageMarker>,
    ) -> Self {
        Self {
            http,
            path: DeleteMessagePath {
                channel_id,
                message_id,
            },
            reason: Ok(None),
        }
    }
}

impl<'a> AuditLogReason<'a> for DeleteMessage<'a> {
    fn reason(mut self, reason: &'a str) -> Self {
        self.reason = validate_audit_reason(reason).and(Ok(Some(reason)));

        self
    }
}

impl IntoFuture for DeleteMessage<'_> {
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

impl TryIntoRequest for DeleteMessage<'_> {
    fn try_into_request(self) -> Result<Request, Error> {
        let DeleteMessagePath {
            channel_id,
            message_id,
        } = self.path;

        let path = Path::builder()
            .resource("channels")
            .id(channel_id)
            .resource("messages")
            .id(message_id)
            .build()
            .map_err(Error::path)?;

        let mut request = Request::builder(Method::Delete, path);

        if let Some(reason) = self.reason.map_err(Error::validation)? {
            request = request.headers(request::audit_header(reason)?);
        }

        request.build()
    }
}
