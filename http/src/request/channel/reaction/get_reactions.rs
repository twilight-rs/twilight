use crate::request::prelude::*;
use std::{
    error::Error,
    fmt::{Display, Formatter, Result as FmtResult},
};
use twilight_model::{
    id::{ChannelId, MessageId, UserId},
    user::User,
};

/// The error created if the reactions can not be retrieved as configured.
#[derive(Clone, Debug)]
#[non_exhaustive]
pub enum GetReactionsError {
    /// The number of reactions to retrieve must be between 1 and 100, inclusive.
    LimitInvalid {
        /// The provided maximum number of reactions to get.
        limit: u64,
    },
}

impl Display for GetReactionsError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            Self::LimitInvalid { .. } => f.write_str("the limit is invalid"),
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

/// Get a list of users that reacted to a message with an `emoji`.
///
/// This endpoint is limited to 100 users maximum, so if a message has more than 100 reactions,
/// requests must be chained until all reactions are retireved.
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
        emoji: RequestReactionType,
    ) -> Self {
        Self {
            channel_id,
            emoji: super::format_emoji(emoji),
            fields: GetReactionsFields::default(),
            fut: None,
            http,
            message_id,
        }
    }

    /// Get users after this id.
    pub fn after(mut self, after: UserId) -> Self {
        self.fields.after.replace(after);

        self
    }

    /// Get users before this id.
    pub fn before(mut self, before: UserId) -> Self {
        self.fields.before.replace(before);

        self
    }

    /// Set the maximum number of users to retrieve.
    ///
    /// The minimum is 1 and the maximum is 100. If no limit is specified, Discord sets the default
    /// to 25.
    ///
    /// # Errors
    ///
    /// Returns [`GetReactionsError::LimitInvalid`] if the amount is greater than 100.
    pub fn limit(mut self, limit: u64) -> Result<Self, GetReactionsError> {
        if !validate::get_reactions_limit(limit) {
            return Err(GetReactionsError::LimitInvalid { limit });
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
                emoji: self.emoji.clone(),
                limit: self.fields.limit,
                message_id: self.message_id.0,
            },
        ))));

        Ok(())
    }
}

poll_req!(GetReactions<'_>, Vec<User>);
