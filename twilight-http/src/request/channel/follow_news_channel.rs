use crate::{
    client::Client,
    error::Error,
    request::{self, AuditLogReason, Request, TryIntoRequest},
    response::{Response, ResponseFuture},
    routing::Route,
};
use serde::Serialize;
use std::future::IntoFuture;
use twilight_model::{
    channel::FollowedChannel,
    id::{Id, marker::ChannelMarker},
};
use twilight_validate::request::{ValidationError, audit_reason as validate_audit_reason};

#[derive(Serialize)]
struct FollowNewsChannelBody {
    webhook_channel_id: Id<ChannelMarker>,
}

/// Follow a news channel by [`Id<ChannelMarker>`]s.
#[must_use = "requests must be configured and executed"]
pub struct FollowNewsChannel<'a> {
    body: FollowNewsChannelBody,
    channel_id: Id<ChannelMarker>,
    http: &'a Client,
    reason: Result<Option<&'a str>, ValidationError>,
}

impl<'a> FollowNewsChannel<'a> {
    pub(crate) const fn new(
        http: &'a Client,
        channel_id: Id<ChannelMarker>,
        webhook_channel_id: Id<ChannelMarker>,
    ) -> Self {
        Self {
            body: FollowNewsChannelBody { webhook_channel_id },
            channel_id,
            http,
            reason: Ok(None),
        }
    }
}

impl<'a> AuditLogReason<'a> for FollowNewsChannel<'a> {
    fn reason(mut self, reason: &'a str) -> Self {
        self.reason = validate_audit_reason(reason).and(Ok(Some(reason)));

        self
    }
}

impl IntoFuture for FollowNewsChannel<'_> {
    type Output = Result<Response<FollowedChannel>, Error>;

    type IntoFuture = ResponseFuture<FollowedChannel>;

    fn into_future(self) -> Self::IntoFuture {
        let http = self.http;

        match self.try_into_request() {
            Ok(request) => http.request(request),
            Err(source) => ResponseFuture::error(source),
        }
    }
}

impl TryIntoRequest for FollowNewsChannel<'_> {
    fn try_into_request(self) -> Result<Request, Error> {
        let mut request = Request::builder(&Route::FollowNewsChannel {
            channel_id: self.channel_id.get(),
        })
        .json(&self.body);

        if let Some(reason) = self.reason.map_err(Error::validation)? {
            request = request.headers(request::audit_header(reason)?);
        }

        request.build()
    }
}
