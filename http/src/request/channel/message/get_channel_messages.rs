use super::GetChannelMessagesConfigured;
use crate::request::prelude::*;
use std::{
    error::Error,
    fmt::{Display, Formatter, Result as FmtResult},
};
use twilight_model::{
    channel::Message,
    id::{ChannelId, MessageId},
};

#[derive(Clone, Debug)]
pub enum GetChannelMessagesError {
    /// The maximum number of messages to retrieve is either 0 or more than 100.
    LimitInvalid,
}

impl Display for GetChannelMessagesError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            Self::LimitInvalid => f.write_str("the limit is invalid"),
        }
    }
}

impl Error for GetChannelMessagesError {}

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

    /// Set the maximum number of messages to retrieve.
    ///
    /// The minimum is 1 and the maximum is 100.
    ///
    /// # Errors
    ///
    /// Returns [`GetChannelMessages::LimitInvalid`] if the
    /// amount is greater than 21600.
    ///
    /// [`GetChannelMessages::LimitInvalid`]: enum.GetChannelMessages.html#variant.LimitInvalid
    pub fn limit(mut self, limit: u64) -> Result<Self, GetChannelMessagesError> {
        if !validate::get_channel_messages_limit(limit) {
            return Err(GetChannelMessagesError::LimitInvalid);
        }

        self.fields.limit.replace(limit);

        Ok(self)
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
