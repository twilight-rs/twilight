use crate::request::prelude::*;
use twilight_model::id::{ChannelId, MessageId};

pub struct CreateReaction<'a> {
    channel_id: ChannelId,
    emoji: String,
    fut: Option<Pending<'a, ()>>,
    http: &'a Client,
    message_id: MessageId,
}

impl<'a> CreateReaction<'a> {
    pub(crate) fn new(
        http: &'a Client,
        channel_id: ChannelId,
        message_id: MessageId,
        emoji: impl Into<String>,
    ) -> Self {
        Self {
            channel_id,
            emoji: emoji.into(),
            fut: None,
            http,
            message_id,
        }
    }

    fn start(&mut self) -> Result<()> {
        self.fut.replace(Box::pin(self.http.verify(Request::from(
            Route::CreateReaction {
                channel_id: self.channel_id.0,
                emoji: self.emoji.clone(),
                message_id: self.message_id.0,
            },
        ))));

        Ok(())
    }
}

poll_req!(CreateReaction<'_>, ());
