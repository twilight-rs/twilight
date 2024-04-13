use crate::{
    client::Client,
    error::Error,
    request::{Request, TryIntoRequest},
    response::{marker::EmptyBody, Response, ResponseFuture},
    routing::Route,
};
use std::future::IntoFuture;
use twilight_model::id::{
    marker::{ChannelMarker, MessageMarker},
    Id,
};

/// Delete all reactions by all users on a message.
#[must_use = "requests must be configured and executed"]
pub struct DeleteAllReactions<'a> {
    channel_id: Id<ChannelMarker>,
    http: &'a Client,
    message_id: Id<MessageMarker>,
}

impl<'a> DeleteAllReactions<'a> {
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
}

impl IntoFuture for DeleteAllReactions<'_> {
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

impl TryIntoRequest for DeleteAllReactions<'_> {
    fn try_into_request(self) -> Result<Request, Error> {
        Ok(Request::from_route(&Route::DeleteMessageReactions {
            channel_id: self.channel_id.get(),
            message_id: self.message_id.get(),
        }))
    }
}
