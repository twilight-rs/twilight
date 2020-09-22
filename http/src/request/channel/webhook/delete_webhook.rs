use crate::request::prelude::*;
use twilight_model::id::WebhookId;

struct DeleteWebhookParams {
    token: Option<String>,
}

/// Delete a webhook by its ID.
pub struct DeleteWebhook<'a> {
    fields: DeleteWebhookParams,
    fut: Option<Pending<'a, ()>>,
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
                Route::DeleteWebhook {
                    webhook_id: self.id.0,
                    token: self.fields.token.clone(),
                },
            ))
        } else {
            Request::from(Route::DeleteWebhook {
                webhook_id: self.id.0,
                token: self.fields.token.clone(),
            })
        };

        self.fut.replace(Box::pin(self.http.verify(request)));

        Ok(())
    }
}

impl<'a> AuditLogReason for DeleteWebhook<'a> {
    fn reason(mut self, reason: impl Into<String>) -> Result<Self, AuditLogReasonError> {
        let reason = AuditLogReasonError::validate(reason.into())?;
        self.reason.replace(reason);

        Ok(self)
    }
}

poll_req!(DeleteWebhook<'_>, ());
