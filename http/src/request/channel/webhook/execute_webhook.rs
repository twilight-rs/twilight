use crate::{
    client::Client,
    error::{Error, ErrorType},
    request::{Form, PendingOption, Request},
    routing::Route,
};
use hyper::StatusCode;
use serde::Serialize;
use std::{
    future::Future,
    pin::Pin,
    task::{Context, Poll},
};
use twilight_model::{
    channel::{embed::Embed, message::AllowedMentions, Message},
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
    files: Vec<(String, Vec<u8>)>,
    fut: Option<PendingOption<'a>>,
    http: &'a Client,
    token: String,
    webhook_id: WebhookId,
}

impl<'a> ExecuteWebhook<'a> {
    pub(crate) fn new(http: &'a Client, webhook_id: WebhookId, token: impl Into<String>) -> Self {
        Self {
            fields: ExecuteWebhookFields::default(),
            files: Vec::new(),
            fut: None,
            http,
            token: token.into(),
            webhook_id,
        }
    }

    /// Specify the [`AllowedMentions`] for the webhook message.
    pub fn allowed_mentions(mut self, allowed_mentions: AllowedMentions) -> Self {
        self.fields.allowed_mentions.replace(allowed_mentions);

        self
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
    ///
    /// This method is repeatable.
    #[deprecated(since = "0.5.5", note = "will be removed in favor of `files`")]
    pub fn file(mut self, name: impl Into<String>, file: impl Into<Vec<u8>>) -> Self {
        self.files.push((name.into(), file.into()));

        self
    }

    /// Attach multiple files to the webhook.
    pub fn files<N: Into<String>, F: Into<Vec<u8>>>(
        mut self,
        files: impl IntoIterator<Item = (N, F)>,
    ) -> Self {
        self.files.extend(
            files
                .into_iter()
                .map(|(name, file)| (name.into(), file.into())),
        );

        self
    }

    /// JSON encoded body of any additional request fields.
    ///
    /// If this method is called, all other fields are ignored, except for
    /// [`file`]. See [Discord Docs/Create Message].
    ///
    /// # Examples
    ///
    /// Without [`payload_json`]:
    ///
    /// ```rust,no_run
    /// use twilight_embed_builder::EmbedBuilder;
    /// # use twilight_http::Client;
    /// use twilight_model::id::{MessageId, WebhookId};
    ///
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// # let client = Client::new("token");
    /// let message = client.execute_webhook(WebhookId(1), "token here")
    ///     .content("some content")
    ///     .embeds(vec![EmbedBuilder::new().title("title").build()?])
    ///     .wait(true)
    ///     .await?
    ///     .unwrap();
    ///
    /// assert_eq!(message.content, "some content");
    /// # Ok(()) }
    /// ```
    ///
    /// With [`payload_json`]:
    ///
    /// ```rust,no_run
    /// # use twilight_http::Client;
    /// use twilight_model::id::{MessageId, WebhookId};
    ///
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// # let client = Client::new("token");
    /// let message = client.execute_webhook(WebhookId(1), "token here")
    ///     .content("some content")
    ///     .payload_json(r#"{ "content": "other content", "embeds": [ { "title": "title" } ] }"#)
    ///     .wait(true)
    ///     .await?
    ///     .unwrap();
    ///
    /// assert_eq!(message.content, "other content");
    /// # Ok(()) }
    /// ```
    ///
    /// [`payload_json`]: Self::payload_json
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

    fn start(&mut self) -> Result<(), Error> {
        let mut request = Request::builder(Route::ExecuteWebhook {
            token: self.token.clone(),
            wait: self.fields.wait,
            webhook_id: self.webhook_id.0,
        });

        // Webhook executions don't need the authorization token, only the
        // webhook token.
        request = request.use_authorization_token(false);

        if !self.files.is_empty() || self.fields.payload_json.is_some() {
            let mut form = Form::new();

            for (index, (name, file)) in self.files.drain(..).enumerate() {
                form.file(format!("{}", index).as_bytes(), name.as_bytes(), &file);
            }

            if let Some(payload_json) = &self.fields.payload_json {
                form.payload_json(&payload_json);
            } else {
                let body = crate::json::to_vec(&self.fields).map_err(Error::json)?;
                form.payload_json(&body);
            }

            request = request.form(form);
        } else {
            request = request.json(&self.fields)?;
        }

        self.fut
            .replace(Box::pin(self.http.request_bytes(request.build())));

        Ok(())
    }
}

impl Future for ExecuteWebhook<'_> {
    type Output = Result<Option<Message>, Error>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        loop {
            if let Some(fut) = self.as_mut().fut.as_mut() {
                let bytes = match fut.as_mut().poll(cx) {
                    Poll::Ready(Ok(bytes)) => bytes,
                    Poll::Ready(Err(Error {
                        kind: ErrorType::Response { status, .. },
                        source: None,
                    })) if status == StatusCode::NOT_FOUND => {
                        return Poll::Ready(Ok(None));
                    }
                    Poll::Ready(Err(why)) => return Poll::Ready(Err(why)),
                    Poll::Pending => return Poll::Pending,
                };

                if !self.fields.wait.unwrap_or_default() {
                    return Poll::Ready(Ok(None));
                }

                let message = crate::json::parse_bytes::<Message>(&bytes)?;

                return Poll::Ready(Ok(Some(message)));
            }

            if let Err(why) = self.as_mut().start() {
                return Poll::Ready(Err(why));
            }
        }
    }
}
