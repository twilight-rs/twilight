use crate::{
    client::Client,
    error::Error as HttpError,
    request::{NullableField, Request, TryIntoRequest},
    response::ResponseFuture,
    routing::Route,
};
use serde::Serialize;
use twilight_model::{
    application::component::Component,
    channel::{
        embed::Embed,
        message::{AllowedMentions, MessageFlags},
        Attachment, Message,
    },
    id::{
        marker::{ChannelMarker, MessageMarker},
        Id,
    },
};
use twilight_validate::message::{
    components as validate_components, content as validate_content, embeds as validate_embeds,
    MessageValidationError,
};

#[derive(Serialize)]
struct UpdateMessageFields<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) allowed_mentions: Option<AllowedMentions>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attachments: Option<NullableField<&'a [Attachment]>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub components: Option<NullableField<&'a [Component]>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    content: Option<NullableField<&'a str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    embeds: Option<&'a [Embed]>,
    #[serde(skip_serializing_if = "Option::is_none")]
    flags: Option<MessageFlags>,
}

/// Update a message by [`Id<ChannelMarker>`] and [`Id<MessageMarker>`].
///
/// You can pass `None` to any of the methods to remove the associated field.
/// For example, if you have a message with an embed you want to remove, you can
/// use `.embed(None)` to remove the embed.
///
/// # Examples
///
/// Replace the content with `"test update"`:
///
/// ```no_run
/// use twilight_http::Client;
/// use twilight_model::id::Id;
///
/// # #[tokio::main]
/// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
/// let client = Client::new("my token".to_owned());
/// client.update_message(Id::new(1), Id::new(2))
///     .content(Some("test update"))?
///     .exec()
///     .await?;
/// # Ok(()) }
/// ```
///
/// Remove the message's content:
///
/// ```no_run
/// # use twilight_http::Client;
/// # use twilight_model::id::Id;
/// #
/// # #[tokio::main]
/// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
/// # let client = Client::new("my token".to_owned());
/// client.update_message(Id::new(1), Id::new(2))
///     .content(None)?
///     .exec()
///     .await?;
/// # Ok(()) }
/// ```
#[must_use = "requests must be configured and executed"]
pub struct UpdateMessage<'a> {
    channel_id: Id<ChannelMarker>,
    fields: UpdateMessageFields<'a>,
    http: &'a Client,
    message_id: Id<MessageMarker>,
}

impl<'a> UpdateMessage<'a> {
    pub(crate) const fn new(
        http: &'a Client,
        channel_id: Id<ChannelMarker>,
        message_id: Id<MessageMarker>,
    ) -> Self {
        Self {
            channel_id,
            fields: UpdateMessageFields {
                allowed_mentions: None,
                attachments: None,
                components: None,
                content: None,
                embeds: None,
                flags: None,
            },
            http,
            message_id,
        }
    }

    /// Specify multiple attachments already present in the target message to
    /// keep.
    ///
    /// If called, all unspecified attachments will be removed from the message.
    /// If not called, all attachments will be kept.
    ///
    /// Calling this method will clear any previous calls.
    pub const fn attachments(mut self, attachments: &'a [Attachment]) -> Self {
        self.fields.attachments = Some(NullableField(Some(attachments)));

        self
    }

    /// Add multiple [`Component`]s to a message.
    ///
    /// Calling this method multiple times will clear previous calls.
    ///
    /// Pass `None` to clear existing components.
    ///
    /// # Errors
    ///
    /// Refer to the errors section of
    /// [`twilight_validate::component::component`] for a list of errors that
    /// may be returned as a result of validating each provided component.
    pub fn components(
        mut self,
        components: Option<&'a [Component]>,
    ) -> Result<Self, MessageValidationError> {
        if let Some(components) = components {
            validate_components(components)?;
        }

        self.fields.components = Some(NullableField(components));

        Ok(self)
    }

    /// Set the content of the message.
    ///
    /// Pass `None` if you want to remove the message content.
    ///
    /// Note that if there is no embed then you will not be able
    /// to remove the content of the message.
    ///
    /// The maximum length is 2000 UTF-16 characters.
    ///
    /// # Errors
    ///
    /// Returns an error of type [`ContentInvalid`] if the content length is too
    /// long.
    ///
    /// [`ContentInvalid`]: twilight_validate::message::MessageValidationErrorType::ContentInvalid
    pub fn content(mut self, content: Option<&'a str>) -> Result<Self, MessageValidationError> {
        if let Some(content_ref) = content.as_ref() {
            validate_content(content_ref)?;
        }

        self.fields.content = Some(NullableField(content));

        Ok(self)
    }

    /// Attach multiple embeds to the message.
    ///
    /// To keep all embeds, do not use this.
    ///
    /// To modify one or more embeds in the message, acquire them from the
    /// previous message, mutate them in place, then pass that list to this
    /// method.
    ///
    /// To remove all embeds pass an empty slice.
    ///
    /// Note that if there is no content or file then you will not be able to
    /// remove all of the embeds.
    ///
    /// Calling this method will clear any previous calls.
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

        self.fields.embeds = Some(embeds);

        Ok(self)
    }

    /// Suppress the embeds in the message.
    pub const fn suppress_embeds(mut self, suppress: bool) -> Self {
        #[allow(clippy::option_if_let_else)]
        let mut bits = if let Some(flags) = self.fields.flags {
            flags.bits()
        } else {
            0
        };

        if suppress {
            bits |= MessageFlags::SUPPRESS_EMBEDS.bits();
        } else {
            bits &= !MessageFlags::SUPPRESS_EMBEDS.bits()
        }

        self.fields.flags = Some(MessageFlags::from_bits_truncate(bits));

        self
    }

    /// Set the [`AllowedMentions`] in the message.
    pub fn allowed_mentions(mut self, allowed: AllowedMentions) -> Self {
        self.fields.allowed_mentions.replace(allowed);

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

impl TryIntoRequest for UpdateMessage<'_> {
    fn try_into_request(self) -> Result<Request, HttpError> {
        let mut request = Request::builder(&Route::UpdateMessage {
            channel_id: self.channel_id.get(),
            message_id: self.message_id.get(),
        });

        request = request.json(&self.fields)?;

        Ok(request.build())
    }
}
