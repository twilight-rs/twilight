use crate::{
    client::Client,
    error::Error,
    request::{PendingResponse, Request},
    response::marker::EmptyBody,
    routing::Route,
};
use twilight_model::id::{ChannelId, MessageId};

/// Delete all reactions by all users on a message.
pub struct DeleteAllReactions<'a> {
    channel_id: ChannelId,
    fut: Option<PendingResponse<'a, EmptyBody>>,
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

    fn start(&mut self) -> Result<(), Error> {
        let request = Request::from_route(Route::DeleteMessageReactions {
            channel_id: self.channel_id.0,
            message_id: self.message_id.0,
        });

        self.fut.replace(Box::pin(self.http.request(request)));

        Ok(())
    }
}

poll_req!(DeleteAllReactions<'_>, EmptyBody);
