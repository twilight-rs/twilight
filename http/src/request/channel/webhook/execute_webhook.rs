use crate::request::prelude::*;
use dawn_model::{
    channel::{embed::Embed, Message},
    id::WebhookId,
};

#[derive(Default, Serialize)]
struct ExecuteWebhookFields {
    avatar_url: Option<String>,
    content: Option<String>,
    embeds: Option<Vec<Embed>>,
    file: Option<Vec<u8>>,
    payload_json: Option<Vec<u8>>,
    tts: Option<bool>,
    username: Option<String>,
    wait: Option<bool>,
}

pub struct ExecuteWebhook<'a> {
    fields: ExecuteWebhookFields,
    fut: Option<Pending<'a, Option<Message>>>,
    http: &'a Client,
    token: String,
    webhook_id: WebhookId,
}

impl<'a> ExecuteWebhook<'a> {
    pub(crate) fn new(http: &'a Client, webhook_id: WebhookId, token: impl Into<String>) -> Self {
        Self {
            fields: ExecuteWebhookFields::default(),
            fut: None,
            http,
            token: token.into(),
            webhook_id,
        }
    }

    pub fn avatar_url(mut self, avatar_url: impl Into<String>) -> Self {
        self.fields.avatar_url.replace(avatar_url.into());

        self
    }

    pub fn content(mut self, content: impl Into<String>) -> Self {
        self.fields.content.replace(content.into());

        self
    }

    pub fn embeds(mut self, embeds: Vec<Embed>) -> Self {
        self.fields.embeds.replace(embeds);

        self
    }

    pub fn file(mut self, file: impl Into<Vec<u8>>) -> Self {
        self.fields.file.replace(file.into());

        self
    }

    pub fn payload_json(mut self, payload_json: impl Into<Vec<u8>>) -> Self {
        self.fields.payload_json.replace(payload_json.into());

        self
    }

    pub fn tts(mut self, tts: bool) -> Self {
        self.fields.tts.replace(tts);

        self
    }

    pub fn username(mut self, username: impl Into<String>) -> Self {
        self.fields.username.replace(username.into());

        self
    }

    pub fn wait(mut self, wait: bool) -> Self {
        self.fields.wait.replace(wait);

        self
    }

    fn start(&mut self) -> Result<()> {
        self.fut.replace(Box::pin(self.http.request(Request::from((
            serde_json::to_vec(&self.fields)?,
            Route::ExecuteWebhook {
                token: self.token.to_owned(),
                wait: self.fields.wait,
                webhook_id: self.webhook_id.0,
            },
        )))));

        Ok(())
    }
}

poll_req!(ExecuteWebhook<'_>, Option<Message>);
