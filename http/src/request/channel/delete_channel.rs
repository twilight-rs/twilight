use crate::request::prelude::*;
use dawn_model::{channel::Channel, id::ChannelId};

pub struct DeleteChannel<'a> {
    channel_id: ChannelId,
    fut: Option<Pending<'a, Channel>>,
    http: &'a Client,
}

impl<'a> DeleteChannel<'a> {
    pub(crate) fn new(http: &'a Client, channel_id: ChannelId) -> Self {
        Self {
            channel_id,
            fut: None,
            http,
        }
    }

    fn start(&mut self) -> Result<()> {
        self.fut.replace(Box::pin(self.http.request(Request::from(
            Route::DeleteChannel {
                channel_id: self.channel_id.0,
            },
        ))));

        Ok(())
    }
}

poll_req!(DeleteChannel<'_>, Channel);
