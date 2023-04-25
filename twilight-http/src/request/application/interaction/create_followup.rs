use crate::{
    client::Client,
    error::Error,
    request::{
        attachment::{AttachmentManager, PartialAttachment},
        Nullable, Request, TryIntoRequest,
    },
    response::{Response, ResponseFuture},
    routing::Route,
};
use serde::Serialize;
use std::future::IntoFuture;
use twilight_model::{
    channel::message::{AllowedMentions, Component, Embed, Message, MessageFlags},
    http::attachment::Attachment,
    id::{marker::ApplicationMarker, Id},
};
use twilight_validate::message::{
    attachment as validate_attachment, components as validate_components,
    content as validate_content, embeds as validate_embeds, MessageValidationError,
};

#[derive(Serialize)]
struct CreateFollowupFields<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    allowed_mentions: Option<Nullable<&'a AllowedMentions>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    attachments: Option<Vec<PartialAttachment<'a>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    components: Option<&'a [Component]>,
    #[serde(skip_serializing_if = "Option::is_none")]
    content: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    embeds: Option<&'a [Embed]>,
    #[serde(skip_serializing_if = "Option::is_none")]
    payload_json: Option<&'a [u8]>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tts: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    flags: Option<MessageFlags>,
}

/// Create a followup message to an interaction, by its token.
///
/// The message must include at least one of [`attachments`], [`components`],
/// [`content`], or [`embeds`].
///
/// This endpoint is not bound to the application's global rate limit.
///
/// # Examples
///
/// ```no_run
/// # #[tokio::main] async fn main() -> Result<(), Box<dyn std::error::Error>> {
/// use std::env;
/// use twilight_http::Client;
/// use twilight_model::id::Id;
///
/// let client = Client::new(env::var("DISCORD_TOKEN")?);
/// let application_id = Id::new(1);
///
/// client
///     .interaction(application_id)
///     .create_followup("webhook token")
///     .content("Pinkie...")
///     .await?;
/// # Ok(()) }
/// ```
///
/// [`attachments`]: Self::attachments
/// [`components`]: Self::components
/// [`content`]: Self::content
/// [`embeds`]: Self::embeds
#[must_use = "requests must be configured and executed"]
pub struct CreateFollowup<'a> {
    application_id: Id<ApplicationMarker>,
    attachment_manager: AttachmentManager<'a>,
    fields: Result<CreateFollowupFields<'a>, MessageValidationError>,
    http: &'a Client,
    token: &'a str,
}

impl<'a> CreateFollowup<'a> {
    pub(crate) const fn new(
        http: &'a Client,
        application_id: Id<ApplicationMarker>,
        token: &'a str,
    ) -> Self {
        Self {
            application_id,
            attachment_manager: AttachmentManager::new(),
            fields: Ok(CreateFollowupFields {
                allowed_mentions: None,
                attachments: None,
                components: None,
                content: None,
                embeds: None,
                payload_json: None,
                tts: None,
                flags: None,
            }),
            http,
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

    /// Add multiple [`Component`]s to a message.
    ///
    /// Calling this method multiple times will clear previous calls.
    ///
    /// # Errors
    ///
    /// Refer to the errors section of
    /// [`twilight_validate::component::component`] for a list of errors that
    /// may be returned as a result of validating each provided component.
    pub fn components(mut self, components: &'a [Component]) -> Self {
        self.fields = self.fields.and_then(|mut fields| {
            validate_components(components)?;
            fields.components = Some(components);

            Ok(fields)
        });

        self
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
    pub fn content(mut self, content: &'a str) -> Self {
        self.fields = self.fields.and_then(|mut fields| {
            validate_content(content)?;
            fields.content = Some(content);

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
    pub fn embeds(mut self, embeds: &'a [Embed]) -> Self {
        self.fields = self.fields.and_then(|mut fields| {
            validate_embeds(embeds)?;
            fields.embeds = Some(embeds);

            Ok(fields)
        });

        self
    }

    /// Set the message's flags.
    ///
    /// The only supported flags are [`EPHEMERAL`] and [`SUPPRESS_EMBEDS`].
    ///
    /// [`EPHEMERAL`]: MessageFlags::EPHEMERAL
    /// [`SUPPRESS_EMBEDS`]: twilight_model::channel::message::MessageFlags::SUPPRESS_EMBEDS
    pub fn flags(mut self, flags: MessageFlags) -> Self {
        if let Ok(fields) = self.fields.as_mut() {
            fields.flags = Some(flags);
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

    /// Specify true if the message is TTS.
    pub fn tts(mut self, tts: bool) -> Self {
        if let Ok(fields) = self.fields.as_mut() {
            fields.tts = Some(tts);
        }

        self
    }
}

impl IntoFuture for CreateFollowup<'_> {
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

impl TryIntoRequest for CreateFollowup<'_> {
    fn try_into_request(self) -> Result<Request, Error> {
        let mut fields = self.fields.map_err(Error::validation)?;
        let mut request = Request::builder(&Route::ExecuteWebhook {
            thread_id: None,
            token: self.token,
            wait: None,
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
                fields.attachments = Some(self.attachment_manager.get_partial_attachments());

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
    fn create_followup_message() -> Result<(), Box<dyn Error>> {
        let application_id = Id::new(1);
        let token = "foo".to_owned();

        let client = Client::new(String::new());
        let req = client
            .interaction(application_id)
            .create_followup(&token)
            .content("test")
            .try_into_request()?;

        assert!(!req.use_authorization_token());
        assert_eq!(
            &Path::WebhooksIdToken(application_id.get(), token),
            req.ratelimit_path()
        );

        Ok(())
    }
}
