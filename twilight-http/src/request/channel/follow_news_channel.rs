use crate::{
    client::Client,
    error::Error,
    request::{Request, TryIntoRequest},
    response::{Response, ResponseFuture},
    routing::Route,
};
use serde::Serialize;
use std::future::IntoFuture;
use twilight_model::{
    channel::FollowedChannel,
    id::{Id, marker::ChannelMarker},
};

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
        }
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
        Request::builder(&Route::FollowNewsChannel {
            channel_id: self.channel_id.get(),
        })
        .json(&self.body)
        .build()
    }
}
