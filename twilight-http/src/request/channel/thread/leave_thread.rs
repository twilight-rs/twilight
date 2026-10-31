use crate::{
    client::Client,
    error::Error,
    request::{Method, Request, TryIntoRequest, path::Path},
    response::{Response, ResponseFuture, marker::EmptyBody},
};
use std::future::IntoFuture;
use twilight_model::id::{Id, marker::ChannelMarker};

struct LeaveThreadPath {
    channel_id: Id<ChannelMarker>,
}

/// Remove the current user from a thread.
///
/// Requires that the thread is not archived.
#[must_use = "requests must be configured and executed"]
pub struct LeaveThread<'a> {
    http: &'a Client,
    path: LeaveThreadPath,
}

impl<'a> LeaveThread<'a> {
    pub(crate) const fn new(http: &'a Client, channel_id: Id<ChannelMarker>) -> Self {
        Self {
            http,
            path: LeaveThreadPath { channel_id },
        }
    }
}

impl IntoFuture for LeaveThread<'_> {
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

impl TryIntoRequest for LeaveThread<'_> {
    fn try_into_request(self) -> Result<Request, Error> {
        let LeaveThreadPath { channel_id } = self.path;

        let path = Path::builder()
            .resource("channels")
            .id(channel_id)
            .resource("thread-members")
            .me()
            .build()
            .map_err(Error::path)?;

        Ok(Request::new(Method::Delete, path))
    }
}
