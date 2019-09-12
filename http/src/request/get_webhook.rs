use dawn_model::{
    channel::Webhook,
    id::WebhookId,
};
use super::prelude::*;

pub struct GetWebhook<'a> {
    token: Option<String>,
    fut: Option<PendingBody<'a, Option<Webhook>>>,
    http: &'a Client,
    id: WebhookId,
}

impl<'a> GetWebhook<'a> {
    pub(crate) fn new(http: &'a Client, id: impl Into<WebhookId>) -> Self {
        Self {
            fut: None,
            http,
            id: id.into(),
            token: None,
        }
    }

    pub fn token(mut self, token: impl Into<String>) -> Self {
        self.token.replace(token.into());

        self
    }

    fn start(&mut self) -> Result<()> {
        self.fut.replace(self.http.request(Request {
            route: Route::GetWebhook {
                token: self.token.as_ref().map(AsRef::as_ref),
                webhook_id: self.id.0,
            },
            ..Default::default()
        })?);

        Ok(())
    }
}

poll_req!(GetWebhook<'_>, Option<Webhook>);
