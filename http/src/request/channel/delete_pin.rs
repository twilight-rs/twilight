use crate::{
    client::Client,
    error::Error,
    request::{self, AuditLogReason, AuditLogReasonError, PendingResponse, Request},
    response::marker::EmptyBody,
    routing::Route,
};
use twilight_model::id::{ChannelId, MessageId};

/// Delete a pin in a channel, by ID.
pub struct DeletePin<'a> {
    channel_id: ChannelId,
    fut: Option<PendingResponse<'a, EmptyBody>>,
    http: &'a Client,
    message_id: MessageId,
    reason: Option<String>,
}

impl<'a> DeletePin<'a> {
    pub(crate) fn new(http: &'a Client, channel_id: ChannelId, message_id: MessageId) -> Self {
        Self {
            channel_id,
            fut: None,
            http,
            message_id,
            reason: None,
        }
    }

    fn start(&mut self) -> Result<(), Error> {
        let mut request = Request::builder(Route::UnpinMessage {
            channel_id: self.channel_id.0,
            message_id: self.message_id.0,
        });

        if let Some(reason) = &self.reason {
            request = request.headers(request::audit_header(reason)?);
        }

        self.fut
            .replace(Box::pin(self.http.request(request.build())));

        Ok(())
    }
}

impl<'a> AuditLogReason for DeletePin<'a> {
    fn reason(mut self, reason: impl Into<String>) -> Result<Self, AuditLogReasonError> {
        self.reason
            .replace(AuditLogReasonError::validate(reason.into())?);

        Ok(self)
    }
}

poll_req!(DeletePin<'_>, EmptyBody);
