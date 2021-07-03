use crate::{
    client::Client,
    error::Error,
    request::{PendingResponse, Request},
    routing::Route,
};
use twilight_model::{
    channel::Message,
    id::{ChannelId, MessageId},
};

/// Crosspost a message by [`ChannelId`] and [`MessageId`].
pub struct CrosspostMessage<'a> {
    channel_id: ChannelId,
    fut: Option<PendingResponse<'a, Message>>,
    http: &'a Client,
    message_id: MessageId,
}

impl<'a> CrosspostMessage<'a> {
    pub(crate) fn new(http: &'a Client, channel_id: ChannelId, message_id: MessageId) -> Self {
        Self {
            channel_id,
            fut: None,
            http,
            message_id,
        }
    }

    fn start(&mut self) -> Result<(), Error> {
        let request = Request::from_route(Route::CrosspostMessage {
            channel_id: self.channel_id.0,
            message_id: self.message_id.0,
        });

        self.fut.replace(Box::pin(self.http.request(request)));

        Ok(())
    }
}

poll_req!(CrosspostMessage<'_>, Message);
