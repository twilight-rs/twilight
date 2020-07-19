use crate::request::prelude::*;
use twilight_model::{channel::Webhook, id::WebhookId};

#[derive(Default)]
struct GetWebhookFields {
    token: Option<String>,
}

/// Get a webhook by ID.
pub struct GetWebhook<'a> {
    fields: GetWebhookFields,
    fut: Option<PendingOption<'a>>,
    http: &'a Client,
    id: WebhookId,
}

impl<'a> GetWebhook<'a> {
    pub(crate) fn new(http: &'a Client, id: WebhookId) -> Self {
        Self {
            fields: GetWebhookFields::default(),
            fut: None,
            http,
            id,
        }
    }

    /// Specify the token for auth, if not already authenticated with a Bot token.
    pub fn token(mut self, token: impl Into<String>) -> Self {
        self.fields.token.replace(token.into());

        self
    }

    fn start(&mut self) -> Result<()> {
        self.fut
            .replace(Box::pin(self.http.request_bytes(Request::from(
                Route::GetWebhook {
                    token: self.fields.token.clone(),
                    webhook_id: self.id.0,
                },
            ))));

        Ok(())
    }
}

poll_req!(opt, GetWebhook<'_>, Webhook);
