use crate::{
    client::Client,
    error::Error as HttpError,
    request::{
        validate_inner::{self, ComponentValidationError, ComponentValidationErrorType},
        AttachmentFile, Form, PartialAttachment, Request,
    },
    response::ResponseFuture,
    routing::Route,
};
use serde::Serialize;
use std::{
    borrow::Cow,
    error::Error,
    fmt::{Display, Formatter, Result as FmtResult},
};
use twilight_model::{
    application::component::Component,
    channel::{
        embed::Embed,
        message::{AllowedMentions, MessageFlags},
        Message,
    },
    id::ApplicationId,
};

/// A followup message can not be created as configured.
#[derive(Debug)]
pub struct CreateFollowupMessageError {
    kind: CreateFollowupMessageErrorType,
    source: Option<Box<dyn Error + Send + Sync>>,
}

impl CreateFollowupMessageError {
    /// Immutable reference to the type of error that occurred.
    #[must_use = "retrieving the type has no effect if left unused"]
    pub const fn kind(&self) -> &CreateFollowupMessageErrorType {
        &self.kind
    }

    /// Consume the error, returning the source error if there is any.
    #[must_use = "consuming the error and retrieving the source has no effect if left unused"]
    pub fn into_source(self) -> Option<Box<dyn Error + Send + Sync>> {
        self.source
    }

    /// Consume the error, returning the owned error type and the source error.
    #[must_use = "consuming the error into its parts has no effect if left unused"]
    pub fn into_parts(
        self,
    ) -> (
        CreateFollowupMessageErrorType,
        Option<Box<dyn Error + Send + Sync>>,
    ) {
        (self.kind, self.source)
    }
}

impl Display for CreateFollowupMessageError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match &self.kind {
            CreateFollowupMessageErrorType::ComponentCount { count } => {
                Display::fmt(count, f)?;
                f.write_str(" components were provided, but only ")?;
                Display::fmt(&ComponentValidationError::COMPONENT_COUNT, f)?;

                f.write_str(" root components are allowed")
            }
            CreateFollowupMessageErrorType::ComponentInvalid { .. } => {
                f.write_str("a provided component is invalid")
            }
        }
    }
}

impl Error for CreateFollowupMessageError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        self.source
            .as_ref()
            .map(|source| &**source as &(dyn Error + 'static))
    }
}

/// Type of [`CreateFollowupMessageError`] that occurred.
#[derive(Debug)]
#[non_exhaustive]
pub enum CreateFollowupMessageErrorType {
    /// An invalid message component was provided.
    ComponentInvalid {
        /// Additional details about the validation failure type.
        kind: ComponentValidationErrorType,
    },
    /// Too many message components were provided.
    ComponentCount {
        /// Number of components that were provided.
        count: usize,
    },
}

#[derive(Serialize)]
pub(crate) struct CreateFollowupMessageFields<'a> {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    attachments: Vec<PartialAttachment<'a>>,
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
    allowed_mentions: Option<&'a AllowedMentions>,
}

/// Create a followup message to an interaction.
///
/// You must specify at least one of [`content`], [`embeds`], or [`files`].
///
/// # Examples
///
/// ```no_run
/// # #[tokio::main] async fn main() -> Result<(), Box<dyn std::error::Error>> {
/// use std::env;
/// use twilight_http::Client;
/// use twilight_model::id::ApplicationId;
///
/// let client = Client::new(env::var("DISCORD_TOKEN")?);
/// client.set_application_id(ApplicationId::new(1).expect("non zero"));
///
/// client
///     .create_followup_message("webhook token")?
///     .content("Pinkie...")
///     .exec()
///     .await?;
/// # Ok(()) }
/// ```
///
/// [`content`]: Self::content
/// [`embeds`]: Self::embeds
/// [`files`]: Self::files
#[must_use = "requests must be configured and executed"]
pub struct CreateFollowupMessage<'a> {
    application_id: ApplicationId,
    attachments: Cow<'a, [AttachmentFile<'a>]>,
    pub(crate) fields: CreateFollowupMessageFields<'a>,
    http: &'a Client,
    token: &'a str,
}

impl<'a> CreateFollowupMessage<'a> {
    pub(crate) const fn new(
        http: &'a Client,
        application_id: ApplicationId,
        token: &'a str,
    ) -> Self {
        Self {
            fields: CreateFollowupMessageFields {
                attachments: Vec::new(),
                components: None,
                content: None,
                embeds: None,
                payload_json: None,
                tts: None,
                flags: None,
                allowed_mentions: None,
            },
            attachments: Cow::Borrowed(&[]),
            http,
            token,
            application_id,
        }
    }

    /// Specify the [`AllowedMentions`] for the webhook message.
    pub const fn allowed_mentions(mut self, allowed_mentions: &'a AllowedMentions) -> Self {
        self.fields.allowed_mentions = Some(allowed_mentions);

        self
    }

    /// Add multiple [`Component`]s to a message.
    ///
    /// Calling this method multiple times will clear previous calls.
    ///
    /// # Errors
    ///
    /// Returns an [`CreateFollowupMessageErrorType::ComponentCount`] error type
    /// if too many components are provided.
    ///
    /// Returns an [`CreateFollowupMessageErrorType::ComponentInvalid`] error
    /// type if one of the provided components is invalid.
    pub fn components(
        mut self,
        components: &'a [Component],
    ) -> Result<Self, CreateFollowupMessageError> {
        validate_inner::components(components).map_err(|source| {
            let (kind, inner_source) = source.into_parts();

            match kind {
                ComponentValidationErrorType::ComponentCount { count } => {
                    CreateFollowupMessageError {
                        kind: CreateFollowupMessageErrorType::ComponentCount { count },
                        source: inner_source,
                    }
                }
                other => CreateFollowupMessageError {
                    kind: CreateFollowupMessageErrorType::ComponentInvalid { kind: other },
                    source: inner_source,
                },
            }
        })?;

        self.fields.components = Some(components);

        Ok(self)
    }

    /// The content of the webhook's message.
    ///
    /// Up to 2000 UTF-16 codepoints.
    pub const fn content(mut self, content: &'a str) -> Self {
        self.fields.content = Some(content);

        self
    }

    /// Set the list of embeds of the webhook's message.
    pub const fn embeds(mut self, embeds: &'a [Embed]) -> Self {
        self.fields.embeds = Some(embeds);

        self
    }

    /// Set if the followup should be ephemeral.
    pub const fn ephemeral(mut self, ephemeral: bool) -> Self {
        if ephemeral {
            self.fields.flags = Some(MessageFlags::EPHEMERAL);
        } else {
            self.fields.flags = None;
        }

        self
    }

    /// Attach multiple files to the message.
    ///
    /// Calling this method will clear any previous calls.
    #[allow(clippy::missing_const_for_fn)] // False positive
    pub fn attach(mut self, attachments: &'a [AttachmentFile<'a>]) -> Self {
        self.attachments = Cow::Borrowed(attachments);

        self
    }

    /// Attach multiple files to the message.
    ///
    /// Calling this method will clear any previous calls.
    #[deprecated(since = "0.7.2", note = "Use attach instead")]
    pub fn files(mut self, files: &'a [(&'a str, &'a [u8])]) -> Self {
        self.attachments = Cow::Owned(AttachmentFile::from_pairs(files));

        self
    }

    /// JSON encoded body of any additional request fields.
    ///
    /// If this method is called, all other fields are ignored, except for
    /// [`attach`]. See [Discord Docs/Create Message].
    ///
    /// # Examples
    ///
    /// Without [`payload_json`]:
    ///
    /// ```no_run
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// use std::env;
    /// use twilight_embed_builder::EmbedBuilder;
    /// use twilight_http::Client;
    /// use twilight_model::id::{MessageId, ApplicationId};
    ///
    /// let client = Client::new(env::var("DISCORD_TOKEN")?);
    /// client.set_application_id(ApplicationId::new(1).expect("non zero"));
    ///
    /// let message = client.create_followup_message("token here")?
    ///     .content("some content")
    ///     .embeds(&[EmbedBuilder::new().title("title").build()?])
    ///     .exec()
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
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// use std::env;
    /// use twilight_http::Client;
    /// use twilight_model::id::{MessageId, ApplicationId};
    ///
    /// let client = Client::new(env::var("DISCORD_TOKEN")?);
    /// client.set_application_id(ApplicationId::new(1).expect("non zero"));
    ///
    /// let message = client.create_followup_message("token here")?
    ///     .content("some content")
    ///     .payload_json(br#"{ "content": "other content", "embeds": [ { "title": "title" } ] }"#)
    ///     .exec()
    ///     .await?
    ///     .model()
    ///     .await?;
    ///
    /// assert_eq!(message.content, "other content");
    /// # Ok(()) }
    /// ```
    ///
    /// [`attach`]: Self::attach
    /// [`payload_json`]: Self::payload_json
    /// [Discord Docs/Create Message]: https://discord.com/developers/docs/resources/channel#create-message-params
    pub const fn payload_json(mut self, payload_json: &'a [u8]) -> Self {
        self.fields.payload_json = Some(payload_json);

        self
    }

    /// Specify true if the message is TTS.
    pub const fn tts(mut self, tts: bool) -> Self {
        self.fields.tts = Some(tts);

        self
    }

    // `self` needs to be consumed and the client returned due to parameters
    // being consumed in request construction.
    fn request(&mut self) -> Result<Request, HttpError> {
        let mut request = Request::builder(&Route::ExecuteWebhook {
            thread_id: None,
            token: self.token,
            wait: None,
            webhook_id: self.application_id.get(),
        });

        if !self.attachments.is_empty() || self.fields.payload_json.is_some() {
            let mut form = Form::new();

            if !self.attachments.is_empty() {
                for (index, attachment) in self.attachments.iter().enumerate() {
                    form.attach(
                        index as u64,
                        attachment.filename.as_bytes(),
                        attachment.file,
                    );
                    self.fields.attachments.push(PartialAttachment {
                        id: index as u64,
                        filename: attachment.filename,
                        description: attachment.description,
                    })
                }
            }

            if let Some(payload_json) = &self.fields.payload_json {
                form.payload_json(payload_json);
            } else {
                let body = crate::json::to_vec(&self.fields).map_err(HttpError::json)?;
                form.payload_json(&body);
            }

            request = request.form(form);
        } else {
            request = request.json(&self.fields)?;
        }

        Ok(request.use_authorization_token(false).build())
    }

    /// Execute the request, returning a future resolving to a [`Response`].
    ///
    /// [`Response`]: crate::response::Response
    pub fn exec(mut self) -> ResponseFuture<Message> {
        match self.request() {
            Ok(request) => self.http.request(request),
            Err(source) => ResponseFuture::error(source),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::client::Client;
    use std::error::Error;
    use twilight_http_ratelimiting::Path;
    use twilight_model::id::ApplicationId;

    #[test]
    fn test_create_followup_message() -> Result<(), Box<dyn Error>> {
        let application_id = ApplicationId::new(1).expect("non zero id");
        let token = "foo".to_owned().into_boxed_str();

        let client = Client::new(String::new());
        client.set_application_id(application_id);
        let req = client
            .create_followup_message(&token)?
            .content("test")
            .request()?;

        assert!(!req.use_authorization_token());
        assert_eq!(
            &Path::WebhooksIdToken(application_id.get(), token),
            req.ratelimit_path()
        );

        Ok(())
    }
}
