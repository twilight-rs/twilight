use crate::json_to_vec;
use crate::request::prelude::*;
use twilight_model::id::{ChannelId, MessageId};

#[derive(Serialize)]
struct DeleteMessagesFields {
    messages: Vec<MessageId>,
}

pub struct DeleteMessages<'a> {
    channel_id: ChannelId,
    fields: DeleteMessagesFields,
    fut: Option<Pending<'a, ()>>,
    http: &'a Client,
    reason: Option<String>,
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
            reason: None,
        }
    }

    pub fn reason(mut self, reason: impl Into<String>) -> Self {
        self.reason.replace(reason.into());

        self
    }

    fn start(&mut self) -> Result<()> {
        let request = if let Some(reason) = &self.reason {
            let headers = audit_header(&reason)?;
            Request::from((
                json_to_vec(&self.fields)?,
                headers,
                Route::DeleteMessages {
                    channel_id: self.channel_id.0,
                },
            ))
        } else {
            Request::from((
                json_to_vec(&self.fields)?,
                Route::DeleteMessages {
                    channel_id: self.channel_id.0,
                },
            ))
        };

        self.fut.replace(Box::pin(self.http.verify(request)));

        Ok(())
    }
}

poll_req!(DeleteMessages<'_>, ());
