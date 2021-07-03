use crate::{
    client::Client,
    error::Error,
    request::{self, AuditLogReason, AuditLogReasonError, PendingResponse, Request},
    routing::Route,
};
use twilight_model::{channel::Channel, id::ChannelId};

/// Delete a channel by ID.
pub struct DeleteChannel<'a> {
    channel_id: ChannelId,
    fut: Option<PendingResponse<'a, Channel>>,
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

    fn start(&mut self) -> Result<(), Error> {
        let mut request = Request::builder(Route::DeleteChannel {
            channel_id: self.channel_id.0,
        });

        if let Some(reason) = &self.reason {
            request = request.headers(request::audit_header(reason)?);
        }

        self.fut
            .replace(Box::pin(self.http.request(request.build())));

        Ok(())
    }
}

impl<'a> AuditLogReason for DeleteChannel<'a> {
    fn reason(mut self, reason: impl Into<String>) -> Result<Self, AuditLogReasonError> {
        self.reason
            .replace(AuditLogReasonError::validate(reason.into())?);

        Ok(self)
    }
}

poll_req!(DeleteChannel<'_>, Channel);
