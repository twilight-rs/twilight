use super::get_channel_messages::GetChannelMessagesPath;
use crate::{
    client::Client,
    error::Error,
    request::{Request, TryIntoRequest},
    response::{Response, ResponseFuture, marker::ListBody},
    routing::Route,
};
use std::future::IntoFuture;
use twilight_model::{
    channel::Message,
    id::{
        Id,
        marker::{ChannelMarker, MessageMarker},
    },
};
use twilight_validate::request::{
    ValidationError, get_channel_messages_limit as validate_get_channel_messages_limit,
};

struct GetChannelMessagesConfiguredPath {
    after: Option<Id<MessageMarker>>,
    around: Option<Id<MessageMarker>>,
    before: Option<Id<MessageMarker>>,
    channel_id: Id<ChannelMarker>,
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
    http: &'a Client,
    path: Result<GetChannelMessagesConfiguredPath, ValidationError>,
}

impl<'a> GetChannelMessagesConfigured<'a> {
    pub(super) fn new(
        http: &'a Client,
        base_path: Result<GetChannelMessagesPath, ValidationError>,
        after: Option<Id<MessageMarker>>,
        around: Option<Id<MessageMarker>>,
        before: Option<Id<MessageMarker>>,
    ) -> Self {
        Self {
            http,
            path: base_path.map(|base_path| GetChannelMessagesConfiguredPath {
                after,
                around,
                before,
                channel_id: base_path.channel_id,
                limit: base_path.limit,
            }),
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
    pub fn limit(mut self, limit: u16) -> Self {
        self.path = self.path.and_then(|mut path| {
            validate_get_channel_messages_limit(limit)?;
            path.limit = Some(limit);

            Ok(path)
        });

        self
    }
}

impl IntoFuture for GetChannelMessagesConfigured<'_> {
    type Output = Result<Response<ListBody<Message>>, Error>;

    type IntoFuture = ResponseFuture<ListBody<Message>>;

    fn into_future(self) -> Self::IntoFuture {
        let http = self.http;

        match self.try_into_request() {
            Ok(request) => http.request(request),
            Err(source) => ResponseFuture::error(source),
        }
    }
}

impl TryIntoRequest for GetChannelMessagesConfigured<'_> {
    fn try_into_request(self) -> Result<Request, Error> {
        let path = self.path.map_err(Error::validation)?;

        Ok(Request::from_route(&Route::GetMessages {
            after: path.after.map(Id::get),
            around: path.around.map(Id::get),
            before: path.before.map(Id::get),
            channel_id: path.channel_id.get(),
            limit: path.limit,
        }))
    }
}
