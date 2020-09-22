use crate::request::prelude::*;
use twilight_model::{channel::Channel, id::ChannelId};

/// Delete a channel by ID.
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

    #[deprecated(
        since = "0.1.5",
        note = "please prefer the request::AuditLogReason trait"
    )]
    /// Attach an audit log reason to this request.
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

impl<'a> AuditLogReason for DeleteChannel<'a> {
    fn reason(mut self, reason: impl Into<String>) -> Result<Self, AuditLogReasonError> {
        let reason = AuditLogReasonError::validate(reason.into())?;
        self.reason.replace(reason);

        Ok(self)
    }
}

poll_req!(DeleteChannel<'_>, Channel);
