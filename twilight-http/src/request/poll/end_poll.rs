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
    channel::Message,
    id::{
        Id,
        marker::{ChannelMarker, MessageMarker},
    },
};

#[derive(Serialize)]
struct EndPollPath {
    channel_id: Id<ChannelMarker>,
    message_id: Id<MessageMarker>,
}

// Ends a poll in a channel.
#[must_use = "requests must be configured and executed"]
pub struct EndPoll<'a> {
    http: &'a Client,
    path: EndPollPath,
}

impl<'a> EndPoll<'a> {
    pub(crate) const fn new(
        http: &'a Client,
        channel_id: Id<ChannelMarker>,
        message_id: Id<MessageMarker>,
    ) -> Self {
        Self {
            http,
            path: EndPollPath {
                channel_id,
                message_id,
            },
        }
    }
}

impl IntoFuture for EndPoll<'_> {
    type Output = Result<Response<Message>, Error>;
    type IntoFuture = ResponseFuture<Message>;

    fn into_future(self) -> Self::IntoFuture {
        let http = self.http;

        match self.try_into_request() {
            Ok(request) => http.request(request),
            Err(source) => ResponseFuture::error(source),
        }
    }
}

impl TryIntoRequest for EndPoll<'_> {
    fn try_into_request(self) -> Result<Request, Error> {
        Ok(Request::from_route(&Route::EndPoll {
            channel_id: self.path.channel_id.get(),
            message_id: self.path.message_id.get(),
        }))
    }
}
