use super::prelude::*;
use dawn_model::{
    channel::Message,
    id::{ChannelId, MessageId},
};

// nb: after, around, and before are mutually exclusive, so we use this
// "configured" request to utilize the type system to prevent these from being
// set in combination.
pub struct GetChannelMessagesConfigured<'a> {
    limit: Option<u64>,
    after: Option<MessageId>,
    around: Option<MessageId>,
    before: Option<MessageId>,
    channel_id: ChannelId,
    fut: Option<Pending<'a, Vec<Message>>>,
    http: &'a Client,
}

impl<'a> GetChannelMessagesConfigured<'a> {
    pub(crate) fn new(
        http: &'a Client,
        channel_id: ChannelId,
        after: Option<MessageId>,
        around: Option<MessageId>,
        before: Option<MessageId>,
        limit: Option<u64>,
    ) -> Self {
        Self {
            after,
            around,
            before,
            channel_id,
            fut: None,
            http,
            limit,
        }
    }

    pub fn limit(mut self, limit: u64) -> Self {
        self.limit.replace(limit);

        self
    }

    fn start(&mut self) -> Result<()> {
        self.fut.replace(Box::pin(self.http.request(Request::from(
            Route::GetMessages {
                after: self.after.map(|x| x.0),
                around: self.around.map(|x| x.0),
                before: self.before.map(|x| x.0),
                channel_id: self.channel_id.0,
                limit: self.limit,
            },
        ))));

        Ok(())
    }
}

poll_req!(GetChannelMessagesConfigured<'_>, Vec<Message>);
