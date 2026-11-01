use crate::{
    client::Client,
    error::Error,
    request::{Request, TryIntoRequest},
    response::{Response, ResponseFuture, marker::EmptyBody},
    routing::Route,
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
        Ok(Request::from_route(&Route::JoinThread {
            channel_id: self.path.channel_id.get(),
        }))
    }
}
