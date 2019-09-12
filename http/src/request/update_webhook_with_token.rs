use dawn_model::{
    channel::Webhook,
    id::WebhookId,
};
use super::prelude::*;

#[derive(Serialize)]
pub struct UpdateWebhookWithToken<'a> {
    avatar: Option<String>,
    name: Option<String>,
    #[serde(skip)]
    fut: Option<PendingBody<'a, Webhook>>,
    #[serde(skip)]
    http: &'a Client,
    #[serde(skip)]
    token: String,
    #[serde(skip)]
    webhook_id: WebhookId,
}

impl<'a> UpdateWebhookWithToken<'a> {
    pub(crate) fn new(
        http: &'a Client,
        webhook_id: impl Into<WebhookId>,
        token: impl Into<String>,
    ) -> Self {
        Self {
            avatar: None,
            fut: None,
            http,
            name: None,
            token: token.into(),
            webhook_id: webhook_id.into(),
        }
    }

    pub fn avatar(mut self, avatar: impl Into<String>) -> Self {
        self.avatar.replace(avatar.into());

        self
    }

    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name.replace(name.into());

        self
    }

    fn start(&mut self) -> Result<()> {
        self.fut.replace(self.http.request(Request {
            body: Some(serde_json::to_vec(&self)?),
            route: Route::UpdateWebhook {
                token: Some(&self.token),
                webhook_id: self.webhook_id.0,
            },
            ..Default::default()
        })?);

        Ok(())
    }
}

poll_req!(UpdateWebhookWithToken<'_>, Webhook);
