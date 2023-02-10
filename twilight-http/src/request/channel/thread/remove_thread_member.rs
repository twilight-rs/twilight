use crate::{
    client::Client,
    error::Error,
    request::{Request, TryIntoRequest},
    response::{marker::EmptyBody, Response, ResponseFuture},
    routing::Route,
};
use std::future::IntoFuture;
use twilight_model::id::{
    marker::{ChannelMarker, UserMarker},
    Id,
};

/// Remove another member from a thread.
///
/// Requires that the thread is not archived.
///
/// Requires the [`MANAGE_THREADS`] permission, unless both the thread is a
/// [`PrivateThread`], and the current user is the creator of the thread.
///
/// [`PrivateThread`]: twilight_model::channel::ChannelType::PrivateThread
/// [`MANAGE_THREADS`]: twilight_model::guild::Permissions::MANAGE_THREADS
#[must_use = "requests must be configured and executed"]
pub struct RemoveThreadMember<'a> {
    channel_id: Id<ChannelMarker>,
    http: &'a Client,
    user_id: Id<UserMarker>,
}

impl<'a> RemoveThreadMember<'a> {
    pub(crate) const fn new(
        http: &'a Client,
        channel_id: Id<ChannelMarker>,
        user_id: Id<UserMarker>,
    ) -> Self {
        Self {
            channel_id,
            http,
            user_id,
        }
    }
}

impl IntoFuture for RemoveThreadMember<'_> {
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

impl TryIntoRequest for RemoveThreadMember<'_> {
    fn try_into_request(self) -> Result<Request, Error> {
        Ok(Request::from_route(&Route::RemoveThreadMember {
            channel_id: self.channel_id.get(),
            user_id: self.user_id.get(),
        }))
    }
}
