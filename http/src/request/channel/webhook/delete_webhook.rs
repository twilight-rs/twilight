use crate::{
    client::Client,
    error::Error,
    request::{self, AuditLogReason, AuditLogReasonError, Request, TryIntoRequest},
    response::{marker::EmptyBody, ResponseFuture},
    routing::Route,
};
use twilight_model::id::{marker::WebhookMarker, Id};

struct DeleteWebhookParams<'a> {
    token: Option<&'a str>,
}

/// Delete a webhook by its ID.
#[must_use = "requests must be configured and executed"]
pub struct DeleteWebhook<'a> {
    fields: DeleteWebhookParams<'a>,
    http: &'a Client,
    id: Id<WebhookMarker>,
    reason: Option<&'a str>,
}

impl<'a> DeleteWebhook<'a> {
    pub(crate) const fn new(http: &'a Client, id: Id<WebhookMarker>) -> Self {
        Self {
            fields: DeleteWebhookParams { token: None },
            http,
            id,
            reason: None,
        }
    }

    /// Specify the token for auth, if not already authenticated with a Bot token.
    pub const fn token(mut self, token: &'a str) -> Self {
        self.fields.token = Some(token);

        self
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

impl<'a> AuditLogReason<'a> for DeleteWebhook<'a> {
    fn reason(mut self, reason: &'a str) -> Result<Self, AuditLogReasonError> {
        self.reason.replace(AuditLogReasonError::validate(reason)?);

        Ok(self)
    }
}

impl TryIntoRequest for DeleteWebhook<'_> {
    fn try_into_request(self) -> Result<Request, Error> {
        let mut request = Request::builder(&Route::DeleteWebhook {
            webhook_id: self.id.get(),
            token: self.fields.token,
        });

        if let Some(reason) = self.reason.as_ref() {
            let header = request::audit_header(reason)?;

            request = request.headers(header);
        }

        Ok(request.build())
    }
}
