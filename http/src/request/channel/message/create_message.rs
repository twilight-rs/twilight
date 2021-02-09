use super::super::allowed_mentions::{AllowedMentions, AllowedMentionsBuilder, Unspecified};
use crate::request::{multipart::Form, prelude::*};
use std::{
    collections::HashMap,
    error::Error,
    fmt::{Display, Formatter, Result as FmtResult},
};
use twilight_model::{
    channel::{embed::Embed, message::MessageReference, Message},
    id::{ChannelId, MessageId},
};

/// The error created when a messsage can not be created as configured.
#[derive(Debug)]
pub struct CreateMessageError {
    kind: CreateMessageErrorType,
    source: Option<Box<dyn Error + Send + Sync>>,
}

impl CreateMessageError {
    /// Immutable reference to the type of error that occurred.
    #[must_use = "retrieving the type has no effect if left unused"]
    pub fn kind(&self) -> &CreateMessageErrorType {
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
}

impl Display for CreateMessageError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match &self.kind {
            CreateMessageErrorType::ContentInvalid { .. } => {
                f.write_str("the message content is invalid")
            }
            CreateMessageErrorType::EmbedTooLarge { .. } => {
                f.write_str("the embed's contents are too long")
            }
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
pub(crate) struct CreateMessageFields {
    #[serde(skip_serializing_if = "Option::is_none")]
    content: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    embed: Option<Embed>,
    #[serde(skip_serializing_if = "Option::is_none")]
    message_reference: Option<MessageReference>,
    #[serde(skip_serializing_if = "Option::is_none")]
    nonce: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    payload_json: Option<Vec<u8>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) allowed_mentions: Option<AllowedMentions>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tts: Option<bool>,
}

/// Send a message to a channel.
///
/// # Example
///
/// ```rust,no_run
/// use twilight_http::Client;
/// use twilight_model::id::ChannelId;
///
/// # #[tokio::main]
/// # async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
/// let client = Client::new("my token");
///
/// let channel_id = ChannelId(123);
/// let message = client
///     .create_message(channel_id)
///     .content("Twilight is best pony")?
///     .tts(true)
///     .await?;
/// # Ok(()) }
/// ```
pub struct CreateMessage<'a> {
    attachments: HashMap<String, Vec<u8>>,
    channel_id: ChannelId,
    pub(crate) fields: CreateMessageFields,
    fut: Option<Pending<'a, Message>>,
    http: &'a Client,
}

impl<'a> CreateMessage<'a> {
    pub(crate) fn new(http: &'a Client, channel_id: ChannelId) -> Self {
        Self {
            attachments: HashMap::new(),
            channel_id,
            fields: CreateMessageFields {
                allowed_mentions: http.default_allowed_mentions(),
                ..CreateMessageFields::default()
            },
            fut: None,
            http,
        }
    }

    /// Return a new [`AllowedMentionsBuilder`].
    pub fn allowed_mentions(
        self,
    ) -> AllowedMentionsBuilder<'a, Unspecified, Unspecified, Unspecified> {
        AllowedMentionsBuilder::for_builder(self)
    }

    /// Attach a new file to the message.
    ///
    /// The file is raw binary data. It can be an image, or any other kind of file.
    pub fn attachment(mut self, name: impl Into<String>, file: impl Into<Vec<u8>>) -> Self {
        self.attachments.insert(name.into(), file.into());

        self
    }

    /// Insert multiple attachments into the message.
    pub fn attachments<N: Into<String>, F: Into<Vec<u8>>>(
        mut self,
        attachments: impl IntoIterator<Item = (N, F)>,
    ) -> Self {
        for (name, file) in attachments {
            self = self.attachment(name, file);
        }

        self
    }

    /// Set the content of the message.
    ///
    /// The maximum length is 2000 UTF-16 characters.
    ///
    /// # Errors
    ///
    /// Returns a [`CreateMessageErrorType::ContentInvalid`] error type if the
    /// content length is too long.
    pub fn content(self, content: impl Into<String>) -> Result<Self, CreateMessageError> {
        self._content(content.into())
    }

    fn _content(mut self, content: String) -> Result<Self, CreateMessageError> {
        if !validate::content_limit(&content) {
            return Err(CreateMessageError {
                kind: CreateMessageErrorType::ContentInvalid { content },
                source: None,
            });
        }

        self.fields.content.replace(content);

        Ok(self)
    }

    /// Set the embed of the message.
    ///
    /// Embed total character length must not exceed 6000 characters. Additionally, the internal
    /// fields also have character limits. Refer to [the discord docs] for more information.
    ///
    /// # Examples
    ///
    /// See [`EmbedBuilder`] for an example.
    ///
    /// # Errors
    ///
    /// Returns a [`CreateMessageErrorType::EmbedTooLarge`] error type if the
    /// embed is too large.
    ///
    /// [the discord docs]: https://discord.com/developers/docs/resources/channel#embed-limits
    /// [`EmbedBuilder`]: https://docs.rs/twilight-embed-builder/*/twilight_embed_builder
    pub fn embed(mut self, embed: Embed) -> Result<Self, CreateMessageError> {
        if let Err(source) = validate::embed(&embed) {
            return Err(CreateMessageError {
                kind: CreateMessageErrorType::EmbedTooLarge {
                    embed: Box::new(embed),
                },
                source: Some(Box::new(source)),
            });
        }

        self.fields.embed.replace(embed);

        Ok(self)
    }

    /// Whether to fail sending if the reply no longer exists.
    pub fn fail_if_not_exists(mut self) -> Self {
        self.fields.message_reference = Some(self.fields.message_reference.map_or_else(
            || MessageReference {
                channel_id: None,
                guild_id: None,
                message_id: None,
                fail_if_not_exists: Some(true),
            },
            |message_reference| MessageReference {
                fail_if_not_exists: Some(true),
                ..message_reference
            },
        ));

        self
    }

    /// Attach a nonce to the message, for optimistic message sending.
    pub fn nonce(mut self, nonce: u64) -> Self {
        self.fields.nonce.replace(nonce);

        self
    }

    /// JSON encoded body of any additional request fields.  See [Discord Docs/Create Message]
    ///
    /// [Discord Docs/Create Message]: https://discord.com/developers/docs/resources/channel#create-message-params
    pub fn payload_json(mut self, payload_json: impl Into<Vec<u8>>) -> Self {
        self.fields.payload_json.replace(payload_json.into());

        self
    }

    /// Specify the ID of another message to create a reply to.
    pub fn reply(mut self, other: MessageId) -> Self {
        let channel_id = self.channel_id;

        self.fields.message_reference = Some(self.fields.message_reference.map_or_else(
            || MessageReference {
                channel_id: Some(channel_id),
                guild_id: None,
                message_id: Some(other),
                fail_if_not_exists: None,
            },
            |message_reference| MessageReference {
                channel_id: Some(channel_id),
                message_id: Some(other),
                ..message_reference
            },
        ));

        self
    }

    /// Specify true if the message is TTS.
    pub fn tts(mut self, tts: bool) -> Self {
        self.fields.tts.replace(tts);

        self
    }

    fn start(&mut self) -> Result<()> {
        self.fut.replace(Box::pin(self.http.request(
            if self.attachments.is_empty() {
                Request::from((
                    crate::json_to_vec(&self.fields).map_err(HttpError::json)?,
                    Route::CreateMessage {
                        channel_id: self.channel_id.0,
                    },
                ))
            } else {
                let mut multipart = Form::new();

                for (index, (name, file)) in self.attachments.drain().enumerate() {
                    multipart.file(format!("{}", index).as_bytes(), name.as_bytes(), &file);
                }

                let body = crate::json_to_vec(&self.fields).map_err(HttpError::json)?;
                multipart.part(b"payload_json", &body);

                Request::from((
                    multipart,
                    Route::CreateMessage {
                        channel_id: self.channel_id.0,
                    },
                ))
            },
        )));

        Ok(())
    }
}

poll_req!(CreateMessage<'_>, Message);
