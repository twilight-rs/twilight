use super::RequestReactionType;
use crate::{
    client::Client,
    error::Error,
    request::{Method, Request, TryIntoRequest, path::Path},
    response::{Response, ResponseFuture, marker::EmptyBody},
};
use std::future::IntoFuture;
use twilight_model::id::{
    Id,
    marker::{ChannelMarker, MessageMarker, UserMarker},
};

/// User to delete the reaction of.
pub(crate) enum TargetUser {
    /// Delete a reaction of the current user.
    Current,
    /// Delete a reaction from a user by their ID.
    Id(Id<UserMarker>),
}

struct DeleteReactionPath<'a> {
    channel_id: Id<ChannelMarker>,
    emoji: &'a RequestReactionType<'a>,
    message_id: Id<MessageMarker>,
    target_user: TargetUser,
}

/// Delete one reaction by a user on a message.
#[must_use = "requests must be configured and executed"]
pub struct DeleteReaction<'a> {
    http: &'a Client,
    path: DeleteReactionPath<'a>,
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
            http,
            path: DeleteReactionPath {
                channel_id,
                emoji,
                message_id,
                target_user,
            },
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
        let DeleteReactionPath {
            channel_id,
            emoji,
            message_id,
            target_user,
        } = self.path;

        let path = match target_user {
            TargetUser::Current => Path::builder()
                .resource("channels")
                .id(channel_id)
                .resource("messages")
                .id(message_id)
                .resource("reactions")
                .id(emoji)
                .no_resource()
                .me()
                .build()
                .map_err(Error::path)?,
            TargetUser::Id(user_id) => Path::builder()
                .resource("channels")
                .id(channel_id)
                .resource("messages")
                .id(message_id)
                .resource("reactions")
                .id(emoji)
                .no_resource()
                .id(user_id)
                .build()
                .map_err(Error::path)?,
        };

        Ok(Request::new(Method::Delete, path))
    }
}
