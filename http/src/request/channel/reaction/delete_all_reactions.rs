use crate::request::prelude::*;
use twilight_model::id::{ChannelId, MessageId};

/// Delete all reactions by all users on a message.
pub struct DeleteAllReactions<'a> {
    channel_id: ChannelId,
    fut: Option<Pending<'a, ()>>,
    http: &'a Client,
    message_id: MessageId,
}

impl<'a> DeleteAllReactions<'a> {
    pub(crate) fn new(http: &'a Client, channel_id: ChannelId, message_id: MessageId) -> Self {
        Self {
            channel_id,
            fut: None,
            http,
            message_id,
        }
    }

    fn start(&mut self) -> Result<()> {
        self.fut.replace(Box::pin(self.http.verify(Request::from(
            Route::DeleteMessageReactions {
                channel_id: self.channel_id.0,
                message_id: self.message_id.0,
            },
        ))));

        Ok(())
    }
}

poll_req!(DeleteAllReactions<'_>, ());
