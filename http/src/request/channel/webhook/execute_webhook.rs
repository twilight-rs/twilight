use crate::json_to_vec;
use crate::request::prelude::*;
use std::borrow::Cow;
use twilight_model::{
    channel::{embed::Embed, Message},
    id::WebhookId,
};

#[derive(Default, Serialize)]
struct ExecuteWebhookFields<'a> {
    avatar_url: Option<Cow<'a, str>>,
    content: Option<Cow<'a, str>>,
    embeds: Option<Cow<'a, [Embed]>>,
    file: Option<Cow<'a, [u8]>>,
    payload_json: Option<Cow<'a, [u8]>>,
    tts: Option<bool>,
    username: Option<Cow<'a, str>>,
    wait: Option<bool>,
}

/// Executes a webhook, sending a message to its channel.
///
/// You can only specify one of [`content`], [`embeds`], or [`file`].
///
/// # Examples
///
/// ```rust,no_run
/// use twilight_http::Client;
/// use twilight_model::id::WebhookId;
///
/// # #[tokio::main]
/// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
/// let client = Client::new("my token");
/// let id = WebhookId(432);
///
/// let webhook = client
///     .execute_webhook(id, "webhook token")
///     .content("Pinkie...")
///     .await?;
/// # Ok(()) }
/// ```
///
/// [`content`]: #method.content
/// [`embeds`]: #method.embeds
/// [`file`]: #method.file
pub struct ExecuteWebhook<'a> {
    fields: ExecuteWebhookFields<'a>,
    fut: Option<Pending<'a, Option<Message>>>,
    http: &'a Client,
    token: Cow<'a, str>,
    webhook_id: WebhookId,
}

impl<'a> ExecuteWebhook<'a> {
    pub(crate) fn new(
        http: &'a Client,
        webhook_id: WebhookId,
        token: impl Into<Cow<'a, str>>,
    ) -> Self {
        Self {
            fields: ExecuteWebhookFields::default(),
            fut: None,
            http,
            token: token.into(),
            webhook_id,
        }
    }

    /// The URL of the avatar of the webhook.
    pub fn avatar_url(mut self, avatar_url: impl Into<Cow<'a, str>>) -> Self {
        self.fields.avatar_url.replace(avatar_url.into());

        self
    }

    /// The content of the webook's message.
    ///
    /// Up to 2000 UTF-16 codepoints, same as a message.
    pub fn content(mut self, content: impl Into<Cow<'a, str>>) -> Self {
        self.fields.content.replace(content.into());

        self
    }

    /// Set the list of embeds of the webhook's message.
    pub fn embeds(mut self, embeds: impl Into<Cow<'a, [Embed]>>) -> Self {
        self.fields.embeds.replace(embeds.into());

        self
    }

    /// Attach a file to the webhook.
    pub fn file(mut self, file: impl Into<Cow<'a, [u8]>>) -> Self {
        self.fields.file.replace(file.into());

        self
    }

    /// JSON encoded body of any additional request fields. See [Discord Docs/Create Message]
    ///
    /// [Discord Docs/Create Message]: https://discord.com/developers/docs/resources/channel#create-message-params
    pub fn payload_json(mut self, payload_json: impl Into<Cow<'a, [u8]>>) -> Self {
        self.fields.payload_json.replace(payload_json.into());

        self
    }

    /// Specify true if the message is TTS.
    pub fn tts(mut self, tts: bool) -> Self {
        self.fields.tts.replace(tts);

        self
    }

    /// Specify the username of the webhook's message.
    pub fn username(mut self, username: impl Into<Cow<'a, str>>) -> Self {
        self.fields.username.replace(username.into());

        self
    }

    /// If true, wait for the message to send before sending a response. See [Discord Docs/Execute
    /// Webhook]
    ///
    /// [Discord Docs/Execute Webhook]: https://discord.com/developers/docs/resources/webhook#execute-webhook-querystring-params
    pub fn wait(mut self, wait: bool) -> Self {
        self.fields.wait.replace(wait);

        self
    }

    fn start(&mut self) -> Result<()> {
        self.fut.replace(Box::pin(self.http.request(Request::from((
            json_to_vec(&self.fields)?,
            Route::ExecuteWebhook {
                token: &self.token,
                wait: self.fields.wait,
                webhook_id: self.webhook_id.0,
            },
        )))));

        Ok(())
    }
}

poll_req!(ExecuteWebhook<'_>, Option<Message>);
