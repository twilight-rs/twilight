use crate::request::prelude::*;
use std::borrow::Cow;
use twilight_model::{channel::Webhook, id::WebhookId};

#[derive(Default)]
struct GetWebhookFields<'a> {
    token: Option<Cow<'a, str>>,
}

/// Get a webhook by ID.
pub struct GetWebhook<'a> {
    fields: GetWebhookFields<'a>,
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
    pub fn token(mut self, token: impl Into<Cow<'a, str>>) -> Self {
        self.fields.token.replace(token.into());

        self
    }

    fn start(&mut self) -> Result<()> {
        self.fut
            .replace(Box::pin(self.http.request_bytes(Request::from(
                Route::GetWebhook {
                    token: self.fields.token.as_deref(),
                    webhook_id: self.id.0,
                },
            ))));

        Ok(())
    }
}

poll_req!(opt, GetWebhook<'_>, Webhook);
