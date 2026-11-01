use crate::{
    client::Client,
    error::Error,
    request::{self, AuditLogReason, Request, TryIntoRequest},
    response::{Response, ResponseFuture},
    routing::Route,
};
use std::future::IntoFuture;
use twilight_model::{
    channel::Channel,
    id::{Id, marker::ChannelMarker},
};
use twilight_validate::request::{ValidationError, audit_reason as validate_audit_reason};

struct DeleteChannelPath {
    channel_id: Id<ChannelMarker>,
}

/// Delete a channel by ID.
#[must_use = "requests must be configured and executed"]
pub struct DeleteChannel<'a> {
    http: &'a Client,
    path: DeleteChannelPath,
    reason: Result<Option<&'a str>, ValidationError>,
}

impl<'a> DeleteChannel<'a> {
    pub(crate) const fn new(http: &'a Client, channel_id: Id<ChannelMarker>) -> Self {
        Self {
            http,
            path: DeleteChannelPath { channel_id },
            reason: Ok(None),
        }
    }
}

impl<'a> AuditLogReason<'a> for DeleteChannel<'a> {
    fn reason(mut self, reason: &'a str) -> Self {
        self.reason = validate_audit_reason(reason).and(Ok(Some(reason)));

        self
    }
}

impl IntoFuture for DeleteChannel<'_> {
    type Output = Result<Response<Channel>, Error>;

    type IntoFuture = ResponseFuture<Channel>;

    fn into_future(self) -> Self::IntoFuture {
        let http = self.http;

        match self.try_into_request() {
            Ok(request) => http.request(request),
            Err(source) => ResponseFuture::error(source),
        }
    }
}

impl TryIntoRequest for DeleteChannel<'_> {
    fn try_into_request(self) -> Result<Request, Error> {
        let mut request = Request::builder(&Route::DeleteChannel {
            channel_id: self.path.channel_id.get(),
        });

        if let Some(reason) = self.reason.map_err(Error::validation)? {
            request = request.headers(request::audit_header(reason)?);
        }

        request.build()
    }
}
