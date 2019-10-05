use super::prelude::*;
use crate::Result;
use dawn_model::{channel::Webhook, id::WebhookId};
use futures_util::FutureExt;

#[derive(Serialize)]
pub struct UpdateWebhookWithToken<'a> {
    avatar: Option<String>,
    name: Option<String>,
    #[serde(skip)]
    fut: Option<Pending<'a, Webhook>>,
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
        self.fut.replace(
            self.http
                .request(Request::from((
                    serde_json::to_vec(&self)?,
                    Route::UpdateWebhook {
                        token: Some(self.token.clone()),
                        webhook_id: self.webhook_id.0,
                    },
                )))
                .boxed(),
        );

        Ok(())
    }
}

poll_req!(UpdateWebhookWithToken<'_>, Webhook);
