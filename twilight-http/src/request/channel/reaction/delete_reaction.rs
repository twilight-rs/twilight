use super::RequestReactionType;
use crate::{
    client::Client,
    error::Error,
    request::{Request, TryIntoRequest},
    response::{marker::EmptyBody, Response, ResponseFuture},
    routing::Route,
};
use std::future::IntoFuture;
use twilight_model::id::{
    marker::{ChannelMarker, MessageMarker, UserMarker},
    Id,
};

/// User to delete the reaction of.
pub(crate) enum TargetUser {
    /// Delete a reaction of the current user.
    Current,
    /// Delete a reaction from a user by their ID.
    Id(Id<UserMarker>),
}

/// Delete one reaction by a user on a message.
#[must_use = "requests must be configured and executed"]
pub struct DeleteReaction<'a> {
    channel_id: Id<ChannelMarker>,
    emoji: &'a RequestReactionType<'a>,
    http: &'a Client,
    message_id: Id<MessageMarker>,
    target_user: TargetUser,
}

impl<'a> DeleteReaction<'a> {
    pub(crate) const fn new(
        http: &'a Client,
        channel_id: Id<ChannelMarker>,
        message_id: Id<MessageMarker>,
        emoji: &'a RequestReactionType<'a>,
        target_user: TargetUser,
    ) -> Self {
        Self {
            channel_id,
            emoji,
            http,
            message_id,
            target_user,
        }
    }
}

impl IntoFuture for DeleteReaction<'_> {
    type Output = Result<Response<EmptyBody>, Error>;

    type IntoFuture = ResponseFuture<EmptyBody>;

    fn into_future(self) -> Self::IntoFuture {
        let http = self.http;

        match self.try_into_request() {
            Ok(request) => http.request(request),
            Err(source) => ResponseFuture::error(source),
        }
    }
}

impl TryIntoRequest for DeleteReaction<'_> {
    fn try_into_request(self) -> Result<Request, Error> {
        let route = match self.target_user {
            TargetUser::Current => Route::DeleteReactionCurrentUser {
                channel_id: self.channel_id.get(),
                emoji: self.emoji,
                message_id: self.message_id.get(),
            },
            TargetUser::Id(user_id) => Route::DeleteReaction {
                channel_id: self.channel_id.get(),
                emoji: self.emoji,
                message_id: self.message_id.get(),
                user_id: user_id.get(),
            },
        };

        Ok(Request::from_route(&route))
    }
}
