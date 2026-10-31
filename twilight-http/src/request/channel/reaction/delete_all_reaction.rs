use super::RequestReactionType;
use crate::{
    client::Client,
    error::Error,
    request::{Method, Request, TryIntoRequest, path::Path},
    response::{Response, ResponseFuture, marker::EmptyBody},
};
use std::future::IntoFuture;
use twilight_model::id::{
    Id,
    marker::{ChannelMarker, MessageMarker},
};

struct DeleteAllReactionPath<'a> {
    channel_id: Id<ChannelMarker>,
    emoji: &'a RequestReactionType<'a>,
    message_id: Id<MessageMarker>,
}

/// Remove all reactions of a specified emoji from a message.
#[must_use = "requests must be configured and executed"]
pub struct DeleteAllReaction<'a> {
    http: &'a Client,
    path: DeleteAllReactionPath<'a>,
}

impl<'a> DeleteAllReaction<'a> {
    pub(crate) const fn new(
        http: &'a Client,
        channel_id: Id<ChannelMarker>,
        message_id: Id<MessageMarker>,
        emoji: &'a RequestReactionType<'a>,
    ) -> Self {
        Self {
            http,
            path: DeleteAllReactionPath {
                channel_id,
                emoji,
                message_id,
            },
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
        let DeleteAllReactionPath {
            channel_id,
            emoji,
            message_id,
        } = self.path;

        let path = Path::builder()
            .resource("channels")
            .id(channel_id)
            .resource("messages")
            .id(message_id)
            .resource("reactions")
            .id(emoji)
            .build()
            .map_err(Error::path)?;

        Ok(Request::new(Method::Delete, path))
    }
}
