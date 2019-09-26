use dawn_model::{
    channel::Webhook,
    id::WebhookId,
};
use super::prelude::*;

pub struct GetWebhook<'a> {
    token: Option<String>,
    fut: Option<Pin<Box<dyn Future<Output = Result<Option<Webhook>>> + Send + 'a>>>,
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
        self.fut.replace(Box::pin(self.http.request(Request::from(Route::GetWebhook {
            token: self.token.as_ref().map(ToOwned::to_owned),
            webhook_id: self.id.0,
        }))));

        Ok(())
    }
}

poll_req!(GetWebhook<'_>, Option<Webhook>);
