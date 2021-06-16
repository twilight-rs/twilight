use crate::{
    client::Client,
    error::Error,
    request::{PendingResponse, Request},
    response::marker::ListBody,
    routing::Route,
};
use twilight_model::{channel::Message, id::ChannelId};

/// Get the pins of a channel.
pub struct GetPins<'a> {
    channel_id: ChannelId,
    fut: Option<PendingResponse<'a, ListBody<Message>>>,
    http: &'a Client,
}

impl<'a> GetPins<'a> {
    pub(crate) fn new(http: &'a Client, channel_id: ChannelId) -> Self {
        Self {
            channel_id,
            fut: None,
            http,
        }
    }

    fn start(&mut self) -> Result<(), Error> {
        let request = Request::from_route(Route::GetPins {
            channel_id: self.channel_id.0,
        });

        self.fut.replace(Box::pin(self.http.request(request)));

        Ok(())
    }
}

poll_req!(GetPins<'_>, ListBody<Message>);
