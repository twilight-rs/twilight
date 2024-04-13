use crate::{
    client::Client,
    error::Error,
    request::{Request, TryIntoRequest},
    response::{marker::ListBody, Response, ResponseFuture},
    routing::Route,
};
use std::future::IntoFuture;
use twilight_model::{
    channel::Webhook,
    id::{marker::ChannelMarker, Id},
};

/// Get all the webhooks of a channel.
#[must_use = "requests must be configured and executed"]
pub struct GetChannelWebhooks<'a> {
    channel_id: Id<ChannelMarker>,
    http: &'a Client,
}

impl<'a> GetChannelWebhooks<'a> {
    pub(crate) const fn new(http: &'a Client, channel_id: Id<ChannelMarker>) -> Self {
        Self { channel_id, http }
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
        Ok(Request::from_route(&Route::GetChannelWebhooks {
            channel_id: self.channel_id.get(),
        }))
    }
}
