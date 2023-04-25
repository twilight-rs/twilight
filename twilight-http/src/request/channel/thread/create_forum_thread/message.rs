use super::{CreateForumThread, ForumThread};
use crate::{
    request::{attachment::PartialAttachment, Nullable, TryIntoRequest},
    response::{Response, ResponseFuture},
    Error,
};
use serde::Serialize;
use std::future::IntoFuture;
use twilight_model::{
    channel::message::{AllowedMentions, Component, Embed, MessageFlags},
    http::attachment::Attachment,
    id::{marker::StickerMarker, Id},
};
use twilight_validate::message::{
    attachment_filename as validate_attachment_filename, components as validate_components,
    content as validate_content, embeds as validate_embeds, sticker_ids as validate_sticker_ids,
    MessageValidationError,
};

/// Contents of the first message in the new forum thread.
#[derive(Serialize)]
pub(super) struct CreateForumThreadMessageFields<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(super) allowed_mentions: Option<Nullable<&'a AllowedMentions>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(super) attachments: Option<Vec<PartialAttachment<'a>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(super) components: Option<&'a [Component]>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(super) content: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(super) embeds: Option<&'a [Embed]>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(super) flags: Option<MessageFlags>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(super) payload_json: Option<&'a [u8]>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(super) sticker_ids: Option<&'a [Id<StickerMarker>]>,
}

#[must_use = "requests must be configured and executed"]
pub struct CreateForumThreadMessage<'a>(CreateForumThread<'a>);

impl<'a> CreateForumThreadMessage<'a> {
    pub(super) const fn new(inner: CreateForumThread<'a>) -> Self {
        Self(inner)
    }

    /// Specify the [`AllowedMentions`] for the message.
    ///
    /// Unless otherwise called, the request will use the client's default
    /// allowed mentions. Set to `None` to ignore this default.
    pub const fn allowed_mentions(mut self, allowed_mentions: Option<&'a AllowedMentions>) -> Self {
        self.0.fields.message.allowed_mentions = Some(Nullable(allowed_mentions));

        self
    }

    /// Attach multiple files to the message.
    ///
    /// Calling this method will clear any previous calls.
    ///
    /// # Errors
    ///
    /// Returns an error of type [`AttachmentFilename`] if any filename is
    /// invalid.
    ///
    /// [`AttachmentFilename`]: twilight_validate::message::MessageValidationErrorType::AttachmentFilename
    pub fn attachments(
        mut self,
        attachments: &'a [Attachment],
    ) -> Result<Self, MessageValidationError> {
        attachments
            .iter()
            .try_for_each(|attachment| validate_attachment_filename(&attachment.filename))?;

        self.0.attachment_manager = self
            .0
            .attachment_manager
            .set_files(attachments.iter().collect());

        Ok(self)
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

        self.0.fields.message.components = Some(components);

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

        self.0.fields.message.content = Some(content);

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

        self.0.fields.message.embeds = Some(embeds);

        Ok(self)
    }

    /// Set the message's flags.
    ///
    /// The only supported flags are [`SUPPRESS_EMBEDS`] and
    /// [`SUPPRESS_NOTIFICATIONS`].
    ///
    /// [`SUPPRESS_EMBEDS`]: MessageFlags::SUPPRESS_EMBEDS
    /// [`SUPPRESS_NOTIFICATIONS`]: MessageFlags::SUPPRESS_NOTIFICATIONS
    pub const fn flags(mut self, flags: MessageFlags) -> Self {
        self.0.fields.message.flags = Some(flags);

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
        self.0.fields.message.payload_json = Some(payload_json);

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
        sticker_ids: &'a [Id<StickerMarker>],
    ) -> Result<Self, MessageValidationError> {
        validate_sticker_ids(sticker_ids)?;

        self.0.fields.message.sticker_ids = Some(sticker_ids);

        Ok(self)
    }

    /// Execute the request, returning a future resolving to a [`Response`].
    #[deprecated(since = "0.14.0", note = "use `.await` or `into_future` instead")]
    pub fn exec(self) -> ResponseFuture<ForumThread> {
        self.into_future()
    }
}

impl IntoFuture for CreateForumThreadMessage<'_> {
    type Output = Result<Response<ForumThread>, Error>;

    type IntoFuture = ResponseFuture<ForumThread>;

    fn into_future(self) -> Self::IntoFuture {
        self.0.exec()
    }
}

impl TryIntoRequest for CreateForumThreadMessage<'_> {
    fn try_into_request(self) -> Result<crate::request::Request, Error> {
        self.0.try_into_request()
    }
}
