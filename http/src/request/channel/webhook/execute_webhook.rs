use crate::request::{
    channel::allowed_mentions::{AllowedMentions, AllowedMentionsBuilder, Unspecified},
    prelude::*,
};
use futures_util::future::TryFutureExt;
use twilight_model::{
    channel::{embed::Embed, Message},
    id::WebhookId,
};

#[derive(Default, Serialize)]
pub(crate) struct ExecuteWebhookFields {
    #[serde(skip_serializing_if = "Option::is_none")]
    avatar_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    content: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    embeds: Option<Vec<Embed>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    file: Option<Vec<u8>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    payload_json: Option<Vec<u8>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tts: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    username: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    wait: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) allowed_mentions: Option<AllowedMentions>,
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
/// [`content`]: Self::content
/// [`embeds`]: Self::embeds
/// [`file`]: Self::file
pub struct ExecuteWebhook<'a> {
    pub(crate) fields: ExecuteWebhookFields,
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

    /// Return a new [`AllowedMentionsBuilder`].
    pub fn allowed_mentions(
        self,
    ) -> AllowedMentionsBuilder<'a, Unspecified, Unspecified, Unspecified> {
        AllowedMentionsBuilder::for_webhook(self)
    }

    /// The URL of the avatar of the webhook.
    pub fn avatar_url(mut self, avatar_url: impl Into<String>) -> Self {
        self.fields.avatar_url.replace(avatar_url.into());

        self
    }

    /// The content of the webook's message.
    ///
    /// Up to 2000 UTF-16 codepoints, same as a message.
    pub fn content(mut self, content: impl Into<String>) -> Self {
        self.fields.content.replace(content.into());

        self
    }

    /// Set the list of embeds of the webhook's message.
    pub fn embeds(mut self, embeds: Vec<Embed>) -> Self {
        self.fields.embeds.replace(embeds);

        self
    }

    /// Attach a file to the webhook.
    pub fn file(mut self, file: impl Into<Vec<u8>>) -> Self {
        self.fields.file.replace(file.into());

        self
    }

    /// JSON encoded body of any additional request fields. See [Discord Docs/Create Message]
    ///
    /// [Discord Docs/Create Message]: https://discord.com/developers/docs/resources/channel#create-message-params
    pub fn payload_json(mut self, payload_json: impl Into<Vec<u8>>) -> Self {
        self.fields.payload_json.replace(payload_json.into());

        self
    }

    /// Specify true if the message is TTS.
    pub fn tts(mut self, tts: bool) -> Self {
        self.fields.tts.replace(tts);

        self
    }

    /// Specify the username of the webhook's message.
    pub fn username(mut self, username: impl Into<String>) -> Self {
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
        let request = Request::from((
            crate::json_to_vec(&self.fields)?,
            Route::ExecuteWebhook {
                token: self.token.clone(),
                wait: self.fields.wait,
                webhook_id: self.webhook_id.0,
            },
        ));

        match self.fields.wait {
            Some(true) => {
                self.fut.replace(Box::pin(self.http.request(request)));
            }
            _ => {
                self.fut
                    .replace(Box::pin(self.http.verify(request).map_ok(|_| None)));
            }
        }

        Ok(())
    }
}

poll_req!(ExecuteWebhook<'_>, Option<Message>);
