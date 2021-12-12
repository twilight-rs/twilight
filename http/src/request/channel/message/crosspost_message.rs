use crate::{
    client::Client,
    error::Error,
    request::{Request, TryIntoRequest},
    response::ResponseFuture,
    routing::Route,
};
use twilight_model::{
    channel::Message,
    id::{ChannelId, MessageId},
};

/// Crosspost a message by [`ChannelId`] and [`MessageId`].
#[must_use = "requests must be configured and executed"]
pub struct CrosspostMessage<'a> {
    channel_id: ChannelId,
    http: &'a Client,
    message_id: MessageId,
}

impl<'a> CrosspostMessage<'a> {
    pub(crate) const fn new(
        http: &'a Client,
        channel_id: ChannelId,
        message_id: MessageId,
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
        let http = self.http;

        match self.try_into_request() {
            Ok(request) => http.request(request),
            Err(source) => ResponseFuture::error(source),
        }
    }
}

impl TryIntoRequest for CrosspostMessage<'_> {
    fn try_into_request(self) -> Result<Request, Error> {
        Ok(Request::from_route(&Route::CrosspostMessage {
            channel_id: self.channel_id.get(),
            message_id: self.message_id.get(),
        }))
    }
}
