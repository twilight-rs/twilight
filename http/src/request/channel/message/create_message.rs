use crate::{
    client::Client,
    error::Error as HttpError,
    request::{
        self,
        attachment::AttachmentFile,
        multipart::Form,
        validate_inner::{
            self, ComponentValidationError, ComponentValidationErrorType, EmbedValidationError,
        },
        PartialAttachment, Request,
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
        message::{AllowedMentions, MessageReference},
        Message,
    },
    id::{ChannelId, MessageId, StickerId},
};
use crate::request::channel::message::create_message::CreateMessageErrorType::TooManyStickers;

/// The error created when a message can not be created as configured.
#[derive(Debug)]
pub struct CreateMessageError {
    kind: CreateMessageErrorType,
    source: Option<Box<dyn Error + Send + Sync>>,
}

impl CreateMessageError {
    /// Immutable reference to the type of error that occurred.
    #[must_use = "retrieving the type has no effect if left unused"]
    pub const fn kind(&self) -> &CreateMessageErrorType {
        &self.kind
    }

    /// Consume the error, returning the source error if there is any.
    #[must_use = "consuming the error and retrieving the source has no effect if left unused"]
    pub fn into_source(self) -> Option<Box<dyn Error + Send + Sync>> {
        self.source
    }

    /// Consume the error, returning the owned error type and the source error.
    #[must_use = "consuming the error into its parts has no effect if left unused"]
    pub fn into_parts(self) -> (CreateMessageErrorType, Option<Box<dyn Error + Send + Sync>>) {
        (self.kind, self.source)
    }

    fn embed(source: EmbedValidationError, idx: usize) -> Self {
        Self {
            kind: CreateMessageErrorType::EmbedTooLarge { idx },
            source: Some(Box::new(source)),
        }
    }
}

impl Display for CreateMessageError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match &self.kind {
            CreateMessageErrorType::ComponentCount { count } => {
                Display::fmt(count, f)?;
                f.write_str(" components were provided, but only ")?;
                Display::fmt(&ComponentValidationError::COMPONENT_COUNT, f)?;

                f.write_str(" root components are allowed")
            }
            CreateMessageErrorType::ComponentInvalid { .. } => {
                f.write_str("a provided component is invalid")
            }
            CreateMessageErrorType::ContentInvalid => f.write_str("the message content is invalid"),
            CreateMessageErrorType::EmbedTooLarge { idx } => {
                f.write_str("the embed at index ")?;
                Display::fmt(&idx, f)?;

                f.write_str("'s contents are too long")
            },
            CreateMessageErrorType::TooManyStickers { count } => {
                Display::fmt(count, f)?;
                f.write_str(" stickers were provided, but max 3 are supported")
            },
        }
    }
}

impl Error for CreateMessageError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        self.source
            .as_ref()
            .map(|source| &**source as &(dyn Error + 'static))
    }
}

/// Type of [`CreateMessageError`] that occurred.
#[derive(Debug)]
#[non_exhaustive]
pub enum CreateMessageErrorType {
    /// Too many message components were provided.
    ComponentCount {
        /// Number of components that were provided.
        count: usize,
    },
    /// An invalid message component was provided.
    ComponentInvalid {
        /// Additional details about the validation failure type.
        kind: ComponentValidationErrorType,
    },
    /// Returned when the content is over 2000 UTF-16 characters.
    ContentInvalid,
    /// Returned when the length of the embed is over 6000 characters.
    EmbedTooLarge {
        /// Index of the embed.
        idx: usize,
    },
    TooManyStickers {
        /// Number of stickers provided
        count: usize,
    },
}

#[derive(Serialize)]
pub(crate) struct CreateMessageFields<'a> {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    attachments: Vec<PartialAttachment<'a>>,
    #[serde(skip_serializing_if = "request::slice_is_empty")]
    components: &'a [Component],
    #[serde(skip_serializing_if = "Option::is_none")]
    content: Option<&'a str>,
    #[serde(skip_serializing_if = "request::slice_is_empty")]
    embeds: &'a [Embed],
    #[serde(skip_serializing_if = "Option::is_none")]
    message_reference: Option<MessageReference>,
    #[serde(skip_serializing_if = "Option::is_none")]
    nonce: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    payload_json: Option<&'a [u8]>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) allowed_mentions: Option<AllowedMentions>,
    #[serde(skip_serializing_if = "request::slice_is_empty")]
    sticker_ids: &'a [StickerId],
    #[serde(skip_serializing_if = "Option::is_none")]
    tts: Option<bool>,

}

/// Send a message to a channel.
///
/// # Example
///
/// ```no_run
/// use twilight_http::Client;
/// use twilight_model::id::ChannelId;
///
/// # #[tokio::main]
/// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
/// let client = Client::new("my token".to_owned());
///
/// let channel_id = ChannelId::new(123).expect("non zero");
/// let message = client
///     .create_message(channel_id)
///     .content("Twilight is best pony")?
///     .tts(true)
///     .exec()
///     .await?;
/// # Ok(()) }
/// ```
#[must_use = "requests must be configured and executed"]
pub struct CreateMessage<'a> {
    attachments: Cow<'a, [AttachmentFile<'a>]>,
    channel_id: ChannelId,
    pub(crate) fields: CreateMessageFields<'a>,
    http: &'a Client,
}

impl<'a> CreateMessage<'a> {
    pub(crate) const fn new(http: &'a Client, channel_id: ChannelId) -> Self {
        Self {
            channel_id,
            fields: CreateMessageFields {
                attachments: Vec::new(),
                components: &[],
                content: None,
                embeds: &[],
                message_reference: None,
                nonce: None,
                payload_json: None,
                allowed_mentions: None,
                sticker_ids: &[],
                tts: None,
            },
            attachments: Cow::Borrowed(&[]),
            http,
        }
    }

    /// Specify the [`AllowedMentions`] for the message.
    pub fn allowed_mentions(mut self, allowed_mentions: AllowedMentions) -> Self {
        self.fields.allowed_mentions.replace(allowed_mentions);

        self
    }

    /// Add multiple [`Component`]s to a message.
    ///
    /// Calling this method multiple times will clear previous calls.
    ///
    /// # Errors
    ///
    /// Returns an [`CreateMessageErrorType::ComponentCount`] error type if
    /// too many components are provided.
    ///
    /// Returns an [`CreateMessageErrorType::ComponentInvalid`] error type if
    /// one of the provided components is invalid.
    pub fn components(mut self, components: &'a [Component]) -> Result<Self, CreateMessageError> {
        validate_inner::components(components).map_err(|source| {
            let (kind, inner_source) = source.into_parts();

            match kind {
                ComponentValidationErrorType::ComponentCount { count } => CreateMessageError {
                    kind: CreateMessageErrorType::ComponentCount { count },
                    source: inner_source,
                },
                other => CreateMessageError {
                    kind: CreateMessageErrorType::ComponentInvalid { kind: other },
                    source: inner_source,
                },
            }
        })?;

        self.fields.components = components;

        Ok(self)
    }

    /// Set the content of the message.
    ///
    /// The maximum length is 2000 UTF-16 characters.
    ///
    /// # Errors
    ///
    /// Returns a [`CreateMessageErrorType::ContentInvalid`] error type if the
    /// content length is too long.
    pub fn content(mut self, content: &'a str) -> Result<Self, CreateMessageError> {
        if !validate_inner::content_limit(content) {
            return Err(CreateMessageError {
                kind: CreateMessageErrorType::ContentInvalid,
                source: None,
            });
        }

        self.fields.content.replace(content);

        Ok(self)
    }

    /// Attach multiple embeds to the message.
    ///
    /// Embed total character length must not exceed 6000 characters.
    /// Additionally, the internal fields also have character limits. Refer to
    /// [the discord docs] for more information.
    ///
    /// # Errors
    ///
    /// Returns a [`CreateMessageErrorType::EmbedTooLarge`] error type if an
    /// embed is too large.
    ///
    /// [the discord docs]: https://discord.com/developers/docs/resources/channel#embed-limits
    pub fn embeds(mut self, embeds: &'a [Embed]) -> Result<Self, CreateMessageError> {
        for (idx, embed) in embeds.iter().enumerate() {
            validate_inner::embed(embed)
                .map_err(|source| CreateMessageError::embed(source, idx))?;
        }

        self.fields.embeds = embeds;

        Ok(self)
    }

    /// Whether to fail sending if the reply no longer exists.
    pub const fn fail_if_not_exists(mut self) -> Self {
        // Clippy recommends using `Option::map_or_else` which is not `const`.
        #[allow(clippy::option_if_let_else)]
        let reference = if let Some(reference) = self.fields.message_reference {
            MessageReference {
                fail_if_not_exists: Some(true),
                ..reference
            }
        } else {
            MessageReference {
                channel_id: None,
                guild_id: None,
                message_id: None,
                fail_if_not_exists: Some(true),
            }
        };

        self.fields.message_reference = Some(reference);

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
    #[deprecated(since = "0.7.2", note = "Use attachments instead")]
    pub fn files(mut self, files: &'a [(&'a str, &'a [u8])]) -> Self {
        self.attachments = Cow::Owned(AttachmentFile::from_pairs(files));

        self
    }

    /// Attach a nonce to the message, for optimistic message sending.
    pub const fn nonce(mut self, nonce: u64) -> Self {
        self.fields.nonce = Some(nonce);

        self
    }

    /// JSON encoded body of any additional request fields.
    ///
    /// If this method is called, all other fields are ignored, except for
    /// [`attach`]. See [Discord Docs/Create Message].
    ///
    /// [`attach`]: Self::attach
    /// [Discord Docs/Create Message]: https://discord.com/developers/docs/resources/channel#create-message-params
    pub const fn payload_json(mut self, payload_json: &'a [u8]) -> Self {
        self.fields.payload_json = Some(payload_json);

        self
    }

    /// Specify the ID of another message to create a reply to.
    pub const fn reply(mut self, other: MessageId) -> Self {
        let channel_id = self.channel_id;

        // Clippy recommends using `Option::map_or_else` which is not `const`.
        #[allow(clippy::option_if_let_else)]
        let reference = if let Some(reference) = self.fields.message_reference {
            MessageReference {
                channel_id: Some(channel_id),
                message_id: Some(other),
                ..reference
            }
        } else {
            MessageReference {
                channel_id: Some(channel_id),
                guild_id: None,
                message_id: Some(other),
                fail_if_not_exists: None,
            }
        };

        self.fields.message_reference = Some(reference);

        self
    }

    /// Specify true if the message is TTS.
    pub const fn tts(mut self, tts: bool) -> Self {
        self.fields.tts = Some(tts);

        self
    }

    /// Add multiple stickers to the message.
    ///
    /// Up to 3 stickers can be added to a message.  Stickers are Discord
    /// snowflakes.
    ///
    /// # Errors
    ///
    /// Returns a [`CreateMessageErrorType::TooManyStickers`] error type if
    /// more than three stickers are provided
    pub fn stickers(mut self, stickers: &'a [StickerId]) -> Result<Self, CreateMessageError> {
        if !validate_inner::sticker_limit(stickers) {
            return Err(CreateMessageError {
                kind: TooManyStickers { count: stickers.len() },
                source: None,
            })
        }

        self.fields.sticker_ids = stickers;

        Ok(self)
    }

    /// Execute the request, returning a future resolving to a [`Response`].
    ///
    /// [`Response`]: crate::response::Response
    pub fn exec(mut self) -> ResponseFuture<Message> {
        let mut request = Request::builder(&Route::CreateMessage {
            channel_id: self.channel_id.get(),
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
                let body = match crate::json::to_vec(&self.fields) {
                    Ok(body) => body,
                    Err(source) => return ResponseFuture::error(HttpError::json(source)),
                };

                form.payload_json(&body);
            }

            request = request.form(form);
        } else {
            request = match request.json(&self.fields) {
                Ok(request) => request,
                Err(source) => return ResponseFuture::error(source),
            };
        }

        self.http.request(request.build())
    }
}
