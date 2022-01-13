use crate::{
    client::Client,
    error::Error as HttpError,
    request::{
        self, attachment::AttachmentFile, multipart::Form, PartialAttachment, Request,
        TryIntoRequest,
    },
    response::ResponseFuture,
    routing::Route,
};
use serde::Serialize;
use std::borrow::Cow;
use twilight_model::{
    application::component::Component,
    channel::{
        embed::Embed,
        message::{AllowedMentions, MessageReference},
        Message,
    },
    id::{
        marker::{ChannelMarker, MessageMarker, StickerMarker},
        Id,
    },
};
use twilight_validate::message::{
    components as validate_components, content as validate_content, embeds as validate_embeds,
    stickers as validate_stickers, MessageValidationError,
};

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
    sticker_ids: &'a [Id<StickerMarker>],
    #[serde(skip_serializing_if = "Option::is_none")]
    tts: Option<bool>,
}

/// Send a message to a channel.
///
/// # Example
///
/// ```no_run
/// use twilight_http::Client;
/// use twilight_model::id::Id;
///
/// # #[tokio::main]
/// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
/// let client = Client::new("my token".to_owned());
///
/// let channel_id = Id::new(123);
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
    channel_id: Id<ChannelMarker>,
    pub(crate) fields: CreateMessageFields<'a>,
    http: &'a Client,
}

impl<'a> CreateMessage<'a> {
    pub(crate) const fn new(http: &'a Client, channel_id: Id<ChannelMarker>) -> Self {
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
    /// Refer to the errors section of
    /// [`twilight_validate::component::component`] for a list of errors that
    /// may be returned as a result of validating each provided component.
    pub fn components(
        mut self,
        components: &'a [Component],
    ) -> Result<Self, MessageValidationError> {
        validate_components(components)?;

        self.fields.components = components;

        Ok(self)
    }

    /// Set the content of the message.
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
    /// Returns an error of type [`TooManyEmbeds`] if there are too many embeds.
    ///
    /// Otherwise, refer to the errors section of [`embed`] for a list of errors
    /// that may occur.
    ///
    /// [`TooManyEmbeds`]: twilight_validate::message::MessageValidationErrorType::TooManyEmbeds
    /// [`embed`]: twilight_validate::embed::embed
    pub fn embeds(mut self, embeds: &'a [Embed]) -> Result<Self, MessageValidationError> {
        validate_embeds(embeds)?;

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
    pub const fn reply(mut self, other: Id<MessageMarker>) -> Self {
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

    /// Set the IDs of up to 3 guild stickers.
    ///
    /// # Errors
    ///
    /// Returns an error of type [`StickersInvalid`] if the length is invalid.
    ///
    /// [`StickersInvalid`]: MessageValidationErrorType::StickersInvalid
    pub fn stickers(
        mut self,
        stickers: &'a [Id<StickerMarker>],
    ) -> Result<Self, MessageValidationError> {
        validate_stickers(stickers)?;

        self.fields.sticker_ids = stickers;

        Ok(self)
    }

    /// Execute the request, returning a future resolving to a [`Response`].
    ///
    /// [`Response`]: crate::response::Response
    pub fn exec(self) -> ResponseFuture<Message> {
        let http = self.http;

        match self.try_into_request() {
            Ok(request) => http.request(request),
            Err(source) => ResponseFuture::error(source),
        }
    }
}

impl TryIntoRequest for CreateMessage<'_> {
    fn try_into_request(mut self) -> Result<Request, HttpError> {
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
                let body = crate::json::to_vec(&self.fields).map_err(HttpError::json)?;

                form.payload_json(&body);
            }

            request = request.form(form);
        } else {
            request = request.json(&self.fields)?;
        }

        Ok(request.build())
    }
}
