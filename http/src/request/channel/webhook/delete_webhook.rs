use crate::{
    client::Client,
    error::Error,
    request::{self, AuditLogReason, AuditLogReasonError, PendingResponse, Request},
    response::marker::EmptyBody,
    routing::Route,
};
use twilight_model::id::WebhookId;

struct DeleteWebhookParams {
    token: Option<String>,
}

/// Delete a webhook by its ID.
pub struct DeleteWebhook<'a> {
    fields: DeleteWebhookParams,
    fut: Option<PendingResponse<'a, EmptyBody>>,
    http: &'a Client,
    id: WebhookId,
    reason: Option<String>,
}

impl<'a> DeleteWebhook<'a> {
    pub(crate) fn new(http: &'a Client, id: WebhookId) -> Self {
        Self {
            fields: DeleteWebhookParams { token: None },
            fut: None,
            http,
            id,
            reason: None,
        }
    }

    /// Specify the token for auth, if not already authenticated with a Bot token.
    pub fn token(mut self, token: impl Into<String>) -> Self {
        self.fields.token.replace(token.into());

        self
    }

    fn start(&mut self) -> Result<(), Error> {
        let mut request = Request::builder(Route::DeleteWebhook {
            webhook_id: self.id.0,
            token: self.fields.token.clone(),
        });

        if let Some(reason) = self.reason.as_ref() {
            request = request.headers(request::audit_header(reason)?);
        }

        self.fut
            .replace(Box::pin(self.http.request(request.build())));

        Ok(())
    }
}

impl<'a> AuditLogReason for DeleteWebhook<'a> {
    fn reason(mut self, reason: impl Into<String>) -> Result<Self, AuditLogReasonError> {
        self.reason
            .replace(AuditLogReasonError::validate(reason.into())?);

        Ok(self)
    }
}

poll_req!(DeleteWebhook<'_>, EmptyBody);
