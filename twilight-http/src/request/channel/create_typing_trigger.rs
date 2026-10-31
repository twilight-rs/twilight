use crate::{
    client::Client,
    error::Error,
    request::{Method, Request, TryIntoRequest, path::Path},
    response::{Response, ResponseFuture, marker::EmptyBody},
};
use std::future::IntoFuture;
use twilight_model::id::{Id, marker::ChannelMarker};

struct CreateTypingTriggerPath {
    channel_id: Id<ChannelMarker>,
}

/// Fire a Typing Start event in the channel.
#[must_use = "requests must be configured and executed"]
pub struct CreateTypingTrigger<'a> {
    http: &'a Client,
    path: CreateTypingTriggerPath,
}

impl<'a> CreateTypingTrigger<'a> {
    pub(crate) const fn new(http: &'a Client, channel_id: Id<ChannelMarker>) -> Self {
        Self {
            http,
            path: CreateTypingTriggerPath { channel_id },
        }
    }
}

impl IntoFuture for CreateTypingTrigger<'_> {
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

impl TryIntoRequest for CreateTypingTrigger<'_> {
    fn try_into_request(self) -> Result<Request, Error> {
        let CreateTypingTriggerPath { channel_id } = self.path;

        let path = Path::builder()
            .resource("channels")
            .id(channel_id)
            .resource("typing")
            .build()
            .map_err(Error::path)?;

        Ok(Request::new(Method::Post, path))
    }
}
