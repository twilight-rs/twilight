use super::prelude::*;
use dawn_model::{
    id::{ChannelId, MessageId, UserId},
    user::User,
};

#[derive(Default)]
struct GetReactionsFields {
    after: Option<UserId>,
    before: Option<UserId>,
    limit: Option<u64>,
}

pub struct GetReactions<'a> {
    channel_id: ChannelId,
    emoji: String,
    fields: GetReactionsFields,
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
            channel_id: channel_id.into(),
            emoji: emoji.into(),
            fields: GetReactionsFields::default(),
            fut: None,
            http,
            message_id: message_id.into(),
        }
    }

    pub fn after(mut self, after: UserId) -> Self {
        self.fields.after.replace(after);

        self
    }

    pub fn before(mut self, before: UserId) -> Self {
        self.fields.before.replace(before);

        self
    }

    pub fn limit(mut self, limit: u64) -> Self {
        self.fields.limit.replace(limit);

        self
    }

    fn start(&mut self) -> Result<()> {
        self.fut.replace(Box::pin(self.http.request(Request::from(
            Route::GetReactionUsers {
                after: self.fields.after.map(|x| x.0),
                before: self.fields.before.map(|x| x.0),
                channel_id: self.channel_id.0,
                emoji: self.emoji.to_owned(),
                limit: self.fields.limit,
                message_id: self.message_id.0,
            },
        ))));

        Ok(())
    }
}

poll_req!(GetReactions<'_>, Vec<User>);
