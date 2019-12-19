use crate::request::prelude::*;
use dawn_model::id::{ChannelId, MessageId};

pub struct DeleteReaction<'a> {
    channel_id: ChannelId,
    emoji: String,
    fut: Option<Pending<'a, ()>>,
    http: &'a Client,
    message_id: MessageId,
    target_user: String,
}

impl<'a> DeleteReaction<'a> {
    pub(crate) fn new(
        http: &'a Client,
        channel_id: ChannelId,
        message_id: MessageId,
        emoji: impl Into<String>,
        target_user: impl Into<String>,
    ) -> Self {
        Self {
            channel_id,
            emoji: emoji.into(),
            fut: None,
            http,
            message_id,
            target_user: target_user.into(),
        }
    }

    fn start(&mut self) -> Result<()> {
        self.fut.replace(Box::pin(self.http.verify(Request::from(
            Route::DeleteReaction {
                channel_id: self.channel_id.0,
                emoji: self.emoji.clone(),
                message_id: self.message_id.0,
                user: self.target_user.clone(),
            },
        ))));

        Ok(())
    }
}

poll_req!(DeleteReaction<'_>, ());
