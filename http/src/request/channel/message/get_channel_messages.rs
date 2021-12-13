use super::GetChannelMessagesConfigured;
use crate::{
    client::Client,
    request::Request,
    response::{marker::ListBody, ResponseFuture},
    routing::Route,
};
use twilight_model::{
    channel::Message,
    id::{ChannelId, MessageId},
};
use twilight_validate::misc::{
    get_channel_messages_limit as validate_get_channel_messages_limit, ValidationError,
};

struct GetChannelMessagesFields {
    limit: Option<u64>,
}

/// Get channel messages, by [`ChannelId`].
///
/// Only one of [`after`], [`around`], and [`before`] can be specified at a time.
/// Once these are specified, the type returned is [`GetChannelMessagesConfigured`].
///
/// If [`limit`] is unspecified, the default set by Discord is 50.
///
/// # Examples
///
/// ```rust,no_run
/// use twilight_http::Client;
/// use twilight_model::id::{ChannelId, MessageId};
///
/// # #[tokio::main]
/// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
/// let client = Client::new("my token".to_owned());
/// let channel_id = ChannelId::new(123).expect("non zero");
/// let message_id = MessageId::new(234).expect("non zero");
///
/// let messages = client
///     .channel_messages(channel_id)
///     .before(message_id)
///     .limit(6u64)?
///     .exec()
///     .await?;
///
/// # Ok(()) }
/// ```
///
/// [`after`]: Self::after
/// [`around`]: Self::around
/// [`before`]: Self::before
/// [`GetChannelMessagesConfigured`]: super::GetChannelMessagesConfigured
/// [`limit`]: Self::limit
#[must_use = "requests must be configured and executed"]
pub struct GetChannelMessages<'a> {
    channel_id: ChannelId,
    fields: GetChannelMessagesFields,
    http: &'a Client,
}

impl<'a> GetChannelMessages<'a> {
    pub(crate) const fn new(http: &'a Client, channel_id: ChannelId) -> Self {
        Self {
            channel_id,
            fields: GetChannelMessagesFields { limit: None },
            http,
        }
    }

    pub const fn after(self, message_id: MessageId) -> GetChannelMessagesConfigured<'a> {
        GetChannelMessagesConfigured::new(
            self.http,
            self.channel_id,
            Some(message_id),
            None,
            None,
            self.fields.limit,
        )
    }

    pub const fn around(self, message_id: MessageId) -> GetChannelMessagesConfigured<'a> {
        GetChannelMessagesConfigured::new(
            self.http,
            self.channel_id,
            None,
            Some(message_id),
            None,
            self.fields.limit,
        )
    }

    pub const fn before(self, message_id: MessageId) -> GetChannelMessagesConfigured<'a> {
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
    /// Returns an error of type [`GetChannelMessages`] error type if the amount
    /// is less than 1 or greater than 100.
    ///
    /// [`GetChannelMessages`]: twilight_validate::misc::ValidationErrorType::GetChannelMessages
    pub const fn limit(mut self, limit: u64) -> Result<Self, ValidationError> {
        if let Err(source) = validate_get_channel_messages_limit(limit) {
            return Err(source);
        }

        self.fields.limit = Some(limit);

        Ok(self)
    }

    /// Execute the request, returning a future resolving to a [`Response`].
    ///
    /// [`Response`]: crate::response::Response
    pub fn exec(self) -> ResponseFuture<ListBody<Message>> {
        let request = Request::from_route(&Route::GetMessages {
            after: None,
            around: None,
            before: None,
            channel_id: self.channel_id.get(),
            limit: self.fields.limit,
        });

        self.http.request(request)
    }
}
