use super::prelude::*;
use dawn_model::{
    id::{ChannelId, MessageId, UserId},
    user::User,
};

pub struct GetReactions<'a> {
    after: Option<UserId>,
    before: Option<UserId>,
    limit: Option<u64>,
    channel_id: ChannelId,
    emoji: String,
    fut: Option<Pending<'a, Vec<User>>>,
    http: &'a Client,
    message_id: MessageId,
}

impl<'a> GetReactions<'a> {
    pub(crate) fn new(
        http: &'a Client,
        channel_id: impl Into<ChannelId>,
        message_id: impl Into<MessageId>,
        emoji: impl Into<String>,
    ) -> Self {
        Self {
            after: None,
            before: None,
            channel_id: channel_id.into(),
            emoji: emoji.into(),
            fut: None,
            http,
            limit: None,
            message_id: message_id.into(),
        }
    }

    pub fn after(mut self, after: UserId) -> Self {
        self.after.replace(after);

        self
    }

    pub fn before(mut self, before: UserId) -> Self {
        self.before.replace(before);

        self
    }

    pub fn limit(mut self, limit: u64) -> Self {
        self.limit.replace(limit);

        self
    }

    fn start(&mut self) -> Result<()> {
        self.fut.replace(Box::pin(self.http.request(Request::from(
            Route::GetReactionUsers {
                after: self.after.map(|x| x.0),
                before: self.before.map(|x| x.0),
                channel_id: self.channel_id.0,
                emoji: self.emoji.to_owned(),
                limit: self.limit,
                message_id: self.message_id.0,
            },
        ))));

        Ok(())
    }
}

poll_req!(GetReactions<'_>, Vec<User>);
