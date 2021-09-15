use crate::{
    client::Client,
    request::{self, AuditLogReason, AuditLogReasonError, IntoRequest, Request},
    response::{marker::EmptyBody, ResponseFuture},
    routing::Route,
};
use twilight_model::id::{ChannelId, MessageId};

/// Delete a pin in a channel, by ID.
#[must_use = "requests must be configured and executed"]
pub struct DeletePin<'a> {
    channel_id: ChannelId,
    http: &'a Client,
    message_id: MessageId,
    reason: Option<&'a str>,
}

impl<'a> DeletePin<'a> {
    pub(crate) const fn new(
        http: &'a Client,
        channel_id: ChannelId,
        message_id: MessageId,
    ) -> Self {
        Self {
            channel_id,
            http,
            message_id,
            reason: None,
        }
    }

    /// Execute the request, returning a future resolving to a [`Response`].
    ///
    /// [`Response`]: crate::response::Response
    pub fn exec(self) -> ResponseFuture<EmptyBody> {
        let http = self.http;

        match self.into_request() {
            Ok(request) => http.request(request),
            Err(source) => ResponseFuture::error(source),
        }
    }
}

impl<'a> AuditLogReason<'a> for DeletePin<'a> {
    fn reason(mut self, reason: &'a str) -> Result<Self, AuditLogReasonError> {
        self.reason.replace(AuditLogReasonError::validate(reason)?);

        Ok(self)
    }
}

impl IntoRequest for DeletePin<'_> {
    fn into_request(self) -> Result<Request, crate::Error> {
        let mut request = Request::builder(&Route::UnpinMessage {
            channel_id: self.channel_id.get(),
            message_id: self.message_id.get(),
        });

        if let Some(reason) = &self.reason {
            let header = request::audit_header(reason)?;

            request = request.headers(header);
        }

        Ok(request.build())
    }
}
