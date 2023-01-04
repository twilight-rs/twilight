use crate::{
    client::Client,
    error::Error,
    request::{
        attachment::{AttachmentManager, PartialAttachment},
        channel::webhook::ExecuteWebhookAndWait,
        Nullable, Request, TryIntoRequest,
    },
    response::{marker::EmptyBody, Response, ResponseFuture},
    routing::Route,
};
use serde::Serialize;
use std::future::IntoFuture;
use twilight_model::{
    channel::message::{AllowedMentions, Component, Embed, MessageFlags},
    http::attachment::Attachment,
    id::{
        marker::{ChannelMarker, WebhookMarker},
        Id,
    },
};
use twilight_validate::{
    message::{
        attachment as validate_attachment, components as validate_components,
        content as validate_content, embeds as validate_embeds, MessageValidationError,
        MessageValidationErrorType,
    },
    request::webhook_username as validate_webhook_username,
};

#[derive(Serialize)]
pub(crate) struct ExecuteWebhookFields<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    allowed_mentions: Option<Nullable<&'a AllowedMentions>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    attachments: Option<Vec<PartialAttachment<'a>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    avatar_url: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    components: Option<&'a [Component]>,
    #[serde(skip_serializing_if = "Option::is_none")]
    content: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    embeds: Option<&'a [Embed]>,
    #[serde(skip_serializing_if = "Option::is_none")]
    flags: Option<MessageFlags>,
    #[serde(skip_serializing_if = "Option::is_none")]
    payload_json: Option<&'a [u8]>,
    #[serde(skip_serializing_if = "Option::is_none")]
    thread_name: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tts: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    username: Option<&'a str>,
}

/// Execute a webhook, sending a message to its channel.
///
/// The message must include at least one of [`attachments`], [`components`],
/// [`content`], or [`embeds`].
///
/// # Examples
///
/// ```no_run
/// # #[tokio::main] async fn main() -> Result<(), Box<dyn std::error::Error>> {
/// use twilight_http::Client;
/// use twilight_model::id::Id;
///
/// let client = Client::new("my token".to_owned());
/// let id = Id::new(432);
///
/// client
///     .execute_webhook(id, "webhook token")
///     .content("Pinkie...")?
///     .await?;
/// # Ok(()) }
/// ```
///
/// [`attachments`]: Self::attachments
/// [`components`]: Self::components
/// [`content`]: Self::content
/// [`embeds`]: Self::embeds
#[must_use = "requests must be configured and executed"]
pub struct ExecuteWebhook<'a> {
    attachment_manager: AttachmentManager<'a>,
    fields: ExecuteWebhookFields<'a>,
    http: &'a Client,
    thread_id: Option<Id<ChannelMarker>>,
    token: &'a str,
    wait: bool,
    webhook_id: Id<WebhookMarker>,
}

impl<'a> ExecuteWebhook<'a> {
    pub(crate) const fn new(
        http: &'a Client,
        webhook_id: Id<WebhookMarker>,
        token: &'a str,
    ) -> Self {
        Self {
            attachment_manager: AttachmentManager::new(),
            fields: ExecuteWebhookFields {
                attachments: None,
                avatar_url: None,
                components: None,
                content: None,
                embeds: None,
                flags: None,
                payload_json: None,
                thread_name: None,
                tts: None,
                username: None,
                allowed_mentions: None,
            },
            http,
            thread_id: None,
            token,
            wait: false,
            webhook_id,
        }
    }

    /// Specify the [`AllowedMentions`] for the message.
    ///
    /// Unless otherwise called, the request will use the client's default
    /// allowed mentions. Set to `None` to ignore this default.
    pub const fn allowed_mentions(mut self, allowed_mentions: Option<&'a AllowedMentions>) -> Self {
        self.fields.allowed_mentions = Some(Nullable(allowed_mentions));

        self
    }

    /// Attach multiple files to the message.
    ///
    /// Calling this method will clear any previous calls.
    ///
    /// # Errors
    ///
    /// Returns an error of type [`AttachmentDescriptionTooLarge`] if
    /// the attachments's description is too large.
    ///
    /// Returns an error of type [`AttachmentFilename`] if any filename is
    /// invalid.
    ///
    /// [`AttachmentDescriptionTooLarge`]: twilight_validate::message::MessageValidationErrorType::AttachmentDescriptionTooLarge
    /// [`AttachmentFilename`]: twilight_validate::message::MessageValidationErrorType::AttachmentFilename
    pub fn attachments(
        mut self,
        attachments: &'a [Attachment],
    ) -> Result<Self, MessageValidationError> {
        attachments.iter().try_for_each(validate_attachment)?;

        self.attachment_manager = self
            .attachment_manager
            .set_files(attachments.iter().collect());

        Ok(self)
    }

    /// The URL of the avatar of the webhook.
    pub const fn avatar_url(mut self, avatar_url: &'a str) -> Self {
        self.fields.avatar_url = Some(avatar_url);

        self
    }

    /// Set the message's list of [`Component`]s.
    ///
    /// Calling this method will clear previous calls.
    ///
    /// Requires a webhook owned by the application.
    ///
    /// # Errors
    ///
    /// Refer to the errors section of
    /// [`twilight_validate::component::component`] for a list of errors that
    /// may be returned as a result of validating each provided component.
    pub fn components(
        mut self,
        components: &'a [Component],
    ) -> Result<Self, MessageValidationError> {
        validate_components(components)?;

        self.fields.components = Some(components);

        Ok(self)
    }

    /// Set the message's content.
    ///
    /// The maximum length is 2000 UTF-16 characters.
    ///
    /// # Errors
    ///
    /// Returns an error of type [`ContentInvalid`] if the content length is too
    /// long.
    ///
    /// [`ContentInvalid`]: twilight_validate::message::MessageValidationErrorType::ContentInvalid
    pub fn content(mut self, content: &'a str) -> Result<Self, MessageValidationError> {
        validate_content(content)?;

        self.fields.content = Some(content);

        Ok(self)
    }

    /// Set the message's list of embeds.
    ///
    /// Calling this method will clear previous calls.
    ///
    /// The amount of embeds must not exceed [`EMBED_COUNT_LIMIT`]. The total
    /// character length of each embed must not exceed [`EMBED_TOTAL_LENGTH`]
    /// characters. Additionally, the internal fields also have character
    /// limits. Refer to [Discord Docs/Embed Limits] for more information.
    ///
    /// # Errors
    ///
    /// Returns an error of type [`TooManyEmbeds`] if there are too many embeds.
    ///
    /// Otherwise, refer to the errors section of
    /// [`twilight_validate::embed::embed`] for a list of errors that may occur.
    ///
    /// [Discord Docs/Embed Limits]: https://discord.com/developers/docs/resources/channel#embed-limits
    /// [`EMBED_COUNT_LIMIT`]: twilight_validate::message::EMBED_COUNT_LIMIT
    /// [`EMBED_TOTAL_LENGTH`]: twilight_validate::embed::EMBED_TOTAL_LENGTH
    /// [`TooManyEmbeds`]: twilight_validate::message::MessageValidationErrorType::TooManyEmbeds
    pub fn embeds(mut self, embeds: &'a [Embed]) -> Result<Self, MessageValidationError> {
        validate_embeds(embeds)?;

        self.fields.embeds = Some(embeds);

        Ok(self)
    }

    /// Set the message's flags.
    ///
    /// The only supported flag is [`SUPPRESS_EMBEDS`].
    ///
    /// [`SUPPRESS_EMBEDS`]: MessageFlags::SUPPRESS_EMBEDS
    pub const fn flags(mut self, flags: MessageFlags) -> Self {
        self.fields.flags = Some(flags);

        self
    }

    /// JSON encoded body of any additional request fields.
    ///
    /// If this method is called, all other fields are ignored, except for
    /// [`attachments`]. See [Discord Docs/Uploading Files].
    ///
    /// Without [`payload_json`]:
    ///
    /// ```no_run
    /// # #[tokio::main] async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// use twilight_http::Client;
    /// use twilight_model::id::Id;
    /// use twilight_util::builder::embed::EmbedBuilder;
    ///
    /// let client = Client::new("token".to_owned());
    ///
    /// let message = client
    ///     .execute_webhook(Id::new(1), "token here")
    ///     .content("some content")?
    ///     .embeds(&[EmbedBuilder::new().title("title").validate()?.build()])?
    ///     .wait()
    ///     .await?
    ///     .model()
    ///     .await?;
    ///
    /// assert_eq!(message.content, "some content");
    /// # Ok(()) }
    /// ```
    ///
    /// With [`payload_json`]:
    ///
    /// ```no_run
    /// # #[tokio::main] async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// use twilight_http::Client;
    /// use twilight_model::id::Id;
    /// use twilight_util::builder::embed::EmbedBuilder;
    ///
    /// let client = Client::new("token".to_owned());
    ///
    /// let message = client
    ///     .execute_webhook(Id::new(1), "token here")
    ///     .content("some content")?
    ///     .payload_json(br#"{ "content": "other content", "embeds": [ { "title": "title" } ] }"#)
    ///     .wait()
    ///     .await?
    ///     .model()
    ///     .await?;
    ///
    /// assert_eq!(message.content, "other content");
    /// # Ok(()) }
    /// ```
    ///
    /// [Discord Docs/Uploading Files]: https://discord.com/developers/docs/reference#uploading-files
    /// [`attachments`]: Self::attachments
    /// [`payload_json`]: Self::payload_json
    pub const fn payload_json(mut self, payload_json: &'a [u8]) -> Self {
        self.fields.payload_json = Some(payload_json);

        self
    }

    /// Execute in a thread belonging to the channel instead of the channel itself.
    pub fn thread_id(mut self, thread_id: Id<ChannelMarker>) -> Self {
        self.thread_id.replace(thread_id);

        self
    }

    /// Set the name of the created thread when used in a forum channel.
    pub const fn thread_name(mut self, thread_name: &'a str) -> Self {
        self.fields.thread_name = Some(thread_name);

        self
    }

    /// Specify true if the message is TTS.
    pub const fn tts(mut self, tts: bool) -> Self {
        self.fields.tts = Some(tts);

        self
    }

    /// Specify the username of the webhook's message.
    ///
    /// # Errors
    ///
    /// Returns an error of type [`WebhookUsername`] if the webhook's name is
    /// invalid.
    ///
    /// [`WebhookUsername`]: twilight_validate::request::ValidationErrorType::WebhookUsername
    pub fn username(mut self, username: &'a str) -> Result<Self, MessageValidationError> {
        validate_webhook_username(username).map_err(|source| {
            MessageValidationError::from_validation_error(
                MessageValidationErrorType::WebhookUsername,
                source,
            )
        })?;

        self.fields.username = Some(username);

        Ok(self)
    }

    /// Wait for the message to send before sending a response. See
    /// [Discord Docs/Execute Webhook].
    ///
    /// Using this will result in receiving the created message.
    ///
    /// [Discord Docs/Execute Webhook]: https://discord.com/developers/docs/resources/webhook#execute-webhook-querystring-params
    pub const fn wait(mut self) -> ExecuteWebhookAndWait<'a> {
        self.wait = true;

        ExecuteWebhookAndWait::new(self.http, self)
    }

    /// Execute the request, returning a future resolving to a [`Response`].
    #[deprecated(since = "0.14.0", note = "use `.await` or `into_future` instead")]
    pub fn exec(self) -> ResponseFuture<EmptyBody> {
        self.into_future()
    }
}

impl IntoFuture for ExecuteWebhook<'_> {
    type Output = Result<Response<EmptyBody>, Error>;

    type IntoFuture = ResponseFuture<EmptyBody>;

    fn into_future(self) -> Self::IntoFuture {
        let http = self.http;

        match self.try_into_request() {
            Ok(request) => http.request(request),
            Err(source) => ResponseFuture::error(source),
        }
    }
}

impl TryIntoRequest for ExecuteWebhook<'_> {
    fn try_into_request(mut self) -> Result<Request, Error> {
        let mut request = Request::builder(&Route::ExecuteWebhook {
            thread_id: self.thread_id.map(Id::get),
            token: self.token,
            wait: Some(self.wait),
            webhook_id: self.webhook_id.get(),
        });

        // Webhook executions don't need the authorization token, only the
        // webhook token.
        request = request.use_authorization_token(false);

        // Set the default allowed mentions if required.
        if self.fields.allowed_mentions.is_none() {
            if let Some(allowed_mentions) = self.http.default_allowed_mentions() {
                self.fields.allowed_mentions = Some(Nullable(Some(allowed_mentions)));
            }
        }

        // Determine whether we need to use a multipart/form-data body or a JSON
        // body.
        if !self.attachment_manager.is_empty() {
            let form = if let Some(payload_json) = self.fields.payload_json {
                self.attachment_manager.build_form(payload_json)
            } else {
                self.fields.attachments = Some(self.attachment_manager.get_partial_attachments());

                let fields = crate::json::to_vec(&self.fields).map_err(Error::json)?;

                self.attachment_manager.build_form(fields.as_ref())
            };

            request = request.form(form);
        } else if let Some(payload_json) = self.fields.payload_json {
            request = request.body(payload_json.to_vec());
        } else {
            request = request.json(&self.fields)?;
        }

        Ok(request.build())
    }
}
