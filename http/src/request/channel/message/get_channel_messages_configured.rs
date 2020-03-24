use crate::request::prelude::*;
use twilight_model::{
    channel::Message,
    id::{ChannelId, MessageId},
};

struct GetChannelMessagesConfiguredFields {
    limit: Option<u64>,
}

// nb: after, around, and before are mutually exclusive, so we use this
// "configured" request to utilize the type system to prevent these from being
// set in combination.
pub struct GetChannelMessagesConfigured<'a> {
    after: Option<MessageId>,
    around: Option<MessageId>,
    before: Option<MessageId>,
    channel_id: ChannelId,
    fields: GetChannelMessagesConfiguredFields,
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
            fields: GetChannelMessagesConfiguredFields {
                limit,
            },
            fut: None,
            http,
        }
    }

    pub fn limit(mut self, limit: u64) -> Self {
        self.fields.limit.replace(limit);

        self
    }

    fn start(&mut self) -> Result<()> {
        self.fut.replace(Box::pin(self.http.request(Request::from(
            Route::GetMessages {
                after: self.after.map(|x| x.0),
                around: self.around.map(|x| x.0),
                before: self.before.map(|x| x.0),
                channel_id: self.channel_id.0,
                limit: self.fields.limit,
            },
        ))));

        Ok(())
    }
}

poll_req!(GetChannelMessagesConfigured<'_>, Vec<Message>);
