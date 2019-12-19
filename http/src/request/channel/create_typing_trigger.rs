use crate::request::prelude::*;
use dawn_model::id::ChannelId;

pub struct CreateTypingTrigger<'a> {
    channel_id: ChannelId,
    fut: Option<Pending<'a, Option<()>>>,
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

    fn start(&mut self) -> Result<()> {
        self.fut.replace(Box::pin(self.http.verify(Request::from(
            Route::CreateTypingTrigger {
                channel_id: self.channel_id.0,
            },
        ))));

        Ok(())
    }
}

poll_req!(CreateTypingTrigger<'_>, Option<()>);
