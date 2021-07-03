use crate::{
    client::Client,
    error::Error,
    request::{PendingResponse, Request},
    response::marker::EmptyBody,
    routing::Route,
};
use twilight_model::id::ChannelId;

/// Fire a Typing Start event in the channel.
pub struct CreateTypingTrigger<'a> {
    channel_id: ChannelId,
    fut: Option<PendingResponse<'a, EmptyBody>>,
    http: &'a Client,
}

impl<'a> CreateTypingTrigger<'a> {
    pub(crate) fn new(http: &'a Client, channel_id: ChannelId) -> Self {
        Self {
            channel_id,
            fut: None,
            http,
        }
    }

    fn start(&mut self) -> Result<(), Error> {
        let request = Request::from_route(Route::CreateTypingTrigger {
            channel_id: self.channel_id.0,
        });

        self.fut.replace(Box::pin(self.http.request(request)));

        Ok(())
    }
}

poll_req!(CreateTypingTrigger<'_>, EmptyBody);
