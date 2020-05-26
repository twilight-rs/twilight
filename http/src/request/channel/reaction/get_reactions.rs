use crate::request::prelude::*;
use std::{
    error::Error,
    fmt::{Display, Formatter, Result as FmtResult},
};
use twilight_model::{
    id::{ChannelId, MessageId, UserId},
    user::User,
};

#[derive(Clone, Debug)]
pub enum GetReactionsError {
    /// The maximum number of reactions to retrieve is 0 or more than 100.
    LimitInvalid,
}

impl Display for GetReactionsError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            Self::LimitInvalid => f.write_str("the limit is invalid"),
        }
    }
}

impl Error for GetReactionsError {}

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
        channel_id: ChannelId,
        message_id: MessageId,
        emoji: impl Into<String>,
    ) -> Self {
        Self {
            channel_id,
            emoji: emoji.into(),
            fields: GetReactionsFields::default(),
            fut: None,
            http,
            message_id,
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

    /// Set the maximum number of reactions to retrieve.
    ///
    /// The minimum is 1 and the maximum is 100.
    ///
    /// # Errors
    ///
    /// Returns [`GetReactionsError::LimitInvalid`] if the amount is greater
    /// than 100.
    ///
    /// [`GetReactionsError::LimitInvalid`]: enum.GetReactionsError.hLml#variant.LimitInvalid
    pub fn limit(mut self, limit: u64) -> Result<Self, GetReactionsError> {
        if !validate::get_reactions_limit(limit) {
            return Err(GetReactionsError::LimitInvalid);
        }

        self.fields.limit.replace(limit);

        Ok(self)
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
