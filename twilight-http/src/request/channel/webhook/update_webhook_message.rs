//! Update a message created by a webhook via execution.

use crate::{
    client::Client,
    error::Error,
    request::{
        Method, Nullable, Request, TryIntoRequest,
        attachment::{AttachmentManager, PartialAttachment},
        path::Path,
    },
    response::{Response, ResponseFuture},
};
use serde::Serialize;
use std::future::IntoFuture;
use twilight_model::{
    channel::{
        Message,
        message::{AllowedMentions, Component, Embed},
    },
    http::attachment::Attachment,
    id::{
        Id,
        marker::{AttachmentMarker, ChannelMarker, MessageMarker, WebhookMarker},
    },
};
use twilight_validate::message::{
    MessageValidationError, attachment as validate_attachment, content as validate_content,
    embeds as validate_embeds,
};

#[derive(Serialize)]
struct UpdateWebhookMessageBody<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    allowed_mentions: Option<Nullable<&'a AllowedMentions>>,
    /// List of attachments to keep, and new attachments to add.
    #[serde(skip_serializing_if = "Option::is_none")]
    attachments: Option<Nullable<Vec<PartialAttachment<'a>>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    components: Option<Nullable<&'a [Component]>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    content: Option<Nullable<&'a str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    embeds: Option<Nullable<&'a [Embed]>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    payload_json: Option<&'a [u8]>,
}

struct UpdateWebhookMessagePath<'a> {
    message_id: Id<MessageMarker>,
    thread_id: Option<Id<ChannelMarker>>,
    token: &'a str,
    webhook_id: Id<WebhookMarker>,
}

/// Update a message created by a webhook.
///
/// You can pass [`None`] to any of the methods to remove the associated field.
/// Pass [`None`] to [`content`] to remove the content. You must ensure that the
/// message still contains at least one of [`attachments`], [`components`],
/// [`content`], or [`embeds`].
///
/// # Examples
///
/// Update a webhook's message by setting the content to `test <@3>` -
/// attempting to mention user ID 3 - while specifying that no entities can be
/// mentioned.
///
/// ```no_run
/// # #[tokio::main] async fn main() -> Result<(), Box<dyn std::error::Error>> {
/// use twilight_http::Client;
/// use twilight_model::{channel::message::AllowedMentions, id::Id};
///
/// let client = Client::new("token".to_owned());
/// client
///     .update_webhook_message(Id::new(1), "token here", Id::new(2))
///     // By creating a default set of allowed mentions, no entity can be
///     // mentioned.
///     .allowed_mentions(Some(&AllowedMentions::default()))
///     .content(Some("test <@3>"))
///     .await?;
/// # Ok(()) }
/// ```
///
/// [`attachments`]: Self::attachments
/// [`components`]: Self::components
/// [`content`]: Self::content
/// [`embeds`]: Self::embeds
#[must_use = "requests must be configured and executed"]
pub struct UpdateWebhookMessage<'a> {
    attachment_manager: AttachmentManager<'a>,
    body: Result<UpdateWebhookMessageBody<'a>, MessageValidationError>,
    http: &'a Client,
    path: UpdateWebhookMessagePath<'a>,
}

impl<'a> UpdateWebhookMessage<'a> {
    pub(crate) const fn new(
        http: &'a Client,
        webhook_id: Id<WebhookMarker>,
        token: &'a str,
        message_id: Id<MessageMarker>,
    ) -> Self {
        Self {
            attachment_manager: AttachmentManager::new(),
            body: Ok(UpdateWebhookMessageBody {
                allowed_mentions: None,
                attachments: None,
                components: None,
                content: None,
                embeds: None,
                payload_json: None,
            }),
            http,
            path: UpdateWebhookMessagePath {
                message_id,
                thread_id: None,
                token,
                webhook_id,
            },
        }
    }

    /// Specify the [`AllowedMentions`] for the message.
    ///
    /// Unless otherwise called, the request will use the client's default
    /// allowed mentions. Set to `None` to ignore this default.
    pub const fn allowed_mentions(mut self, allowed_mentions: Option<&'a AllowedMentions>) -> Self {
        if let Ok(body) = self.body.as_mut() {
            body.allowed_mentions = Some(Nullable(allowed_mentions));
        }

        self
    }

    /// Attach multiple new files to the message.
    ///
    /// This method clears previous calls.
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
    pub fn attachments(mut self, attachments: &'a [Attachment]) -> Self {
        if self.body.is_ok() {
            if let Err(source) = attachments.iter().try_for_each(validate_attachment) {
                self.body = Err(source);
            } else {
                self.attachment_manager = self
                    .attachment_manager
                    .set_files(attachments.iter().collect());
            }
        }

        self
    }

    /// Set the message's list of [`Component`]s.
    ///
    /// Calling this method will clear previous calls.
    ///
    /// Requires a webhook owned by the application.
    ///
    /// # Editing
    ///
    /// Pass [`None`] to clear existing components.
    ///
    /// # Manual Validation
    ///
    /// Validation of components is not done automatically here, as we don't know which component
    /// version is in use, you can validate them manually using the [`twilight_validate::component::component_v1`]
    /// or [`twilight_validate::component::component_v2`] functions.
    pub fn components(mut self, components: Option<&'a [Component]>) -> Self {
        self.body = self.body.map(|mut body| {
            body.components = Some(Nullable(components));
            body
        });

        self
    }

    /// Set the message's content.
    ///
    /// The maximum length is 2000 UTF-16 characters.
    ///
    /// # Editing
    ///
    /// Pass [`None`] to remove the message content. This is impossible if it
    /// would leave the message empty of `attachments`, `content`, or `embeds`.
    ///
    /// # Errors
    ///
    /// Returns an error of type [`ContentInvalid`] if the content length is too
    /// long.
    ///
    /// [`ContentInvalid`]: twilight_validate::message::MessageValidationErrorType::ContentInvalid
    pub fn content(mut self, content: Option<&'a str>) -> Self {
        self.body = self.body.and_then(|mut body| {
            if let Some(content) = content {
                validate_content(content)?;
            }

            body.content = Some(Nullable(content));

            Ok(body)
        });

        self
    }

    /// Set the message's list of embeds.
    ///
    /// Calling this method will clear previous calls.
    ///
    /// The amount of embeds must not exceed [`EMBED_COUNT_LIMIT`]. The total
    /// character length of each embed must not exceed [`EMBED_TOTAL_LENGTH`]
    /// characters. Additionally, the internal fields also have character
    /// limits. See [Discord Docs/Embed Limits].
    ///
    /// # Editing
    ///
    /// To keep all embeds, do not call this method. To modify one or more
    /// embeds in the message, acquire them from the previous message, mutate
    /// them in place, then pass that list to this method. To remove all embeds,
    /// pass [`None`]. This is impossible if it would leave the message empty of
    /// attachments, content, or embeds.
    ///
    /// # Examples
    ///
    /// Create an embed and update the message with the new embed. The content
    /// of the original message is unaffected and only the embed(s) are
    /// modified.
    ///
    /// ```no_run
    /// # #[tokio::main] async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// use twilight_http::Client;
    /// use twilight_model::id::Id;
    /// use twilight_util::builder::embed::EmbedBuilder;
    ///
    /// let client = Client::new("token".to_owned());
    ///
    /// let webhook_id = Id::new(1);
    /// let message_id = Id::new(2);
    ///
    /// let embed = EmbedBuilder::new()
    ///     .description(
    ///         "Powerful, flexible, and scalable ecosystem of Rust \
    ///     libraries for the Discord API.",
    ///     )
    ///     .title("Twilight")
    ///     .url("https://twilight.rs")
    ///     .validate()?
    ///     .build();
    ///
    /// client
    ///     .update_webhook_message(webhook_id, "token", message_id)
    ///     .embeds(Some(&[embed]))
    ///     .await?;
    /// # Ok(()) }
    /// ```
    ///
    /// # Errors
    ///
    /// Returns an error of type [`TooManyEmbeds`] if there are too many embeds.
    ///
    /// Otherwise, refer to the errors section of
    /// [`twilight_validate::embed::embed`] for a list of errors that may occur.
    ///
    /// [`EMBED_COUNT_LIMIT`]: twilight_validate::message::EMBED_COUNT_LIMIT
    /// [`EMBED_TOTAL_LENGTH`]: twilight_validate::embed::EMBED_TOTAL_LENGTH
    /// [`TooManyEmbeds`]: twilight_validate::message::MessageValidationErrorType::TooManyEmbeds
    /// [Discord Docs/Embed Limits]: https://discord.com/developers/docs/resources/channel#embed-limits
    pub fn embeds(mut self, embeds: Option<&'a [Embed]>) -> Self {
        self.body = self.body.and_then(|mut body| {
            if let Some(embeds) = embeds {
                validate_embeds(embeds)?;
            }

            body.embeds = Some(Nullable(embeds));

            Ok(body)
        });

        self
    }

    /// Specify multiple [`Id<AttachmentMarker>`]s already present in the target
    /// message to keep.
    ///
    /// If called, all unspecified attachments (except ones added with
    /// [`attachments`]) will be removed from the message. If not called, all
    /// attachments will be kept.
    ///
    /// [`attachments`]: Self::attachments
    pub fn keep_attachment_ids(mut self, attachment_ids: &'a [Id<AttachmentMarker>]) -> Self {
        if let Ok(body) = self.body.as_mut() {
            self.attachment_manager = self.attachment_manager.set_ids(attachment_ids.to_vec());

            // Set an empty list. This will be overwritten in `TryIntoRequest` if
            // the actual list is not empty.
            body.attachments = Some(Nullable(Some(Vec::new())));
        }

        self
    }

    /// JSON encoded body of request fields.
    ///
    /// If this method is called, all other methods are ignored, except for
    /// [`attachments`]. If uploading attachments, you must ensure that the
    /// `attachments` key corresponds properly to the provided list. See
    /// [Discord Docs/Create Message] and [`ExecuteWebhook::payload_json`].
    ///
    /// [`attachments`]: Self::attachments
    /// [`ExecuteWebhook::payload_json`]: super::ExecuteWebhook::payload_json
    /// [Discord Docs/Create Message]: https://discord.com/developers/docs/resources/channel#create-message-params
    pub const fn payload_json(mut self, payload_json: &'a [u8]) -> Self {
        if let Ok(body) = self.body.as_mut() {
            body.payload_json = Some(payload_json);
        }

        self
    }

    /// Update in a thread belonging to the channel instead of the channel
    /// itself.
    pub const fn thread_id(mut self, thread_id: Id<ChannelMarker>) -> Self {
        self.path.thread_id.replace(thread_id);

        self
    }
}

impl IntoFuture for UpdateWebhookMessage<'_> {
    type Output = Result<Response<Message>, Error>;

    type IntoFuture = ResponseFuture<Message>;

    fn into_future(self) -> Self::IntoFuture {
        let http = self.http;

        match self.try_into_request() {
            Ok(request) => http.request(request),
            Err(source) => ResponseFuture::error(source),
        }
    }
}

impl TryIntoRequest for UpdateWebhookMessage<'_> {
    fn try_into_request(self) -> Result<Request, Error> {
        let UpdateWebhookMessagePath {
            message_id,
            thread_id,
            token,
            webhook_id,
        } = self.path;

        let mut body = self.body.map_err(Error::validation)?;
        let path = Path::builder()
            .resource("webhooks")
            .id(webhook_id)
            .no_resource()
            .id(token)
            .resource("messages")
            .id(message_id)
            .parameter(("thread_id", thread_id))
            .build()
            .map_err(Error::path)?;
        let mut request = Request::builder(Method::Patch, path);

        // Webhook executions don't need the authorization token, only the
        // webhook token.
        request = request.use_authorization_token(false);

        // Set the default allowed mentions if required.
        if body.allowed_mentions.is_none()
            && let Some(allowed_mentions) = self.http.default_allowed_mentions()
        {
            body.allowed_mentions = Some(Nullable(Some(allowed_mentions)));
        }

        // Determine whether we need to use a multipart/form-data body or a JSON
        // body.
        if !self.attachment_manager.is_empty() {
            let form = if let Some(payload_json) = body.payload_json {
                self.attachment_manager.build_form(payload_json)
            } else {
                body.attachments = Some(Nullable(Some(
                    self.attachment_manager.get_partial_attachments(),
                )));

                let body = crate::json::to_vec(&body).map_err(Error::json)?;

                self.attachment_manager.build_form(body.as_ref())
            };

            request = request.form(form);
        } else if let Some(payload_json) = body.payload_json {
            request = request.body(payload_json.to_vec());
        } else {
            request = request.json(&body);
        }

        request.build()
    }
}
