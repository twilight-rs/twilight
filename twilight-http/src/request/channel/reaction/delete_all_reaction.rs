use super::RequestReactionType;
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

/// Remove all reactions of a specified emoji from a message.
#[must_use = "requests must be configured and executed"]
pub struct DeleteAllReaction<'a> {
    channel_id: Id<ChannelMarker>,
    emoji: &'a RequestReactionType<'a>,
    http: &'a Client,
    message_id: Id<MessageMarker>,
}

impl<'a> DeleteAllReaction<'a> {
    pub(crate) const fn new(
        http: &'a Client,
        channel_id: Id<ChannelMarker>,
        message_id: Id<MessageMarker>,
        emoji: &'a RequestReactionType<'a>,
    ) -> Self {
        Self {
            channel_id,
            emoji,
            http,
            message_id,
        }
    }
}

impl IntoFuture for DeleteAllReaction<'_> {
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

impl TryIntoRequest for DeleteAllReaction<'_> {
    fn try_into_request(self) -> Result<Request, Error> {
        Ok(Request::from_route(&Route::DeleteMessageSpecificReaction {
            channel_id: self.channel_id.get(),
            message_id: self.message_id.get(),
            emoji: self.emoji,
        }))
    }
}
