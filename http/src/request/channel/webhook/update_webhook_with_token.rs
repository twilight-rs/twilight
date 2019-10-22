use crate::request::prelude::*;
use dawn_model::{channel::Webhook, id::WebhookId};

#[derive(Default, Serialize)]
struct UpdateWebhookWithTokenFields {
    avatar: Option<String>,
    name: Option<String>,
}

pub struct UpdateWebhookWithToken<'a> {
    fields: UpdateWebhookWithTokenFields,
    fut: Option<Pending<'a, Webhook>>,
    http: &'a Client,
    token: String,
    webhook_id: WebhookId,
}

impl<'a> UpdateWebhookWithToken<'a> {
    pub(crate) fn new(http: &'a Client, webhook_id: WebhookId, token: impl Into<String>) -> Self {
        Self {
            fields: UpdateWebhookWithTokenFields::default(),
            fut: None,
            http,
            token: token.into(),
            webhook_id,
        }
    }

    pub fn avatar(mut self, avatar: impl Into<String>) -> Self {
        self.fields.avatar.replace(avatar.into());

        self
    }

    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.fields.name.replace(name.into());

        self
    }

    fn start(&mut self) -> Result<()> {
        self.fut.replace(Box::pin(self.http.request(Request::from((
            serde_json::to_vec(&self.fields)?,
            Route::UpdateWebhook {
                token: Some(self.token.clone()),
                webhook_id: self.webhook_id.0,
            },
        )))));

        Ok(())
    }
}

poll_req!(UpdateWebhookWithToken<'_>, Webhook);
