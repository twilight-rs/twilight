use crate::{client::Client, request::Request, response::ResponseFuture, routing::Route};
use twilight_model::{
    channel::Message,
    id::{
        marker::{ChannelMarker, MessageMarker},
        Id,
    },
};

/// Crosspost a message by [`Id<ChannelMarker>`] and [`Id<MessageMarker>`].
#[must_use = "requests must be configured and executed"]
pub struct CrosspostMessage<'a> {
    channel_id: Id<ChannelMarker>,
    http: &'a Client,
    message_id: Id<MessageMarker>,
}

impl<'a> CrosspostMessage<'a> {
    pub(crate) const fn new(
        http: &'a Client,
        channel_id: Id<ChannelMarker>,
        message_id: Id<MessageMarker>,
    ) -> Self {
        Self {
            channel_id,
            http,
            message_id,
        }
    }

    /// Execute the request, returning a future resolving to a [`Response`].
    ///
    /// [`Response`]: crate::response::Response
    pub fn exec(self) -> ResponseFuture<Message> {
        let request = Request::from_route(&Route::CrosspostMessage {
            channel_id: self.channel_id.get(),
            message_id: self.message_id.get(),
        });

        self.http.request(request)
    }
}
