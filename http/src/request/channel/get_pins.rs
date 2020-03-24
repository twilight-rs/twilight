use crate::request::prelude::*;
use twilight_model::{channel::Message, id::ChannelId};

pub struct GetPins<'a> {
    channel_id: ChannelId,
    fut: Option<Pending<'a, Vec<Message>>>,
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

    fn start(&mut self) -> Result<()> {
        self.fut
            .replace(Box::pin(self.http.request(Request::from(Route::GetPins {
                channel_id: self.channel_id.0,
            }))));

        Ok(())
    }
}

poll_req!(GetPins<'_>, Vec<Message>);
