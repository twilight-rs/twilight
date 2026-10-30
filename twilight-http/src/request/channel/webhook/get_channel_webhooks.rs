use crate::{
    client::Client,
    error::Error,
    request::{Method, Request, TryIntoRequest, path::Path},
    response::{Response, ResponseFuture, marker::ListBody},
};
use std::future::IntoFuture;
use twilight_model::{
    channel::Webhook,
    id::{Id, marker::ChannelMarker},
};

struct GetChannelWebhooksPath {
    channel_id: Id<ChannelMarker>,
}

/// Get all the webhooks of a channel.
#[must_use = "requests must be configured and executed"]
pub struct GetChannelWebhooks<'a> {
    http: &'a Client,
    path: GetChannelWebhooksPath,
}

impl<'a> GetChannelWebhooks<'a> {
    pub(crate) const fn new(http: &'a Client, channel_id: Id<ChannelMarker>) -> Self {
        Self {
            http,
            path: GetChannelWebhooksPath { channel_id },
        }
    }
}

impl IntoFuture for GetChannelWebhooks<'_> {
    type Output = Result<Response<ListBody<Webhook>>, Error>;

    type IntoFuture = ResponseFuture<ListBody<Webhook>>;

    fn into_future(self) -> Self::IntoFuture {
        let http = self.http;

        match self.try_into_request() {
            Ok(request) => http.request(request),
            Err(source) => ResponseFuture::error(source),
        }
    }
}

impl TryIntoRequest for GetChannelWebhooks<'_> {
    fn try_into_request(self) -> Result<Request, Error> {
        let GetChannelWebhooksPath { channel_id } = self.path;

        let path = Path::builder()
            .resource("channels")
            .id(channel_id)
            .resource("webhooks")
            .build()
            .map_err(Error::path)?;

        Ok(Request::new(Method::Get, path))
    }
}
