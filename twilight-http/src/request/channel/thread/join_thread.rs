use crate::{
    client::Client,
    error::Error,
    request::{Method, Request, TryIntoRequest, path::Path},
    response::{Response, ResponseFuture, marker::EmptyBody},
};
use std::future::IntoFuture;
use twilight_model::id::{Id, marker::ChannelMarker};

struct JoinThreadPath {
    channel_id: Id<ChannelMarker>,
}

/// Add the current user to a thread.
#[must_use = "requests must be configured and executed"]
pub struct JoinThread<'a> {
    http: &'a Client,
    path: JoinThreadPath,
}

impl<'a> JoinThread<'a> {
    pub(crate) const fn new(http: &'a Client, channel_id: Id<ChannelMarker>) -> Self {
        Self {
            http,
            path: JoinThreadPath { channel_id },
        }
    }
}

impl IntoFuture for JoinThread<'_> {
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

impl TryIntoRequest for JoinThread<'_> {
    fn try_into_request(self) -> Result<Request, Error> {
        let JoinThreadPath { channel_id } = self.path;

        let path = Path::builder()
            .resource("channels")
            .id(channel_id)
            .resource("thread-members")
            .me()
            .build()
            .map_err(Error::path)?;

        Ok(Request::new(Method::Put, path))
    }
}
