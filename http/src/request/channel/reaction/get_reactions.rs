use super::RequestReactionType;
use crate::{
    client::Client,
    error::Error as HttpError,
    request::{Request, TryIntoRequest},
    response::{marker::ListBody, ResponseFuture},
    routing::Route,
};
use twilight_model::{
    id::{
        marker::{ChannelMarker, MessageMarker, UserMarker},
        Id,
    },
    user::User,
};
use twilight_validate::request::{
    get_reactions_limit as validate_get_reactions_limit, ValidationError,
};

struct GetReactionsFields {
    after: Option<Id<UserMarker>>,
    limit: Option<u64>,
}

/// Get a list of users that reacted to a message with an `emoji`.
///
/// This endpoint is limited to 100 users maximum, so if a message has more than 100 reactions,
/// requests must be chained until all reactions are retrieved.
#[must_use = "requests must be configured and executed"]
pub struct GetReactions<'a> {
    channel_id: Id<ChannelMarker>,
    emoji: &'a RequestReactionType<'a>,
    fields: GetReactionsFields,
    http: &'a Client,
    message_id: Id<MessageMarker>,
}

impl<'a> GetReactions<'a> {
    pub(crate) const fn new(
        http: &'a Client,
        channel_id: Id<ChannelMarker>,
        message_id: Id<MessageMarker>,
        emoji: &'a RequestReactionType<'a>,
    ) -> Self {
        Self {
            channel_id,
            emoji,
            fields: GetReactionsFields {
                after: None,
                limit: None,
            },
            http,
            message_id,
        }
    }

    /// Get users after this id.
    pub const fn after(mut self, after: Id<UserMarker>) -> Self {
        self.fields.after = Some(after);

        self
    }

    /// Set the maximum number of users to retrieve.
    ///
    /// The minimum is 1 and the maximum is 100. If no limit is specified, Discord sets the default
    /// to 25.
    ///
    /// # Errors
    ///
    /// Returns an error of type [`GetReactions`] if the amount is greater than
    /// 100.
    ///
    /// [`GetReactions`]: twilight_validate::request::ValidationErrorType::GetReactions
    pub const fn limit(mut self, limit: u64) -> Result<Self, ValidationError> {
        if let Err(source) = validate_get_reactions_limit(limit) {
            return Err(source);
        }

        self.fields.limit = Some(limit);

        Ok(self)
    }

    /// Execute the request, returning a future resolving to a [`Response`].
    ///
    /// [`Response`]: crate::response::Response
    pub fn exec(self) -> ResponseFuture<ListBody<User>> {
        let http = self.http;

        match self.try_into_request() {
            Ok(request) => http.request(request),
            Err(source) => ResponseFuture::error(source),
        }
    }
}

impl TryIntoRequest for GetReactions<'_> {
    fn try_into_request(self) -> Result<Request, HttpError> {
        Ok(Request::from_route(&Route::GetReactionUsers {
            after: self.fields.after.map(Id::get),
            channel_id: self.channel_id.get(),
            emoji: self.emoji,
            limit: self.fields.limit,
            message_id: self.message_id.get(),
        }))
    }
}
