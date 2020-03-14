use crate::request::prelude::*;
use dawn_model::{channel::Channel, id::ChannelId};

pub struct DeleteChannel<'a> {
    channel_id: ChannelId,
    fut: Option<Pending<'a, Channel>>,
    http: &'a Client,
    reason: Option<String>,
}

impl<'a> DeleteChannel<'a> {
    pub(crate) fn new(http: &'a Client, channel_id: ChannelId) -> Self {
        Self {
            channel_id,
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
                headers,
                Route::DeleteChannel {
                    channel_id: self.channel_id.0,
                },
            ))
        } else {
            Request::from(Route::DeleteChannel {
                channel_id: self.channel_id.0,
            })
        };

        self.fut.replace(Box::pin(self.http.request(request)));

        Ok(())
    }
}

poll_req!(DeleteChannel<'_>, Channel);
