use crate::request::prelude::*;
use twilight_model::{channel::Webhook, id::ChannelId};

pub struct GetChannelWebhooks<'a> {
    channel_id: ChannelId,
    fut: Option<Pending<'a, Vec<Webhook>>>,
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

    fn start(&mut self) -> Result<()> {
        self.fut.replace(Box::pin(self.http.request(Request::from(
            Route::GetChannelWebhooks {
                channel_id: self.channel_id.0,
            },
        ))));

        Ok(())
    }
}

poll_req!(GetChannelWebhooks<'_>, Vec<Webhook>);
