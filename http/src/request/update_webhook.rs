use dawn_model::{
    channel::Webhook,
    id::{ChannelId, WebhookId},
};
use super::prelude::*;

#[derive(Serialize)]
pub struct UpdateWebhook<'a> {
    avatar: Option<String>,
    channel_id: Option<ChannelId>,
    name: Option<String>,
    #[serde(skip)]
    fut: Option<Pin<Box<dyn Future<Output = Result<Webhook>> + Send + 'a>>>,
    #[serde(skip)]
    http: &'a Client,
    #[serde(skip)]
    webhook_id: WebhookId,
}

impl<'a> UpdateWebhook<'a> {
    pub(crate) fn new(
        http: &'a Client,
        webhook_id: impl Into<WebhookId>,
    ) -> Self {
        Self {
            avatar: None,
            channel_id: None,
            fut: None,
            http,
            name: None,
            webhook_id: webhook_id.into(),
        }
    }

    pub fn avatar(mut self, avatar: impl Into<String>) -> Self {
        self.avatar.replace(avatar.into());

        self
    }

    pub fn channel_id(mut self, channel_id: impl Into<ChannelId>) -> Self {
        self.channel_id.replace(channel_id.into());

        self
    }

    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name.replace(name.into());

        self
    }

    fn start(&mut self) -> Result<()> {
        self.fut.replace(Box::pin(self.http.request(Request::from((
            serde_json::to_vec(self)?,
            Route::UpdateWebhook {
                token: None,
                webhook_id: self.webhook_id.0,
            },
        )))));

        Ok(())
    }
}

poll_req!(UpdateWebhook<'_>, Webhook);
