use dawn_model::{
    channel::{
        embed::Embed,
        Message,
    },
    id::WebhookId,
};
use super::prelude::*;

#[derive(Serialize)]
pub struct ExecuteWebhook<'a> {
    avatar_url: Option<String>,
    content: Option<String>,
    embeds: Option<Vec<Embed>>,
    file: Option<Vec<u8>>,
    payload_json: Option<Vec<u8>>,
    tts: Option<bool>,
    username: Option<String>,
    wait: Option<bool>,
    #[serde(skip)]
    fut: Option<PendingBody<'a, Option<Message>>>,
    #[serde(skip)]
    http: &'a Client,
    #[serde(skip)]
    token: String,
    #[serde(skip)]
    webhook_id: WebhookId,
}

impl<'a> ExecuteWebhook<'a> {
    pub(crate) fn new(
        http: &'a Client,
        webhook_id: impl Into<WebhookId>,
        token: impl Into<String>,
    ) -> Self {
        Self {
            avatar_url: None,
            content: None,
            embeds: None,
            file: None,
            fut: None,
            http,
            payload_json: None,
            token: token.into(),
            tts: None,
            username: None,
            wait: None,
            webhook_id: webhook_id.into(),
        }
    }

    pub fn avatar_url(mut self, avatar_url: impl Into<String>) -> Self {
        self.avatar_url.replace(avatar_url.into());

        self
    }

    pub fn content(mut self, content: impl Into<String>) -> Self {
        self.content.replace(content.into());

        self
    }

    pub fn embeds(mut self, embeds: Vec<Embed>) -> Self {
        self.embeds.replace(embeds);

        self
    }

    pub fn file(mut self, file: impl Into<Vec<u8>>) -> Self {
        self.file.replace(file.into());

        self
    }

    pub fn payload_json(mut self, payload_json: impl Into<Vec<u8>>) -> Self {
        self.payload_json.replace(payload_json.into());

        self
    }

    pub fn tts(mut self, tts: bool) -> Self {
        self.tts.replace(tts);

        self
    }

    pub fn username(mut self, username: impl Into<String>) -> Self {
        self.username.replace(username.into());

        self
    }

    pub fn wait(mut self, wait: bool) -> Self {
        self.wait.replace(wait);

        self
    }

    fn start(&mut self) -> Result<()> {
        self.fut.replace(self.http.request(Request::from((
            serde_json::to_vec(self)?,
            Route::ExecuteWebhook {
                token: &self.token,
                wait: self.wait,
                webhook_id: self.webhook_id.0,
            },
        )))?);

        Ok(())
    }
}

poll_req!(ExecuteWebhook<'_>, Option<Message>);
