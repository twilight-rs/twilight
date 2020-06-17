use crate::request::prelude::*;
use twilight_model::{
    channel::Message,
    id::{ChannelId, MessageId},
};

/// Get a message by [`ChannelId`] and [`MessageId`].
///
/// [`ChannelId`]: ../../../../twilight_model/id/struct.ChannelId.html
/// [`MessageId`]: ../../../../twilight_model/id/struct.MessageId.html
pub struct GetMessage<'a> {
    channel_id: ChannelId,
    fut: Option<PendingOption<'a>>,
    http: &'a Client,
    message_id: MessageId,
}

impl<'a> GetMessage<'a> {
    pub(crate) fn new(http: &'a Client, channel_id: ChannelId, message_id: MessageId) -> Self {
        Self {
            channel_id,
            fut: None,
            http,
            message_id,
        }
    }

    fn start(&mut self) -> Result<()> {
        self.fut
            .replace(Box::pin(self.http.request_bytes(Request::from(
                Route::GetMessage {
                    channel_id: self.channel_id.0,
                    message_id: self.message_id.0,
                },
            ))));

        Ok(())
    }
}

poll_req!(opt, GetMessage<'_>, Message);
