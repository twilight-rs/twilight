use super::RequestReactionType;
use crate::{
    client::Client,
    request::Request,
    response::{marker::EmptyBody, ResponseFuture},
    routing::Route,
};
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

    /// Execute the request, returning a future resolving to a [`Response`].
    ///
    /// [`Response`]: crate::response::Response
    pub fn exec(self) -> ResponseFuture<EmptyBody> {
        let request = Request::from_route(&Route::DeleteMessageSpecificReaction {
            channel_id: self.channel_id.get(),
            message_id: self.message_id.get(),
            emoji: self.emoji,
        });

        self.http.request(request)
    }
}
