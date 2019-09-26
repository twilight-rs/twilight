use dawn_model::{
    channel::Message,
    id::{ChannelId, MessageId},
};
use super::{
    prelude::*,
    GetChannelMessagesConfigured,
};

pub struct GetChannelMessages<'a> {
    limit: Option<u64>,
    fut: Option<Pin<Box<dyn Future<Output = Result<Vec<Message>>> + Send + 'a>>>,
    channel_id: ChannelId,
    http: &'a Client,
}

impl<'a> GetChannelMessages<'a> {
    pub(crate) fn new(
        http: &'a Client,
        channel_id: impl Into<ChannelId>,
    ) -> Self {
        Self {
            channel_id: channel_id.into(),
            fut: None,
            limit: None,
            http,
        }
    }

    pub fn after(self, message_id: MessageId) -> GetChannelMessagesConfigured<'a> {
        GetChannelMessagesConfigured::new(
            self.http,
            self.channel_id,
            Some(message_id),
            None,
            None,
            self.limit,
        )
    }

    pub fn around(self, message_id: MessageId) -> GetChannelMessagesConfigured<'a> {
        GetChannelMessagesConfigured::new(
            self.http,
            self.channel_id,
            None,
            Some(message_id),
            None,
            self.limit,
        )
    }

    pub fn before(self, message_id: MessageId) -> GetChannelMessagesConfigured<'a> {
        GetChannelMessagesConfigured::new(
            self.http,
            self.channel_id,
            None,
            None,
            Some(message_id),
            self.limit,
        )
    }

    pub fn limit(mut self, limit: u64) -> Self {
        self.limit.replace(limit);

        self
    }

    fn start(&mut self) -> Result<()> {
        self.fut.replace(Box::pin(self.http.request(Request::from(Route::GetMessages {
            after: None,
            around: None,
            before: None,
            channel_id: self.channel_id.0,
            limit: self.limit,
        }))));

        Ok(())
    }
}

poll_req!(GetChannelMessages<'_>, Vec<Message>);
