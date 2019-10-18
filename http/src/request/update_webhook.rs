use super::prelude::*;
use dawn_model::{
    channel::Webhook,
    id::{ChannelId, WebhookId},
};

#[derive(Default, Serialize)]
struct UpdateWebhookFields {
    avatar: Option<String>,
    channel_id: Option<ChannelId>,
    name: Option<String>,
}

pub struct UpdateWebhook<'a> {
    fields: UpdateWebhookFields,
    fut: Option<Pending<'a, Webhook>>,
    http: &'a Client,
    webhook_id: WebhookId,
}

impl<'a> UpdateWebhook<'a> {
    pub(crate) fn new(http: &'a Client, webhook_id: impl Into<WebhookId>) -> Self {
        Self {
            fields: UpdateWebhookFields::default(),
            fut: None,
            http,
            webhook_id: webhook_id.into(),
        }
    }

    pub fn avatar(mut self, avatar: impl Into<String>) -> Self {
        self.fields.avatar.replace(avatar.into());

        self
    }

    pub fn channel_id(mut self, channel_id: impl Into<ChannelId>) -> Self {
        self.fields.channel_id.replace(channel_id.into());

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
                token: None,
                webhook_id: self.webhook_id.0,
            },
        )))));

        Ok(())
    }
}

poll_req!(UpdateWebhook<'_>, Webhook);
