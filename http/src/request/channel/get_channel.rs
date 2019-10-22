use crate::request::prelude::*;
use dawn_model::{channel::Channel, id::ChannelId};

pub struct GetChannel<'a> {
    channel_id: ChannelId,
    fut: Option<Pending<'a, Option<Channel>>>,
    http: &'a Client,
}

impl<'a> GetChannel<'a> {
    pub(crate) fn new(http: &'a Client, channel_id: ChannelId) -> Self {
        Self {
            channel_id,
            fut: None,
            http,
        }
    }

    fn start(&mut self) -> Result<()> {
        self.fut.replace(Box::pin(self.http.request(Request::from(
            Route::GetChannel {
                channel_id: self.channel_id.0,
            },
        ))));

        Ok(())
    }
}

poll_req!(GetChannel<'_>, Option<Channel>);
