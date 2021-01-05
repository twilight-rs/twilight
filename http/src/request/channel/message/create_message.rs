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
#[derive(Clone, Debug)]
#[non_exhaustive]
pub enum CreateMessageError {
    /// Returned when the content is over 2000 UTF-16 characters.
    ContentInvalid {
        /// Provided content.
        content: String,
    },
    /// Returned when the length of the embed is over 6000 characters.
    EmbedTooLarge {
        /// Provided embed.
        embed: Box<Embed>,
        /// The source of the error.
        source: EmbedValidationError,
    },
}

impl Display for CreateMessageError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            Self::ContentInvalid { .. } => f.write_str("the message content is invalid"),
            Self::EmbedTooLarge { .. } => f.write_str("the embed's contents are too long"),
        }
    }
}

impl Error for CreateMessageError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::ContentInvalid { .. } => None,
            Self::EmbedTooLarge { source, .. } => Some(source),
        }
    }
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
    /// Returns [`CreateMessageError::ContentInvalid`] if the content length is
    /// too long.
    pub fn content(self, content: impl Into<String>) -> Result<Self, CreateMessageError> {
        self._content(content.into())
    }

    fn _content(mut self, content: String) -> Result<Self, CreateMessageError> {
        if !validate::content_limit(&content) {
            return Err(CreateMessageError::ContentInvalid { content });
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
    /// Returns [`CreateMessageError::EmbedTooLarge`] if the embed is too large.
    ///
    /// [the discord docs]: https://discord.com/developers/docs/resources/channel#embed-limits
    /// [`EmbedBuilder`]: https://docs.rs/twilight-embed-builder/*/twilight_embed_builder
    pub fn embed(mut self, embed: Embed) -> Result<Self, CreateMessageError> {
        if let Err(source) = validate::embed(&embed) {
            return Err(CreateMessageError::EmbedTooLarge {
                embed: Box::new(embed),
                source,
            });
        }

        self.fields.embed.replace(embed);

        Ok(self)
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
        self.fields.message_reference.replace(MessageReference {
            // This struct only needs the message_id, but as we also have
            // access to the channel_id we send that, as it will be verified
            // by Discord.
            channel_id: Some(self.channel_id),
            guild_id: None,
            message_id: Some(other),
        });

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
                    crate::json_to_vec(&self.fields)?,
                    Route::CreateMessage {
                        channel_id: self.channel_id.0,
                    },
                ))
            } else {
                let mut multipart = Form::new();

                for (index, (name, file)) in self.attachments.drain().enumerate() {
                    multipart.file(format!("{}", index).as_bytes(), name.as_bytes(), &file);
                }

                let body = crate::json_to_vec(&self.fields)?;
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
