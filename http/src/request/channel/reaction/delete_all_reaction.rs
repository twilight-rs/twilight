use super::RequestReactionType;
use crate::{
    client::Client,
    error::Error,
    request::{Pending, Request},
    routing::Route,
};
use twilight_model::id::{ChannelId, MessageId};

/// Remove all reactions of a specified emoji from a message.
pub struct DeleteAllReaction<'a> {
    channel_id: ChannelId,
    emoji: RequestReactionType,
    fut: Option<Pending<'a, ()>>,
    http: &'a Client,
    message_id: MessageId,
}

impl<'a> DeleteAllReaction<'a> {
    pub(crate) fn new(
        http: &'a Client,
        channel_id: ChannelId,
        message_id: MessageId,
        emoji: RequestReactionType,
    ) -> Self {
        Self {
            channel_id,
            emoji,
            fut: None,
            http,
            message_id,
        }
    }

    fn start(&mut self) -> Result<(), Error> {
        let request = Request::from_route(Route::DeleteMessageSpecificReaction {
            channel_id: self.channel_id.0,
            message_id: self.message_id.0,
            emoji: self.emoji.display().to_string(),
        });

        self.fut.replace(Box::pin(self.http.verify(request)));

        Ok(())
    }
}

poll_req!(DeleteAllReaction<'_>, ());
