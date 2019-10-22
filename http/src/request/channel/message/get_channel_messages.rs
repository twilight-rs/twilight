use super::GetChannelMessagesConfigured;
use crate::request::prelude::*;
use dawn_model::{
    channel::Message,
    id::{ChannelId, MessageId},
};

#[derive(Default)]
struct GetChannelMessagesFields {
    limit: Option<u64>,
}

pub struct GetChannelMessages<'a> {
    channel_id: ChannelId,
    fields: GetChannelMessagesFields,
    fut: Option<Pending<'a, Vec<Message>>>,
    http: &'a Client,
}

impl<'a> GetChannelMessages<'a> {
    pub(crate) fn new(http: &'a Client, channel_id: ChannelId) -> Self {
        Self {
            channel_id,
            fields: GetChannelMessagesFields::default(),
            fut: None,
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
            self.fields.limit,
        )
    }

    pub fn around(self, message_id: MessageId) -> GetChannelMessagesConfigured<'a> {
        GetChannelMessagesConfigured::new(
            self.http,
            self.channel_id,
            None,
            Some(message_id),
            None,
            self.fields.limit,
        )
    }

    pub fn before(self, message_id: MessageId) -> GetChannelMessagesConfigured<'a> {
        GetChannelMessagesConfigured::new(
            self.http,
            self.channel_id,
            None,
            None,
            Some(message_id),
            self.fields.limit,
        )
    }

    pub fn limit(mut self, limit: u64) -> Self {
        self.fields.limit.replace(limit);

        self
    }

    fn start(&mut self) -> Result<()> {
        self.fut.replace(Box::pin(self.http.request(Request::from(
            Route::GetMessages {
                after: None,
                around: None,
                before: None,
                channel_id: self.channel_id.0,
                limit: self.fields.limit,
            },
        ))));

        Ok(())
    }
}

poll_req!(GetChannelMessages<'_>, Vec<Message>);
