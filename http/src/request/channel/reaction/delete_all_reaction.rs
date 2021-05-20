use crate::request::prelude::*;
use twilight_model::id::{ChannelId, MessageId};

/// Remove all reactions of a specified emoji from a message.
pub struct DeleteAllReaction<'a> {
    channel_id: ChannelId,
    emoji: String,
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
            emoji: super::format_emoji(emoji),
            fut: None,
            http,
            message_id,
        }
    }

    fn start(&mut self) -> Result<()> {
        let request = Request::from_route(Route::DeleteMessageSpecficReaction {
            channel_id: self.channel_id.0,
            message_id: self.message_id.0,
            emoji: self.emoji.clone(),
        });

        self.fut.replace(Box::pin(self.http.verify(request)));

        Ok(())
    }
}

poll_req!(DeleteAllReaction<'_>, ());
