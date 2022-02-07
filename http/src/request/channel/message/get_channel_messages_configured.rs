use crate::{
    client::Client,
    error::Error as HttpError,
    request::{Request, TryIntoRequest},
    response::{marker::ListBody, ResponseFuture},
    routing::Route,
};
use twilight_model::{
    channel::Message,
    id::{
        marker::{ChannelMarker, MessageMarker},
        Id,
    },
};
use twilight_validate::request::{
    get_channel_messages_limit as validate_get_channel_messages_limit, ValidationError,
};

struct GetChannelMessagesConfiguredFields {
    limit: Option<u16>,
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
    after: Option<Id<MessageMarker>>,
    around: Option<Id<MessageMarker>>,
    before: Option<Id<MessageMarker>>,
    channel_id: Id<ChannelMarker>,
    fields: GetChannelMessagesConfiguredFields,
    http: &'a Client,
}

impl<'a> GetChannelMessagesConfigured<'a> {
    pub(crate) const fn new(
        http: &'a Client,
        channel_id: Id<ChannelMarker>,
        after: Option<Id<MessageMarker>>,
        around: Option<Id<MessageMarker>>,
        before: Option<Id<MessageMarker>>,
        limit: Option<u16>,
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
    /// [`GetChannelMessages`]: twilight_validate::request::ValidationErrorType::GetChannelMessages
    pub const fn limit(mut self, limit: u16) -> Result<Self, ValidationError> {
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
        let http = self.http;

        match self.try_into_request() {
            Ok(request) => http.request(request),
            Err(source) => ResponseFuture::error(source),
        }
    }
}

impl TryIntoRequest for GetChannelMessagesConfigured<'_> {
    fn try_into_request(self) -> Result<Request, HttpError> {
        Ok(Request::from_route(&Route::GetMessages {
            after: self.after.map(Id::get),
            around: self.around.map(Id::get),
            before: self.before.map(Id::get),
            channel_id: self.channel_id.get(),
            limit: self.fields.limit,
        }))
    }
}
