use crate::{client::Client, request::Request, response::ResponseFuture, routing::Route};
use serde::Serialize;
use twilight_model::{
    channel::FollowedChannel,
    id::{marker::ChannelMarker, Id},
};

#[derive(Serialize)]
struct FollowNewsChannelFields {
    webhook_channel_id: Id<ChannelMarker>,
}

/// Follow a news channel by [`Id<ChannelMarker>`]s.
#[must_use = "requests must be configured and executed"]
pub struct FollowNewsChannel<'a> {
    channel_id: Id<ChannelMarker>,
    fields: FollowNewsChannelFields,
    http: &'a Client,
}

impl<'a> FollowNewsChannel<'a> {
    pub(crate) const fn new(
        http: &'a Client,
        channel_id: Id<ChannelMarker>,
        webhook_channel_id: Id<ChannelMarker>,
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
        let mut request = Request::builder(&Route::FollowNewsChannel {
            channel_id: self.channel_id.get(),
        });

        request = match request.json(&self.fields) {
            Ok(request) => request,
            Err(source) => return ResponseFuture::error(source),
        };

        self.http.request(request.build())
    }
}
