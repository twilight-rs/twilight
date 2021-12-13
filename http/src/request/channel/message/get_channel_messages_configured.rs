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
#[must_use = "requests must be configured and executed"]
pub struct GetChannelMessagesConfigured<'a> {
    after: Option<MessageId>,
    around: Option<MessageId>,
    before: Option<MessageId>,
    channel_id: ChannelId,
    fields: GetChannelMessagesConfiguredFields,
    http: &'a Client,
}

impl<'a> GetChannelMessagesConfigured<'a> {
    pub(crate) const fn new(
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
            http,
        }
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
            after: self.after.map(MessageId::get),
            around: self.around.map(MessageId::get),
            before: self.before.map(MessageId::get),
            channel_id: self.channel_id.get(),
            limit: self.fields.limit,
        });

        self.http.request(request)
    }
}
