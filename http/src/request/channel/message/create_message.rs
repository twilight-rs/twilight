use crate::{
    client::Client,
    error::Error as HttpError,
    request::{
        attachment::{AttachmentManager, PartialAttachment},
        NullableField, Request, TryIntoRequest,
    },
    response::ResponseFuture,
    routing::Route,
};
use serde::Serialize;
use twilight_model::{
    application::component::Component,
    channel::{
        embed::Embed,
        message::{AllowedMentions, MessageFlags, MessageReference},
        Message,
    },
    http::attachment::Attachment,
    id::{
        marker::{ChannelMarker, MessageMarker, StickerMarker},
        Id,
    },
};
use twilight_validate::{
    marker::message::{
        AttachmentMarker, ComponentsMarker, ContentMarker, EmbedsMarker, StickerIdsMarker,
    },
    Validated,
};

#[derive(Serialize)]
pub(crate) struct CreateMessageFields<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    allowed_mentions: Option<NullableField<&'a AllowedMentions>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    attachments: Option<Vec<PartialAttachment<'a>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    components: Option<&'a [Component]>,
    #[serde(skip_serializing_if = "Option::is_none")]
    content: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    embeds: Option<&'a [Embed]>,
    #[serde(skip_serializing_if = "Option::is_none")]
    flags: Option<MessageFlags>,
    #[serde(skip_serializing_if = "Option::is_none")]
    message_reference: Option<MessageReference>,
    #[serde(skip_serializing_if = "Option::is_none")]
    nonce: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    payload_json: Option<&'a [u8]>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sticker_ids: Option<&'a [Id<StickerMarker>]>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tts: Option<bool>,
}

/// Send a message to a channel.
///
/// The message must include at least one of [`attachments`], [`content`],
/// [`embeds`], or [`sticker_ids`].
///
/// # Example
///
/// ```no_run
/// # #[tokio::main] async fn main() -> Result<(), Box<dyn std::error::Error>> {
/// use twilight_http::Client;
/// use twilight_model::id::Id;
///
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
///
/// [`attachments`]: Self::attachments
/// [`content`]: Self::content
/// [`embeds`]: Self::embeds
/// [`sticker_ids`]: Self::sticker_ids
#[must_use = "requests must be configured and executed"]
pub struct CreateMessage<'a> {
    attachment_manager: AttachmentManager<'a>,
    channel_id: Id<ChannelMarker>,
    fields: CreateMessageFields<'a>,
    http: &'a Client,
}

impl<'a> CreateMessage<'a> {
    pub(crate) const fn new(http: &'a Client, channel_id: Id<ChannelMarker>) -> Self {
        Self {
            attachment_manager: AttachmentManager::new(),
            channel_id,
            fields: CreateMessageFields {
                attachments: None,
                components: None,
                content: None,
                embeds: None,
                flags: None,
                message_reference: None,
                nonce: None,
                payload_json: None,
                allowed_mentions: None,
                sticker_ids: None,
                tts: None,
            },
            http,
        }
    }

    /// Specify the [`AllowedMentions`] for the message.
    ///
    /// Unless otherwise called, the request will use the client's default
    /// allowed mentions. Set to `None` to ignore this default.
    pub const fn allowed_mentions(mut self, allowed_mentions: Option<&'a AllowedMentions>) -> Self {
        self.fields.allowed_mentions = Some(NullableField(allowed_mentions));

        self
    }

    /// Attach multiple files to the message.
    ///
    /// Calling this method will clear previous calls.
    ///
    /// # Errors
    ///
    /// Returns an error of type [`AttachmentFilename`] if any filename is
    /// invalid.
    ///
    /// [`AttachmentFilename`]: twilight_validate::message::MessageValidationErrorType::AttachmentFilename
    pub fn attachments(
        mut self,
        attachments: Validated<&'a [Attachment], AttachmentMarker>,
    ) -> Self {
        self.attachment_manager = self
            .attachment_manager
            .set_files(attachments.get().iter().collect());

        self
    }

    /// Set the message's list of [`Component`]s.
    ///
    /// Calling this method will clear previous calls.
    ///
    /// # Errors
    ///
    /// Refer to the errors section of
    /// [`twilight_validate::component::component`] for a list of errors that
    /// may be returned as a result of validating each provided component.
    pub fn components(mut self, components: Validated<&'a [Component], ComponentsMarker>) -> Self {
        self.fields.components = Some(components.get());

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
    pub fn content(mut self, content: Validated<&'a str, ContentMarker>) -> Self {
        self.fields.content.replace(content.get());

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
    pub fn embeds(mut self, embeds: Validated<&'a [Embed], EmbedsMarker>) -> Self {
        self.fields.embeds = Some(embeds.get());

        self
    }

    /// Whether to fail sending if the reply no longer exists.
    ///
    /// Defaults to [`true`].
    pub const fn fail_if_not_exists(mut self, fail_if_not_exists: bool) -> Self {
        // Clippy recommends using `Option::map_or_else` which is not `const`.
        #[allow(clippy::option_if_let_else)]
        let reference = if let Some(reference) = self.fields.message_reference {
            MessageReference {
                fail_if_not_exists: Some(fail_if_not_exists),
                ..reference
            }
        } else {
            MessageReference {
                channel_id: None,
                guild_id: None,
                message_id: None,
                fail_if_not_exists: Some(fail_if_not_exists),
            }
        };

        self.fields.message_reference = Some(reference);

        self
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

    /// Attach a nonce to the message, for optimistic message sending.
    pub const fn nonce(mut self, nonce: u64) -> Self {
        self.fields.nonce = Some(nonce);

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
    /// [Discord Docs/Uploading Files]: https://discord.com/developers/docs/reference#uploading-files
    /// [`ExecuteWebhook::payload_json`]: crate::request::channel::webhook::ExecuteWebhook::payload_json
    /// [`attachments`]: Self::attachments
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

    /// Set the IDs of up to 3 guild stickers.
    ///
    /// # Errors
    ///
    /// Returns an error of type [`StickersInvalid`] if the length is invalid.
    ///
    /// [`StickersInvalid`]: twilight_validate::message::MessageValidationErrorType::StickersInvalid
    pub fn sticker_ids(
        mut self,
        sticker_ids: Validated<&'a [Id<StickerMarker>], StickerIdsMarker>,
    ) -> Self {
        self.fields.sticker_ids = Some(sticker_ids.get());

        self
    }

    /// Specify true if the message is TTS.
    pub const fn tts(mut self, tts: bool) -> Self {
        self.fields.tts = Some(tts);

        self
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

        // Set the default allowed mentions if required.
        if self.fields.allowed_mentions.is_none() {
            if let Some(allowed_mentions) = self.http.default_allowed_mentions() {
                self.fields.allowed_mentions = Some(NullableField(Some(allowed_mentions)));
            }
        }

        // Determine whether we need to use a multipart/form-data body or a JSON
        // body.
        if !self.attachment_manager.is_empty() {
            let form = if let Some(payload_json) = self.fields.payload_json {
                self.attachment_manager.build_form(payload_json)
            } else {
                self.fields.attachments = Some(self.attachment_manager.get_partial_attachments());

                let fields = crate::json::to_vec(&self.fields).map_err(HttpError::json)?;

                self.attachment_manager.build_form(fields.as_ref())
            };

            request = request.form(form);
        } else if let Some(payload_json) = self.fields.payload_json {
            request = request.body(payload_json.to_vec())
        } else {
            request = request.json(&self.fields)?;
        }

        Ok(request.build())
    }
}
