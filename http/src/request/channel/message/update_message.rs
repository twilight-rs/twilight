use crate::json_to_vec;
use crate::request::prelude::*;
use std::{
    error::Error,
    fmt::{Display, Formatter, Result as FmtResult},
};
use twilight_model::{
    channel::{embed::Embed, Message},
    id::{ChannelId, MessageId},
};

#[derive(Clone, Debug)]
/// The error created when a message can not be updated as configured.
pub enum UpdateMessageError {
    /// Returned when the content is over 2000 UTF-16 characters.
    ContentInvalid,
    /// Returned when the length of the embed is over 6000 characters.
    EmbedTooLarge {
        /// The source of the error.
        source: EmbedValidationError,
    },
}

impl Display for UpdateMessageError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            Self::ContentInvalid => f.write_str("the message content is invalid"),
            Self::EmbedTooLarge { .. } => f.write_str("the embed's contents are too long"),
        }
    }
}

impl Error for UpdateMessageError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::ContentInvalid => None,
            Self::EmbedTooLarge { source } => Some(source),
        }
    }
}

#[derive(Default, Serialize)]
struct UpdateMessageFields {
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
    content: Option<Option<String>>,
    #[allow(clippy::option_option)]
    embed: Option<Option<Embed>>,
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
///
/// [`ChannelId`]: ../../../../../twilight_model/id/struct.ChannelId.html
/// [`MessageId`]: ../../../../../twilight_model/id/struct.MessageId.html
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
    /// The maximum length is 2000 UTF-16 characters.
    ///
    /// # Errors
    ///
    /// Returns [`UpdateMessageError::ContentInvalid`] if the content length is
    /// too long.
    ///
    /// [`UpdateMessageError::ContentInvalid`]: enum.UpdateMessageError.html#variant.ContentInvalid
    pub fn content(self, content: impl Into<Option<String>>) -> Result<Self, UpdateMessageError> {
        self._content(content.into())
    }

    fn _content(mut self, content: Option<String>) -> Result<Self, UpdateMessageError> {
        if let Some(content) = content.as_ref() {
            if !validate::content_limit(content) {
                return Err(UpdateMessageError::ContentInvalid);
            }
        }

        self.fields.content.replace(content);

        Ok(self)
    }

    /// Set the embed of the message.
    ///
    /// Pass `None` if you want to remove the message embed.
    pub fn embed(self, embed: impl Into<Option<Embed>>) -> Result<Self, UpdateMessageError> {
        self._embed(embed.into())
    }

    fn _embed(mut self, embed: Option<Embed>) -> Result<Self, UpdateMessageError> {
        if let Some(embed) = embed.as_ref() {
            validate::embed(&embed)
                .map_err(|source| UpdateMessageError::EmbedTooLarge { source })?;
        }

        self.fields.embed.replace(embed);

        Ok(self)
    }

    fn start(&mut self) -> Result<()> {
        self.fut.replace(Box::pin(self.http.request(Request::from((
            json_to_vec(&self.fields)?,
            Route::UpdateMessage {
                channel_id: self.channel_id.0,
                message_id: self.message_id.0,
            },
        )))));

        Ok(())
    }
}

poll_req!(UpdateMessage<'_>, Message);
