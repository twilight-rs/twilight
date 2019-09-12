use dawn_model::{
    channel::{
        embed::Embed,
        Message,
    },
    id::{ChannelId, MessageId},
};
use super::prelude::*;

#[derive(Serialize)]
pub struct UpdateMessage<'a> {
    content: Option<String>,
    embed: Option<Embed>,
    #[serde(skip)]
    channel_id: ChannelId,
    #[serde(skip)]
    fut: Option<PendingBody<'a, Message>>,
    #[serde(skip)]
    http: &'a Client,
    #[serde(skip)]
    message_id: MessageId,
}

impl<'a> UpdateMessage<'a> {
    pub(crate) fn new(
        http: &'a Client,
        channel_id: impl Into<ChannelId>,
        message_id: impl Into<MessageId>,
    ) -> Self {
        Self {
            channel_id: channel_id.into(),
            content: None,
            embed: None,
            fut: None,
            http,
            message_id: message_id.into(),
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

    fn start(&mut self) -> Result<()> {
        self.fut.replace(self.http.request(Request {
            body: Some(serde_json::to_vec(self)?),
            route: Route::UpdateMessage {
                channel_id: self.channel_id.0,
                message_id: self.message_id.0,
            },
            ..Default::default()
        })?);

        Ok(())
    }
}

poll_req!(UpdateMessage<'_>, Message);
