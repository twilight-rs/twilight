use super::prelude::*;
use dawn_model::id::WebhookId;

struct DeleteWebhookParams {
    token: Option<String>,
}

pub struct DeleteWebhook<'a> {
    fields: DeleteWebhookParams,
    fut: Option<Pending<'a, ()>>,
    http: &'a Client,
    id: WebhookId,
}

impl<'a> DeleteWebhook<'a> {
    pub(crate) fn new(http: &'a Client, id: WebhookId) -> Self {
        Self {
            fields: DeleteWebhookParams {
                token: None,
            },
            fut: None,
            http,
            id,
        }
    }

    pub fn token(mut self, token: impl Into<String>) -> Self {
        self.fields.token.replace(token.into());

        self
    }

    fn start(&mut self) -> Result<()> {
        self.fut.replace(Box::pin(self.http.verify(Request::from(
            Route::DeleteWebhook {
                webhook_id: self.id.0,
                token: self.fields.token.clone(),
            },
        ))));

        Ok(())
    }
}

poll_req!(DeleteWebhook<'_>, ());
