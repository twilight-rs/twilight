use crate::request::prelude::*;
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
    reason: Option<String>,
}

impl<'a> CreateWebhook<'a> {
    pub(crate) fn new(http: &'a Client, channel_id: ChannelId, name: impl Into<String>) -> Self {
        Self {
            channel_id,
            fields: CreateWebhookFields {
                avatar: None,
                name: name.into(),
            },
            fut: None,
            http,
            reason: None,
        }
    }

    pub fn avatar(mut self, avatar: impl Into<String>) -> Self {
        self.fields.avatar.replace(avatar.into());

        self
    }

    pub fn reason(mut self, reason: impl Into<String>) -> Self {
        self.reason.replace(reason.into());

        self
    }

    fn start(&mut self) -> Result<()> {
        let request = if let Some(reason) = &self.reason {
            let headers = audit_header(&reason)?;
            Request::from((
                serde_json::to_vec(&self.fields)?,
                headers,
                Route::CreateWebhook {
                    channel_id: self.channel_id.0,
                },
            ))
        } else {
            Request::from((
                serde_json::to_vec(&self.fields)?,
                Route::CreateWebhook {
                    channel_id: self.channel_id.0,
                },
            ))
        };

        self.fut.replace(Box::pin(self.http.request(request)));

        Ok(())
    }
}

poll_req!(CreateWebhook<'_>, Webhook);
