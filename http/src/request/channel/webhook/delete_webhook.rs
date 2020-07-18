use crate::request::prelude::*;
use std::borrow::Cow;
use twilight_model::id::WebhookId;

struct DeleteWebhookParams<'a> {
    token: Option<Cow<'a, str>>,
}

/// Delete a webhook by its ID.
pub struct DeleteWebhook<'a> {
    fields: DeleteWebhookParams<'a>,
    fut: Option<Pending<'a, ()>>,
    http: &'a Client,
    id: WebhookId,
    reason: Option<Cow<'a, str>>,
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
    pub fn token(mut self, token: impl Into<Cow<'a, str>>) -> Self {
        self.fields.token.replace(token.into());

        self
    }

    /// Attach an audit log reason to this request.
    pub fn reason(mut self, reason: impl Into<Cow<'a, str>>) -> Self {
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
                    token: self.fields.token.as_deref(),
                },
            ))
        } else {
            Request::from(Route::DeleteWebhook {
                webhook_id: self.id.0,
                token: self.fields.token.as_deref(),
            })
        };

        self.fut.replace(Box::pin(self.http.verify(request)));

        Ok(())
    }
}

poll_req!(DeleteWebhook<'_>, ());
