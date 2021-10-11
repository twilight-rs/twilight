use crate::{
    client::Client,
    error::Error,
    request::{IntoRequest, Request},
    response::ResponseFuture,
    routing::Route,
};
use serde::Serialize;
use twilight_model::{channel::FollowedChannel, id::ChannelId};

#[derive(Serialize)]
struct FollowNewsChannelFields {
    webhook_channel_id: ChannelId,
}

/// Follow a news channel by [`ChannelId`]s.
#[must_use = "requests must be configured and executed"]
pub struct FollowNewsChannel<'a> {
    channel_id: ChannelId,
    fields: FollowNewsChannelFields,
    http: &'a Client,
}

impl<'a> FollowNewsChannel<'a> {
    pub(crate) const fn new(
        http: &'a Client,
        channel_id: ChannelId,
        webhook_channel_id: ChannelId,
    ) -> Self {
        Self {
            channel_id,
            http,
            fields: FollowNewsChannelFields { webhook_channel_id },
        }
    }

    /// Execute the request, returning a future resolving to a [`Response`].
    ///
    /// [`Response`]: crate::response::Response
    pub fn exec(self) -> ResponseFuture<FollowedChannel> {
        let http = self.http;

        match self.into_request() {
            Ok(request) => http.request(request),
            Err(source) => ResponseFuture::error(source),
        }
    }
}

impl IntoRequest for FollowNewsChannel<'_> {
    fn into_request(self) -> Result<Request, Error> {
        let mut request = Request::builder(&Route::FollowNewsChannel {
            channel_id: self.channel_id.get(),
        });

        request = request.json(&self.fields)?;

        Ok(request.build())
    }
}
