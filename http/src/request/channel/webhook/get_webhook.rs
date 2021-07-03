use crate::{
    client::Client,
    error::Error,
    request::{PendingResponse, Request},
    routing::Route,
};
use twilight_model::{channel::Webhook, id::WebhookId};

#[derive(Default)]
struct GetWebhookFields {
    token: Option<String>,
}

/// Get a webhook by ID.
pub struct GetWebhook<'a> {
    fields: GetWebhookFields,
    fut: Option<PendingResponse<'a, Webhook>>,
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

    /// Specify the token for auth, if not already authenticated with a Bot
    /// token.
    pub fn token(mut self, token: impl Into<String>) -> Self {
        self.fields.token.replace(token.into());

        self
    }

    fn start(&mut self) -> Result<(), Error> {
        let mut request = Request::builder(Route::GetWebhook {
            token: self.fields.token.clone(),
            webhook_id: self.id.0,
        });

        // If a webhook token has been configured, then we don't need to use
        // the client's authorization token.
        if self.fields.token.is_some() {
            request = request.use_authorization_token(false);
        }

        self.fut
            .replace(Box::pin(self.http.request(request.build())));

        Ok(())
    }
}

poll_req!(GetWebhook<'_>, Webhook);
