use super::RequestReactionType;
use crate::{
    client::Client,
    error::Error,
    request::{Request, TryIntoRequest},
    response::{Response, ResponseFuture, marker::ListBody},
    routing::Route,
};
use std::future::IntoFuture;
use twilight_model::{
    channel::message::ReactionType,
    id::{
        Id,
        marker::{ChannelMarker, MessageMarker, UserMarker},
    },
    user::User,
};
use twilight_validate::request::{
    ValidationError, get_reactions_limit as validate_get_reactions_limit,
};

struct GetReactionsPath<'a> {
    after: Option<Id<UserMarker>>,
    channel_id: Id<ChannelMarker>,
    emoji: &'a RequestReactionType<'a>,
    kind: Option<ReactionType>,
    limit: Option<u16>,
    message_id: Id<MessageMarker>,
}

/// Get a list of users that reacted to a message with an `emoji`.
///
/// This endpoint is limited to 100 users maximum, so if a message has more than 100 reactions,
/// requests must be chained until all reactions are retrieved.
#[must_use = "requests must be configured and executed"]
pub struct GetReactions<'a> {
    http: &'a Client,
    path: Result<GetReactionsPath<'a>, ValidationError>,
}

impl<'a> GetReactions<'a> {
    pub(crate) const fn new(
        http: &'a Client,
        channel_id: Id<ChannelMarker>,
        message_id: Id<MessageMarker>,
        emoji: &'a RequestReactionType<'a>,
    ) -> Self {
        Self {
            http,
            path: Ok(GetReactionsPath {
                after: None,
                channel_id,
                emoji,
                kind: None,
                limit: None,
                message_id,
            }),
        }
    }

    /// Get users after this id.
    pub const fn after(mut self, after: Id<UserMarker>) -> Self {
        if let Ok(path) = self.path.as_mut() {
            path.after = Some(after);
        }

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
    pub fn limit(mut self, limit: u16) -> Self {
        self.path = self.path.and_then(|mut path| {
            validate_get_reactions_limit(limit)?;
            path.limit = Some(limit);

            Ok(path)
        });

        self
    }

    /// Set the kind of reaction to retrieve.
    ///
    /// This can be either a super reaction or a normal reaction.
    pub const fn kind(mut self, kind: ReactionType) -> Self {
        if let Ok(path) = self.path.as_mut() {
            path.kind = Some(kind);
        }

        self
    }
}

impl IntoFuture for GetReactions<'_> {
    type Output = Result<Response<ListBody<User>>, Error>;

    type IntoFuture = ResponseFuture<ListBody<User>>;

    fn into_future(self) -> Self::IntoFuture {
        let http = self.http;

        match self.try_into_request() {
            Ok(request) => http.request(request),
            Err(source) => ResponseFuture::error(source),
        }
    }
}

impl TryIntoRequest for GetReactions<'_> {
    fn try_into_request(self) -> Result<Request, Error> {
        let path = self.path.map_err(Error::validation)?;

        Ok(Request::from_route(&Route::GetReactionUsers {
            after: path.after.map(Id::get),
            channel_id: path.channel_id.get(),
            emoji: path.emoji,
            limit: path.limit,
            message_id: path.message_id.get(),
            kind: path.kind.map(Into::into),
        }))
    }
}
