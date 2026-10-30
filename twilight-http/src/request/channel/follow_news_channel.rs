use crate::{
    client::Client,
    error::Error,
    request::{self, AuditLogReason, Method, Request, TryIntoRequest, path::Path},
    response::{Response, ResponseFuture},
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

struct FollowNewsChannelPath {
    channel_id: Id<ChannelMarker>,
}

/// Follow a news channel by [`Id<ChannelMarker>`]s.
#[must_use = "requests must be configured and executed"]
pub struct FollowNewsChannel<'a> {
    body: FollowNewsChannelBody,
    http: &'a Client,
    path: FollowNewsChannelPath,
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
            http,
            path: FollowNewsChannelPath { channel_id },
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
        let FollowNewsChannelPath { channel_id } = self.path;

        let path = Path::builder()
            .resource("channels")
            .id(channel_id)
            .resource("followers")
            .build()
            .map_err(Error::path)?;

        let mut request = Request::builder(Method::Post, path).json(&self.body);

        if let Some(reason) = self.reason.map_err(Error::validation)? {
            request = request.headers(request::audit_header(reason)?);
        }

        request.build()
    }
}
