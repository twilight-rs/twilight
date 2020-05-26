use super::allowed_mentions::{AllowedMentions, AllowedMentionsBuilder, Unspecified};
use crate::request::prelude::*;
use reqwest::{
    multipart::{Form, Part},
    Body,
};
use std::{
    collections::HashMap,
    error::Error,
    fmt::{Display, Formatter, Result as FmtResult},
};
use twilight_model::{
    channel::{embed::Embed, Message},
    id::ChannelId,
};

#[derive(Clone, Debug)]
pub enum CreateMessageError {
    ContentInvalid,
    EmbedTooLarge { source: EmbedValidationError },
}

impl Display for CreateMessageError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            Self::ContentInvalid => f.write_str("the message content is invalid"),
            Self::EmbedTooLarge { .. } => f.write_str("the embed's contents are too long"),
        }
    }
}

impl Error for CreateMessageError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::ContentInvalid => None,
            Self::EmbedTooLarge { source } => Some(source),
        }
    }
}

#[derive(Default, Serialize)]
pub(crate) struct CreateMessageFields {
    content: Option<String>,
    embed: Option<Embed>,
    nonce: Option<u64>,
    payload_json: Option<Vec<u8>>,
    tts: Option<bool>,
    pub(crate) allowed_mentions: Option<AllowedMentions>,
}

pub struct CreateMessage<'a> {
    attachments: HashMap<String, Body>,
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

    /// Set the content of the message.
    ///
    /// The maximum length is 2000 UTF-16 characters.
    ///
    /// # Errors
    ///
    /// Returns [`CreateMessageError::ContentInvalid`] if the content length is
    /// too long.
    ///
    /// [`CreateMessageError::ContentInvalid`]: enum.CreateMessageError.html#variant.ContentInvalid
    pub fn content(self, content: impl Into<String>) -> Result<Self, CreateMessageError> {
        self._content(content.into())
    }

    fn _content(mut self, content: String) -> Result<Self, CreateMessageError> {
        if !validate::content_limit(&content) {
            return Err(CreateMessageError::ContentInvalid);
        }

        self.fields.content.replace(content);

        Ok(self)
    }

    pub fn embed(mut self, embed: Embed) -> Result<Self, CreateMessageError> {
        validate::embed(&embed).map_err(|source| CreateMessageError::EmbedTooLarge { source })?;

        self.fields.embed.replace(embed);

        Ok(self)
    }

    pub fn allowed_mentions(
        self,
    ) -> AllowedMentionsBuilder<'a, Unspecified, Unspecified, Unspecified> {
        AllowedMentionsBuilder::for_builder(self)
    }

    pub fn attachment(mut self, name: impl Into<String>, file: impl Into<Body>) -> Self {
        self.attachments.insert(name.into(), file.into());

        self
    }

    pub fn attachments<N: Into<String>, F: Into<Body>>(
        mut self,
        attachments: impl IntoIterator<Item = (N, F)>,
    ) -> Self {
        for (name, file) in attachments {
            self = self.attachment(name, file);
        }

        self
    }

    pub fn nonce(mut self, nonce: u64) -> Self {
        self.fields.nonce.replace(nonce);

        self
    }

    pub fn payload_json(mut self, payload_json: impl Into<Vec<u8>>) -> Self {
        self.fields.payload_json.replace(payload_json.into());

        self
    }

    pub fn tts(mut self, tts: bool) -> Self {
        self.fields.tts.replace(tts);

        self
    }

    fn start(&mut self) -> Result<()> {
        self.fut.replace(Box::pin(self.http.request(
            if self.attachments.is_empty() {
                Request::from((
                    serde_json::to_vec(&self.fields)?,
                    Route::CreateMessage {
                        channel_id: self.channel_id.0,
                    },
                ))
            } else {
                let mut form = Form::new();

                for (index, (name, file)) in self.attachments.drain().enumerate() {
                    form = form.part(format!("{}", index), Part::stream(file).file_name(name));
                }

                Request::from((
                    serde_json::to_vec(&self.fields)?,
                    form,
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
