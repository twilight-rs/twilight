use crate::request::prelude::*;
use std::{
    error::Error,
    fmt::{Display, Formatter, Result as FmtResult},
};
use twilight_model::{
    channel::Message,
    id::{ChannelId, MessageId},
};

/// The error returned if the request can not be created as configured.
#[derive(Clone, Debug)]
#[non_exhaustive]
pub enum GetChannelMessagesConfiguredError {
    /// The maximum number of messages to retrieve is either 0 or more than 100.
    LimitInvalid {
        /// Provided maximum number of messages to retrieve.
        limit: u64,
    },
}

impl Display for GetChannelMessagesConfiguredError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            Self::LimitInvalid { .. } => f.write_str("the limit is invalid"),
        }
    }
}

impl Error for GetChannelMessagesConfiguredError {}

struct GetChannelMessagesConfiguredFields {
    limit: Option<u64>,
}

/// This struct is returned when one of `after`, `around`, or `before` is specified in
/// [`GetChannelMessages`].
///
/// [`GetChannelMessages`]: super::GetChannelMessages
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
            fields: GetChannelMessagesConfiguredFields { limit },
            fut: None,
            http,
        }
    }

    /// Set the maximum number of messages to retrieve.
    ///
    /// The minimum is 1 and the maximum is 100.
    ///
    /// # Errors
    ///
    /// Returns [`GetChannelMessagesConfiguredError::LimitInvalid`] if the
    /// amount is greater than 21600.
    pub fn limit(mut self, limit: u64) -> Result<Self, GetChannelMessagesConfiguredError> {
        if !validate::get_channel_messages_limit(limit) {
            return Err(GetChannelMessagesConfiguredError::LimitInvalid { limit });
        }

        self.fields.limit.replace(limit);

        Ok(self)
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
