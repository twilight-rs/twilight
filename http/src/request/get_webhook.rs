use super::prelude::*;
use dawn_model::{channel::Webhook, id::WebhookId};

#[derive(Default)]
struct GetWebhookFields {
    token: Option<String>,
}

pub struct GetWebhook<'a> {
    fields: GetWebhookFields,
    fut: Option<Pending<'a, Option<Webhook>>>,
    http: &'a Client,
    id: WebhookId,
}

impl<'a> GetWebhook<'a> {
    pub(crate) fn new(http: &'a Client, id: impl Into<WebhookId>) -> Self {
        Self {
            fields: GetWebhookFields::default(),
            fut: None,
            http,
            id: id.into(),
        }
    }

    pub fn token(mut self, token: impl Into<String>) -> Self {
        self.fields.token.replace(token.into());

        self
    }

    fn start(&mut self) -> Result<()> {
        self.fut.replace(Box::pin(self.http.request(Request::from(
            Route::GetWebhook {
                token: self.fields.token.clone(),
                webhook_id: self.id.0,
            },
        ))));

        Ok(())
    }
}

poll_req!(GetWebhook<'_>, Option<Webhook>);
