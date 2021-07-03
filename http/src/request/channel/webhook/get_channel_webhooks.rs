use crate::{
    client::Client,
    error::Error,
    request::{PendingResponse, Request},
    response::marker::ListBody,
    routing::Route,
};
use twilight_model::{channel::Webhook, id::ChannelId};

/// Get all the webhooks of a channel.
pub struct GetChannelWebhooks<'a> {
    channel_id: ChannelId,
    fut: Option<PendingResponse<'a, ListBody<Webhook>>>,
    http: &'a Client,
}

impl<'a> GetChannelWebhooks<'a> {
    pub(crate) fn new(http: &'a Client, channel_id: ChannelId) -> Self {
        Self {
            channel_id,
            fut: None,
            http,
        }
    }

    fn start(&mut self) -> Result<(), Error> {
        let request = Request::from_route(Route::GetChannelWebhooks {
            channel_id: self.channel_id.0,
        });

        self.fut.replace(Box::pin(self.http.request(request)));

        Ok(())
    }
}

poll_req!(GetChannelWebhooks<'_>, ListBody<Webhook>);
