use crate::{
    client::Client,
    error::Error,
    request::{self, AuditLogReason, AuditLogReasonError, Request, TryIntoRequest},
    response::{marker::EmptyBody, ResponseFuture},
    routing::Route,
};
use twilight_model::id::{marker, Id};

/// Delete a message by [`Id<Channel>`] and [`Id<Message>`].
#[must_use = "requests must be configured and executed"]
pub struct DeleteMessage<'a> {
    channel_id: Id<marker::Channel>,
    http: &'a Client,
    message_id: Id<marker::Message>,
    reason: Option<&'a str>,
}

impl<'a> DeleteMessage<'a> {
    pub(crate) const fn new(
        http: &'a Client,
        channel_id: Id<marker::Channel>,
        message_id: Id<marker::Message>,
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

        match self.try_into_request() {
            Ok(request) => http.request(request),
            Err(source) => ResponseFuture::error(source),
        }
    }
}

impl<'a> AuditLogReason<'a> for DeleteMessage<'a> {
    fn reason(mut self, reason: &'a str) -> Result<Self, AuditLogReasonError> {
        self.reason.replace(AuditLogReasonError::validate(reason)?);

        Ok(self)
    }
}

impl TryIntoRequest for DeleteMessage<'_> {
    fn try_into_request(self) -> Result<Request, Error> {
        let mut request = Request::builder(&Route::DeleteMessage {
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
