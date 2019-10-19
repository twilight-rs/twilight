use super::prelude::*;
use dawn_model::{
    channel::{embed::Embed, Message},
    id::ChannelId,
};

#[derive(Default, Serialize)]
struct CreateMessageFields {
    content: Option<String>,
    embed: Option<Embed>,
    file: Option<Vec<u8>>,
    nonce: Option<u64>,
    payload_json: Option<Vec<u8>>,
    tts: Option<bool>,
}

pub struct CreateMessage<'a> {
    channel_id: ChannelId,
    fields: CreateMessageFields,
    fut: Option<Pending<'a, Message>>,
    http: &'a Client,
}

impl<'a> CreateMessage<'a> {
    pub(crate) fn new(http: &'a Client, channel_id: ChannelId) -> Self {
        Self {
            channel_id,
            fields: CreateMessageFields::default(),
            fut: None,
            http,
        }
    }

    pub fn content(mut self, content: impl Into<String>) -> Self {
        self.fields.content.replace(content.into());

        self
    }

    pub fn embed(mut self, embed: Embed) -> Self {
        self.fields.embed.replace(embed);

        self
    }

    pub fn file(mut self, file: impl Into<Vec<u8>>) -> Self {
        self.fields.file.replace(file.into());

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
        self.fut.replace(Box::pin(self.http.request(Request::from((
            serde_json::to_vec(&self.fields)?,
            Route::CreateMessage {
                channel_id: self.channel_id.0,
            },
        )))));

        Ok(())
    }
}

poll_req!(CreateMessage<'_>, Message);
