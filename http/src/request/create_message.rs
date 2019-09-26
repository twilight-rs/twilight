use super::prelude::*;
use dawn_model::{
    channel::{embed::Embed, Message},
    id::ChannelId,
};

#[derive(Serialize)]
pub struct CreateMessage<'a> {
    content: Option<String>,
    embed: Option<Embed>,
    file: Option<Vec<u8>>,
    nonce: Option<u64>,
    payload_json: Option<Vec<u8>>,
    tts: Option<bool>,
    #[serde(skip)]
    channel_id: ChannelId,
    #[serde(skip)]
    fut: Option<Pin<Box<dyn Future<Output = Result<Message>> + Send + 'a>>>,
    #[serde(skip)]
    http: &'a Client,
}

impl<'a> CreateMessage<'a> {
    pub(crate) fn new(http: &'a Client, channel_id: impl Into<ChannelId>) -> Self {
        Self {
            content: None,
            embed: None,
            file: None,
            nonce: None,
            payload_json: None,
            tts: None,
            channel_id: channel_id.into(),
            fut: None,
            http,
        }
    }

    pub fn content(mut self, content: impl Into<String>) -> Self {
        self.content.replace(content.into());

        self
    }

    pub fn embed(mut self, embed: Embed) -> Self {
        self.embed.replace(embed);

        self
    }

    pub fn file(mut self, file: impl Into<Vec<u8>>) -> Self {
        self.file.replace(file.into());

        self
    }

    pub fn nonce(mut self, nonce: u64) -> Self {
        self.nonce.replace(nonce);

        self
    }

    pub fn payload_json(mut self, payload_json: impl Into<Vec<u8>>) -> Self {
        self.payload_json.replace(payload_json.into());

        self
    }

    pub fn tts(mut self, tts: bool) -> Self {
        self.tts.replace(tts);

        self
    }

    fn start(&mut self) -> Result<()> {
        self.fut.replace(Box::pin(self.http.request(Request::from((
            serde_json::to_vec(self)?,
            Route::CreateMessage {
                channel_id: self.channel_id.0,
            },
        )))));

        Ok(())
    }
}

poll_req!(CreateMessage<'_>, Message);
