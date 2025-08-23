//! Update a followup message created from a interaction.

use crate::{
    client::Client,
    error::Error,
    request::{
        Nullable, Request, TryIntoRequest,
        attachment::{AttachmentManager, PartialAttachment},
    },
    response::{Response, ResponseFuture},
    routing::Route,
};
use serde::Serialize;
use std::future::IntoFuture;
use twilight_model::{
    channel::message::{AllowedMentions, Component, Embed, Message},
    http::attachment::Attachment,
    id::{
        Id,
        marker::{ApplicationMarker, AttachmentMarker, MessageMarker},
    },
};
use twilight_validate::message::{
    MessageValidationError, attachment as validate_attachment, components as validate_components,
    content as validate_content, embeds as validate_embeds,
};

#[derive(Serialize)]
struct UpdateFollowupFields<'a> {
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

/// Edit a followup message of an interaction, by its token and the message ID.
///
/// You can pass [`None`] to any of the methods to remove the associated field.
/// Pass [`None`] to [`content`] to remove the content. You must ensure that the
/// message still contains at least one of [`attachments`], [`components`],
/// [`content`], or [`embeds`].
///
/// This endpoint is not bound to the application's global rate limit.
///
/// # Examples
///
/// Update a followup message by setting the content to `test <@3>` -
/// attempting to mention user ID 3 - while specifying that no entities can be
/// mentioned.
///
/// ```no_run
/// # #[tokio::main] async fn main() -> Result<(), Box<dyn std::error::Error>> {
/// use std::env;
/// use twilight_http::Client;
/// use twilight_model::{channel::message::AllowedMentions, id::Id};
///
/// let client = Client::new(env::var("DISCORD_TOKEN")?);
/// let application_id = Id::new(1);
///
/// client
///     .interaction(application_id)
///     .update_followup("token here", Id::new(2))
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
pub struct UpdateFollowup<'a> {
    application_id: Id<ApplicationMarker>,
    attachment_manager: AttachmentManager<'a>,
    fields: Result<UpdateFollowupFields<'a>, MessageValidationError>,
    http: &'a Client,
    message_id: Id<MessageMarker>,
    token: &'a str,
}

impl<'a> UpdateFollowup<'a> {
    pub(crate) const fn new(
        http: &'a Client,
        application_id: Id<ApplicationMarker>,
        token: &'a str,
        message_id: Id<MessageMarker>,
    ) -> Self {
        Self {
            application_id,
            attachment_manager: AttachmentManager::new(),
            fields: Ok(UpdateFollowupFields {
                allowed_mentions: None,
                attachments: None,
                components: None,
                content: None,
                embeds: None,
                payload_json: None,
            }),
            http,
            message_id,
            token,
        }
    }

    /// Specify the [`AllowedMentions`] for the message.
    ///
    /// Unless otherwise called, the request will use the client's default
    /// allowed mentions. Set to `None` to ignore this default.
    pub fn allowed_mentions(mut self, allowed_mentions: Option<&'a AllowedMentions>) -> Self {
        if let Ok(fields) = self.fields.as_mut() {
            fields.allowed_mentions = Some(Nullable(allowed_mentions));
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
    /// the attachments' description is too large.
    ///
    /// Returns an error of type [`AttachmentFilename`] if any filename is
    /// invalid.
    ///
    /// [`AttachmentDescriptionTooLarge`]: twilight_validate::message::MessageValidationErrorType::AttachmentDescriptionTooLarge
    /// [`AttachmentFilename`]: twilight_validate::message::MessageValidationErrorType::AttachmentFilename
    pub fn attachments(mut self, attachments: &'a [Attachment]) -> Self {
        if self.fields.is_ok() {
            if let Err(source) = attachments.iter().try_for_each(validate_attachment) {
                self.fields = Err(source);
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
    /// # Editing
    ///
    /// Pass [`None`] to clear existing components.
    ///
    /// # Errors
    ///
    /// Refer to the errors section of
    /// [`twilight_validate::component::component`] for a list of errors that
    /// may be returned as a result of validating each provided component.
    pub fn components(mut self, components: Option<&'a [Component]>) -> Self {
        self.fields = self.fields.and_then(|mut fields| {
            if let Some(components) = components {
                validate_components(components)?;
            }

            fields.components = Some(Nullable(components));

            Ok(fields)
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
        self.fields = self.fields.and_then(|mut fields| {
            if let Some(content_ref) = content.as_ref() {
                validate_content(content_ref)?;
            }

            fields.content = Some(Nullable(content));

            Ok(fields)
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
    /// `attachments`, `content`, or `embeds`.
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
    /// let application_id = Id::new(1);
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
    ///     .interaction(application_id)
    ///     .update_followup("token", message_id)
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
        self.fields = self.fields.and_then(|mut fields| {
            if let Some(embeds) = embeds {
                validate_embeds(embeds)?;
            }

            fields.embeds = Some(Nullable(embeds));

            Ok(fields)
        });

        self
    }

    /// Specify multiple [`Id<AttachmentMarker>`]s already present in the target
    /// message to keep.
    ///
    /// If called, all unspecified attachments (except ones added with
    /// [`attachments`]) will be removed from the message. This is impossible if
    /// it would leave the message empty of `attachments`, `content`, or
    /// `embeds`. If not called, all attachments will be kept.
    ///
    /// [`attachments`]: Self::attachments
    pub fn keep_attachment_ids(mut self, attachment_ids: &'a [Id<AttachmentMarker>]) -> Self {
        if let Ok(fields) = self.fields.as_mut() {
            self.attachment_manager = self.attachment_manager.set_ids(attachment_ids.to_vec());

            // Set an empty list. This will be overwritten in `TryIntoRequest` if
            // the actual list is not empty.
            fields.attachments = Some(Nullable(Some(Vec::new())));
        }

        self
    }

    /// JSON encoded body of any additional request fields.
    ///
    /// If this method is called, all other fields are ignored, except for
    /// [`attachments`]. See [Discord Docs/Uploading Files].
    ///
    /// # Examples
    ///
    /// See [`ExecuteWebhook::payload_json`] for examples.
    ///
    /// [`attachments`]: Self::attachments
    /// [`ExecuteWebhook::payload_json`]: crate::request::channel::webhook::ExecuteWebhook::payload_json
    /// [Discord Docs/Uploading Files]: https://discord.com/developers/docs/reference#uploading-files
    pub fn payload_json(mut self, payload_json: &'a [u8]) -> Self {
        if let Ok(fields) = self.fields.as_mut() {
            fields.payload_json = Some(payload_json);
        }

        self
    }
}

impl IntoFuture for UpdateFollowup<'_> {
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

impl TryIntoRequest for UpdateFollowup<'_> {
    fn try_into_request(self) -> Result<Request, Error> {
        let mut fields = self.fields.map_err(Error::validation)?;
        let mut request = Request::builder(&Route::UpdateWebhookMessage {
            message_id: self.message_id.get(),
            thread_id: None,
            token: self.token,
            webhook_id: self.application_id.get(),
        });

        // Interaction executions don't need the authorization token, only the
        // interaction token.
        request = request.use_authorization_token(false);

        // Set the default allowed mentions if required.
        if fields.allowed_mentions.is_none() {
            if let Some(allowed_mentions) = self.http.default_allowed_mentions() {
                fields.allowed_mentions = Some(Nullable(Some(allowed_mentions)));
            }
        }

        // Determine whether we need to use a multipart/form-data body or a JSON
        // body.
        if !self.attachment_manager.is_empty() {
            let form = if let Some(payload_json) = fields.payload_json {
                self.attachment_manager.build_form(payload_json)
            } else {
                fields.attachments = Some(Nullable(Some(
                    self.attachment_manager.get_partial_attachments(),
                )));

                let fields = crate::json::to_vec(&fields).map_err(Error::json)?;

                self.attachment_manager.build_form(fields.as_ref())
            };

            request = request.form(form);
        } else if let Some(payload_json) = fields.payload_json {
            request = request.body(payload_json.to_vec());
        } else {
            request = request.json(&fields);
        }

        request.build()
    }
}

#[cfg(test)]
mod tests {
    use crate::{client::Client, request::TryIntoRequest};
    use std::error::Error;
    use twilight_http_ratelimiting::Path;
    use twilight_model::id::Id;

    #[test]
    fn update_followup_message() -> Result<(), Box<dyn Error>> {
        let application_id = Id::new(1);
        let message_id = Id::new(2);
        let token = "foo".to_owned();

        let client = Client::new(String::new());
        let req = client
            .interaction(application_id)
            .update_followup(&token, message_id)
            .content(Some("test"))
            .try_into_request()?;

        assert!(!req.use_authorization_token());
        assert_eq!(
            &Path::WebhooksIdTokenMessagesId(application_id.get(), token),
            req.ratelimit_path()
        );

        Ok(())
    }
}
