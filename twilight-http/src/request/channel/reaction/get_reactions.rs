use super::RequestReactionType;
use crate::{
    client::Client,
    error::Error,
    request::{Request, TryIntoRequest},
    response::{marker::ListBody, Response, ResponseFuture},
    routing::Route,
};
use std::future::IntoFuture;
use twilight_model::{
    channel::message::ReactionType,
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
    limit: Option<u16>,
    kind: Option<ReactionType>,
}

/// Get a list of users that reacted to a message with an `emoji`.
///
/// This endpoint is limited to 100 users maximum, so if a message has more than 100 reactions,
/// requests must be chained until all reactions are retrieved.
#[must_use = "requests must be configured and executed"]
pub struct GetReactions<'a> {
    channel_id: Id<ChannelMarker>,
    emoji: &'a RequestReactionType<'a>,
    fields: Result<GetReactionsFields, ValidationError>,
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
            fields: Ok(GetReactionsFields {
                after: None,
                limit: None,
                kind: None,
            }),
            http,
            message_id,
        }
    }

    /// Get users after this id.
    pub fn after(mut self, after: Id<UserMarker>) -> Self {
        if let Ok(fields) = self.fields.as_mut() {
            fields.after = Some(after);
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
        self.fields = self.fields.and_then(|mut fields| {
            validate_get_reactions_limit(limit)?;
            fields.limit = Some(limit);

            Ok(fields)
        });

        self
    }

    /// Set the kind of reaction to retrieve.
    ///
    /// This can be either a super reaction or a normal reaction.
    pub fn kind(mut self, kind: ReactionType) -> Self {
        if let Ok(fields) = self.fields.as_mut() {
            fields.kind = Some(kind);
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
        let fields = self.fields.map_err(Error::validation)?;

        Ok(Request::from_route(&Route::GetReactionUsers {
            after: fields.after.map(Id::get),
            channel_id: self.channel_id.get(),
            emoji: self.emoji,
            limit: fields.limit,
            message_id: self.message_id.get(),
            kind: fields.kind.map(Into::into),
        }))
    }
}
