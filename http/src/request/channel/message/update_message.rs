use crate::request::{channel::allowed_mentions::AllowedMentions, prelude::*};
use std::{
    error::Error,
    fmt::{Display, Formatter, Result as FmtResult},
};
use twilight_model::{
    channel::{embed::Embed, message::MessageFlags, Message},
    id::{ChannelId, MessageId},
};

/// The error created when a message can not be updated as configured.
#[derive(Debug)]
pub struct UpdateMessageError {
    kind: UpdateMessageErrorType,
    source: Option<Box<dyn Error + Send + Sync>>,
}

impl UpdateMessageError {
    /// Immutable reference to the type of error that occurred.
    #[must_use = "retrieving the type has no effect if left unused"]
    pub fn kind(&self) -> &UpdateMessageErrorType {
        &self.kind
    }

    /// Consume the error, returning the source error if there is any.
    #[must_use = "consuming the error and retrieving the source has no effect if left unused"]
    pub fn into_source(self) -> Option<Box<dyn Error + Send + Sync>> {
        self.source
    }

    /// Consume the error, returning the owned error type and the source error.
    #[must_use = "consuming the error into its parts has no effect if left unused"]
    pub fn into_parts(self) -> (UpdateMessageErrorType, Option<Box<dyn Error + Send + Sync>>) {
        (self.kind, self.source)
    }
}

impl Display for UpdateMessageError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match &self.kind {
            UpdateMessageErrorType::ContentInvalid { .. } => {
                f.write_str("the message content is invalid")
            }
            UpdateMessageErrorType::EmbedTooLarge { .. } => {
                f.write_str("the embed's contents are too long")
            }
        }
    }
}

impl Error for UpdateMessageError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        self.source
            .as_ref()
            .map(|source| &**source as &(dyn Error + 'static))
    }
}

/// Type of [`UpdateMessageError`] that occurred.
#[derive(Debug)]
#[non_exhaustive]
pub enum UpdateMessageErrorType {
    /// Returned when the content is over 2000 UTF-16 characters.
    ContentInvalid {
        /// Provided content.
        content: String,
    },
    /// Returned when the length of the embed is over 6000 characters.
    EmbedTooLarge {
        /// Provided embed.
        embed: Box<Embed>,
    },
}

#[derive(Default, Serialize)]
struct UpdateMessageFields {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) allowed_mentions: Option<AllowedMentions>,
    // We don't serialize if this is Option::None, to avoid overwriting the
    // field without meaning to.
    //
    // So we use a nested Option, representing the following states:
    //
    // - Some(Some(String)): Modifying the "content" from one state to a string;
    // - Some(None): Removing the "content" by giving the Discord API a written
    //   `"content": null` in the JSON;
    // - None: Don't serialize the field at all, not modifying the state.
    #[allow(clippy::option_option)]
    #[serde(skip_serializing_if = "Option::is_none")]
    content: Option<Option<String>>,
    #[allow(clippy::option_option)]
    #[serde(skip_serializing_if = "Option::is_none")]
    embed: Option<Option<Embed>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    flags: Option<MessageFlags>,
}

/// Update a message by [`ChannelId`] and [`MessageId`].
///
/// You can pass `None` to any of the methods to remove the associated field.
/// For example, if you have a message with an embed you want to remove, you can
/// use `.embed(None)` to remove the embed.
///
/// # Examples
///
/// Replace the content with `"test update"`:
///
/// ```rust,no_run
/// use twilight_http::Client;
/// use twilight_model::id::{ChannelId, MessageId};
///
/// # #[tokio::main]
/// # async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
/// let client = Client::new("my token");
/// client.update_message(ChannelId(1), MessageId(2))
///     .content("test update".to_owned())?
///     .await?;
/// # Ok(()) }
/// ```
///
/// Remove the message's content:
///
/// ```rust,no_run
/// # use twilight_http::Client;
/// # use twilight_model::id::{ChannelId, MessageId};
/// #
/// # #[tokio::main]
/// # async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
/// # let client = Client::new("my token");
/// client.update_message(ChannelId(1), MessageId(2))
///     .content(None)?
///     .await?;
/// # Ok(()) }
/// ```
pub struct UpdateMessage<'a> {
    channel_id: ChannelId,
    fields: UpdateMessageFields,
    fut: Option<Pending<'a, Message>>,
    http: &'a Client,
    message_id: MessageId,
}

impl<'a> UpdateMessage<'a> {
    pub(crate) fn new(http: &'a Client, channel_id: ChannelId, message_id: MessageId) -> Self {
        Self {
            channel_id,
            fields: UpdateMessageFields::default(),
            fut: None,
            http,
            message_id,
        }
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
    /// Returns an [`UpdateMessageErrorType::ContentInvalid`] error type if the
    /// content length is too long.
    pub fn content(self, content: impl Into<Option<String>>) -> Result<Self, UpdateMessageError> {
        self._content(content.into())
    }

    fn _content(mut self, content: Option<String>) -> Result<Self, UpdateMessageError> {
        if let Some(content_ref) = content.as_ref() {
            if !validate::content_limit(content_ref) {
                return Err(UpdateMessageError {
                    kind: UpdateMessageErrorType::ContentInvalid {
                        content: content.expect("content is known to be some"),
                    },
                    source: None,
                });
            }
        }

        self.fields.content.replace(content);

        Ok(self)
    }

    /// Set the embed of the message.
    ///
    /// Pass `None` if you want to remove the message embed.
    ///
    /// Note that if there is no content then you will not be
    /// able to remove the embed of the message.
    pub fn embed(self, embed: impl Into<Option<Embed>>) -> Result<Self, UpdateMessageError> {
        self._embed(embed.into())
    }

    fn _embed(mut self, embed: Option<Embed>) -> Result<Self, UpdateMessageError> {
        if let Some(embed_ref) = embed.as_ref() {
            if let Err(source) = validate::embed(&embed_ref) {
                return Err(UpdateMessageError {
                    kind: UpdateMessageErrorType::EmbedTooLarge {
                        embed: Box::new(embed.expect("embed is known to be some")),
                    },
                    source: Some(Box::new(source)),
                });
            }
        }

        self.fields.embed.replace(embed);

        Ok(self)
    }

    /// Suppress the embeds in the message.
    pub fn suppress_embeds(mut self, suppress: bool) -> Self {
        let mut flags = self.fields.flags.unwrap_or_else(MessageFlags::empty);

        if suppress {
            flags |= MessageFlags::SUPPRESS_EMBEDS;
        } else {
            flags &= !MessageFlags::SUPPRESS_EMBEDS;
        }
        self.fields.flags.replace(flags);

        self
    }

    /// Set the allowed mentions in the message.
    ///
    /// Use the [`build_solo`] method to get a [`AllowedMentions`] structure.
    ///
    /// [`build_solo`]: super::super::allowed_mentions::AllowedMentionsBuilder::build_solo
    pub fn allowed_mentions(mut self, allowed: AllowedMentions) -> Self {
        self.fields.allowed_mentions.replace(allowed);

        self
    }

    fn start(&mut self) -> Result<()> {
        self.fut.replace(Box::pin(self.http.request(Request::from((
            crate::json_to_vec(&self.fields).map_err(HttpError::json)?,
            Route::UpdateMessage {
                channel_id: self.channel_id.0,
                message_id: self.message_id.0,
            },
        )))));

        Ok(())
    }
}

poll_req!(UpdateMessage<'_>, Message);
