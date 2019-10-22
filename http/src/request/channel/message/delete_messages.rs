use crate::request::prelude::*;
use dawn_model::id::{ChannelId, MessageId};

#[derive(Serialize)]
struct DeleteMessagesFields {
    messages: Vec<MessageId>,
}

pub struct DeleteMessages<'a> {
    channel_id: ChannelId,
    fields: DeleteMessagesFields,
    fut: Option<Pending<'a, ()>>,
    http: &'a Client,
}

impl<'a> DeleteMessages<'a> {
    pub(crate) fn new(
        http: &'a Client,
        channel_id: ChannelId,
        message_ids: impl Into<Vec<MessageId>>,
    ) -> Self {
        Self {
            channel_id,
            fields: DeleteMessagesFields {
                messages: message_ids.into(),
            },
            fut: None,
            http,
        }
    }

    fn start(&mut self) -> Result<()> {
        self.fut.replace(Box::pin(self.http.request(Request::from((
            serde_json::to_vec(&self.fields)?,
            Route::DeleteMessages {
                channel_id: self.channel_id.0,
            },
        )))));

        Ok(())
    }
}

poll_req!(DeleteMessages<'_>, ());
