use super::prelude::*;
use dawn_model::{channel::Webhook, id::ChannelId};

#[derive(Serialize)]
struct CreateWebhookFields {
    avatar: Option<String>,
    name: String,
}

pub struct CreateWebhook<'a> {
    channel_id: ChannelId,
    fields: CreateWebhookFields,
    fut: Option<Pending<'a, Webhook>>,
    http: &'a Client,
}

impl<'a> CreateWebhook<'a> {
    pub(crate) fn new(
        http: &'a Client,
        channel_id: impl Into<ChannelId>,
        name: impl Into<String>,
    ) -> Self {
        Self {
            channel_id: channel_id.into(),
            fields: CreateWebhookFields {
                avatar: None,
                name: name.into(),
            },
            fut: None,
            http,
        }
    }

    pub fn avatar(mut self, avatar: impl Into<String>) -> Self {
        self.fields.avatar.replace(avatar.into());

        self
    }

    fn start(&mut self) -> Result<()> {
        self.fut.replace(Box::pin(self.http.request(Request::from((
            serde_json::to_vec(&self.fields)?,
            Route::CreateWebhook {
                channel_id: self.channel_id.0,
            },
        )))));

        Ok(())
    }
}

poll_req!(CreateWebhook<'_>, Webhook);
